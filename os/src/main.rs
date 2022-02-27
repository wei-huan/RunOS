#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[macro_use]
mod console;
mod boards;
mod config;
mod lang_items;
mod mm;
mod cpus;
mod sync;
mod sbi;

use core::arch::asm;
use core::arch::global_asm;
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

static mut START: usize = 0;

// #[no_mangle]
// fn os_main(hartid: usize) {
//     unsafe {
//         if hartid == 0 {
//             clear_bss();
//             for i in 1..2 {
//                 let mask: usize = 1 << i;
//                 sbi::send_ipi(&mask as *const _ as usize);
//             }
//             asm!("fence");
//             START = 1;
//             println!("cpu{}", hartid);
//             loop{}
//         } else {
//             while START == 0 {}
//             asm!("fence");
//             println!("cpu{}", hartid);
//             panic!("from hart{}", hartid);
//         }
//     }
// }

#[no_mangle]
fn os_main(hartid: usize){
    unsafe {
        if START == 0 {
            println!("cpu{}", hartid);
            clear_bss();
            for i in 0..2 {
                if hartid != i {
                    println!("wake hart{}", i);
                    let mask: usize = 1 << i;
                    sbi::send_ipi(&mask as *const _ as usize);
                }
            }
            asm!("fence");
            START = 1;
        } else {
            while START == 0 {
            }
            asm!("fence");
            println!("hart{} now wake", hartid);
        }
        panic!("from hart{}", hartid);
    }
}


// #[no_mangle]
// fn os_main(hartid: usize) {
//     println!("Hello World!");
//     if hartid == 0 {
//         clear_bss();
//         // println!("cpu{}", hartid);
//         let mask: usize = 1 << 1;
//         sbi::send_ipi(&mask as *const _ as usize);
//         // loop{};
//     } else if hartid == 1 {
//         // println!("cpu{}", hartid);
//         let mask: usize = 1 << 2;
//         sbi::send_ipi(&mask as *const _ as usize);
//         loop{};
//     } else if hartid == 2 {
//         // println!("cpu{}", hartid);
//         let mask: usize = 1 << 3;
//         sbi::send_ipi(&mask as *const _ as usize);
//         loop{};
//     } else {
//         println!("cpu{}", hartid);
//         loop{};
//     }
// }
