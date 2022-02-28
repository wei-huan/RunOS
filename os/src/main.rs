#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
mod console;
mod boards;
mod config;
mod cpus;
mod lang_items;
mod mm;
mod opensbi;
mod sync;

use crate::opensbi::{send_ipi, shutdown};
use core::arch::global_asm;
use core::sync::atomic::{AtomicBool, Ordering};
use sync::Mutex;
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
    for i in 0..2 {
        if i != hartid {
            send_ipi(i);
        }
    }
}

// static m: Mutex<bool> = Mutex::new(false, "boot_harts");
static LOCKED: AtomicBool = AtomicBool::new(false);
#[no_mangle]
fn os_main(hartid: usize) {
    while LOCKED.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) == Ok(false) {
        core::hint::spin_loop();
    }
    println!("cpu{}", hartid);
    boot_all_harts(hartid);
    LOCKED.store(false, Ordering::Release);
    loop{}
    shutdown();
}
