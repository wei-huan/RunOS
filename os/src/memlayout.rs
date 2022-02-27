// Physical memory layout

// qemu -machine virt is set up like this,
// based on qemu's hw/riscv/virt.c:
//
// 00001000 -- boot ROM, provided by qemu
// 02000000 -- CLINT
// 0C000000 -- PLIC
// 10000000 -- uart0
// 10001000 -- virtio disk
// 80000000 -- boot ROM jumps here in machine mode
//             -kernel loads the kernel here
// unused RAM after 80000000.

// the kernel uses physical memory thus:
// 80000000 -- entry.S, then kernel text and data
// end -- start of kernel page allocation area
// PHYSTOP -- end RAM used by the kernel

// qemu puts UART registers here in physical memory.
pub const UART0: usize = 0x1000_0000;
pub const UART0_IRQ: u32 = 10;

// virtio mmio interface
pub const VIRTIO0: usize = 0x1000_1000;
pub const VIRTIO0_IRQ: u32 = 1;

// core local interrupter (CLINT), which contains the timer
pub const CLINT: usize = 0x2000000;
pub const fn clint_mtimecmp(hartid: usize) -> usize {
    CLINT + 0x4000 + 8 * hartid
}
pub const CLINT_MTIME: usize = CLINT + 0xBFF8; // Cycles since boot.

// qemu puts platform-level interrupt controller (PLIC) here.
pub const PLIC: usize = 0x0C00_0000;
pub const PLIC_PRIORITY: usize = PLIC + 0x0;
pub const PLIC_PENDING: usize = PLIC + 0x1000;
#[allow(non_snake_case)]
pub const fn PLIC_SENABLE(hart: usize) -> usize {
    PLIC + 0x2080 + hart * 0x100
}
#[allow(non_snake_case)]
pub const fn PLIC_SPRIORITY(hart: usize) -> usize {
    PLIC + 0x201000 + hart * 0x2000
}
#[allow(non_snake_case)]
pub const fn PLIC_SCLAIM(hart: usize) -> usize {
    PLIC + 0x201004 + hart * 0x2000
}

// the kernel expects there to be RAM
// for use by the kernel and user psges
// from physical address 0x80000000 to PHYSTOP.
pub const KERNBASE: usize = 0x8000_0000;
pub const PHYSTOP: usize = KERNBASE + 128 * 1024 * 1024;
