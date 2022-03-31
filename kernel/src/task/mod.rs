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
use crate::scheduler::{add_task, schedule};
use riscv::register::sstatus::{SPP, set_spp};

/// 将当前任务退出重新加入就绪队列，并调度新的任务
pub fn exit_current_and_run_next(exit_code: i32) -> ! {
    // Set SPP as Supervisor
    unsafe {
        set_spp(SPP::Supervisor);
    }
    // take from Processor
    let task = take_current_task().unwrap();
    // **** access current TCB exclusively
    let mut task_inner = task.inner_exclusive_access();
    // Change status to Ready
    task_inner.task_status = TaskStatus::Ready;
    // Record exit code
    task_inner.exit_code = exit_code;
    // Delete children task
    task_inner.children.clear();
    // Clear bss section
    task_inner.addrspace.clear_bss_pages();
    // Reset Task context
    let kernel_stack_top = task.kernel_stack.get_top();
    task_inner.task_cx = TaskContext::goto_trap_return(kernel_stack_top);
    // Reset Trap context
    let trap_cx = task_inner.get_trap_cx();
    *trap_cx = TrapContext::app_init_context(
        task.entry_point,
        task_inner.ustack_bottom,
        kernel_token(),
        kernel_stack_top,
        user_trap_handler as usize,
    );
    // drop trap_cx
    drop(trap_cx);
    // drop inner
    drop(task_inner);
    // Push back to ready queue.
    add_task(task);
    // 回到调度程序
    schedule();
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
    // jump to scheduling cycle
    schedule()
}
