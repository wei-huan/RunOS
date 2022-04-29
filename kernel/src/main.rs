#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate fdt;
extern crate fat32;

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

use core::arch::global_asm;
#[cfg(not(feature = "k210"))]
use crate::cpu::SMP_START;
#[cfg(not(feature = "k210"))]
use core::sync::atomic::Ordering;
use riscv::asm::ebreak;

global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}

// qemu opensbi
#[no_mangle]
// #[cfg(all(feature = "qemu", feature = "opensbi"))]
fn os_main(hartid: usize, dtb_ptr: *mut u8) {
    if !SMP_START.load(Ordering::Acquire) {
        clear_bss();
        trap::init();
        dt::init(dtb_ptr);
        mm::boot_init();
        fs::init_rootfs();
        logger::init();
        logger::show_basic_info();
        fs::list_apps();
        scheduler::add_initproc();
        timer::init();
        // SMP_START will turn to true in this function
        cpu::boot_all_harts(hartid);
        scheduler::schedule();
    } else {
        trap::init();
        mm::init();
        timer::init();
        scheduler::schedule();
    }
}

// k210 opensbi
#[no_mangle]
#[cfg(all(feature = "k210", feature = "opensbi"))]
fn os_main(hartid: usize, dtb_ptr: *mut u8) {
    clear_bss();
    // println!("here 0");
    trap::init();
    dt::init(dtb_ptr);
    mm::boot_init();
    logger::init();
    logger::show_machine_sbi_os_info();
    scheduler::add_apps();
    timer::init();
    log::debug!("here 1");
    // SMP_START will turn to true in this function
    cpu::boot_all_harts(hartid);
    log::debug!("here 2");
    scheduler::schedule();
}

// SMP 其他核的启动主函数
#[no_mangle]
#[cfg(all(feature = "k210", feature = "opensbi"))]
fn os_sub_main(hartid: usize, dtb_ptr: *mut u8) {
    trap::init();
    timer::init();
    log::debug!("here 4");
    scheduler::schedule();
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
        logger::init();
        logger::show_machine_sbi_os_info();
        scheduler::add_apps();
        timer::init();
        // SMP_START will turn to true in this function
        cpu::boot_all_harts(hartid);
        // log::debug!("here 4");
        scheduler::schedule();
    } else {
        trap::init();
        mm::init();
        log::debug!("here 5");
        timer::init();
        scheduler::schedule();
    }
}
