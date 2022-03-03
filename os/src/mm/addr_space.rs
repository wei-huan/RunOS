use super::{
    address::{PhysAddr, VirtAddr, VirtPageNum},
    page_table::{PTEFlags, PageTable},
    section::{MapType, Permission, Section},
};
use crate::config::{MEMORY_END, PAGE_SIZE, TRAMPOLINE, TRAP_CONTEXT, USER_STACK_SIZE};
use spin::Mutex;
use alloc::vec::Vec;
use core::arch::asm;
use lazy_static::lazy_static;
use riscv::register::satp;

extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss();
    fn ebss();
    fn ekernel();
    fn strampoline();
}

lazy_static! {
    pub static ref KERNEL_SPACE: Mutex<AddrSpace> =
    Mutex::new(AddrSpace::create_kernel_space());
}

pub struct AddrSpace {
    page_table: PageTable,
    sections: Vec<Section>,
}

impl AddrSpace {
    pub fn new_empty() -> Self {
        Self {
            page_table: PageTable::new(),
            sections: Vec::new(),
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
        // map trampoline
        // kernel_space.map_trampoline();
        // map kernel sections
        println!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
        println!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
        println!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
        println!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
        println!("mapping .text section");
        kernel_space.push_section(
            Section::new(
                (stext as usize).into(),
                (etext as usize).into(),
                MapType::Identical,
                Permission::R | Permission::X,
            ),
            None,
        );
        println!("mapping .rodata section");
        kernel_space.push_section(
            Section::new(
                (srodata as usize).into(),
                (erodata as usize).into(),
                MapType::Identical,
                Permission::R,
            ),
            None,
        );
        println!("mapping .data section");
        kernel_space.push_section(
            Section::new(
                (sdata as usize).into(),
                (edata as usize).into(),
                MapType::Identical,
                Permission::R | Permission::W,
            ),
            None,
        );
        println!("mapping .bss section");
        kernel_space.push_section(
            Section::new(
                (sbss as usize).into(),
                (ebss as usize).into(),
                MapType::Identical,
                Permission::R | Permission::W,
            ),
            None,
        );
        println!("mapping physical memory");
        kernel_space.push_section(
            Section::new(
                (ekernel as usize).into(),
                MEMORY_END.into(),
                MapType::Identical,
                Permission::R | Permission::W,
            ),
            None,
        );
        // println!("mapping kernel finish");
        kernel_space
    }
    pub fn create_user_space(elf_data: &[u8]) -> (Self, usize, usize) {
        let mut user_space = Self::new_empty();
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
                let start_va: VirtAddr = (ph.virtual_addr() as usize).into();
                let end_va: VirtAddr = ((ph.virtual_addr() + ph.mem_size()) as usize).into();
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
                let section = Section::new(start_va, end_va, MapType::Framed, map_perm);
                max_end_vpn = section.vpn_range.get_end();
                user_space.push_section(
                    section,
                    Some(&elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize]),
                );
            }
        }
        // map user stack with U flags
        let max_end_va: VirtAddr = max_end_vpn.into();
        let mut user_stack_low: usize = max_end_va.into();
        // guard page
        user_stack_low += PAGE_SIZE;
        let user_stack_high = user_stack_low + USER_STACK_SIZE;
        user_space.push_section(
            Section::new(
                user_stack_low.into(),
                user_stack_high.into(),
                MapType::Framed,
                Permission::R | Permission::W | Permission::U,
            ),
            None,
        );
        // map TrapContext
        user_space.push_section(
            Section::new(
                TRAP_CONTEXT.into(),
                TRAMPOLINE.into(),
                MapType::Framed,
                Permission::R | Permission::W,
            ),
            None,
        );
        (
            user_space,
            user_stack_high,
            elf.header.pt2.entry_point() as usize,
        )
    }
    pub fn activate(&mut self) {
        let root_ppn = self.page_table.get_root_ppn();
        let satp = 8usize << 60 | root_ppn.0;
        unsafe {
            satp::write(satp);
            asm!("sfence.vma");
        }
    }
}
