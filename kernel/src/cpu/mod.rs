mod cpu;
mod cpus;

pub use cpu::{current_trap_cx, current_user_token, Cpu};
pub use cpus::{
    cpu_id, current_task, exit_back_to_schedule, schedule_new, suspend_back_to_schedule,
    take_current_task, take_my_cpu, CPUS,
};

use crate::dt::CPU_NUMS;
use crate::opensbi::hart_start;
use core::sync::atomic::{AtomicBool, Ordering};

pub static SMP_START: AtomicBool = AtomicBool::new(false);
pub fn boot_all_harts(hartid: usize) {
    extern "C" {
        fn _start();
    }
    SMP_START.store(true, Ordering::Relaxed);
    let ncpu = CPU_NUMS.load(Ordering::Acquire);
    for i in 0..ncpu {
        if i != hartid {
            // priv: 1 for supervisor; 0 for user;
            hart_start(i, _start as usize, 1).unwrap();
        }
    }
}

// use crate::task::{fetch_task, idle_task, TaskContext, TaskStatus};

// #[no_mangle]
// pub fn schedule() {
//     log::debug!("Starting scheduling");
//     loop {
//         if let Some(task) = fetch_task() {
//             let mut cpu = take_my_cpu();
//             // access coming task PCB exclusively
//             let mut task_inner = task.inner_exclusive_access();
//             let next_task_cx_ptr = &task_inner.task_cx as *const TaskContext;
//             task_inner.task_status = TaskStatus::Running;
//             drop(task_inner);
//             // release coming task PCB manually
//             cpu.current = Some(task);
//             // release processor manually
//             drop(cpu);
//             // schedule new task
//             schedule_new(next_task_cx_ptr);
//         } else {
//             idle_task();
//         }
//     }
// }
