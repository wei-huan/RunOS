mod address;
mod address_space;
mod frame;
mod heap;
mod page_table;
mod section;

pub use address::{addr_test, PhysAddr, PhysPageNum, StepByOne, VirtAddr, VirtPageNum};
pub use address_space::{kernel_translate, kernel_token, kernel_remap_trampoline, remap_test, AddrSpace, KERNEL_SPACE};
pub use frame::{frame_alloc, frame_allocator_test, frame_dealloc, frame_test, Frame};
pub use heap::{heap_test, init_heap, whereis_heap};
pub use page_table::{
    translated_byte_buffer, translated_str, PageTable, PageTableEntry, UserBuffer,
};
pub use section::Permission;

use core::arch::asm;
use riscv::register::satp;

pub fn boot_init() {
    heap::init_heap();
    frame::init_frame_allocator();
    KERNEL_SPACE.lock().activate();
}

pub fn init() {
    // KERNEL_SPACE.lock().activate();
    let satp = kernel_token();
    unsafe {
        satp::write(satp);
        asm!("sfence.vma");
    }
}
