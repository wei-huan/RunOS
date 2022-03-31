#![allow(unused)]

mod cpu;
mod cpus;

pub use cpu::{current_kstack_top, current_token, current_trap_cx, current_user_token, Cpu};
pub use cpus::{cpu_id, current_task, take_current_task, take_my_cpu, CPUS};

use crate::dt::CPU_NUMS;
#[cfg(feature = "opensbi")]
use crate::opensbi::hart_start;
#[cfg(feature = "rustsbi")]
use crate::rustsbi::hart_start;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub static SMP_START: AtomicBool = AtomicBool::new(false);
pub static BOOT_HARTID: AtomicUsize = AtomicUsize::new(0);

#[cfg(feature = "rustsbi")]
pub fn boot_all_harts() {
    SMP_START.store(true, Ordering::Relaxed);
}

// #[cfg(feature = "opensbi")]
pub fn boot_all_harts(my_hartid: usize) {
    extern "C" {
        fn _start();
    }
    BOOT_HARTID.store(my_hartid, Ordering::Relaxed);
    SMP_START.store(true, Ordering::Relaxed);
    let ncpu = CPU_NUMS.load(Ordering::Acquire);
    for i in 0..ncpu {
        if i != my_hartid {
            // priv: 1 for supervisor; 0 for user;
            hart_start(i, _start as usize, 1).unwrap();
        }
    }
}
