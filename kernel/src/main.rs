#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate fdt;

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
mod scheduler;
mod sync;
mod syscall;
mod task;
mod timer;
mod trap;
mod utils;

use crate::cpu::SMP_START;
use core::{arch::global_asm};
use core::sync::atomic::Ordering;
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

#[no_mangle]
fn os_main(hartid: usize, fdt: *mut u8) {
    if !SMP_START.load(Ordering::Acquire) {
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

        scheduler::add_apps();
        cpu::boot_all_harts(hartid);
        scheduler::schedule();
    } else {
        trap::init();
        mm::init();
        timer::init();
        scheduler::schedule();
    }
}
