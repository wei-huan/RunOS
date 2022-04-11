use super::{
    address::{PhysAddr, VirtAddr, VirtPageNum},
    page_table::{PTEFlags, PageTable, PageTableEntry},
    section::{MapType, Permission, Section},
};
use crate::config::{MEMORY_END, PAGE_SIZE, TRAMPOLINE, TRAP_CONTEXT, USER_STACK_SIZE};
use crate::platform::MMIO;
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
    fn etrampoline();
}

lazy_static! {
    pub static ref KERNEL_SPACE: Mutex<AddrSpace> = Mutex::new(AddrSpace::create_kernel_space());
}

pub fn kernel_token() -> usize {
    KERNEL_SPACE.lock().get_token()
}

pub fn kernel_translate(vpn: VirtPageNum) -> Option<PageTableEntry> {
    KERNEL_SPACE.lock().page_table.translate(vpn)
}

pub struct AddrSpace {
    pub page_table: PageTable,
    sections: Vec<Section>,
}

impl AddrSpace {
    pub fn new_empty() -> Self {
        Self {
            page_table: PageTable::new(),
            sections: Vec::new(),
        }
    }
    pub fn get_token(&self) -> usize {
        self.page_table.get_token()
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
    fn push_section(&mut self, mut section: Section, data: Option<&[u8]>) {
        section.map(&mut self.page_table);
        if let Some(data) = data {
            section.copy_data(&mut self.page_table, data);
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
        println!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
        println!(".trampoline [{:#x}, {:#x})", strampoline as usize, etrampoline as usize);
        println!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
        println!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
        println!(".bss [{:#x}, {:#x})", sbss_with_stack as usize, ebss as usize);
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
        println!("mapping kernel finish");
        kernel_space
    }
    /// Include sections in elf and trampoline and TrapContext and user stack,
    /// also returns user_sp and entry point.
    pub fn create_user_space(elf_data: &[u8]) -> (Self, usize, usize) {
        // println!("create_user_space");
        let mut user_space = Self::new_empty();
        // map trampoline
        user_space.map_trampoline();
        // map program headers of elf, with U flag
        let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
        let elf_header = elf.header;
        let magic = elf_header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");
        let ph_count = elf_header.pt2.ph_count();
        let mut max_end_vpn = VirtPageNum(0);
        for i in 0..ph_count {
            let ph = elf.program_header(i).unwrap();
            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                // first section header is dummy, not match program header, so i + 1
                let sect = elf.section_header((i + 1).try_into().unwrap()).unwrap();
                let name = sect.get_name(&elf).unwrap();
                // println!("name: {}", name);
                let start_va: VirtAddr = (ph.virtual_addr() as usize).into();
                // println!("start_va: 0x{:X}", usize::from(start_va));
                let end_va: VirtAddr = ((ph.virtual_addr() + ph.mem_size()) as usize).into();
                // println!("end_va: 0x{:X}", usize::from(end_va));
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
                user_space.push_section(
                    section,
                    Some(&elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize]),
                );
            }
        }
        // clear bss section
        user_space.clear_bss_pages();
        // map user stack with U flags
        let max_end_va: VirtAddr = max_end_vpn.into();
        let mut user_stack_bottom: usize = max_end_va.into();
        // guard page
        user_stack_bottom += PAGE_SIZE;
        let user_stack_high = user_stack_bottom + USER_STACK_SIZE;
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
        unsafe { asm!("fence.i") }
        (
            user_space,
            user_stack_high,
            elf.header.pt2.entry_point() as usize,
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
        addr_space
    }
    #[allow(unused)]
    pub fn recycle_data_pages(&mut self) {
        self.sections.clear();
    }
    pub fn activate(&mut self) {
        let satp = self.page_table.get_token();
        unsafe {
            satp::write(satp);
            asm!("sfence.vma");
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
