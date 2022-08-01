use super::cpu::Cpu;
use crate::sync::UPSafeCell;
use crate::task::{TaskControlBlock, ProcessControlBlock};
use alloc::sync::Arc;
use array_macro::array;
use core::arch::asm;
use core::cell::RefMut;
use lazy_static::*;

#[cfg(feature = "platform-k210")]
const CPU_NUM: usize = 2;
#[cfg(not(feature = "platform-k210"))]
const CPU_NUM: usize = 4;

#[inline(always)]
pub fn hart_id() -> usize {
    let id;
    unsafe { asm!("mv {0}, tp", out(reg) id) };
    id
}

lazy_static! {
    pub static ref CPUS: [UPSafeCell<Cpu>; CPU_NUM] =
        array![_ => unsafe {UPSafeCell::new(Cpu::new())}; CPU_NUM];
}

pub fn take_my_cpu() -> RefMut<'static, Cpu> {
    // log::debug!("hart id: {}", hart_id());
    CPUS[hart_id()].exclusive_access()
}

pub fn take_current_task() -> Option<Arc<TaskControlBlock>> {
    take_my_cpu().take_current()
}

pub fn current_task() -> Option<Arc<TaskControlBlock>> {
    take_my_cpu().current()
}

pub fn current_process() -> Option<Arc<ProcessControlBlock>> {
    current_task().unwrap().process.upgrade()
}

pub fn current_trap_cx_user_va() -> usize {
    current_task()
        .unwrap()
        .acquire_inner_lock()
        .res
        .as_ref()
        .unwrap()
        .trap_cx_user_va()
}