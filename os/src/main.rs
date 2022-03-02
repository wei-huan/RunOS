#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(const_fn_trait_bound)]

extern crate alloc;
extern crate spin;
#[macro_use]
mod console;
mod boards;
mod config;
mod dt;
mod lang_items;
mod logging;
mod mm;
mod opensbi;
mod proc;
mod rustsbi;
mod sync;
mod timer;
mod trap;

use crate::opensbi::{send_ipi, shutdown};
use core::arch::global_asm;
use core::sync::atomic::{AtomicBool, Ordering};
use sync::intr_off;
use sync::Mutex;

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

static STARTED: Mutex<bool> = Mutex::new(false);
#[no_mangle]
fn os_main(hartid: usize, fdt: *mut u8) {
    if hartid == 0 {
        intr_off();
        clear_bss();
        mm::init();
        dt::init(hartid, fdt);
        dt::fdt_print();
        trap::init();
        boot_all_harts(hartid);
        // let mut a = STARTED.lock().deref_mut();
        // STARTED.store(true, Ordering::Release);
        // println!("cpu0");
        // while STARTED.compare_exchange(false, true, Ordering::SeqCst, Ordering::Acquire) != Ok(false) {};
    } else {
        intr_off();
        trap::init();
        // println!("cpu1");
        // while !STARTED.load(Ordering::Acquire) {};
    }
    shutdown();
}

// lazy_static! {
//     pub static ref first_cpu: RefCell<bool> = RefCell::new(true);
// }

// #[no_mangle]
// fn os_main(hartid: usize, fdt: *mut u8) {
//     if *first_cpu {
//         intr_off();
//         clear_bss();
//         mm::init();
//         dt::init(hartid, fdt);
//         trap::init();
//         unsafe {*first_cpu = false;}
//         boot_all_harts(hartid);
//         // println!("cpu0");
//     } else {
//         intr_off();
//         trap::init();
//         // println!("cpu1");
//     }
//     shutdown();
// }

// fn boot_all_harts() {
//     send_ipi((1 << 1) as *const usize as usize);
// }

// static STARTED: AtomicBool = AtomicBool::new(false);
// #[no_mangle]
// fn os_main(hartid: usize) {
//     if hartid == 0 {
//         clear_bss();
//         mm::init();
//         trap::init();
//         boot_all_harts();
//         // while STARTED.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) == Ok(false) {};
//     } else {
//         trap::init();
//     }
//     // while !STARTED.load(Ordering::Relaxed) {}
//     loop {
//         println!("cpu{} get process {}", hartid, PROCESS.lock().get_pid());
//         let start = get_time();
//         while get_time() - start <= 0x1000000 {}
//     }
//     shutdown();
// }
