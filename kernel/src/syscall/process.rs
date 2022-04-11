use crate::cpu::current_task;
use crate::scheduler::add_task;
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};
use crate::timer::get_time_ms;

pub fn sys_exit(exit_code: i32) -> ! {
    exit_current_and_run_next(exit_code);
}

pub fn sys_yield() -> ! {
    suspend_current_and_run_next();
}

pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}

pub fn sys_fork() -> isize {
    let current_task = current_task().unwrap();
    let new_task = current_task.fork();
    let new_pid = new_task.pid.0;
    // modify trap context of new_task, because it returns immediately after switching
    let trap_cx = new_task.inner_exclusive_access().get_trap_cx();
    // we do not have to move to next instruction since we have done it before
    // for child process, fork returns 0
    trap_cx.x[10] = 0;
    // add new task to scheduler
    add_task(new_task);
    new_pid as isize
}
