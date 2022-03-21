mod round_robin;

use crate::fs::{open_file, OpenFlags, ROOT_INODE};
use crate::task::TaskControlBlock;
use alloc::sync::Arc;
use lazy_static::*;
use round_robin::RoundRobinScheduler;
use spin::Mutex;

pub trait Scheduler: Send {
    fn schedule(&mut self);
    fn add_to_readyqueue(&mut self, task: Arc<TaskControlBlock>);
    fn fetch_from_readyqueue(&mut self) -> Option<Arc<TaskControlBlock>>;
    // fn block(&self, tid: Tid);
    // fn unblock(&self, token: WakeToken);
    // fn active_on_cpu(&self) -> Option<Arc<SpinMutex<Task, SameHartDeadlockDetection>>>;
}

lazy_static! {
    pub static ref SCHEDULER: Mutex<RoundRobinScheduler> = Mutex::new(RoundRobinScheduler::new());
}

pub fn schedule() {
    SCHEDULER.lock().schedule()
}

pub fn add_task(task: Arc<TaskControlBlock>) {
    SCHEDULER.lock().add_to_readyqueue(task);
}

pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    SCHEDULER.lock().fetch_from_readyqueue()
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
