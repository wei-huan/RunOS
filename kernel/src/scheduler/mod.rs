mod round_robin;

use crate::fs::{open_file, OpenFlags, ROOT_INODE};
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

pub fn add_apps() {
    for app in ROOT_INODE.ls() {
        if let Some(app_inode) = open_file(app.as_str(), OpenFlags::RDONLY) {
            let elf_data = app_inode.read_all();
            let new_task = TaskControlBlock::new(elf_data.as_slice());
            add_task(Arc::new(new_task));
        }
    }
}

global_asm!(include_str!("schedule.S"));
extern "C" {
    pub fn __schedule_new(next_task_cx_ptr: *const TaskContext) -> !;
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

