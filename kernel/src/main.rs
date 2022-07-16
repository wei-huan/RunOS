#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(step_trait)]

extern crate alloc;
extern crate fdt;
extern crate owo_colors;
extern crate runfs;

#[macro_use]
mod console;
mod config;
mod cpu;
mod drivers;
mod dt;
mod fpu;
mod fs;
mod lang_items;
mod logger;
mod logo;
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

// #[cfg(all(feature = "qemu", feature = "opensbi"))]
use crate::cpu::SMP_START;
// #[cfg(all(feature = "qemu", feature = "opensbi"))]
use crate::cpu::hart_id;
use crate::owo_colors::OwoColorize;
use core::arch::global_asm;
use core::sync::atomic::Ordering;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("firm_apps.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}

// qemu opensbi
#[cfg(all(feature = "platform-qemu", feature = "opensbi"))]
#[no_mangle]
fn os_main(hartid: usize, dtb_ptr: *mut u8) {
    if !SMP_START.load(Ordering::Acquire) {
        clear_bss();
        println!("here 0");
        trap::init();
        dt::init(dtb_ptr);
        mm::boot_init();
        fpu::init();
        logo::show();
        scheduler::add_initproc();
        logger::init();
        logger::show_basic_info();
        log::info!(
            "{}",
            alloc::format!("Main Hart {} successfully booted", hart_id()).green()
        );
        fs::list_apps();
        timer::init();
        // SMP_START will turn to true in this function
        cpu::boot_all_harts(hartid);
        scheduler::schedule();
    } else {
        trap::init();
        mm::init();
        timer::init();
        log::info!(
            "{}",
            alloc::format!("Hart {} successfully booted", hart_id()).green()
        );
        scheduler::schedule();
    }
}


// k210 rustsbi
#[cfg(all(feature = "platform-k210", feature = "rustsbi"))]
#[no_mangle]
fn os_main(hartid: usize, dtb_ptr: *mut u8) {
    if hartid == 0 {
        clear_bss();
        trap::init();
        dt::init(dtb_ptr);
        mm::boot_init();
        fpu::init();
        // fs::init_rootfs();
        logo::show();
        logger::init();
        logger::show_basic_info();
        scheduler::add_initproc();
        log::info!(
            "{}",
            alloc::format!("Main Hart {} successfully booted", hart_id()).green()
        );
        // fs::list_apps();
        timer::init();
        // SMP_START will turn to true in this function
        // cpu::boot_all_harts(hartid);
        scheduler::schedule();
    } else {
        trap::init();
        mm::init();
        fpu::init();
        timer::init();
        log::info!(
            "{}",
            alloc::format!("Hart {} successfully booted", hart_id()).green()
        );
        scheduler::schedule();
    }
}


// // qemu rustsbi
// #[no_mangle]
// #[cfg(all(feature = "k210", feature = "rustsbi"))]
// fn os_main(hartid: usize, dtb_ptr: *mut u8) {
//     if hartid == 0 {
//         clear_bss();
//         // println!("here 0");
//         trap::init();
//         dt::init(dtb_ptr);
//         mm::boot_init();
//         // fs::init_rootfs();
//         logo::show();
//         logger::init();
//         logger::show_basic_info();
//         fs::list_apps();
//         timer::init();
//         scheduler::add_initproc();
//         // SMP_START will turn to true in this function
//         cpu::boot_all_harts(hartid);
//         // log::info!("here 4");
//         scheduler::schedule();
//     } else {
//         log::info!(
//             "{}",
//             alloc::format!("Hart {} successfully booted", hart_id()).green()
//         );
//         trap::init();
//         mm::init();
//         timer::init();
//         scheduler::schedule();
//     }
// }

// // qemu rustsbi
// #[cfg(all(feature = "k210", feature = "opensbi"))]
// #[no_mangle]
// fn os_main(hartid: usize, dtb_ptr: *mut u8) {
//     if hartid == 0 {
//         clear_bss();
//         println!("here 0");
//         dt::init(dtb_ptr);
//         logger::init();
//         mm::boot_init();
//         logger::show_machine_sbi_os_info();
//         // scheduler::add_apps();
//         trap::init();
//         // timer::init();
//         // SMP_START will turn to true in this function
//         cpu::boot_all_harts(hartid);
//         println!("here 1");
//         loop {}
//     } else {
//         trap::init();
//         mm::init();
//         timer::init();
//         println!("here 2");
//         loop {}
//     }
// }

// qemu opensbi
// #[cfg(all(feature = "qemu", feature = "opensbi"))]
// #[no_mangle]
// fn os_main(hartid: usize, dtb_ptr: *mut u8) {
//     // use alloc::sync::Arc;
//     // use drivers::BLOCK_DEVICE;
//     // use runfs::RunFileSystem;
//     // use spin::rwlock::RwLock;
//     if !SMP_START.load(Ordering::Acquire) {
//         clear_bss();
//         // println!("here 0");
//         trap::init();
//         dt::init(dtb_ptr);
//         mm::boot_init();
//         logo::show();
//         scheduler::add_initproc();
//         logger::init();
//         logger::show_basic_info();
//         // let runfs:Arc<RwLock<RunFileSystem>> = Arc::new(RwLock::new(RunFileSystem::new(BLOCK_DEVICE.clone())));
//         // let root_dir = Arc::new(runfs.read().root_vfile(&runfs));
//         // println!("runfs: {:#?}", runfs.read().bpb());
//         // let ls = root_dir.ls();
//         // println!("ls: {:#?}", ls);
//         // let file = root_dir.find_vfile_bypath("initproc").unwrap();
//         // println!("file: {:#?}", file.name());
//         // timer::init();
//         // SMP_START will turn to true in this function
//         // cpu::boot_all_harts(hartid);
//         scheduler::schedule();
//         loop {}
//     }
// }
