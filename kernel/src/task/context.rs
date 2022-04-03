use crate::cpu::current_hstack_top;
use crate::scheduler::schedule;
use crate::trap::trap_return;
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
    #[allow(unused)]
    pub fn get_sp(&self) -> usize {
        self.sp
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
}
