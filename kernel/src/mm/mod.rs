mod address;
mod address_space;
mod frame;
mod heap;
mod page_table;
mod section;

pub use address::{addr_test, PhysAddr, PhysPageNum, StepByOne, VirtAddr, VirtPageNum};
pub use address_space::{kernel_token, kernel_translate, remap_test, AddrSpace, KERNEL_SPACE};
pub use frame::{frame_alloc, frame_allocator_test, frame_dealloc, frame_test, Frame};
pub use heap::{heap_test, init_heap, whereis_heap};
pub use page_table::{
    translated_byte_buffer, translated_ref, translated_refmut, translated_str, PageTable,
    PageTableEntry, UserBuffer,
};
pub use section::Permission;

use core::arch::asm;

pub fn boot_init() {
    heap::init_heap();
    frame::init_frame_allocator();
    // #[cfg(any(feature = "qemu", feature = "rustsbi"))]
    KERNEL_SPACE.lock().activate();
}

pub fn init() {
    // #[cfg(any(feature = "qemu", feature = "rustsbi"))]
    KERNEL_SPACE.lock().activate();
}

#[inline(always)]
#[allow(unused)]
pub fn sfence(vaddr: Option<VirtAddr>, asid: Option<u16>) {
    unsafe {
        match (vaddr, asid) {
            (Some(vaddr), Some(asid)) => {
                let vaddr: usize = vaddr.into();
                asm!("sfence.vma {}, {}", in(reg) vaddr, in(reg) asid);
            }
            (Some(vaddr), None) => {
                let vaddr: usize = vaddr.into();
                asm!("sfence.vma {}, zero", in(reg) vaddr);
            }
            (None, Some(asid)) => asm!("sfence.vma zero, {}", in(reg) asid),
            (None, None) => asm!("sfence.vma zero, zero"),
        }
    }
}
