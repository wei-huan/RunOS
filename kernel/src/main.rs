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
mod platform;
#[cfg(not(any(feature = "rustsbi")))]
mod opensbi;
#[cfg(feature = "rustsbi")]
mod rustsbi;
mod scheduler;
mod sync;
mod syscall;
mod task;
mod timer;
mod trap;
mod utils;

use crate::cpu::SMP_START;
use core::arch::global_asm;
use core::sync::atomic::Ordering;
use dt::{CPU_NUMS, TIMER_FREQ};
use log::*;
#[cfg(not(any(feature = "rustsbi")))]
use opensbi::{impl_id, impl_version, spec_version};

global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}

#[cfg(feature = "qemu")]
#[no_mangle]
fn os_main(_hartid: usize, fdt: *mut u8) {
    if !SMP_START.load(Ordering::Acquire) {
        clear_bss();
        println!("fdt: 0x{:X}", fdt as usize);
        dt::init(fdt);
        logging::init();
        mm::boot_init();

        let n_cpus = CPU_NUMS.load(Ordering::Relaxed);
        let timebase_frequency = TIMER_FREQ.load(Ordering::Relaxed);

        info!("MyOS version {}", env!("CARGO_PKG_VERSION"));
        info!("=== Machine Info ===");
        info!(" Total CPUs: {}", n_cpus);
        info!(" Timer Clock: {}Hz", timebase_frequency);
        #[cfg(not(any(feature = "rustsbi")))]
        {
            info!("=== SBI Implementation ===");
            let (impl_major, impl_minor) = {
                let version = impl_version();
                (version >> 16, version & 0xFFFF)
            };
            let (spec_major, spec_minor) = {
                let version = spec_version();
                (version.major, version.minor)
            };
            info!(
                " Implementor: {:?} (version: {}.{})",
                impl_id(),
                impl_major,
                impl_minor
            );
            info!(" Spec Version: {}.{}", spec_major, spec_minor);
        }
        info!("=== MyOS Info ===");

        scheduler::add_apps();
        trap::init();
        timer::init();
        // SMP_START will turn to true in this function
        #[cfg(not(any(feature = "rustsbi")))]
        cpu::boot_all_harts(_hartid);
        scheduler::schedule();
    } else {
        trap::init();
        mm::init();
        timer::init();
        scheduler::schedule();
    }
}

// k210
#[cfg(not(any(feature = "qemu")))]
#[no_mangle]
fn os_main() {
    clear_bss();
    dt::init();
    logging::init();
    println!("here 0");
    mm::boot_init();
    println!("here 1");
    let n_cpus = CPU_NUMS.load(Ordering::Relaxed);
    let timebase_frequency = TIMER_FREQ.load(Ordering::Relaxed);
    println!("here 2");
    info!("MyOS version {}", env!("CARGO_PKG_VERSION"));
    info!("=== Machine Info ===");
    info!(" Total CPUs: {}", n_cpus);
    info!(" Timer Clock: {}Hz", timebase_frequency);
    #[cfg(not(any(feature = "rustsbi")))]
    {
        info!("=== SBI Implementation ===");
        let (impl_major, impl_minor) = {
            let version = impl_version();
            (version >> 16, version & 0xFFFF)
        };
        let (spec_major, spec_minor) = {
            let version = spec_version();
            (version.major, version.minor)
        };
        info!(
            " Implementor: {:?} (version: {}.{})",
            impl_id(),
            impl_major,
            impl_minor
        );
        info!(" Spec Version: {}.{}", spec_major, spec_minor);
    }
    info!("=== MyOS Info ===");

    scheduler::add_apps();
    trap::init();
    timer::init();
    scheduler::schedule();
}
