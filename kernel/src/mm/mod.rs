mod address;
mod address_space;
mod frame;
mod heap;
mod mmap;
mod page_table;
mod section;

pub use address::{addr_test, PhysAddr, PhysPageNum, StepByOne, VirtAddr, VirtPageNum};
pub use address_space::{kernel_token, kernel_translate, remap_test, AddrSpace, KERNEL_SPACE};
pub use frame::{add_free, frame_alloc, frame_allocator_test, frame_dealloc, frame_test, Frame};
pub use heap::{heap_test, init_heap, whereis_heap};
pub use mmap::{MMapFlags, MMapProts};
pub use page_table::{
    translated_array_copy, translated_byte_buffer, translated_ref, translated_refmut,
    translated_str, PageTable, PageTableEntry, UserBuffer,
};
pub use section::MapPermission;

pub fn boot_init() {
    heap::init_heap();
    frame::init_frame_allocator();
    KERNEL_SPACE.lock().activate();
}

pub fn init() {
    KERNEL_SPACE.lock().activate();
}
