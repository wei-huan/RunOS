#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate fdt;
#[cfg(feature = "board_k210")]
#[path = "boards/k210.rs"]
mod boards;
#[cfg(not(any(feature = "board_k210")))]
#[path = "boards/qemu.rs"]
mod boards;

#[macro_use]
mod console;
mod dt;
mod fs;
mod mm;
mod cpus;
mod sync;
mod trap;
mod utils;
mod timer;
mod config;
mod logging;
mod opensbi;
mod drivers;
mod lang_items;

use crate::opensbi::hart_start;
use core::arch::global_asm;
use core::sync::atomic::Ordering;
use dt::{CPU_NUMS, TIMER_FREQ};
use log::*;

global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}

fn boot_all_harts(hartid: usize) {
    extern "C" {
        fn _start();
    }
    let ncpu = CPU_NUMS.load(Ordering::Acquire);
    for i in 0..ncpu {
        if i != hartid {
            // priv: 1 for supervisor; 0 for user;
            hart_start(i, _start as usize, 1);
        }
    }
}

#[no_mangle]
fn os_main(hartid: usize, fdt: *mut u8) {
    clear_bss();
    logging::init();
    mm::boot_init();
    mm::remap_test();
    // println!("[kernel] Hello, world!");
    trap::init();
    // println!("fdt: 0x{:X}", fdt as usize);
    dt::init(fdt);
    println!("[kernel] Hello, world!");
    timer::boot_init();
    // let n_cpus = CPU_NUMS.load(Ordering::Relaxed);
    // let timebase_frequency = TIMER_FREQ.load(Ordering::Relaxed);
    // info!("MyOS version {}", env!("CARGO_PKG_VERSION"));
    // info!("=== Machine Info ===");
    // info!(" Total CPUs: {}", n_cpus);
    // info!(" Timer Clock: {}Hz", timebase_frequency);
    // info!("=== SBI Implementation ===");
    info!("=== MyOS Info ===");
    loop{}
}
