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
use core::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
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

// static STARTED: Mutex<i32> = Mutex::new(0);
// static SCEHDULER: Mutex<i32> = Mutex::new(0);
// #[no_mangle]
// fn os_main(hartid: usize) {
//     println!("cpu{}", hartid);
//     let mut x = STARTED.lock();
//     if *x == 0 {
//         *x = 1;
//         boot_all_harts(hartid);
//         drop(x);
//     } else {}
//     let mut y = SCEHDULER.lock();
//     println!("cpu{} have scheduler {} hhh!", hartid, *y);
//     *y += 1;
//     drop(y);
//     shutdown();
// }

// static LOCKED: AtomicBool = AtomicBool::new(false);
// #[no_mangle]
// fn os_main(hartid: usize) {
//     while LOCKED.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) == Ok(false) {
//         core::hint::spin_loop();
//     }
//     println!("cpu{}", hartid);
//     boot_all_harts(hartid);
//     LOCKED.store(false, Ordering::Release);
//     shutdown();
// }

// static STARTED: AtomicBool = AtomicBool::new(false);
// #[no_mangle]
// fn os_main(hartid: usize) {
//     let ptr = &mut 1;
//     let SCHEDULER = AtomicPtr::new(ptr);
//     println!("cpu{}", hartid);
//     if STARTED.load(Ordering::SeqCst) == false {
//         boot_all_harts(hartid);
//         STARTED.store(true, Ordering::SeqCst);
//     }
//     shutdown();
// }

