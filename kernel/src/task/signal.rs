#![allow(unused)]

use core::mem::size_of;

pub const SIGDEF: i32 = 0;

/* ISO C99 signals.  */
pub const SIGINT: i32 = 2; /* Interactive attention signal.  */
pub const SIGILL: i32 = 4; /* Illegal instruction.  */
pub const SIGABRT: i32 = 6; /* Abnormal termination.  */
pub const SIGIOT: i32 = 6;
pub const SIGFPE: i32 = 8; /* Erroneous arithmetic operation.  */
pub const SIGSEGV: i32 = 11; /* Invalid access to storage.  */
pub const SIGTERM: i32 = 15; /* Termination request.  */

/* Historical signals specified by POSIX. */
pub const SIGHUP: i32 = 1; /* Hangup.  */
pub const SIGQUIT: i32 = 3; /* Quit.  */
pub const SIGTRAP: i32 = 5; /* Trace/breakpoint trap.  */
pub const SIGBUS: i32 = 7; /* Bus error.  */
pub const SIGKILL: i32 = 9; /* Killed.  */
pub const SIGUSR1: i32 = 10; /* User-defined signal 1.  */
pub const SIGUSR2: i32 = 12; /* User-defined signal 2.  */
pub const SIGPIPE: i32 = 13; /* Broken pipe.  */
pub const SIGALRM: i32 = 14; /* Alarm clock.  */

/* New(er) POSIX signals (1003.1-2008, 1003.1-2013).  */
pub const SIGSTKFLT: i32 = 16; /* Stack fault on coprocessor */
pub const SIGCHLD: i32 = 17; /* Child terminated or stopped.  */
pub const SIGCONT: i32 = 18; /* Continue.  */
pub const SIGSTOP: i32 = 19; /* Stop, unblockable.  */
pub const SIGTSTP: i32 = 20; /* Keyboard stop.  */
pub const SIGTTIN: i32 = 21; /* Background read from control terminal.  */
pub const SIGTTOU: i32 = 22; /* Background write to control terminal.  */
pub const SIGURG: i32 = 23; /* Urgent condition on socket */
pub const SIGXCPU: i32 = 24; /* CPU time limit exceeded */
pub const SIGXFSZ: i32 = 25; /* File size limit exceeded */
pub const SIGVTALRM: i32 = 26; /* Virtual alarm clock */
pub const SIGPROF: i32 = 27; /* Profiling timer expired */
pub const SIGWINCH: i32 = 28; /* Window resize signal */
pub const SIGIO: i32 = 29; /* I/O now possible */
pub const SIGPOLL: i32 = SIGIO; /* Pollable event */
/*
pub const  SIGLOST		29
*/
pub const SIGPWR: i32 = 30; /* Power failure */
pub const SIGSYS: i32 = 31; /* Bad system call.  */
pub const SIGUNUSED: i32 = 31; /* Synonymous with SIGSYS */

/* These should not be considered constants from userland.  */
pub const SIGRTMIN: i32 = 32;

pub const NSIG: usize = 64;
pub const NSIG_BPW: usize = 32;
pub const NSIG_WORDS: usize = NSIG / NSIG_BPW;

pub const SIGRTMAX: i32 = NSIG as i32;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SigSet {
    sig: [u64; NSIG_WORDS],
}

impl SigSet {
    // please make sure that signum is valid
    pub fn add_sig(&mut self, signum: i32) {
        self.sig[(signum as usize) / NSIG] |= 0x01 << ((signum as usize) % NSIG - 1);
    }
    // please make sure that signum is valid
    pub fn clear_sig(&mut self, signum: i32) {
        self.sig[(signum as usize) / NSIG] &= !(0x01 << ((signum as usize) % NSIG - 1));
    }
    // please make sure that signum is valid
    pub fn contains_sig(&self, signum: i32) -> bool {
        (self.sig[(signum as usize) / NSIG] & (0x01 << ((signum as usize) % NSIG - 1))) > 0
    }
    pub fn block_with_other(&mut self, other: SigSet) {
        for i in 0..NSIG_WORDS {
            self.sig[i] |= other.sig[i];
        }
    }
    pub fn unblock_with_other(&mut self, other: SigSet) {
        for i in 0..NSIG_WORDS {
            self.sig[i] &= !other.sig[i];
        }
    }
    pub fn check_error(&self) -> Option<(i32, &'static str)> {
        if self.contains_sig(SIGINT) {
            Some((-2, "Killed, SIGINT=2"))
        } else if self.contains_sig(SIGILL) {
            Some((-4, "Illegal Instruction, SIGILL=4"))
        } else if self.contains_sig(SIGABRT) {
            Some((-6, "Aborted, SIGABRT=6"))
        } else if self.contains_sig(SIGFPE) {
            Some((-8, "Erroneous Arithmetic Operation, SIGFPE=8"))
        } else if self.contains_sig(SIGKILL) {
            Some((-9, "Killed, SIGKILL=9"))
        } else if self.contains_sig(SIGSEGV) {
            Some((-11, "Segmentation Fault, SIGSEGV=11"))
        } else {
            None
        }
    }
}

#[repr(C)]
pub struct UContext {
    pub __bits: [usize; 25],
}

impl UContext {
    pub fn new() -> Self {
        Self { __bits: [0; 25] }
    }

    pub fn as_bytes(&self) -> &[u8] {
        let size = core::mem::size_of::<Self>();
        unsafe { core::slice::from_raw_parts(self as *const _ as usize as *mut u8, size) }
    }

    pub fn pc_offset() -> usize {
        176
    }

    pub fn mc_pc(&mut self) -> &mut usize {
        &mut self.__bits[Self::pc_offset() / size_of::<usize>()]
    }
}
