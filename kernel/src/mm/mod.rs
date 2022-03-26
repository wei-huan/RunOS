mod address_space;
mod address;
mod frame;
mod heap;
mod page_table;
mod section;

pub use address_space::{kernel_token, remap_test, AddrSpace, KERNEL_SPACE};
pub use address::{addr_test, PhysAddr, PhysPageNum, StepByOne, VirtAddr, VirtPageNum};
pub use frame::{frame_alloc, frame_allocator_test, frame_dealloc, frame_test, Frame};
pub use heap::{heap_test, init_heap, whereis_heap};
pub use section::Permission;
pub use page_table::{PageTable, PageTableEntry, UserBuffer, translated_byte_buffer, translated_str};

use riscv::register::satp;
use core::arch::asm;

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
