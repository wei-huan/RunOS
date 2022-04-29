mod round_robin;

use crate::fs::{open, DiskInodeType, OpenFlags};
use crate::task::{TaskContext, TaskControlBlock};
use alloc::sync::Arc;
use core::arch::global_asm;
use lazy_static::*;
use round_robin::RoundRobinScheduler;

pub trait Scheduler {
    fn schedule(&self) -> !;
    fn add_task(&self, task: Arc<TaskControlBlock>);
    fn fetch_task(&self) -> Option<Arc<TaskControlBlock>>;
}

lazy_static! {
    pub static ref SCHEDULER: RoundRobinScheduler = RoundRobinScheduler::new();
}

pub fn schedule() -> ! {
    SCHEDULER.schedule()
}

pub fn add_task(task: Arc<TaskControlBlock>) {
    SCHEDULER.add_task(task);
}

global_asm!(include_str!("schedule.S"));
extern "C" {
    pub fn __schedule(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
    pub fn __schedule_new(next_task_cx_ptr: *const TaskContext) -> !;
}

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new({
        let inode = open("/", "initproc", OpenFlags::RDONLY, DiskInodeType::File).unwrap();
        let v = inode.read_all();
        TaskControlBlock::new(v.as_slice())
    });
}

pub fn add_initproc() {
    add_task(INITPROC.clone());
}

#[inline(always)]
pub fn go_to_schedule() -> ! {
    let schedule_task = TaskContext::goto_schedule();
    unsafe { __schedule_new(&schedule_task as *const TaskContext) };
}

#[inline(always)]
pub fn save_current_and_goto_schedule(current_task_cx_ptr: *mut TaskContext) {
    let schedule_task = TaskContext::goto_schedule();
    unsafe { __schedule(current_task_cx_ptr, &schedule_task as *const TaskContext) };
}
