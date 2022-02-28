#[allow(unused)]
// 内核堆大小2MB
pub const KERNEL_HEAP_SIZE: usize = 0x20_0000;
// 给buddy_system_allocator使用的，这个值大于32即可
pub const HEAP_ALLOCATOR_MAX_ORDER: usize = 64;

pub const CLOCK_FREQ: usize = 12500000;
