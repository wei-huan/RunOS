use super::cpu::Cpu;
use crate::sync::UPSafeCell;
use crate::task::TaskControlBlock;
use alloc::sync::Arc;
use array_macro::array;
use core::arch::asm;
use core::cell::RefMut;
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
        array![_ => unsafe {UPSafeCell::new(Cpu::new())}; CPU_NUM];
}

pub fn take_my_cpu() -> RefMut<'static, Cpu> {
    CPUS[cpu_id()].exclusive_access()
}

pub fn take_current_task() -> Option<Arc<TaskControlBlock>> {
    take_my_cpu().take_current()
}

pub fn current_task() -> Option<Arc<TaskControlBlock>> {
    take_my_cpu().current()
}
