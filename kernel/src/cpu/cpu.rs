use crate::cpu::{current_process, current_task, hart_id};
use crate::mm::kernel_token;
use crate::task::{TaskContext, TaskControlBlock};
use crate::trap::TrapContext;
use crate::utils::get_boot_stack;
use alloc::sync::Arc;

// Per-CPU state
pub struct Cpu {
    pub current: Option<Arc<TaskControlBlock>>, // The task running on this cpu, or None.
    idle_task_cx: TaskContext,
    // statistics
    pub task_cnt: usize, // 有任务次数
    pub idle_cnt: usize, // 没有任务次数
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            current: None,
            idle_task_cx: TaskContext::zero_init(),
            task_cnt: 0,
            idle_cnt: 0,
        }
    }
    pub fn get_idle_task_cx_ptr(&mut self) -> *mut TaskContext {
        &mut self.idle_task_cx as *mut _
    }
    pub fn take_current(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.current.take()
    }
    pub fn current(&self) -> Option<Arc<TaskControlBlock>> {
        self.current.as_ref().map(Arc::clone)
    }
}

pub fn current_user_token() -> usize {
    let current_process = current_process().unwrap();
    let token = current_process.acquire_inner_lock().get_user_token();
    token
}

#[allow(unused)]
pub fn current_token() -> usize {
    if let Some(current_process) = current_process() {
        let token = current_process.acquire_inner_lock().get_user_token();
        return token;
    } else {
        return kernel_token();
    }
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    current_task().unwrap().acquire_inner_lock().get_trap_cx()
}

#[allow(unused)]
pub fn current_kstack_top() -> usize {
    current_task().unwrap().kernel_stack.get_top()
}

pub fn current_hstack_top() -> usize {
    let (_, top) = get_boot_stack(hart_id());
    top
}

pub fn current_stack_top() -> usize {
    if let Some(task) = current_task() {
        // task kernel stack
        task.kernel_stack.get_top()
    } else {
        // boot stack
        current_hstack_top()
    }
}
