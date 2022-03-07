mod heap;
mod address;
mod frame;
mod page_table;
mod section;
mod addr_space;

pub use frame::{frame_allocator_test, frame_test, frame_alloc, frame_dealloc, Frame};
pub use heap::{whereis_heap, init_heap, heap_test};
pub use address::{PhysAddr, PhysPageNum, StepByOne, VirtAddr, VirtPageNum, addr_test};
pub use page_table::{PageTableEntry, PageTable, UserBuffer};
pub use addr_space::{kernel_token, AddrSpace, KERNEL_SPACE};

pub fn init() {
    heap::init_heap();
    frame::init_frame_allocator();
    KERNEL_SPACE.lock().activate();
}
