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
mod config;
mod cpus;
mod logging;
mod lang_items;
mod mm;
mod opensbi;
mod trap;
mod sync;
mod drivers;
mod fs;
mod dt;
mod timer;
mod utils;

use log::*;
use core::arch::global_asm;
use crate::opensbi::{hart_start};
use dt::CPU_NUMS;
use core::sync::atomic::{AtomicBool, Ordering};
// use sync::Mutex;
// use boards::CPU_NUM;

global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}

fn boot_all_harts(hartid: usize) {
    let ncpu = CPU_NUMS.load(Ordering::Acquire);
    for i in 0..ncpu {
        if i != hartid {
            hart_start(i, 0x80200000, 1);
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
        logging::init();
        info!("start cpu{}", hartid);
        while START.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) == Ok(false) {
            info!("loop");
            core::hint::spin_loop();
        }
        boot_all_harts(hartid);
    } else {
        info!("cpu{}", hartid);
        loop{}
    }
}
