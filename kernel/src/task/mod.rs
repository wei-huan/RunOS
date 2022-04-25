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
    let mut task_inner = task.acquire_inner_lock();
    // Change status to Ready
    task_inner.task_status = TaskStatus::Zombie;
    // Record exit code
    task_inner.exit_code = exit_code;
    // do not move to its parent but under initproc
    // ++++++ access initproc TCB exclusively
    if task.pid.0 != 1 {
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
    let mut schedule_task = TaskContext::zero_init();
    unsafe { __schedule_new(&mut schedule_task as *const _) };
}

pub fn suspend_current_and_run_next() -> ! {
    // There must be an application running.
    let task = take_current_task().unwrap();
    // ---- access current TCB exclusively
    let mut task_inner = task.acquire_inner_lock();
    // Change status to Ready
    task_inner.task_status = TaskStatus::Ready;
    // Reset Task context
    // 直接传栈底可能有bug，反正不健壮，以后要改
    let kernel_stack_top = task.kernel_stack.get_top();
    // 应该改成上一个栈帧的位置
    task_inner.task_cx = TaskContext::goto_trap_return(kernel_stack_top);
    // drop inner
    drop(task_inner);
    // Push back to ready queue.
    add_task(task);
    // jump to schedule cycle
    let mut schedule_task = TaskContext::zero_init();
    unsafe { __schedule_new(&mut schedule_task as *const _) };
}
