use super::Scheduler;
use crate::cpu::take_my_cpu;
use crate::scheduler;
use crate::task::{idle_task, TaskContext, TaskControlBlock, TaskStatus};
use crate::timer;
use crate::trap;
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
    fn schedule(&self) -> ! {
        log::debug!("Start scheduling");
        if let Some(task) = self.fetch_task() {
            let mut task_inner = task.inner_exclusive_access();
            let next_task_cx_ptr = &task_inner.task_cx as *const TaskContext;
            task_inner.task_status = TaskStatus::Running;
            // release coming task PCB manually
            drop(task_inner);
            // add task cx to current cpu
            let mut cpu = take_my_cpu();
            cpu.current = Some(task);
            // release cpu manually
            drop(cpu);
            // schedule new task
            log::debug!("Go new");
            unsafe { scheduler::__goto_user(next_task_cx_ptr) }
        } else {
            trap::init();
            timer::init();
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
