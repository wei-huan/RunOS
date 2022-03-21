mod cpu;
mod cpus;

pub use cpu::{current_trap_cx, current_user_token, Cpu};
pub use cpus::{cpu_id, current_task, take_current_task, CPUS, schedule_new, exit_back_to_schedule, suspend_back_to_schedule};

use crate::task::{fetch_task, idle_task, TaskContext, TaskStatus};

#[no_mangle]
pub fn schedule() {
    log::debug!("Starting scheduling");
    loop {
        if let Some(task) = fetch_task() {
            let mut cpu = CPUS[cpu_id()].exclusive_access();
            // access coming task PCB exclusively
            let mut task_inner = task.inner_exclusive_access();
            let next_task_cx_ptr = &task_inner.task_cx as *const TaskContext;
            task_inner.task_status = TaskStatus::Running;
            drop(task_inner);
            // release coming task PCB manually
            cpu.current = Some(task);
            // release processor manually
            drop(cpu);
            // schedule new task
            schedule_new(next_task_cx_ptr);
        } else {
            idle_task();
        }
    }
}
