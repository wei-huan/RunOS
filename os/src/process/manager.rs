use super::ProcessControlBlock;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;
use spin::Mutex;

pub struct ProcessManager {
    ready_queue: VecDeque<Arc<ProcessControlBlock>>,
}

/// 多CPU共用全局FIFO调度器
/// A simple FIFO scheduler.
impl ProcessManager {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    pub fn add(&mut self, task: Arc<ProcessControlBlock>) {
        self.ready_queue.push_back(task);
    }
    pub fn fetch(&mut self) -> Option<Arc<ProcessControlBlock>> {
        self.ready_queue.pop_front()
    }
}

lazy_static! {
    pub static ref PROCESS_MANAGER: Mutex<ProcessManager> = Mutex::new(ProcessManager::new());
}

pub fn add_process(task: Arc<ProcessControlBlock>) {
    PROCESS_MANAGER.lock().add(task);
}

pub fn fetch_process() -> Option<Arc<ProcessControlBlock>> {
    PROCESS_MANAGER.lock().fetch()
}
