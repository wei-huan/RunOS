use crate::timer::get_time_ms;

pub fn sys_exit(_exit_code: i32) -> ! {
    // exit_current_and_run_next(exit_code);
    panic!("Unreachable in sys_exit!");
}

pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}
