use crate::cpu::current_user_token;
use crate::mm::translated_refmut;

const RUSAGE_SELF: i32 = 0;
const RUSAGE_CHILDREN: i32 = -1;
const RUSAGE_BOTH: i32 = -2;
const RUSAGE_THREAD: i32 = 1;

pub struct RUsage{
	ru_utime: ,	/* user time used */
	ru_stime: ,	/* system time used */
	ru_maxrss: u64,	/* maximum resident set size */
	ru_ixrss: u64,	/* integral shared memory size */
	ru_idrss: u64,	/* integral unshared data size */
	ru_isrss: u64,	/* integral unshared stack size */
	ru_minflt: u64,	/* page reclaims */
	ru_majflt: u64,	/* page faults */
	ru_nswap: u64,	/* swaps */
	ru_inblock: u64,	/* block input operations */
	ru_oublock: u64,	/* block output operations */
	ru_msgsnd: u64,	/* messages sent */
	ru_msgrcv: u64,	/* messages received */
	ru_nsignals: u64,	/* signals received */
	ru_nvcsw: u64,	/* voluntary context switches */
	ru_nivcsw: u64,	/* involuntary " */
}

pub fn sys_rusage(who: i32, ru: *mut RUsage) -> isize{
    0
}