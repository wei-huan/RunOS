use super::Scheduler;
use super::__schedule;
use crate::cpu::{hart_id, take_my_cpu};
use crate::dt::CPU_NUMS;
use crate::sync::interrupt_off;
use crate::task::{idle_task, TaskContext, TaskControlBlock, TaskStatus};
use alloc::{collections::VecDeque, sync::Arc, vec::Vec};
use core::sync::atomic::Ordering;
use spin::Mutex;

struct Queue {
    queue: VecDeque<Arc<TaskControlBlock>>,
}

pub struct RoundRobinScheduler {
    ready_queues: Vec<Mutex<Queue>>,
}

impl RoundRobinScheduler {
    pub fn new() -> Self {
        let cpu_num = CPU_NUMS.load(Ordering::Acquire);
        let mut ready_queues: Vec<Mutex<Queue>> = Vec::with_capacity(cpu_num);
        for _ in 0..cpu_num {
            ready_queues.push(Mutex::new(Queue {
                queue: VecDeque::new(),
            }));
        }
        Self { ready_queues }
    }
    pub fn add_task_to_designate_queue(&self, task: Arc<TaskControlBlock>, queue_id: usize) {
        // log::debug!("Hart {} add task {} to queue {}",hart_id(), task.pid.0, queue_id);
        self.ready_queues[queue_id].lock().queue.push_back(task);
    }
    fn queue_len(&self, queue_id: usize) -> usize {
        self.ready_queues[queue_id].lock().queue.len()
    }
    #[allow(unused)]
    pub fn have_ready_task(&self) -> bool {
        for i in 0..self.ready_queues.len() {
            if self.queue_len(i) > 0 {
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
            if let Some(task) = self.fetch_task() {
                // if hart_id() == 1 {
                //     log::debug!("have task");
                // }
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
                // cpu task count + 1
                cpu.task_cnt += 1;
                // release cpu manually
                drop(cpu);
                // schedule new task
                unsafe { __schedule(idle_task_cx_ptr, next_task_cx_ptr) }
            } else {
                idle_task();
                // log::debug!("Hart {} have no task", hart_id());
            }
        }
    }
    fn add_task(&self, task: Arc<TaskControlBlock>) {
        let (i, selected) = self
            .ready_queues
            .iter()
            .enumerate()
            .min_by_key(|queue| queue.1.lock().queue.len())
            .unwrap_or((0, &self.ready_queues[0]));
        // if i == 1 {
        //     log::debug!("Hart {} add task {} to queue {}", hart_id(), task.pid.0, i);
        // }
        selected.lock().queue.push_back(task);
    }
    fn fetch_task(&self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queues[hart_id()].lock().queue.pop_front()
    }
}
