use core::mem::size_of;

use crate::config::USER_STACK_SIZE;
use crate::cpu::{current_task, current_user_token};
use crate::mm::translated_refmut;
use crate::syscall::EINVAL;
use crate::timer::{get_time_sec_usec, TimeVal};

#[repr(C)]
pub struct RUsage {
    ru_utime: TimeVal, /* user time used */
    ru_stime: TimeVal, /* system time used */
    ru_maxrss: u64,    /* maximum resident set size */
    ru_ixrss: u64,     /* integral shared memory size */
    ru_idrss: u64,     /* integral unshared data size */
    ru_isrss: u64,     /* integral unshared stack size */
    ru_minflt: u64,    /* page reclaims */
    ru_majflt: u64,    /* page faults */
    ru_nswap: u64,     /* swaps */
    ru_inblock: u64,   /* block input operations */
    ru_oublock: u64,   /* block output operations */
    ru_msgsnd: u64,    /* messages sent */
    ru_msgrcv: u64,    /* messages received */
    ru_nsignals: u64,  /* signals received */
    ru_nvcsw: u64,     /* voluntary context switches */
    ru_nivcsw: u64,    /* involuntary " */
}

const RUSAGE_SELF: i32 = 0;
const RUSAGE_CHILDREN: i32 = -1;
const RUSAGE_BOTH: i32 = -2;
const RUSAGE_THREAD: i32 = 1;

pub fn sys_getrusage(who: i32, ru: *mut RUsage) -> isize {
    // log::debug!("sys_rusage: {}", who);
    if who != RUSAGE_SELF {
        return -EINVAL;
    }
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    let rusage = translated_refmut(token, ru);
    let (sec, usec) = get_time_sec_usec();
    rusage.ru_utime.sec = sec;
    rusage.ru_utime.usec = usec;
    rusage.ru_stime.sec = sec;
    rusage.ru_stime.usec = usec;
    // rusage.ru_maxrss = ;
    rusage.ru_ixrss = 0;
    // rusage.ru_idrss = 0;
    rusage.ru_isrss = (USER_STACK_SIZE / size_of::<i32>()) as u64;
    rusage.ru_minflt = 0;
    rusage.ru_majflt = 0;
    rusage.ru_nswap = 0;

    rusage.ru_inblock = 100;
    rusage.ru_oublock = 100;

    rusage.ru_msgsnd = 0;
    rusage.ru_msgrcv = 0;
    rusage.ru_nsignals = 0;

    rusage.ru_nvcsw = 10;
    rusage.ru_nivcsw = 10;
    0
}
