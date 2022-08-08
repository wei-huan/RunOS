// pub const PAGE_SIZE: usize = 4096;
// pub const PAGE_SIZE_BITS: usize = 0xc;
// pub const MEMORY_END: usize = 0x80800000;
// // 内核堆大小 3MB
// pub const KERNEL_HEAP_SIZE: usize = 0x41_0000;
// // 给 buddy_system_allocator 使用的，这个值大于 32 即可
// pub const HEAP_ALLOCATOR_MAX_ORDER: usize = 32;
// pub const KERNEL_STACK_SIZE: usize = PAGE_SIZE * 1;
// pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;
// pub const USER_STACK_SIZE: usize = PAGE_SIZE * 8;
// pub const USER_STACK_HIGH: usize = TRAP_CONTEXT - PAGE_SIZE; // 再减一个 PAGE_SIZE 为 Guard Page
// pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
// // 启动栈大小 16 KB
// pub const BOOT_STACK_SIZE: usize = PAGE_SIZE * 4;
// pub const MMAP_BASE: usize = 0x10_0000_0000; // 0xFFFFFFC000000000; //
// // dynamic link library loader base address
// pub const DLL_LOADER_BASE: usize = 0x30_0000_0000;

// #[inline(always)]
// pub fn page_aligned_up(addr: usize) -> usize {
//     (addr + PAGE_SIZE - 1) / PAGE_SIZE * PAGE_SIZE
// }

// #[inline(always)]
// pub fn is_page_aligned(addr: usize) -> bool {
//     addr % PAGE_SIZE == 0
// }

pub const PAGE_SIZE: usize = 4096;
pub const PAGE_SIZE_BITS: usize = 0xc;
pub const MEMORY_END: usize = 0x80800000;
pub const KERNEL_HEAP_SIZE: usize = 0x41_0000;

// for buddy_system_allocator
pub const HEAP_ALLOCATOR_MAX_ORDER: usize = 32;

// Kernel and User Address Space 's Address
pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;

// Kernel Address Space 's Address
pub const KERNEL_STACK_BASE: usize = TRAMPOLINE - 2 * PAGE_SIZE; // stack grow down, so stack base address is high end
pub const KERNEL_STACK_SIZE: usize = PAGE_SIZE * 1;
pub const BOOT_STACK_SIZE: usize = PAGE_SIZE * 4; // 16 KB

// User Address Space 's Address
pub const TRAP_CONTEXT_BASE: usize = TRAMPOLINE - PAGE_SIZE;
pub const USER_STACK_BASE: usize = TRAP_CONTEXT_BASE - PAGE_SIZE; // stack grow down, so stack base address is high end
pub const USER_STACK_SIZE: usize = PAGE_SIZE * 8;
pub const MMAP_BASE: usize = 0x10_0000_0000; // 0xFFFFFFC000000000;
// pub const HEAP_BASE: usize = 0x20_0000_0000;
pub const DLL_LOADER_BASE: usize = 0x30_0000_0000; // dynamic link library loader base address

// Process RLimit
#[allow(unused)]
pub const FD_LIMIT: usize = 48;

#[allow(unused)]
#[inline(always)]
pub fn page_aligned_down(addr: usize) -> usize {
    addr / PAGE_SIZE * PAGE_SIZE
}

#[allow(unused)]
#[inline(always)]
pub fn page_aligned_up(addr: usize) -> usize {
    (addr + PAGE_SIZE - 1) / PAGE_SIZE * PAGE_SIZE
}

#[allow(unused)]
#[inline(always)]
pub fn is_page_aligned(addr: usize) -> bool {
    addr % PAGE_SIZE == 0
}
