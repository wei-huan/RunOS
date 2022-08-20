use super::Scheduler;
use super::__schedule;
use crate::cpu::take_my_cpu;
use crate::sync::interrupt_off;
use crate::task::{TaskContext, TaskControlBlock, TaskStatus};
use alloc::{collections::VecDeque, sync::Arc};
use spin::Mutex;

type TaskQueue = VecDeque<Arc<TaskControlBlock>>;

pub struct RoundRobinScheduler {
    ready_queue: Mutex<TaskQueue>,
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
        loop {
            interrupt_off();
            let mut cpu = take_my_cpu();
            if let Some(last_task) = cpu.take_current() {
                self.add_task(last_task);
            }
            if let Some(task) = self.fetch_task() {
                let idle_task_cx_ptr = cpu.get_idle_task_cx_ptr();
                // access coming task TCB exclusively
                let mut task_inner = task.acquire_inner_lock();
                let next_task_cx_ptr = &task_inner.task_cx as *const TaskContext;
                task_inner.task_status = TaskStatus::Running;
                // release coming task PCB manually
                drop(task_inner);
                // add task cx to current cpu
                cpu.current = Some(task);
                // cpu task count + 1
                cpu.task_cnt += 1;
                // release cpu manually
                drop(cpu);
                // schedule new task
                unsafe { __schedule(idle_task_cx_ptr, next_task_cx_ptr) }
            } else {
                // idle_task();
                // log::debug!("Hart {} have no task", hart_id());
            }
        }
    }
    fn add_task(&self, task: Arc<TaskControlBlock>) {
        self.ready_queue.lock().push_back(task);
    }
    fn fetch_task(&self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.lock().pop_front()
    }
}
