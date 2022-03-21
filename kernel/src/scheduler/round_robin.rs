use super::Scheduler;
use crate::cpu::{take_my_cpu, schedule_new};
use crate::task::{idle_task, TaskContext, TaskControlBlock, TaskStatus};
use alloc::{collections::VecDeque, sync::Arc};

pub struct RoundRobinScheduler {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

impl RoundRobinScheduler {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
}

impl Scheduler for RoundRobinScheduler {
    fn schedule(&mut self) {
        log::debug!("Starting scheduling");
        loop {
            if let Some(task) = self.fetch_from_readyqueue() {
                log::debug!("Have Process0");
                let mut cpu = take_my_cpu();
                let mut task_inner = task.inner_exclusive_access();
                log::debug!("Have Process2");
                let next_task_cx_ptr = &task_inner.task_cx as *const TaskContext;
                task_inner.task_status = TaskStatus::Running;
                drop(task_inner);
                // release coming task PCB manually
                cpu.current = Some(task);
                // release processor manually
                drop(cpu);
                // schedule new task
                schedule_new(next_task_cx_ptr);
                log::debug!("Have Process8");
            } else {
                log::debug!("No Process");
                idle_task();
            }
        }
    }
    fn add_to_readyqueue(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    fn fetch_from_readyqueue(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop_front()
    }
}
