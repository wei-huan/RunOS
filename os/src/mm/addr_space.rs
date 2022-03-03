use super::{page_table::PageTable, section::Section};
use lazy_static::lazy_static;
use riscv::register::satp;
use crate::sync::Mutex;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::arch::asm;
use core::ops::Add;

lazy_static! {
    pub static ref KERNEL_SPACE: Arc<Mutex<AddrSpace>> = Arc::new(Mutex::new(AddrSpace::create_kernelspace()));
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
    pub fn create_userspace() -> Self {
        Self {
            page_table: PageTable::new(),
            sections: Vec::new(),
        }
    }
    pub fn create_kernelspace() -> Self {
        Self {
            page_table: PageTable::new(),
            sections: Vec::new(),
        }
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
