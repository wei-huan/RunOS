use crate::trap::user_trap_return;
use riscv::register::sstatus::{self, Sstatus, SPP};

#[repr(C)]
pub struct ProcessContext {
    ra: usize,
    sp: usize,
    sstatus: Sstatus,
    s: [usize; 12],
}

impl ProcessContext {
    pub fn zero_init() -> Self {
        let sstatus = sstatus::read();
        Self {
            ra: 0,
            sp: 0,
            sstatus,
            s: [0; 12],
        }
    }
    pub fn goto_trap_return(kstack_ptr: usize) -> Self {
        let sstatus = sstatus::read();
        // sstatus.set_spp(SPP::User);
        Self {
            ra: user_trap_return as usize,
            sp: kstack_ptr,
            sstatus,
            s: [0; 12],
        }
    }
}
