use super::current_task;
use crate::sync::{interrupt_get, interrupt_on, IntrLock};
use crate::task::{TaskContext, TaskControlBlock};
use crate::trap::TrapContext;
use alloc::sync::Arc;

// Per-CPU state
pub struct Cpu {
    pub current: Option<Arc<TaskControlBlock>>, // The task running on this cpu, or None.
    kernel_task_cx: TaskContext,
    intr_depth: usize, // 中断嵌套深度
    intr_status: bool, // 本层中断状态
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            current: None,
            kernel_task_cx: TaskContext::zero_init(),
            intr_depth: 0,
            intr_status: false,
        }
    }
    pub fn take_kernel_task_cx_ptr(&mut self) -> *mut TaskContext {
        &mut self.kernel_task_cx as *mut _
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

pub fn current_trap_cx() -> &'static mut TrapContext {
    current_task()
        .unwrap()
        .inner_exclusive_access()
        .get_trap_cx()
}
