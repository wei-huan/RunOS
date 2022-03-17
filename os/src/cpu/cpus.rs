use super::cpu::Cpu;
use crate::task::{ProcessControlBlock, ProcessContext, __switch};
use crate::sync::UPSafeCell;
use alloc::sync::Arc;
use array_macro::array;
use core::arch::asm;
use lazy_static::*;

const CPU_NUM: usize = 4;

// Must be called with interrupts disabled,
// to prevent race with process being moved
// to a different CPU.
#[inline]
pub fn cpu_id() -> usize {
    let id;
    unsafe { asm!("mv {0}, tp", out(reg) id) };
    id
}

lazy_static! {
    pub static ref CPUS: [UPSafeCell<Cpu>; CPU_NUM] =
        array![_ => UPSafeCell::new(Cpu::new()); CPU_NUM];
}

pub fn take_current_process() -> Option<Arc<ProcessControlBlock>> {
    CPUS[cpu_id()].exclusive_access().take_current()
}

pub fn current_process() -> Option<Arc<ProcessControlBlock>> {
    CPUS[cpu_id()].exclusive_access().current()
}

pub fn schedule(switched_task_cx_ptr: *mut ProcessContext) {
    let mut cpu = CPUS[cpu_id()].exclusive_access();
    let idle_task_cx_ptr = cpu.take_idle_proc_cx_ptr();
    drop(cpu);
    unsafe {
        __switch(switched_task_cx_ptr, idle_task_cx_ptr);
    }
}
