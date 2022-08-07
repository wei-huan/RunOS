use core::mem::size_of;

pub const SIGDEF: usize = 0;

/* ISO C99 signals.  */
pub const SIGINT: usize = 2; /* Interactive attention signal.  */
pub const SIGILL: usize = 4; /* Illegal instruction.  */
pub const SIGABRT: usize = 6; /* Abnormal termination.  */
pub const SIGIOT: usize = 6;
pub const SIGFPE: usize = 8; /* Erroneous arithmetic operation.  */
pub const SIGSEGV: usize = 11; /* Invalid access to storage.  */
pub const SIGTERM: usize = 15; /* Termination request.  */

/* Historical signals specified by POSIX. */
pub const SIGHUP: usize = 1; /* Hangup.  */
pub const SIGQUIT: usize = 3; /* Quit.  */
pub const SIGTRAP: usize = 5; /* Trace/breakpoint trap.  */
pub const SIGBUS: usize = 7; /* Bus error.  */
pub const SIGKILL: usize = 9; /* Killed.  */
pub const SIGUSR1: usize = 10; /* User-defined signal 1.  */
pub const SIGUSR2: usize = 12; /* User-defined signal 2.  */
pub const SIGPIPE: usize = 13; /* Broken pipe.  */
pub const SIGALRM: usize = 14; /* Alarm clock.  */

/* New(er) POSIX signals (1003.1-2008, 1003.1-2013).  */
pub const SIGSTKFLT: usize = 16; /* Stack fault on coprocessor */
pub const SIGCHLD: usize = 17; /* Child terminated or stopped.  */
pub const SIGCONT: usize = 18; /* Continue.  */
pub const SIGSTOP: usize = 19; /* Stop, unblockable.  */
pub const SIGTSTP: usize = 20; /* Keyboard stop.  */
pub const SIGTTIN: usize = 21; /* Background read from control terminal.  */
pub const SIGTTOU: usize = 22; /* Background write to control terminal.  */
pub const SIGURG: usize = 23; /* Urgent condition on socket */
pub const SIGXCPU: usize = 24; /* CPU time limit exceeded */
pub const SIGXFSZ: usize = 25; /* File size limit exceeded */
pub const SIGVTALRM: usize = 26; /* Virtual alarm clock */
pub const SIGPROF: usize = 27; /* Profiling timer expired */
pub const SIGWINCH: usize = 28; /* Window resize signal */
pub const SIGIO: usize = 29; /* I/O now possible */
pub const SIGPOLL: usize = SIGIO; /* Pollable event */
/*
pub const  SIGLOST		29
*/
pub const SIGPWR: usize = 30; /* Power failure */
pub const SIGSYS: usize = 31; /* Bad system call.  */
pub const SIGUNUSED: usize = 31; /* Synonymous with SIGSYS */

/* These should not be considered constants from userland.  */
pub const SIGRTMIN: usize = 32;

pub const NSIG: usize = 64;
pub const NSIG_BPW: usize = 32;
pub const NSIG_WORDS: usize = NSIG / NSIG_BPW;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SigSet {
    sig: [usize; NSIG_WORDS],
}

impl SigSet {
    pub fn add_sig(&mut self, signum: usize) {
        self.sig[signum / NSIG] |= 0x01 << (signum % NSIG - 1);
    }
    pub fn clear_sig(&mut self, signum: usize) {
        self.sig[signum / NSIG] &= !(0x01 << (signum % NSIG - 1));
    }
    pub fn contains_sig(&self, signum: usize) -> bool {
        (self.sig[signum / NSIG] & (0x01 << (signum % NSIG - 1))) > 0
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
