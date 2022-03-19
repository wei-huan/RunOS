mod context;
mod kerneltask;
mod kernelstack;
mod manager;
mod pid;
mod task;
mod recyclealloc;
mod signal;
mod switch;

pub use context::TaskContext;
pub use kerneltask::idle_task;
pub use manager::fetch_task;
pub use pid::{pid_alloc, PidHandle};
pub use task::{TaskControlBlock, TaskStatus};
pub use switch::__switch;


use crate::trap::{TrapContext, user_trap_handler};
use crate::cpu::schedule_new;
use crate::cpu::take_current_task;
use crate::fs::{open_file, OpenFlags, ROOT_INODE};
use alloc::sync::Arc;
use manager::add_task;
use crate::mm::kernel_token;

pub fn add_apps() {
    for app in ROOT_INODE.ls() {
        if let Some(app_inode) = open_file(app.as_str(), OpenFlags::RDONLY) {
            let elf_data = app_inode.read_all();
            let new_task = TaskControlBlock::new(elf_data.as_slice());
            add_task(Arc::new(new_task));
        }
    }
}

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
    // Refresh Task context
    let kernel_stack_top =task.kernel_stack.get_top();
    inner.task_cx = TaskContext::goto_trap_return(kernel_stack_top);
    // Refresh Trap context
    let trap_cx = inner.get_trap_cx();
    *trap_cx = TrapContext::app_init_context(
        task.entry_point,
        inner.ustack_bottom,
        kernel_token(),
        kernel_stack_top,
        user_trap_handler as usize,
    );
    // deallocate user space
    // inner.addrspace.recycle_data_pages();
    drop(inner);
    add_task(task);
    // we do not have to save task context
    let mut _unused = TaskContext::zero_init();
    schedule_new(&mut _unused as *mut _);
}
