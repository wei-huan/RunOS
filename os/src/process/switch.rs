use super::ProcessContext;
use core::arch::global_asm;

global_asm!(include_str!("switch.S"));

extern "C" {
    pub fn __switch(current_proc_cx_ptr: *mut ProcessContext, next_proc_cx_ptr: *const ProcessContext);
    pub fn __first_switch(first_proc_cx_ptr: *const ProcessContext);
}