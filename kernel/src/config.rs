pub const PAGE_SIZE: usize = 4096;
pub const PAGE_SIZE_BITS: usize = 0xc;
#[cfg(feature = "platform-qemu")]
pub const MEMORY_END: usize = 0x80AE0000;
#[cfg(feature = "platform-k210")]
pub const MEMORY_END: usize = 0x80800000;
pub const KERNEL_HEAP_SIZE: usize = 0x44_0000;

// for buddy_system_allocator
pub const HEAP_ALLOCATOR_MAX_ORDER: usize = 32;

// Kernel and User Address Space 's Address
pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const SIGRETURN_TRAMPOLINE: usize = TRAMPOLINE - PAGE_SIZE;

// Kernel Address Space 's Address
pub const KERNEL_STACK_BASE: usize = SIGRETURN_TRAMPOLINE - PAGE_SIZE; // stack grow down, so stack base address is high end
pub const KERNEL_STACK_SIZE: usize = PAGE_SIZE * 4;
pub const BOOT_STACK_SIZE: usize = PAGE_SIZE * 2; // 16 KB

// User Address Space 's Address
pub const TRAP_CONTEXT_BASE: usize = SIGRETURN_TRAMPOLINE - PAGE_SIZE;
pub const USER_STACK_BASE: usize = TRAP_CONTEXT_BASE - PAGE_SIZE; // stack grow down, so stack base address is high end
pub const USER_STACK_SIZE: usize = PAGE_SIZE * 12;
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
