mod context;
mod kernel_stack;
mod kernel_task;
mod manager;
mod pid;
mod recycle_allocator;
mod signal;
mod switch;
mod task;

pub use context::TaskContext;
pub use kernel_task::idle_task;
pub use manager::fetch_task;
pub use pid::{pid_alloc, PidHandle};
pub use switch::__switch;
pub use task::{TaskControlBlock, TaskStatus};
pub use manager::add_task;

use crate::cpu::{exit_back_to_schedule, suspend_back_to_schedule, take_current_task};
use crate::fs::{open_file, OpenFlags, ROOT_INODE};
use crate::mm::kernel_token;
use crate::trap::{user_trap_handler, TrapContext};
use alloc::sync::Arc;
use lazy_static::*;

// pub fn add_apps() {
//     for app in ROOT_INODE.ls() {
//         if let Some(app_inode) = open_file(app.as_str(), OpenFlags::RDONLY) {
//             let elf_data = app_inode.read_all();
//             let new_task = TaskControlBlock::new(elf_data.as_slice());
//             add_task(Arc::new(new_task));
//         }
//     }
// }

/// 将当前任务退出重新加入就绪队列，并调度新的任务
// pub fn exit_current_and_run_next(exit_code: i32) {
//     // take from Processor
//     let task = take_current_task().unwrap();
//     // **** access current TCB exclusively
//     let mut inner = task.inner_exclusive_access();
//     // Change status to Ready
//     inner.task_status = TaskStatus::Ready;
//     // Record exit code
//     inner.exit_code = exit_code;
//     // Delete children task
//     inner.children.clear();
//     // Reset Task context
//     let kernel_stack_top = task.kernel_stack.get_top();
//     inner.task_cx = TaskContext::goto_trap_return(kernel_stack_top);
//     // Reset Trap context
//     let trap_cx = inner.get_trap_cx();
//     *trap_cx = TrapContext::app_init_context(
//         inner.entry_point,
//         inner.ustack_bottom,
//         kernel_token(),
//         kernel_stack_top,
//         user_trap_handler as usize,
//     );
//     // Clear bss section
//     inner.addrspace.clear_bss_pages();

//     drop(inner);
//     // push back to ready queue.
//     add_task(task);
//     // 回到调度程序
//     exit_back_to_schedule();
// }

/// 将当前任务退出重新加入就绪队列，并调度新的任务
pub fn exit_current_and_run_next(exit_code: i32) {
    // take from Processor
    let task = take_current_task().unwrap();
    // **** access current TCB exclusively
    let mut inner = task.inner_exclusive_access();
    // Change status to Zombie
    inner.task_status = TaskStatus::Zombie;
    // Record exit code
    inner.exit_code = exit_code;
    // do not move to its parent but under initproc

    // ++++++ access initproc TCB exclusively
    {
        let mut initproc_inner = INITPROC.inner_exclusive_access();
        for child in inner.children.iter() {
            child.inner_exclusive_access().parent = Some(Arc::downgrade(&INITPROC));
            initproc_inner.children.push(child.clone());
        }
    }
    // ++++++ release parent PCB

    inner.children.clear();
    // deallocate user space
    inner.addrspace.recycle_data_pages();
    drop(inner);
    // **** release current PCB
    // drop task manually to maintain rc correctly
    drop(task);
    // 回到调度程序
    exit_back_to_schedule();
}

pub fn suspend_current_and_run_next() {
    // There must be an application running.
    let task = take_current_task().unwrap();
    // ---- access current TCB exclusively
    let mut inner = task.inner_exclusive_access();
    let task_cx_ptr = &mut inner.task_cx as *mut TaskContext;
    // Change status to Ready
    inner.task_status = TaskStatus::Ready;
    drop(inner);
    // push back to ready queue.
    add_task(task);
    // jump to scheduling cycle
    suspend_back_to_schedule(task_cx_ptr);
}

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new({
        let inode = open_file("initproc", OpenFlags::RDONLY).unwrap();
        let v = inode.read_all();
        TaskControlBlock::new(v.as_slice())
    });
}

pub fn add_initproc() {
    add_task(INITPROC.clone());
}
