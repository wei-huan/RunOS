use super::{
    address::{PhysAddr, VirtAddr, VirtPageNum},
    page_table::{PTEFlags, PageTable, PageTableEntry},
    section::{MapPermission, MapType, Section},
};
use crate::config::{
    DLL_LOADER_BASE, MEMORY_END, PAGE_SIZE, SIGRETURN_TRAMPOLINE, TRAMPOLINE, TRAP_CONTEXT_BASE,
    USER_STACK_BASE,
};
use crate::platform::MMIO;
use crate::task::{
    AuxHeader, AT_BASE, AT_CLKTCK, AT_EGID, AT_ENTRY, AT_EUID, AT_FLAGS, AT_GID, AT_HWCAP,
    AT_NOTELF, AT_PAGESZ, AT_PHDR, AT_PHENT, AT_PHNUM, AT_PLATFORM, AT_SECURE, AT_UID,
};
use alloc::vec::Vec;
use alloc::{
    string::{String, ToString},
    sync::Arc,
};
use core::arch::asm;
use lazy_static::lazy_static;
use riscv::register::satp;
use spin::Mutex;

extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss_with_stack();
    fn ebss();
    fn ekernel();
    fn strampoline();
    fn ssignaltrampoline();
}

lazy_static! {
    pub static ref KERNEL_SPACE: Mutex<AddrSpace> = Mutex::new(AddrSpace::create_kernel_space());
}

pub fn kernel_token() -> usize {
    KERNEL_SPACE.lock().token()
}

#[allow(unused)]
pub fn kernel_translate(vpn: VirtPageNum) -> Option<PageTableEntry> {
    KERNEL_SPACE.lock().page_table.translate(vpn)
}

pub struct AddrSpace {
    pub page_table: PageTable,
    sections: Vec<Section>,
    pub mmap_sections: Vec<Section>,
}

impl AddrSpace {
    pub fn new_empty() -> Self {
        Self {
            page_table: PageTable::new(),
            sections: Vec::new(),
            mmap_sections: Vec::new(),
        }
    }
    pub fn token(&self) -> usize {
        self.page_table.token()
    }
    /// Assume that no conflicts.
    pub fn insert_framed_area(
        &mut self,
        name: String,
        start_va: VirtAddr,
        end_va: VirtAddr,
        permission: MapPermission,
    ) {
        self.push_section(
            Section::new(name, start_va, end_va, MapType::Framed, permission),
            None,
        );
    }
    /// Assume that no conflicts.
    pub fn insert_mmap_area(
        &mut self,
        name: String,
        start_va: VirtAddr,
        end_va: VirtAddr,
        permission: MapPermission,
    ) {
        self.push_mmap_section(
            Section::new(name, start_va, end_va, MapType::Framed, permission),
            None,
        );
    }
    pub fn remove_area_with_start_vpn(&mut self, start_vpn: VirtPageNum) {
        if let Some((idx, area)) = self
            .sections
            .iter_mut()
            .enumerate()
            .find(|(_, area)| area.vpn_range.get_start() == start_vpn)
        {
            area.unmap(&mut self.page_table);
            self.sections.remove(idx);
        }
    }
    #[allow(unused)]
    pub fn remove_area_with_name(&mut self, name: &str) {
        if let Some((idx, area)) = self
            .sections
            .iter_mut()
            .enumerate()
            .find(|(_, area)| area.name == name)
        {
            area.unmap(&mut self.page_table);
            self.sections.remove(idx);
        }
    }
    pub fn remove_mmap_area_with_start_vpn(&mut self, start_vpn: VirtPageNum) {
        if let Some((idx, area)) = self
            .mmap_sections
            .iter_mut()
            .enumerate()
            .find(|(_, area)| area.vpn_range.get_start() == start_vpn)
        {
            area.unmap(&mut self.page_table);
            self.mmap_sections.remove(idx);
        }
    }
    pub fn push_section(&mut self, mut section: Section, data: Option<&[u8]>) {
        log::trace!(
            "section range start: {:#?}, end: {:#?}",
            section.vpn_range.get_start(),
            section.vpn_range.get_end()
        );
        section.map(&mut self.page_table);
        if let Some(data) = data {
            section.copy_data(&mut self.page_table, data, 0);
        }
        self.sections.push(section);
    }
    pub fn push_mmap_section(&mut self, mut section: Section, data: Option<&[u8]>) {
        section.map(&mut self.page_table);
        if let Some(data) = data {
            section.copy_data(&mut self.page_table, data, 0);
        }
        self.mmap_sections.push(section);
    }
    pub fn is_section_conflict(&self, start: usize, len: usize) -> bool {
        let left_vpn = VirtAddr::from(start).floor();
        let right_vpn = VirtAddr::from(start + len).ceil();
        if let Some(_) = self.sections.iter().find(|section| {
            section.vpn_range.is_left_cover(left_vpn, right_vpn)
                || section.vpn_range.is_right_cover(left_vpn, right_vpn)
                || section.vpn_range.is_full_cover(left_vpn, right_vpn)
                || section.vpn_range.is_be_covered(left_vpn, right_vpn)
        }) {
            return true;
        }
        return false;
    }
    pub fn is_mmap_section_conflict(&self, start: usize, len: usize) -> bool {
        let left_vpn = VirtAddr::from(start).floor();
        let right_vpn = VirtAddr::from(start + len).ceil();
        if let Some(_) = self.mmap_sections.iter().find(|section| {
            section.vpn_range.is_left_cover(left_vpn, right_vpn)
                || section.vpn_range.is_right_cover(left_vpn, right_vpn)
                || section.vpn_range.is_full_cover(left_vpn, right_vpn)
                || section.vpn_range.is_be_covered(left_vpn, right_vpn)
        }) {
            return true;
        }
        return false;
    }
    pub fn fix_mmap_section_conflict(&mut self, start: usize, len: usize) {
        let left_vpn = VirtAddr::from(start).floor();
        let right_vpn = VirtAddr::from(start + len).ceil();
        log::trace!(
            "fix_mmap_section_conflict left: {:?}, right: {:?}",
            left_vpn,
            right_vpn
        );
        let mut need_remove: Vec<usize> = Vec::new();
        let mut new_mmap_sections: Vec<Section> = Vec::new();
        for (index, section) in self.mmap_sections.iter_mut().enumerate() {
            log::trace!(
                "fix_mmap_section_conflict mmap_section left: {:?}, right: {:?}",
                section.vpn_range.get_start(),
                section.vpn_range.get_end()
            );
            // delete section
            if section.vpn_range.is_be_covered(left_vpn, right_vpn) {
                need_remove.push(index);
            }
            // truncate section right part
            else if section.vpn_range.is_left_cover(left_vpn, right_vpn) {
                section.modify_section_end(&mut self.page_table, left_vpn);
            }
            // truncate section left part
            else if section.vpn_range.is_right_cover(left_vpn, right_vpn) {
                section.modify_section_start(&mut self.page_table, right_vpn);
            }
            // full_cover divide section to two parts
            else if section.vpn_range.is_be_covered(left_vpn, right_vpn) {
                let (new_left, new_right) =
                    section.divide_into_two(&mut self.page_table, left_vpn, right_vpn);
                need_remove.push(index);
                new_mmap_sections.push(new_left);
                new_mmap_sections.push(new_right);
            } else {
                // no conflict
            }
        }
        for i in need_remove {
            self.mmap_sections.remove(i);
        }
        self.mmap_sections.append(&mut new_mmap_sections);
    }
    fn push_section_with_offset(
        &mut self,
        mut section: Section,
        offset: usize,
        data: Option<&[u8]>,
    ) {
        section.map(&mut self.page_table);
        if let Some(data) = data {
            section.copy_data(&mut self.page_table, data, offset);
        }
        self.sections.push(section);
    }
    /// Mention that trampoline is not collected by areas.
    fn map_trampoline(&mut self) {
        self.page_table.map(
            VirtAddr::from(TRAMPOLINE).into(),
            PhysAddr::from(strampoline as usize).into(),
            PTEFlags::R | PTEFlags::X,
        )
    }
    fn map_signal_trampoline(&mut self) {
        self.page_table.map(
            VirtAddr::from(SIGRETURN_TRAMPOLINE).into(),
            PhysAddr::from(ssignaltrampoline as usize).into(),
            PTEFlags::R | PTEFlags::X | PTEFlags::U,
        );
    }
    pub fn create_kernel_space() -> Self {
        let mut kernel_space = Self::new_empty();
        // map trampoline
        kernel_space.map_trampoline();
        // map kernel sections
        // println!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
        // println!(
        //     ".trampoline [{:#x}, {:#x})",
        //     strampoline as usize, etrampoline as usize
        // );
        // println!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
        // println!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
        // println!(
        //     ".bss [{:#x}, {:#x})",
        //     sbss_with_stack as usize, ebss as usize
        // );
        // println!("mapping .text section");
        kernel_space.push_section(
            Section::new(
                ".text".to_string(),
                (stext as usize).into(),
                (etext as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::X,
            ),
            None,
        );
        // println!("mapping .rodata section");
        kernel_space.push_section(
            Section::new(
                ".rodata".to_string(),
                (srodata as usize).into(),
                (erodata as usize).into(),
                MapType::Identical,
                MapPermission::R,
            ),
            None,
        );
        // println!("mapping .data section");
        kernel_space.push_section(
            Section::new(
                ".data".to_string(),
                (sdata as usize).into(),
                (edata as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );
        // println!("mapping .bss section");
        kernel_space.push_section(
            Section::new(
                ".bss".to_string(),
                (sbss_with_stack as usize).into(),
                (ebss as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );
        // println!("mapping physical memory");
        kernel_space.push_section(
            Section::new(
                ".phys_mm".to_string(),
                (ekernel as usize).into(),
                MEMORY_END.into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );
        // println!("mapping memory-mapped registers");
        for pair in MMIO {
            kernel_space.push_section(
                Section::new(
                    ".MMIO".to_string(),
                    (*pair).0.into(),
                    ((*pair).0 + (*pair).1).into(),
                    MapType::Identical,
                    MapPermission::R | MapPermission::W,
                ),
                None,
            );
        }
        // unsafe { asm!("fence.i") }
        // println!("mapping kernel finish");
        kernel_space
    }
    // /// load dynamic link library loader
    // pub fn load_dll_loader(&mut self) -> usize {
    //     if let Some(app_vfile) = open("/", "libc.so", OpenFlags::RDONLY) {
    //         let all_data = app_vfile.read_all();
    //         let elf_data = all_data.as_slice();
    //         let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
    //         let elf_header = elf.header;
    //         let magic = elf_header.pt1.magic;
    //         assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");
    //         let ph_count = elf_header.pt2.ph_count();
    //         for i in 0..ph_count {
    //             let ph = elf.program_header(i).unwrap();
    //             let start_va: VirtAddr = (DLL_LOADER_BASE + ph.virtual_addr() as usize).into();
    //             let end_va: VirtAddr =
    //                 (DLL_LOADER_BASE + ph.virtual_addr() as usize + ph.mem_size() as usize).into();
    //             let offset = start_va.page_offset();
    //             if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
    //                 let sect = elf.section_header((i + 1).try_into().unwrap()).unwrap();
    //                 let name: String =
    //                     "dll_".to_string() + &sect.get_name(&elf).unwrap().to_string();
    //                 let mut map_perm = MapPermission::U;
    //                 let ph_flags = ph.flags();
    //                 if ph_flags.is_read() {
    //                     map_perm |= MapPermission::R;
    //                 }
    //                 if ph_flags.is_write() {
    //                     map_perm |= MapPermission::W;
    //                 }
    //                 if ph_flags.is_execute() {
    //                     map_perm |= MapPermission::X;
    //                 }
    //                 let section = Section::new(name, start_va, end_va, MapType::Framed, map_perm);
    //                 self.push_section_with_offset(
    //                     section,
    //                     offset,
    //                     Some(
    //                         &elf.input
    //                             [ph.offset() as usize..(ph.offset() + ph.file_size()) as usize],
    //                     ),
    //                 );
    //             }
    //         }
    //         return elf_header.pt2.entry_point() as usize;
    //     } else {
    //         log::error!("can't find dll loader libc.so");
    //         return 0;
    //     }
    // }
    /// Include sections in elf and trampoline and TrapContext and user stack,
    /// also returns user_sp and entry point.
    pub fn create_user_space(elf_data: &[u8]) -> (Self, usize, usize, usize, Vec<AuxHeader>) {
        // log::debug!("create_user_space");
        let mut auxv: Vec<AuxHeader> = Vec::new();
        let mut user_space = Self::new_empty();
        // map trampoline
        user_space.map_signal_trampoline();
        user_space.map_trampoline();
        // map program headers of elf, with U flag
        let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
        let elf_header = elf.header;
        let magic = elf_header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");
        let ph_count = elf_header.pt2.ph_count();
        let mut max_end_vpn = VirtPageNum(0);

        auxv.push(AuxHeader {
            aux_type: AT_PHENT,
            value: elf.header.pt2.ph_entry_size() as usize,
        }); // ELF64 header 64bytes
        auxv.push(AuxHeader {
            aux_type: AT_PHNUM,
            value: ph_count as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_PAGESZ,
            value: PAGE_SIZE as usize,
        });

        let mut head_va = 0;
        let mut at_base = 0;
        for i in 0..ph_count {
            let ph = elf.program_header(i).unwrap();
            let sect = elf.section_header((i + 1).try_into().unwrap()).unwrap();
            let name = sect.get_name(&elf).unwrap();
            log::trace!(
                "program header name: {:#?} type: {:#?}, vaddr: [{:#X?}, {:#X?})",
                name,
                ph.get_type().unwrap(),
                ph.virtual_addr(),
                ph.virtual_addr() + ph.mem_size()
            );
            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                // first section header is dummy, not match program header, so set i + 1
                let sect = elf.section_header((i + 1).try_into().unwrap()).unwrap();
                let name = sect.get_name(&elf).unwrap();
                let start_va: VirtAddr = (ph.virtual_addr() as usize).into();
                let end_va: VirtAddr = ((ph.virtual_addr() + ph.mem_size()) as usize).into();
                let offset = start_va.page_offset();
                let mut map_perm = MapPermission::U;
                let ph_flags = ph.flags();
                if ph_flags.is_read() {
                    map_perm |= MapPermission::R;
                }
                if ph_flags.is_write() {
                    map_perm |= MapPermission::W;
                }
                if ph_flags.is_execute() {
                    map_perm |= MapPermission::X;
                }
                // log::debug!("program header map_perm: {:#?}", map_perm);
                let section = Section::new(
                    name.to_string(),
                    start_va,
                    end_va,
                    MapType::Framed,
                    map_perm,
                );
                max_end_vpn = section.vpn_range.get_end();
                user_space.push_section_with_offset(
                    section,
                    offset,
                    Some(&elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize]),
                );
                if head_va == 0 {
                    head_va = start_va.0;
                }
            }
            // load dll
            // else if ph.get_type().unwrap() == xmas_elf::program::Type::Interp {
            //     at_base = user_space.load_dll_loader();
            //     // log::debug!("Have Interp, need dll {:#X?}", at_base);
            //     if at_base != 0 {
            //         at_base += DLL_LOADER_BASE;
            //     } else {
            //         log::error!("dynamic linker error !");
            //     }
            // }
            // else do nothing
        }

        if at_base != 0 {
            auxv.push(AuxHeader {
                aux_type: AT_BASE,
                value: DLL_LOADER_BASE as usize,
            });
        } else {
            auxv.push(AuxHeader {
                aux_type: AT_BASE,
                value: 0 as usize,
            });
        }
        auxv.push(AuxHeader {
            aux_type: AT_FLAGS,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_ENTRY,
            value: elf.header.pt2.entry_point() as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_UID,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_EUID,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_GID,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_EGID,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_PLATFORM,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_HWCAP,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_CLKTCK,
            value: 100 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_SECURE,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_NOTELF,
            value: 0x112d as usize,
        });

        let ph_head_addr = head_va + elf.header.pt2.ph_offset() as usize;
        auxv.push(AuxHeader {
            aux_type: AT_PHDR,
            value: ph_head_addr as usize,
        });

        // clear bss section
        // user_space.clear_bss_pages();

        let heap_start_virt: VirtAddr = max_end_vpn.into();
        let mut heap_start: usize = heap_start_virt.into();
        heap_start += PAGE_SIZE;

        // // map user stack with U flags
        let user_stack_high = USER_STACK_BASE;
        // let user_stack_bottom = user_stack_high - USER_STACK_SIZE;
        // // println!("user_stack_bottom: 0x{:X}", usize::from(user_stack_bottom));
        // // println!("user_stack_high: 0x{:X}", usize::from(user_stack_high));
        // user_space.push_section(
        //     Section::new(
        //         ".ustack".to_string(),
        //         user_stack_bottom.into(),
        //         user_stack_high.into(),
        //         MapType::Framed,
        //         MapPermission::R | MapPermission::W | MapPermission::U,
        //     ),
        //     None,
        // );

        // map TrapContext
        user_space.push_section(
            Section::new(
                ".trap_cx".to_string(),
                TRAP_CONTEXT_BASE.into(),
                SIGRETURN_TRAMPOLINE.into(),
                MapType::Framed,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );
        let entry;
        // 静态链接程序
        if at_base == 0 {
            entry = elf.header.pt2.entry_point() as usize;
        } else {
            entry = at_base;
        }
        // log::debug!("entry: {:#X}", entry);
        (user_space, heap_start, user_stack_high, entry, auxv)
    }
    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.page_table.translate(vpn)
    }
    #[allow(unused)]
    pub fn clear_bss_pages(&mut self) {
        let sect_iterator = self.sections.iter_mut();
        for sect in sect_iterator {
            if sect.name == ".bss" {
                // println!("clear bss");
                sect.clear(&mut self.page_table);
            }
        }
    }
    // pub fn from_existed_user(user_space: &AddrSpace) -> AddrSpace {
    //     let mut addr_space = Self::new_empty();
    //     // map trampoline
    //     addr_space.map_trampoline();
    //     // copy data sections/trap_context/user_stack/heap
    //     for area in user_space.sections.iter() {
    //         let new_area = Section::from_another(area);
    //         addr_space.push_section(new_area, None);
    //         // copy data from another space
    //         for vpn in area.vpn_range {
    //             let src_ppn = user_space.translate(vpn).unwrap().ppn();
    //             let dst_ppn = addr_space.translate(vpn).unwrap().ppn();
    //             dst_ppn
    //                 .get_bytes_array()
    //                 .copy_from_slice(src_ppn.get_bytes_array());
    //         }
    //     }
    //     // copy mmap_sections
    //     for area in user_space.mmap_sections.iter() {
    //         let new_area = Section::from_another(area);
    //         addr_space.push_mmap_section(new_area, None);
    //         // copy data from another space
    //         for vpn in area.vpn_range {
    //             let src_ppn = user_space.translate(vpn).unwrap().ppn();
    //             let dst_ppn = addr_space.translate(vpn).unwrap().ppn();
    //             dst_ppn
    //                 .get_bytes_array()
    //                 .copy_from_slice(src_ppn.get_bytes_array());
    //         }
    //     }
    //     addr_space
    // }
    /* share .text section in lmbench */
    pub fn from_existed_user(user_space: &mut AddrSpace) -> AddrSpace {
        let mut addr_space = Self::new_empty();
        // map trampoline
        addr_space.map_signal_trampoline();
        addr_space.map_trampoline();
        // share map .text sections/user_stack/heap
        for area in user_space
            .sections
            .iter()
            .filter(|sect| !(sect.perm.contains(MapPermission::W)))
        {
            let mut new_area = (*area).clone();
            new_area
                .share_map(&mut addr_space.page_table, &mut user_space.page_table)
                .unwrap();
            addr_space.sections.push(new_area);
        }
        // copy data sections/trap_context/user_stack/heap
        for area in user_space
            .sections
            .iter()
            .filter(|sect| (sect.perm.contains(MapPermission::W)))
        {
            let new_area = Section::from_another(area);
            addr_space.push_section(new_area, None);
            // copy data from another space
            for vpn in area.vpn_range {
                let src_ppn = user_space.translate(vpn).unwrap().ppn();
                let dst_ppn = addr_space.translate(vpn).unwrap().ppn();
                dst_ppn
                    .get_bytes_array()
                    .copy_from_slice(src_ppn.get_bytes_array());
            }
        }
        // copy mmap_sections
        for area in user_space.mmap_sections.iter() {
            let new_area = Section::from_another(area);
            addr_space.push_mmap_section(new_area, None);
            // copy data from another space
            for vpn in area.vpn_range {
                let src_ppn = user_space.translate(vpn).unwrap().ppn();
                let dst_ppn = addr_space.translate(vpn).unwrap().ppn();
                dst_ppn
                    .get_bytes_array()
                    .copy_from_slice(src_ppn.get_bytes_array());
            }
        }
        addr_space
    }
    /* copy on write */
    // pub fn from_existed_user(user_space: &mut AddrSpace) -> AddrSpace {
    //     let mut addr_space = Self::new_empty();
    //     // map trampoline
    //     addr_space.map_trampoline();
    //     // share map data sections/user_stack/heap
    //     for area in user_space
    //         .sections
    //         .iter()
    //         .filter(|sect| sect.name != ".trap_cx")
    //     {
    //         let mut new_area = (*area).clone();
    //         new_area
    //             .share_map(&mut addr_space.page_table, &mut user_space.page_table)
    //             .unwrap();
    //         addr_space.sections.push(new_area);
    //     }
    //     // share map mmap_sections
    //     for area in user_space.mmap_sections.iter() {
    //         let mut new_area = (*area).clone();
    //         new_area
    //             .share_map(&mut addr_space.page_table, &mut user_space.page_table)
    //             .unwrap();
    //         addr_space.mmap_sections.push(new_area);
    //     }
    //     // copy trap_cx section
    //     if let Some(area) = user_space
    //         .sections
    //         .iter()
    //         .find(|sect| sect.name == ".trap_cx")
    //     {
    //         let new_area = Section::from_another(area);
    //         addr_space.push_section(new_area, None);
    //         // copy data from another space
    //         for vpn in area.vpn_range {
    //             let src_ppn = user_space.translate(vpn).unwrap().ppn();
    //             let dst_ppn = addr_space.translate(vpn).unwrap().ppn();
    //             dst_ppn
    //                 .get_bytes_array()
    //                 .copy_from_slice(src_ppn.get_bytes_array());
    //         }
    //     }
    //     addr_space
    // }
    pub fn recycle_data_pages(&mut self) {
        self.sections.clear();
    }
    pub fn activate(&mut self) {
        let satp = self.page_table.token();
        unsafe {
            satp::write(satp);
            asm!("sfence.vma");
        }
    }
    // size 最终会按页对齐
    pub fn alloc_heap_section(&mut self, heap_start: usize, size: usize) {
        // heap_start 本身在task创建时已经按页对齐了
        let start_va = heap_start.into();
        let end_va = (heap_start + size).into();
        self.insert_framed_area(
            ".heap".to_string(),
            start_va,
            end_va,
            MapPermission::R | MapPermission::W | MapPermission::U,
        )
    }
    // size 最终会按页对齐
    pub fn dealloc_heap_section(&mut self) {
        if let Some((idx, area)) = self
            .sections
            .iter_mut()
            .enumerate()
            .find(|(_, area)| area.name == ".heap")
        {
            area.unmap(&mut self.page_table);
            self.sections.remove(idx);
        }
    }
    // return start_va usize, end_va usize
    pub fn get_section_range(&self, name: &str) -> (VirtPageNum, VirtPageNum) {
        let sect_iterator = self.sections.iter();
        for sect in sect_iterator {
            if sect.name == name {
                return sect.get_section_range();
            }
        }
        // Null
        panic!("can't find section range");
    }
    pub fn modify_section_end(&mut self, name: &str, new_end_vpn: VirtPageNum) {
        let sect_iterator = self.sections.iter_mut();
        for sect in sect_iterator {
            if sect.name == name {
                sect.modify_section_end(&mut self.page_table, new_end_vpn);
            }
        }
    }
    // length 最终会按页对齐, 返回 end_va
    pub fn create_mmap_section(
        &mut self,
        mmap_start: usize,
        length: usize,
        permission: MapPermission,
    ) -> VirtAddr {
        let start_va = mmap_start.into();
        let end_va = (mmap_start + length).into();
        // log::debug!("start: {:#?}, end: {:#?}", start_va, end_va);
        self.insert_mmap_area(".mmap".to_string(), start_va, end_va, permission);
        end_va
    }
    pub fn set_pte_flags(&self, vpn: VirtPageNum, flags: PTEFlags) {
        self.page_table.set_pte_flags(vpn, flags)
    }
    pub fn do_copy_on_write(&mut self, addr: usize) -> Result<(), ()> {
        let fault_vpn = VirtAddr::from(addr).floor();
        // search in common sections
        if let Some(sect) = self
            .sections
            .iter_mut()
            .find(|area| area.vpn_range.is_include(fault_vpn))
        {
            let frame = sect.data_frames.remove(&fault_vpn).unwrap();
            // other address space have already copied to other frame, so don't need to move and copy in this address space
            if Arc::strong_count(&frame) == 1 {
                sect.data_frames.insert(fault_vpn, frame);
                self.page_table
                    .set_pte_flags(fault_vpn, PTEFlags::from_bits(sect.perm.bits()).unwrap());
            } else {
                sect.unmap_one_page(&mut self.page_table, fault_vpn);
                sect.map_one_page(&mut self.page_table, fault_vpn);
                let new_frame = sect.data_frames.get_mut(&fault_vpn).unwrap();
                new_frame
                    .ppn
                    .get_bytes_array()
                    .copy_from_slice(frame.ppn.get_bytes_array());
            }
            Ok(())
        }
        // search in mmap sections
        else if let Some(sect) = self
            .mmap_sections
            .iter_mut()
            .find(|area| area.vpn_range.is_include(fault_vpn))
        {
            let frame = sect.data_frames.remove(&fault_vpn).unwrap();
            if Arc::strong_count(&frame) == 1 {
                sect.data_frames.insert(fault_vpn, frame);
                self.page_table
                    .set_pte_flags(fault_vpn, PTEFlags::from_bits(sect.perm.bits()).unwrap());
            } else {
                sect.unmap_one_page(&mut self.page_table, fault_vpn);
                sect.map_one_page(&mut self.page_table, fault_vpn);
                let new_frame = sect.data_frames.get_mut(&fault_vpn).unwrap();
                new_frame
                    .ppn
                    .get_bytes_array()
                    .copy_from_slice(frame.ppn.get_bytes_array());
            }
            Ok(())
        } else {
            Err(()) // not found vpn
        }
    }
}

#[allow(unused)]
pub fn remap_test() {
    let mut kernel_space = KERNEL_SPACE.lock();
    let mid_text: VirtAddr = ((stext as usize + etext as usize) / 2).into();
    let mid_rodata: VirtAddr = ((srodata as usize + erodata as usize) / 2).into();
    let mid_data: VirtAddr = ((sdata as usize + edata as usize) / 2).into();
    assert!(!kernel_space
        .page_table
        .translate(mid_text.floor())
        .unwrap()
        .is_writable(),);
    assert!(!kernel_space
        .page_table
        .translate(mid_rodata.floor())
        .unwrap()
        .is_writable(),);
    assert!(!kernel_space
        .page_table
        .translate(mid_data.floor())
        .unwrap()
        .is_executable(),);
    println!("remap_test passed!");
}
