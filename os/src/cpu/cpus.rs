use super::cpu::Cpu;
use crate::task::{TaskControlBlock, TaskContext, __switch};
use crate::sync::UPSafeCell;
use alloc::sync::Arc;
use array_macro::array;
use core::arch::asm;
use lazy_static::*;

const CPU_NUM: usize = 4;

// Must be called with interrupts disabled,
// to prevent race with task being moved
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

pub fn take_current_task() -> Option<Arc<TaskControlBlock>> {
    CPUS[cpu_id()].exclusive_access().take_current()
}

pub fn current_task() -> Option<Arc<TaskControlBlock>> {
    CPUS[cpu_id()].exclusive_access().current()
}

/// 从一个任务A切换到另一个任务B
pub fn schedule_new(switched_task_cx_ptr: *mut TaskContext) {
    let mut cpu = CPUS[cpu_id()].exclusive_access();
    let kernel_task_cx_ptr = cpu.take_kernel_task_cx_ptr();
    drop(cpu);
    unsafe {
        __switch(switched_task_cx_ptr, kernel_task_cx_ptr);
    }
}
