use super::{
    address::{PhysAddr, VirtAddr, VirtPageNum},
    page_table::{PTEFlags, PageTable, PageTableEntry},
    section::{MapType, Permission, Section},
};
use crate::config::{
    MEMORY_END, PAGE_SIZE, TRAMPOLINE, TRAP_CONTEXT, USER_STACK_HIGH, USER_STACK_SIZE,
};
use crate::platform::MMIO;
use crate::task::{
    AuxHeader, AT_BASE, AT_CLKTCK, AT_EGID, AT_ENTRY, AT_EUID, AT_FLAGS, AT_GID, AT_HWCAP,
    AT_NOTELF, AT_PAGESZ, AT_PHDR, AT_PHENT, AT_PHNUM, AT_PLATFORM, AT_SECURE, AT_UID,
};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
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
    // fn etrampoline();
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
    mmap_sections: Vec<Section>,
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
        permission: Permission,
    ) {
        self.push_section(
            Section::new(name, start_va, end_va, MapType::Framed, permission),
            None,
        );
    }
    pub fn insert_mmap_area(
        &mut self,
        name: String,
        start_va: VirtAddr,
        end_va: VirtAddr,
        permission: Permission,
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
            .sections
            .iter_mut()
            .enumerate()
            .find(|(_, area)| area.vpn_range.get_start() == start_vpn)
        {
            area.unmap(&mut self.page_table);
            self.mmap_sections.remove(idx);
        }
    }
    fn push_section(&mut self, mut section: Section, data: Option<&[u8]>) {
        section.map(&mut self.page_table);
        if let Some(data) = data {
            section.copy_data(&mut self.page_table, data, 0);
        }
        self.sections.push(section);
    }
    fn push_mmap_section(&mut self, mut section: Section, data: Option<&[u8]>) {
        section.map(&mut self.page_table);
        if let Some(data) = data {
            section.copy_data(&mut self.page_table, data, 0);
        }
        self.mmap_sections.push(section);
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
                Permission::R | Permission::X,
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
                Permission::R,
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
                Permission::R | Permission::W,
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
                Permission::R | Permission::W,
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
                Permission::R | Permission::W,
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
                    Permission::R | Permission::W,
                ),
                None,
            );
        }
        // unsafe { asm!("fence.i") }
        // println!("mapping kernel finish");
        kernel_space
    }
    /// Include sections in elf and trampoline and TrapContext and user stack,
    /// also returns user_sp and entry point.
    // pub fn create_user_space(elf_data: &[u8]) -> (Self, usize, usize, usize) {
    //     // println!("create_user_space");
    //     let mut user_space = Self::new_empty();
    //     // map trampoline
    //     user_space.map_trampoline();
    //     // map program headers of elf, with U flag
    //     let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
    //     let elf_header = elf.header;
    //     let magic = elf_header.pt1.magic;
    //     assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");
    //     let ph_count = elf_header.pt2.ph_count();
    //     // println!("ph_count: {}", ph_count);
    //     let mut max_end_vpn = VirtPageNum(0);
    //     for i in 0..ph_count {
    //         let ph = elf.program_header(i).unwrap();
    //         if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
    //             // first section header is dummy, not match program header, so i + 1
    //             let sect = elf.section_header((i + 1).try_into().unwrap()).unwrap();
    //             let name = sect.get_name(&elf).unwrap();
    //             // println!("name: {}", name);
    //             let start_va: VirtAddr = (ph.virtual_addr() as usize).into();
    //             // println!("start_va: {:#X}", usize::from(start_va));
    //             let end_va: VirtAddr = ((ph.virtual_addr() + ph.mem_size()) as usize).into();
    //             // println!("end_va: {:#X}", usize::from(end_va));
    //             let offset = start_va.0 - start_va.floor().0 * PAGE_SIZE;
    //             // println!("offset: {:#X}", offset);
    //             let mut map_perm = Permission::U;
    //             let ph_flags = ph.flags();
    //             if ph_flags.is_read() {
    //                 map_perm |= Permission::R;
    //             }
    //             if ph_flags.is_write() {
    //                 map_perm |= Permission::W;
    //             }
    //             if ph_flags.is_execute() {
    //                 map_perm |= Permission::X;
    //             }
    //             // println!("map_perm: {:#?}", map_perm);
    //             let section = Section::new(
    //                 name.to_string(),
    //                 start_va,
    //                 end_va,
    //                 MapType::Framed,
    //                 map_perm,
    //             );
    //             max_end_vpn = section.vpn_range.get_end();
    //             // println!("range: 0x{:X}", (usize::from(end_va) - usize::from(start_va)));
    //             // println!("start_vpn: {:?} end_vpn: {:?}", section.vpn_range.get_start(), section.vpn_range.get_end());
    //             // println!("ph file_size: 0x{:X}", ph.file_size());
    //             // println!("ph mem_size: 0x{:X}", ph.mem_size());
    //             // println!("");
    //             // user_space.push_section(
    //             //     section,
    //             //     Some(&elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize]),
    //             // );
    //             if offset == 0 {
    //                 user_space.push_section(
    //                     section,
    //                     Some(
    //                         &elf.input
    //                             [ph.offset() as usize..(ph.offset() + ph.file_size()) as usize],
    //                     ),
    //                 );
    //             } else {
    //                 user_space.push_section_with_offset(
    //                     section,
    //                     offset,
    //                     Some(
    //                         &elf.input
    //                             [ph.offset() as usize..(ph.offset() + ph.file_size()) as usize],
    //                     ),
    //                 );
    //             }
    //         }
    //     }
    //     // clear bss section
    //     user_space.clear_bss_pages();

    //     let max_end_va: VirtAddr = max_end_vpn.into();
    //     let mut heap_start: usize = max_end_va.into();
    //     //guard page
    //     heap_start += PAGE_SIZE;

    //     // map user stack with U flags
    //     // user stack is set just below the trap_cx
    //     let user_stack_high = USER_STACK_HIGH;
    //     let user_stack_bottom = user_stack_high - USER_STACK_SIZE;
    //     // println!("user_stack_bottom: 0x{:X}", usize::from(user_stack_bottom));
    //     // println!("user_stack_high: 0x{:X}", usize::from(user_stack_high));
    //     user_space.push_section(
    //         Section::new(
    //             ".ustack".to_string(),
    //             user_stack_bottom.into(),
    //             user_stack_high.into(),
    //             MapType::Framed,
    //             Permission::R | Permission::W | Permission::U,
    //         ),
    //         None,
    //     );

    //     // map TrapContext
    //     user_space.push_section(
    //         Section::new(
    //             ".trap_cx".to_string(),
    //             TRAP_CONTEXT.into(),
    //             TRAMPOLINE.into(),
    //             MapType::Framed,
    //             Permission::R | Permission::W,
    //         ),
    //         None,
    //     );
    //     // unsafe { asm!("fence.i") }
    //     (
    //         user_space,
    //         heap_start,
    //         user_stack_high,
    //         elf.header.pt2.entry_point() as usize,
    //     )
    // }
    pub fn create_user_space(elf_data: &[u8]) -> (Self, usize, usize, usize, Vec<AuxHeader>) {
        // println!("create_user_space");
        let mut auxv: Vec<AuxHeader> = Vec::new();
        let mut user_space = Self::new_empty();
        // map trampoline
        user_space.map_trampoline();
        // map program headers of elf, with U flag
        let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
        let elf_header = elf.header;
        let magic = elf_header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");
        let ph_count = elf_header.pt2.ph_count();
        // println!("ph_count: {}", ph_count);
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
        auxv.push(AuxHeader {
            aux_type: AT_BASE,
            value: 0 as usize,
        });
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

        let mut head_va = 0;

        for i in 0..ph_count {
            let ph = elf.program_header(i).unwrap();
            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                // first section header is dummy, not match program header, so i + 1
                let sect = elf.section_header((i + 1).try_into().unwrap()).unwrap();
                let name = sect.get_name(&elf).unwrap();
                // println!("name: {}", name);
                let start_va: VirtAddr = (ph.virtual_addr() as usize).into();
                // println!("start_va: {:#X}", usize::from(start_va));
                let end_va: VirtAddr = ((ph.virtual_addr() + ph.mem_size()) as usize).into();
                // println!("end_va: {:#X}", usize::from(end_va));
                let offset = start_va.0 - start_va.floor().0 * PAGE_SIZE;
                // println!("offset: {:#X}", offset);
                let mut map_perm = Permission::U;
                let ph_flags = ph.flags();
                if ph_flags.is_read() {
                    map_perm |= Permission::R;
                }
                if ph_flags.is_write() {
                    map_perm |= Permission::W;
                }
                if ph_flags.is_execute() {
                    map_perm |= Permission::X;
                }
                // println!("map_perm: {:#?}", map_perm);
                let section = Section::new(
                    name.to_string(),
                    start_va,
                    end_va,
                    MapType::Framed,
                    map_perm,
                );
                max_end_vpn = section.vpn_range.get_end();
                // println!("range: 0x{:X}", (usize::from(end_va) - usize::from(start_va)));
                // println!("start_vpn: {:?} end_vpn: {:?}", section.vpn_range.get_start(), section.vpn_range.get_end());
                // println!("ph file_size: 0x{:X}", ph.file_size());
                // println!("ph mem_size: 0x{:X}", ph.mem_size());
                // println!("");
                // user_space.push_section(
                //     section,
                //     Some(&elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize]),
                // );
                if offset == 0 {
                    user_space.push_section(
                        section,
                        Some(
                            &elf.input
                                [ph.offset() as usize..(ph.offset() + ph.file_size()) as usize],
                        ),
                    );
                } else {
                    user_space.push_section_with_offset(
                        section,
                        offset,
                        Some(
                            &elf.input
                                [ph.offset() as usize..(ph.offset() + ph.file_size()) as usize],
                        ),
                    );
                }

                if start_va.aligned() {
                    head_va = start_va.0;
                }
            }
        }
        let ph_head_addr = head_va + elf.header.pt2.ph_offset() as usize;
        auxv.push(AuxHeader {
            aux_type: AT_PHDR,
            value: ph_head_addr as usize,
        });

        // clear bss section
        user_space.clear_bss_pages();

        let max_end_va: VirtAddr = max_end_vpn.into();
        let mut heap_start: usize = max_end_va.into();
        //guard page
        heap_start += PAGE_SIZE;

        // map user stack with U flags
        // user stack is set just below the trap_cx
        let user_stack_high = USER_STACK_HIGH;
        let user_stack_bottom = user_stack_high - USER_STACK_SIZE;
        // println!("user_stack_bottom: 0x{:X}", usize::from(user_stack_bottom));
        // println!("user_stack_high: 0x{:X}", usize::from(user_stack_high));
        user_space.push_section(
            Section::new(
                ".ustack".to_string(),
                user_stack_bottom.into(),
                user_stack_high.into(),
                MapType::Framed,
                Permission::R | Permission::W | Permission::U,
            ),
            None,
        );

        // map TrapContext
        user_space.push_section(
            Section::new(
                ".trap_cx".to_string(),
                TRAP_CONTEXT.into(),
                TRAMPOLINE.into(),
                MapType::Framed,
                Permission::R | Permission::W,
            ),
            None,
        );
        // unsafe { asm!("fence.i") }
        (
            user_space,
            heap_start,
            user_stack_high,
            elf.header.pt2.entry_point() as usize,
            auxv,
        )
    }
    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.page_table.translate(vpn)
    }
    pub fn clear_bss_pages(&mut self) {
        let sect_iterator = self.sections.iter_mut();
        for sect in sect_iterator {
            if sect.name == ".bss" {
                // println!("clear bss");
                sect.clear(&mut self.page_table);
            }
        }
    }
    pub fn from_existed_user(user_space: &AddrSpace) -> AddrSpace {
        let mut addr_space = Self::new_empty();
        // map trampoline
        addr_space.map_trampoline();
        // copy data sections/trap_context/user_stack
        for area in user_space.sections.iter() {
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
        // copy data mmap_sections
        for area in user_space.mmap_sections.iter() {
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
        addr_space
    }
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
            Permission::R | Permission::W | Permission::U,
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
    pub fn get_section_range(&self, name: &str) -> (usize, usize) {
        let sect_iterator = self.sections.iter();
        for sect in sect_iterator {
            if sect.name == name {
                return sect.get_section_range();
            }
        }
        // NULL
        return (0, 0);
    }
    // 如果存在要找的段就调整，不存在就啥都不做
    pub fn modify_section_end(&mut self, name: &str, new_end_vpn: VirtPageNum) {
        let sect_iterator = self.sections.iter_mut();
        for sect in sect_iterator {
            if sect.name == name {
                sect.modify_section_end(&mut self.page_table, new_end_vpn);
            }
        }
    }
    // size 最终会按页对齐
    pub fn create_mmap_section(&mut self, mmap_start: usize, size: usize, permission: Permission) {
        let start_va = mmap_start.into();
        let end_va = (mmap_start + size).into();
        self.insert_mmap_area(".mmap".to_string(), start_va, end_va, permission)
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
