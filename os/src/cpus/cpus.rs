use super::cpu::Cpu;
// use crate::boards::CPU_NUM;
use crate::sync::{IntrLock, intr_off, intr_get};
use array_macro::array;
use core::arch::asm;
use core::cell::UnsafeCell;

pub static CPUS: Cpus = Cpus::new();

const CPU_NUM: usize = 4;

pub struct Cpus([UnsafeCell<Cpu>; CPU_NUM]);
unsafe impl Sync for Cpus {}

impl Cpus {
    const fn new() -> Self {
        Self(array![_ => UnsafeCell::new(Cpu::new()); CPU_NUM])
    }

    // Must be called with interrupts disabled,
    // to prevent race with process being moved
    // to a different CPU.
    #[inline]
    pub unsafe fn cpu_id() -> usize {
        let id;
        asm!("mv {0}, tp", out(reg) id);
        id
    }

    // Return the pointer this Cpus's Cpu struct.
    // interrupts must be disabled.
    pub unsafe fn my_cpu(&self) -> &mut Cpu {
        let id = Self::cpu_id();
        &mut *self.0[id].get()
    }

    // intr_lock() disable interrupts on mycpu().
    // if all `intr_lock`'s are dropped, interrupts may recover
    // to previous state.
    pub fn intr_lock(&self) -> IntrLock {
        let old = intr_get();
        intr_off();
        unsafe { self.my_cpu().lock(old) }
    }

    // It is only safe to call it in Mutex's force_unlock().
    // It cannot be used anywhere else.
    pub unsafe fn intr_unlock(&self) {
        self.my_cpu().unlock();
    }
}
