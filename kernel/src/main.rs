#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(step_trait)]

extern crate alloc;
extern crate fat32;
extern crate fdt;

#[macro_use]
mod console;
mod config;
mod cpu;
mod drivers;
mod dt;
mod fs;
mod lang_items;
mod logger;
mod mm;
mod logo;

#[cfg(feature = "opensbi")]
mod opensbi;
mod platform;
#[cfg(feature = "rustsbi")]
mod rustsbi;
mod scheduler;
mod sync;
mod syscall;
mod task;
mod timer;
mod trap;
mod utils;

// #[cfg(all(feature = "qemu", feature = "opensbi"))]
use crate::cpu::SMP_START;
use core::arch::global_asm;
// #[cfg(all(feature = "qemu", feature = "opensbi"))]
use core::sync::atomic::Ordering;

global_asm!(include_str!("entry.asm"));
// global_asm!(include_str!("firm_apps.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}

// qemu opensbi
// #[cfg(all(feature = "qemu", feature = "opensbi"))]
#[no_mangle]
fn os_main(hartid: usize, dtb_ptr: *mut u8) {
    if !SMP_START.load(Ordering::Acquire) {
        clear_bss();
        // println!("here 0");
        trap::init();
        dt::init(dtb_ptr);
        mm::boot_init();
        // fs::init_rootfs();
        scheduler::add_initproc();
        logo::show();
        logger::init();
        logger::show_basic_info();
        fs::list_apps();
        timer::init();
        // SMP_START will turn to true in this function
        cpu::boot_all_harts(hartid);
        // log::info!("here 4");
        scheduler::schedule();
    } else {
        // log::info!("here 5");
        trap::init();
        mm::init();
        timer::init();
        scheduler::schedule();
    }
}

// k210 rustsbi
#[no_mangle]
#[cfg(all(feature = "k210", feature = "rustsbi"))]
fn os_main(hartid: usize, dtb_ptr: *mut u8) {
    if hartid == 0 {
        clear_bss();
        println!("here 0");
        trap::init();
        dt::init(dtb_ptr);
        mm::boot_init();
        // fs::init_rootfs();
        logger::init();
        logger::show_basic_info();
        fs::list_apps();
        scheduler::add_initproc();
        timer::init();
        // SMP_START will turn to true in this function
        cpu::boot_all_harts(hartid);
        // log::info!("here 4");
        scheduler::schedule();
    } else {
        // log::info!("here 5");
        trap::init();
        mm::init();
        timer::init();
        scheduler::schedule();
    }
}

// qemu rustsbi
#[no_mangle]
#[cfg(all(feature = "k210", feature = "opensbi"))]
fn os_main(hartid: usize, dtb_ptr: *mut u8) {
    if hartid == 0 {
        clear_bss();
        println!("here 0");
        dt::init(dtb_ptr);
        logger::init();
        mm::boot_init();
        logger::show_machine_sbi_os_info();
        // scheduler::add_apps();
        trap::init();
        // timer::init();
        // SMP_START will turn to true in this function
        cpu::boot_all_harts(hartid);
        println!("here 1");
        loop {}
    } else {
        trap::init();
        mm::init();
        timer::init();
        println!("here 2");
        loop {}
    }
}
