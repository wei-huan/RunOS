mod cpu;
mod cpus;

pub use cpu::{
    current_hstack_top, current_stack_top, current_token, current_trap_cx, current_user_token, Cpu,
};
pub use cpus::{current_task, hart_id, take_current_task, take_my_cpu, CPUS};

use crate::dt::CPU_NUMS;
#[cfg(feature = "opensbi")]
use crate::opensbi::hart_start;
#[cfg(feature = "rustsbi")]
use crate::rustsbi::send_ipi;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub static SMP_START: AtomicBool = AtomicBool::new(false);
pub static BOOT_HARTID: AtomicUsize = AtomicUsize::new(0);

#[cfg(all(feature = "opensbi", feature = "qemu"))]
pub fn boot_all_harts(my_hartid: usize) {
    extern "C" {
        fn _start();
    }
    BOOT_HARTID.store(my_hartid, Ordering::Relaxed);
    SMP_START.store(true, Ordering::Relaxed);
    // remote_fence_i();
    let ncpu = CPU_NUMS.load(Ordering::Acquire);
    for i in 0..ncpu {
        if i != my_hartid {
            // priv: 1 for supervisor; 0 for user;
            hart_start(i, _start as usize, 1).unwrap();
        }
    }
}

// #[cfg(all(feature = "opensbi", feature = "k210"))]
// pub fn boot_all_harts(my_hartid: usize) {
//     #[cfg(feature = "k210")]
//     extern "C" {
//         fn _warm_start();
//     }
//     BOOT_HARTID.store(my_hartid, Ordering::Relaxed);
//     SMP_START.store(true, Ordering::Relaxed);
//     let ncpu = CPU_NUMS.load(Ordering::Acquire);
//     for i in 0..ncpu {
//         if i != my_hartid {
//             // priv: 1 for supervisor; 0 for user;
//             hart_start(i, _warm_start as usize, 1).unwrap();
//         }
//     }
// }

// #[cfg(feature = "k210")]
pub fn boot_all_harts(my_hartid: usize) {
    BOOT_HARTID.store(my_hartid, Ordering::Relaxed);
    SMP_START.store(true, Ordering::Relaxed);
    let ncpu = CPU_NUMS.load(Ordering::Acquire);
    for i in 1..ncpu {
        let mask: usize = 1 << i;
        send_ipi(&mask as *const _ as usize);
    }
}
