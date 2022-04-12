mod context;
mod kernel_stack;
mod kernel_task;
mod pid;
mod recycle_allocator;
mod switch;
mod task;

pub use context::TaskContext;
pub use kernel_task::idle_task;
pub use pid::{pid_alloc, PidHandle};
pub use switch::__switch;
pub use task::{TaskControlBlock, TaskStatus};

use crate::cpu::take_current_task;
use crate::scheduler::{__schedule_new, add_task, INITPROC};
use alloc::sync::Arc;

/// 将当前任务退出重新加入就绪队列，并调度新的任务
pub fn exit_current_and_run_next(exit_code: i32) -> ! {
    // log::debug!("exit");
    // take from Processor
    let task = take_current_task().unwrap();
    // **** access current TCB exclusively
    let mut task_inner = task.inner_exclusive_access();
    // Change status to Ready
    task_inner.task_status = TaskStatus::Zombie;
    // Record exit code
    task_inner.exit_code = exit_code;
    // do not move to its parent but under initproc
    // ++++++ access initproc TCB exclusively
    {
        let mut initproc_inner = INITPROC.inner_exclusive_access();
        for child in task_inner.children.iter() {
            child.inner_exclusive_access().parent = Some(Arc::downgrade(&INITPROC));
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
    let mut schedule_task = TaskContext::zero_init();
    unsafe { __schedule_new(&mut schedule_task as *const _) };
}

pub fn suspend_current_and_run_next() -> ! {
    // log::debug!("suspend");
    // There must be an application running.
    let task = take_current_task().unwrap();
    // ---- access current TCB exclusively
    let mut task_inner = task.inner_exclusive_access();
    // Change status to Ready
    task_inner.task_status = TaskStatus::Ready;
    // Reset Task context
    let kernel_stack_top = task.kernel_stack.get_top();
    task_inner.task_cx = TaskContext::goto_trap_return(kernel_stack_top);
    // drop inner
    drop(task_inner);
    // Push back to ready queue.
    add_task(task);
    // jump to schedule cycle
    let mut schedule_task = TaskContext::zero_init();
    unsafe { __schedule_new(&mut schedule_task as *const _) };
}
