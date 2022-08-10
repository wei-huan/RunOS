use super::Scheduler;
use super::__schedule;
use crate::cpu::{hart_id, take_my_cpu};
use crate::dt::CPU_NUMS;
use crate::scheduler::add_task;
use crate::sync::interrupt_off;
use crate::task::{TaskContext, TaskControlBlock, TaskStatus};
use alloc::{collections::VecDeque, sync::Arc, vec::Vec};
use core::sync::atomic::Ordering;
use spin::Mutex;
type TaskQueue = VecDeque<Arc<TaskControlBlock>>;

pub struct RoundRobinScheduler {
    ready_queues: Vec<Mutex<TaskQueue>>,
}

impl RoundRobinScheduler {
    pub fn new() -> Self {
        let cpu_num = CPU_NUMS.load(Ordering::Acquire);
        let mut ready_queues: Vec<Mutex<TaskQueue>> = Vec::with_capacity(cpu_num);
        for _ in 0..cpu_num {
            ready_queues.push(Mutex::new(VecDeque::new()));
        }
        Self { ready_queues }
    }
    pub fn add_task2designate_ready_queue(&self, task: Arc<TaskControlBlock>, queue_id: usize) {
        // log::debug!("Hart {} add task {} to ready queue {}",hart_id(), task.pid.0, queue_id);
        self.ready_queues[queue_id].lock().push_back(task);
    }
    fn ready_queue_len(&self, queue_id: usize) -> usize {
        self.ready_queues[queue_id].lock().len()
    }
    #[allow(unused)]
    pub fn have_ready_task(&self) -> bool {
        for i in 0..self.ready_queues.len() {
            if self.ready_queue_len(i) > 0 {
                return true;
            }
        }
        return false;
    }
}

impl Scheduler for RoundRobinScheduler {
    fn schedule(&self) {
        loop {
            interrupt_off();
            let mut cpu = take_my_cpu();
            if let Some(last_task) = cpu.take_current() {
                add_task(last_task);
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
        let (_, selected) = self
            .ready_queues
            .iter()
            .enumerate()
            .min_by_key(|queue| queue.1.lock().len())
            .unwrap_or((0, &self.ready_queues[0]));
        // log::debug!("Hart {} add task {} to queue {}", hart_id(), task.pid.0, i);
        selected.lock().push_back(task);
        // self.ready_queues[0].lock().push_back(task);
    }
    fn fetch_task(&self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queues[hart_id()].lock().pop_front()
    }
}
