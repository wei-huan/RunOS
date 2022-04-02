use super::current_task;
use crate::cpu::cpu_id;
use crate::mm::kernel_token;
use crate::sync::{interrupt_get, interrupt_on, IntrLock};
use crate::task::TaskControlBlock;
use crate::trap::TrapContext;
use crate::utils::get_boot_stack_top;
use alloc::sync::Arc;

// Per-CPU state
pub struct Cpu {
    pub current: Option<Arc<TaskControlBlock>>, // The task running on this cpu, or None.
    intr_depth: usize,                          // 中断嵌套深度
    intr_status: bool,                          // 本层中断状态
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            current: None,
            intr_depth: 0,
            intr_status: false,
        }
    }
    pub fn take_current(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.current.take()
    }
    pub fn current(&self) -> Option<Arc<TaskControlBlock>> {
        self.current.as_ref().map(Arc::clone)
    }
    // interrupts must be disabled.
    #[allow(unused)]
    pub unsafe fn lock(&mut self, old: bool) -> IntrLock {
        if self.intr_depth == 0 {
            self.intr_status = old;
        }
        self.intr_depth += 1;
        IntrLock { cpu: self }
    }
    // interrupts must be disabled.
    pub unsafe fn unlock(&self) {
        assert!(!interrupt_get(), "unlock - interruptible");
        let mut int_depth = self.intr_depth;
        assert!(int_depth >= 1, "unlock");
        int_depth -= 1;
        if int_depth == 0 && self.intr_status {
            interrupt_on()
        }
    }
}

pub fn current_user_token() -> usize {
    let task = current_task().unwrap();
    let token = task.inner_exclusive_access().get_user_token();
    token
}

pub fn current_token() -> usize {
    if let Some(task) = current_task() {
        let token = task.inner_exclusive_access().get_user_token();
        return token;
    } else {
        return kernel_token();
    }
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    current_task()
        .unwrap()
        .inner_exclusive_access()
        .get_trap_cx()
}

pub fn current_kstack_top() -> usize {
    current_task().unwrap().kernel_stack.get_top()
}

pub fn current_stack_top() -> usize {
    if let Some(task) = current_task() {
        // task kernel stack
        current_task().unwrap().kernel_stack.get_top()
    } else {
        // boot stack
        get_boot_stack_top(cpu_id())
    }
}
