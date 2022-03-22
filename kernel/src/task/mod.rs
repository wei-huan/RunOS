mod context;
mod kernel_stack;
mod kernel_task;
mod pid;
mod recycle_allocator;
mod signal;
mod switch;
mod task;

pub use context::TaskContext;
pub use kernel_task::idle_task;
pub use pid::{pid_alloc, PidHandle};
pub use switch::__switch;
pub use task::{TaskControlBlock, TaskStatus};

use crate::cpu::take_current_task;
// use crate::fs::{open_file, OpenFlags, ROOT_INODE};
use crate::mm::kernel_token;
use crate::trap::{user_trap_handler, TrapContext};
// use alloc::sync::Arc;
use crate::scheduler::{schedule, add_task, __save_current_tx};

/// 将当前任务退出重新加入就绪队列，并调度新的任务
pub fn exit_current_and_run_next(exit_code: i32) {
    // take from Processor
    let task = take_current_task().unwrap();
    // **** access current TCB exclusively
    let mut inner = task.inner_exclusive_access();
    // Change status to Ready
    inner.task_status = TaskStatus::Ready;
    // Record exit code
    inner.exit_code = exit_code;
    // Delete children task
    inner.children.clear();
    // Reset Task context
    let kernel_stack_top = task.kernel_stack.get_top();
    inner.task_cx = TaskContext::goto_trap_return(kernel_stack_top);
    // Reset Trap context
    let trap_cx = inner.get_trap_cx();
    *trap_cx = TrapContext::app_init_context(
        task.entry_point,
        inner.ustack_bottom,
        kernel_token(),
        kernel_stack_top,
        user_trap_handler as usize,
    );
    // Clear bss section
    inner.addrspace.clear_bss_pages();
    // drop inner
    drop(inner);
    // Push back to ready queue.
    add_task(task);
    // 回到调度程序
    schedule();
}

pub fn suspend_current_and_run_next() {
    // There must be an application running.
    let task = take_current_task().unwrap();
    // ---- access current TCB exclusively
    let mut inner = task.inner_exclusive_access();
    let task_cx_ptr = &mut inner.task_cx as *mut TaskContext;
    // Change status to Ready
    inner.task_status = TaskStatus::Ready;
    // drop inner
    drop(inner);
    // Push back to ready queue.
    add_task(task);
    // jump to scheduling cycle
    unsafe {
        __save_current_tx(task_cx_ptr);
    }
    schedule()
}
