mod cpu;
mod cpus;

pub use cpu::{current_trap_cx, current_user_token, Cpu};
pub use cpus::{cpu_id, current_task, take_current_task, CPUS, schedule_new};

use crate::task::{fetch_task, idle_task, TaskContext, TaskStatus, __switch};

pub fn schedule() {
    loop {
        let mut cpu = CPUS[cpu_id()].exclusive_access();
        if let Some(task) = fetch_task() {
            let kernel_task_cx_ptr = cpu.take_kernel_task_cx_ptr();
            // access coming task TCB exclusively
            let mut task_inner = task.inner_exclusive_access();
            let next_task_cx_ptr = &task_inner.task_cx as *const TaskContext;
            task_inner.task_status = TaskStatus::Running;
            drop(task_inner);
            // release coming task TCB manually
            cpu.current = Some(task);
            // release processor manually
            drop(cpu);
            unsafe {
                __switch(kernel_task_cx_ptr, next_task_cx_ptr);
            }
        } else {
            idle_task();
        }
    }
}
