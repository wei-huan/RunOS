#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
mod console;
mod cpus;
mod boards;
mod lang_items;
mod config;
mod mm;
mod opensbi;
mod sync;

use core::arch::asm;
use core::arch::global_asm;
use core::sync::atomic::{AtomicBool, Ordering};
use crate::opensbi::{shutdown, send_ipi};
// use boards::CPU_NUM;
global_asm!(include_str!("entry.asm"));

#[no_mangle]
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}

// static STARTED: AtomicBool = AtomicBool::new(false);

// #[no_mangle]
// fn os_main(hartid: usize) {
//     if STARTED.load(Ordering::SeqCst) == false {
//         println!("cpu{}", hartid);
//         clear_bss();
//         // wake up;
//         for i in 0..2 {
//             if hartid != i {
//                 println!("wake hart{}", i);
//                 opensbi::send_ipi(i);
//             }
//         }
//         unsafe {asm!("fence");}
//         STARTED.store(true, Ordering::SeqCst);
//     } else {
//         println!("here waiting");
//         while !STARTED.load(Ordering::SeqCst) {}
//         unsafe {asm!("fence");}
//         println!("hart{} now wake", hartid);
//     }
//     loop {}
//         // panic!("from hart{}", hartid);
// }

static mut start: usize = 0;

#[no_mangle]
fn os_main(hartid: usize) {
    unsafe {
        if start == 0 {
            println!("cpu{}", hartid);
            clear_bss();
            for i in 0..4 {
                if i != hartid {
                    send_ipi(i);
                }
            }
            // let (err, val) = send_ipi(1);
            // // println!("err: {}, val {:X}", err, val);
            // send_ipi(2);
            // send_ipi(3);
            start = 1;
        } else {
            println!("cpu{}", hartid);
        }
    }
    loop{};
    shutdown();
}
