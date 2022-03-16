use crate::process::ProcessControlBlock;
use crate::sync::UPSafeCell;
use crate::sync::{interrupt_get, interrupt_on, IntrLock};
use crate::trap::TrapContext;
use alloc::sync::Arc;
use lazy_static::*;

// Per-CPU state
pub struct Cpu {
    pub current: Option<Arc<ProcessControlBlock>>, // The process running on this cpu, or None.
    pub intr_depth: usize,             // 中断嵌套深度
    pub intr_status: bool,                         // 本层中断状态
    // 统计信息
    pub idle_ms: usize, // 空闲时长，单位mm
    pub usage: f64,     // 一秒钟内的使用率
}

impl Cpu {
    pub const fn new() -> Self {
        Self {
            current: None,
            intr_depth: 0,
            intr_status: false,
            idle_ms: 0,
            usage: 0.0,
        }
    }
    pub fn set_current(&mut self, op: Option<Arc<ProcessControlBlock>>){
        self.current = op;
    }
    pub fn take_current(&mut self) -> Option<Arc<ProcessControlBlock>> {
        self.current.take()
    }
    pub fn current(&self) -> Option<Arc<ProcessControlBlock>> {
        self.current.as_ref().map(Arc::clone)
    }
    // interrupts must be disabled.
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

lazy_static! {
    pub static ref CPU: UPSafeCell<Cpu> = UPSafeCell::new(Cpu::new());
}

pub fn take_current_task() -> Option<Arc<ProcessControlBlock>> {
    CPU.exclusive_access().take_current()
}

pub fn current_task() -> Option<Arc<ProcessControlBlock>> {
    CPU.exclusive_access().current()
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

// pub fn schedule() -> !{
//     switch()
// }
