use crate::cpu::current_hstack_top;
use crate::scheduler::schedule;
use crate::trap::trap_return;
use core::arch::asm;
use riscv::register::sstatus::{self, set_spp, SPP};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TaskContext {
    ra: usize,
    sp: usize,
    sstatus: usize,
    s: [usize; 12],
}

impl TaskContext {
    pub fn zero_init() -> Self {
        let sstatus = sstatus::read();
        let sstatus = sstatus.bits();
        Self {
            ra: schedule as usize,
            sp: current_hstack_top(),
            sstatus,
            s: [0; 12],
        }
    }
    pub fn goto_trap_return(kstack_ptr: usize) -> Self {
        unsafe {
            set_spp(SPP::User);
        }
        let sstatus = sstatus::read();
        let sstatus = sstatus.bits();
        Self {
            ra: trap_return as usize,
            sp: kstack_ptr,
            sstatus,
            s: [0; 12],
        }
    }
    #[inline(always)]
    pub fn back_to_last_frame() -> Self {
        let sstatus = sstatus::read();
        let sstatus = sstatus.bits();
        let mut fp: usize;
        unsafe {
            asm!("mv {}, s0", out(reg) fp);
        }
        let ra = unsafe { *((fp - 8) as *const usize) } as usize;
        let mut s = [0; 12];
        for i in 0..12 {
            s[i] = unsafe { *((fp - (8 * (i + 2))) as *const usize) } as usize;
        }
        Self {
            ra,
            sp: fp,
            sstatus,
            s,
        }
    }
}
