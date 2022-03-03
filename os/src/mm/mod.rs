mod heap;
mod address;
mod frame;
mod page_table;
mod section;
mod addr_space;

pub use heap::{whereis_heap, init_heap, heap_test};
pub use address::{addr_test};
pub use frame::{frame_allocator_test, frame_test};
pub use page_table::PageTableEntry;

pub fn init() {
    heap::init_heap();
    frame::init_frame_allocator();
}
