#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

#[cfg(feature = "board_k210")]
#[path = "boards/k210.rs"]
mod boards;
#[cfg(not(any(feature = "board_k210")))]
#[path = "boards/qemu.rs"]
mod boards;

#[macro_use]
mod console;
mod drivers;
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
mod lang_items;

use crate::opensbi::hart_start;
use core::arch::global_asm;
use core::sync::atomic::{AtomicBool, Ordering};
use dt::CPU_NUMS;
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

static START: AtomicBool = AtomicBool::new(false);
#[no_mangle]
fn os_main(hartid: usize, fdt: *mut u8) {
    if !START.load(Ordering::Acquire) {
        clear_bss();
        trap::init();
        dt::init(fdt);
        // mm::init();
        logging::init();
        info!("0");
        while START.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) == Ok(false)
        {
            core::hint::spin_loop();
        }
        boot_all_harts(hartid);
    } else {
        info!("A");
        loop {}
    }
}
