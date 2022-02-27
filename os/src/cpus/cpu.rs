use crate::sync::{intr_get, intr_on, IntrLock};
use alloc::sync::Arc;
use core::cell::UnsafeCell;

// Per-CPU state
pub struct Cpu {
    pub proc: Option<Arc<i32>>, // The process running on this cpu, or None.
    pub int_depth: UnsafeCell<isize>, // 中断嵌套深度
    pub int_status: bool,       // 本层中断状态
}

impl Cpu {
    pub const fn new() -> Self {
        Self {
            proc: None,
            int_depth: UnsafeCell::new(0),
            int_status: false,
        }
    }

    // interrupts must be disabled.
    pub unsafe fn lock(&mut self, old: bool) -> IntrLock {
        if *self.int_depth.get() == 0 {
            self.int_status = old;
        }
        *self.int_depth.get() += 1;
        IntrLock { cpu: self }
    }

    // interrupts must be disabled.
    pub unsafe fn unlock(&self) {
        assert!(!intr_get(), "unlock - interruptible");
        let int_depth = self.int_depth.get();
        assert!(*int_depth >= 1, "unlock");
        *int_depth -= 1;
        if *int_depth == 0 && self.int_status {
            intr_on()
        }
    }
}
