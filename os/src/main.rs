#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate spin;

#[macro_use]
mod console;
mod boards;
mod config;
mod lang_items;
mod mm;
mod opensbi;
mod proc;
mod sync;
mod timer;

use crate::opensbi::{send_ipi, shutdown};
use core::arch::global_asm;
use core::sync::atomic::{AtomicBool, Ordering};
use proc::PROCESS;
use timer::get_time;

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

static STARTED: AtomicBool = AtomicBool::new(false);
#[no_mangle]
fn os_main(hartid: usize) {
    println!("cpu{} in main", hartid);
    if STARTED.load(Ordering::SeqCst) == false {
        boot_all_harts(hartid);
        STARTED.store(true, Ordering::SeqCst);
    }
    while !STARTED.load(Ordering::SeqCst) {}
    loop {
        println!("cpu{} get process {}", hartid, PROCESS.lock().get_pid());
        let start = get_time();
        while get_time() - start <= 0x1000000 {}
    }
}
