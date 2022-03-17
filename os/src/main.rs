#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate fdt;
// #[cfg(feature = "platform-k210")]
// #[path = "platform/k210.rs"]
// mod platform;
// #[cfg(not(any(feature = "platform-k210")))]
// #[path = "platform/qemu.rs"]

#[macro_use]
mod console;
mod config;
mod cpu;
mod drivers;
mod dt;
mod fs;
mod lang_items;
mod logging;
mod mm;
mod opensbi;
mod platform;
mod task;
mod sync;
mod syscall;
mod timer;
mod trap;
mod utils;

use crate::opensbi::hart_start;
use core::arch::global_asm;
use core::sync::atomic::{AtomicBool, Ordering};
use dt::{CPU_NUMS, TIMER_FREQ};
use log::*;
use opensbi::{impl_id, impl_version, spec_version};

global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}

fn boot_all_harts(hartid: usize) {
    extern "C" {
        fn _start();
    }
    let ncpu = CPU_NUMS.load(Ordering::Acquire);
    for i in 0..ncpu {
        if i != hartid {
            // priv: 1 for supervisor; 0 for user;
            hart_start(i, _start as usize, 1).unwrap();
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
        mm::boot_init();
        timer::init();
        let n_cpus = CPU_NUMS.load(Ordering::Relaxed);
        let timebase_frequency = TIMER_FREQ.load(Ordering::Relaxed);
        let (impl_major, impl_minor) = {
            let version = impl_version();
            // This is how OpenSBI encodes their version, hopefully will be the same
            // between others
            (version >> 16, version & 0xFFFF)
        };
        let (spec_major, spec_minor) = {
            let version = spec_version();
            (version.major, version.minor)
        };

        info!("MyOS version {}", env!("CARGO_PKG_VERSION"));

        info!("=== Machine Info ===");
        info!(" Total CPUs: {}", n_cpus);
        info!(" Timer Clock: {}Hz", timebase_frequency);

        info!("=== SBI Implementation ===");
        info!(
            " Implementor: {:?} (version: {}.{})",
            impl_id(),
            impl_major,
            impl_minor
        );
        info!(" Spec Version: {}.{}", spec_major, spec_minor);

        info!("=== MyOS Info ===");
        task::add_apps();
        START.store(true, Ordering::Relaxed);
        boot_all_harts(hartid);
        cpu::run_processes();
    } else {
        trap::init();
        mm::init();
        timer::init();
        cpu::run_processes();
    }
    unreachable!();
}
