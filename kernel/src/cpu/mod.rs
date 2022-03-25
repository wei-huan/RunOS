#![allow(unused)]

mod cpu;
mod cpus;

pub use cpu::{current_trap_cx, current_user_token, Cpu};
pub use cpus::{cpu_id, current_task, take_current_task, take_my_cpu, CPUS};

use crate::dt::CPU_NUMS;
use core::sync::atomic::{AtomicBool, Ordering};

#[cfg(not(any(feature = "rustsbi")))]
use crate::opensbi::hart_start;

pub static SMP_START: AtomicBool = AtomicBool::new(false);

#[cfg(not(any(feature = "rustsbi")))]
pub fn boot_all_harts(my_hartid: usize) {
    extern "C" {
        fn _start();
    }
    SMP_START.store(true, Ordering::Relaxed);
    let ncpu = CPU_NUMS.load(Ordering::Acquire);
    for i in 0..ncpu {
        if i != my_hartid {
            // priv: 1 for supervisor; 0 for user;
            hart_start(i, _start as usize, 1).unwrap();
        }
    }
}
