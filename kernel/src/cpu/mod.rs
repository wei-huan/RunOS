mod cpu;
mod cpus;

pub use cpu::{
    current_kstack_top, current_hstack_top, current_stack_top, current_token, current_trap_cx, current_user_token, Cpu,
};
pub use cpus::{hart_id, current_task, take_current_task, take_my_cpu, CPUS};

use crate::dt::CPU_NUMS;
#[cfg(feature = "opensbi")]
use crate::opensbi::{hart_start, remote_fence_i};
#[cfg(feature = "rustsbi")]
use crate::rustsbi::send_ipi;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub static SMP_START: AtomicBool = AtomicBool::new(false);
pub static BOOT_HARTID: AtomicUsize = AtomicUsize::new(0);


#[cfg(feature = "opensbi")]
pub fn boot_all_harts(my_hartid: usize) {
    // #[cfg(feature = "qemu")]
    extern "C" {
        fn _start();
    }
    // #[cfg(feature = "k210")]
    // extern "C" {
    //     fn _warm_start();
    // }
    BOOT_HARTID.store(my_hartid, Ordering::Relaxed);
    SMP_START.store(true, Ordering::Relaxed);
    // remote_fence_i();
    let ncpu = CPU_NUMS.load(Ordering::Acquire);
    for i in 0..ncpu {
        if i != my_hartid {
            // priv: 1 for supervisor; 0 for user;
            #[cfg(feature = "qemu")]
            hart_start(i, _start as usize, 1).unwrap();
            #[cfg(feature = "k210")]
            hart_start(i, _warm_start as usize, 1).unwrap();
        }
    }
}

#[cfg(not(feature = "opensbi"))]
pub fn boot_all_harts(my_hartid: usize) {
    let ncpu = CPU_NUMS.load(Ordering::Acquire);
    for i in 1..ncpu {
        let mask: usize = 1 << i;
        send_ipi(&mask as *const _ as usize);
    }
}
