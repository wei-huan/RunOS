use super::Scheduler;
use crate::cpu::{schedule_new, take_my_cpu};
use crate::task::{idle_task, TaskContext, TaskControlBlock, TaskStatus};
use alloc::{collections::VecDeque, sync::Arc};
use spin::Mutex;

pub struct RoundRobinScheduler {
    ready_queue: Mutex<VecDeque<Arc<TaskControlBlock>>>,
}

impl RoundRobinScheduler {
    pub fn new() -> Self {
        Self {
            ready_queue: Mutex::new(VecDeque::new()),
        }
    }
}

impl Scheduler for RoundRobinScheduler {
    fn schedule(&self) {
        log::debug!("Starting scheduling");
        if let Some(task) = self.fetch_task() {
            let mut cpu = take_my_cpu();
            let mut task_inner = task.inner_exclusive_access();
            let next_task_cx_ptr = &task_inner.task_cx as *const TaskContext;
            task_inner.task_status = TaskStatus::Running;
            drop(task_inner);
            // release coming task PCB manually
            cpu.current = Some(task);
            // release processor manually
            drop(cpu);
            // schedule new task
            schedule_new(next_task_cx_ptr);
            log::debug!("Schedule Back");
        } else {
            log::debug!("No Process");
            idle_task();
        }
    }
    fn add_task(&self, task: Arc<TaskControlBlock>) {
        self.ready_queue.lock().push_back(task);
    }
    fn fetch_task(&self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.lock().pop_front()
    }
}
