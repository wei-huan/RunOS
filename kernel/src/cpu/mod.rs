mod cpu;
mod cpus;

pub use cpu::*;
pub use cpus::*;

use crate::dt::CPU_NUMS;
#[cfg(feature = "opensbi")]
use crate::opensbi::hart_start;
#[cfg(feature = "rustsbi")]
use crate::rustsbi::send_ipi;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub static SMP_START: AtomicBool = AtomicBool::new(false);
pub static BOOT_HARTID: AtomicUsize = AtomicUsize::new(0);

#[cfg(feature = "platform-qemu")]
pub fn boot_all_harts(my_hartid: usize) {
    extern "C" {
        fn _start();
    }
    BOOT_HARTID.store(my_hartid, Ordering::Relaxed);
    SMP_START.store(true, Ordering::Relaxed);
    let ncpu = CPU_NUMS.load(Ordering::Acquire);
    for id in (0..ncpu).filter(|i| *i != my_hartid) {
        // priv: 1 for supervisor; 0 for user;
        hart_start(id, _start as usize, 1).unwrap();
    }
}

#[cfg(feature = "platform-k210")]
pub fn boot_all_harts(my_hartid: usize) {
    BOOT_HARTID.store(my_hartid, Ordering::Relaxed);
    SMP_START.store(true, Ordering::Relaxed);
    let ncpu = CPU_NUMS.load(Ordering::Acquire);
    for i in 1..ncpu {
        let mask: usize = 1 << i;
        send_ipi(&mask as *const _ as usize);
    }
}
