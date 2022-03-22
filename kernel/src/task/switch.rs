use super::TaskContext;
use core::arch::global_asm;

global_asm!(include_str!("switch.S"));

extern "C" {
    // ! __switch will return
    pub fn __switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
}