use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};
use crate::timer::get_time_ms;

pub fn sys_exit(exit_code: i32) -> ! {
    exit_current_and_run_next(exit_code);
    unreachable!();
}

pub fn sys_yield() -> !{
    suspend_current_and_run_next();
}

pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}
