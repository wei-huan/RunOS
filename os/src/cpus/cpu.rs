use crate::sync::{interrupt_get, interrupt_on, IntrLock};
use crate::process::{ProcessControlBlock};
use alloc::sync::Arc;
use core::cell::UnsafeCell;

// Per-CPU state
pub struct Cpu {
    pub process: Option<Arc<ProcessControlBlock>>, // The process running on this cpu, or None.
    pub intr_depth: UnsafeCell<isize>, // 中断嵌套深度
    pub intr_status: bool,       // 本层中断状态
}

impl Cpu {
    pub const fn new() -> Self {
        Self {
            process: None,
            intr_depth: UnsafeCell::new(0),
            intr_status: false,
        }
    }

    // interrupts must be disabled.
    pub unsafe fn lock(&mut self, old: bool) -> IntrLock {
        if *self.intr_depth.get() == 0 {
            self.intr_status = old;
        }
        *self.intr_depth.get() += 1;
        IntrLock { cpu: self }
    }

    // interrupts must be disabled.
    pub unsafe fn unlock(&self) {
        assert!(!interrupt_get(), "unlock - interruptible");
        let int_depth = self.intr_depth.get();
        assert!(*int_depth >= 1, "unlock");
        *int_depth -= 1;
        if *int_depth == 0 && self.intr_status {
            interrupt_on()
        }
    }
}
