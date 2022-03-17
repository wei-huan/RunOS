use crate::trap::user_trap_return;

#[repr(C)]
pub struct ProcessContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl ProcessContext {
    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }
    pub fn goto_trap_return(kstack_ptr: usize) -> Self {
        Self {
            ra: user_trap_return as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}
