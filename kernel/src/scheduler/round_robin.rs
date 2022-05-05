use super::Scheduler;
use super::__switch;
use crate::cpu::take_my_cpu;
use crate::task::{TaskContext, TaskControlBlock, TaskStatus};
use alloc::{collections::VecDeque, sync::Arc};
use spin::Mutex;

struct Queue {
    queue: Mutex<VecDeque<Arc<TaskControlBlock>>>,
}

pub struct RoundRobinScheduler {
    ready_queues: Mutex<VecDeque<Queue>>,
}

impl RoundRobinScheduler {
    pub fn new() -> Self {
        Self {
            ready_queues: Mutex::new(VecDeque::new()),
        }
    }
}

impl Scheduler for RoundRobinScheduler {
    fn schedule(&self) {
        loop {
            if let Some(task) = self.fetch_task() {
                // log::debug!("have task");
                let mut cpu = take_my_cpu();
                let idle_task_cx_ptr = cpu.get_idle_task_cx_ptr();
                // access coming task TCB exclusively
                let mut task_inner = task.acquire_inner_lock();
                let next_task_cx_ptr = &task_inner.task_cx as *const TaskContext;
                task_inner.task_status = TaskStatus::Running;
                // release coming task PCB manually
                drop(task_inner);
                // add task cx to current cpu
                cpu.current = Some(task);
                // release cpu manually
                drop(cpu);
                // schedule new task
                unsafe { __switch(idle_task_cx_ptr, next_task_cx_ptr) }
            }
            // log::info!("Back to Schedule");
        }
    }
    fn add_task(&self, task: Arc<TaskControlBlock>) {
        self.ready_queue.lock().push_back(task);
    }
    fn fetch_task(&self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.lock().pop_front()
    }
}
