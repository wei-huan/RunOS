mod heap;
mod address;
mod frame;

pub use heap::{whereis_heap, init_heap, heap_test};
pub use address::{addr_test};
pub use frame::{frame_test};
