mod context;
mod kernel_stack;
mod idle_task;
mod pid;
mod recycle_allocator;
mod task;

pub use context::TaskContext;
pub use idle_task::idle_task;
pub use pid::{pid_alloc, PidHandle};
pub use task::{TaskControlBlock, TaskControlBlockInner, TaskStatus};

use crate::cpu::{take_current_task, hart_id};
use crate::scheduler::{add_task_to_designate_queue, save_current_and_back_to_schedule, INITPROC};
use alloc::sync::Arc;

pub fn suspend_current_and_run_next() {
    // log::debug!("suspend");
    // There must be an application running.
    let task = take_current_task().unwrap();
    // ---- access current TCB exclusively
    let mut task_inner = task.acquire_inner_lock();
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    // Change status to Ready
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);
    // ---- release current PCB

    // push back to ready queue.
    add_task_to_designate_queue(task, hart_id());
    // jump to scheduling cycle
    // log::debug!("suspend 1");
    save_current_and_back_to_schedule(task_cx_ptr);
    // log::debug!("back to suspend");
}

/// 将当前任务退出重新加入就绪队列，并调度新的任务
pub fn exit_current_and_run_next(exit_code: i32) {
    // take from Processor
    let task = take_current_task().unwrap();
    // **** access current TCB exclusively
    let mut task_inner = task.acquire_inner_lock();
    // Change status to Ready
    task_inner.task_status = TaskStatus::Zombie;
    // Record exit code
    task_inner.exit_code = exit_code;
    // do not move to its parent but under initproc
    // ++++++ access initproc TCB exclusively
    // pid 0 for initproc , pid 1 for user_shell
    if task.pid.0 >= 2 {
        let mut initproc_inner = INITPROC.acquire_inner_lock();
        for child in task_inner.children.iter() {
            child.acquire_inner_lock().parent = Some(Arc::downgrade(&INITPROC));
            initproc_inner.children.push(child.clone());
        }
    }
    // ++++++ release parent PCB
    task_inner.children.clear();
    // drop inner
    task_inner.addrspace.recycle_data_pages();
    drop(task_inner);
    // **** release current TCB
    // drop task manually to maintain rc correctly
    drop(task);
    // jump to schedule cycle
    // we do not have to save task context
    let mut _unused = TaskContext::zero_init();
    save_current_and_back_to_schedule(&mut _unused as *mut _);
}
