#[allow(unused)]

// 内核堆大小2MB
pub const KERNEL_HEAP_SIZE: usize = 0x20_0000;
// 内核堆最大页是2^4个页框，也就是buddy_system分配的最大块内存有64KB
pub const HEAP_ALLOCATOR_MAX_ORDER: usize = 64;

pub const MEMORY_END: usize = 0x80800000;
