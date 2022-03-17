use super::current_process;
use super::{cpu_id, CPUS};
use crate::process::{ProcessContext, ProcessControlBlock, __switch};
use crate::sync::{interrupt_get, interrupt_on, IntrLock};
use crate::trap::TrapContext;
use alloc::sync::Arc;

// Per-CPU state
pub struct Cpu {
    pub current: Option<Arc<ProcessControlBlock>>, // The process running on this cpu, or None.
    idle_proc_cx: ProcessContext,
    intr_depth: usize, // 中断嵌套深度
    intr_status: bool, // 本层中断状态
    // 统计信息
    idle_ms: usize, // 空闲时长，单位mm
    usage: f64,     // 一秒钟内的使用率
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            current: None,
            idle_proc_cx: ProcessContext::zero_init(),
            intr_depth: 0,
            intr_status: false,
            idle_ms: 0,
            usage: 0.0,
        }
    }
    // pub fn set_current(&mut self, op: Option<Arc<ProcessControlBlock>>){
    //     self.current = op;
    // }
    pub fn take_idle_proc_cx_ptr(&mut self) -> *mut ProcessContext {
        &mut self.idle_proc_cx as *mut _
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

pub fn current_user_token() -> usize {
    let task = current_process().unwrap();
    let token = task.inner_exclusive_access().get_user_token();
    token
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    current_process()
        .unwrap()
        .inner_exclusive_access()
        .get_trap_cx()
}

pub fn schedule(switched_task_cx_ptr: *mut ProcessContext) {
    let mut cpu = CPUS[cpu_id()].exclusive_access();
    let idle_task_cx_ptr = cpu.take_idle_proc_cx_ptr();
    drop(cpu);
    unsafe {
        __switch(switched_task_cx_ptr, idle_task_cx_ptr);
    }
}
