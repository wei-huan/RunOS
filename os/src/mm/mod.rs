mod heap;
mod frame;
mod address;
mod section;
mod page_table;
mod addr_space;

pub use heap::{whereis_heap, init_heap, heap_test};
pub use address::{addr_test, VPNRange, PhysAddr, PhysPageNum, StepByOne, VirtAddr, VirtPageNum};
pub use frame::{frame_allocator_test, frame_test, frame_alloc, frame_dealloc, Frame};
pub use page_table::{PageTable, PageTableEntry};
pub use section::Permission;
pub use addr_space::{KERNEL_SPACE, kernel_token, AddrSpace};

pub fn init() {
    heap::init_heap();
    frame::init_frame_allocator();
    KERNEL_SPACE.lock().activate();
}
