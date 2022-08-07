use bitflags::*;

use super::SigSet;

/// Action for a signal
#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct SignalAction {
    pub sa_handler: usize, // type: void (*__sighandler_t) (int) or void (*_sa_sigaction)(int signo, struct siginfo *info, void *context);
    pub sa_mask: SigSet,
    pub sa_flags: SAFlags,
    pub sa_restorer: usize, 
}

bitflags! {
    #[derive(Default)]
    pub struct SAFlags: usize {
        const SA_NOCLDSTOP = 1;		 /* Don't send SIGCHLD when children stop.  */
        const SA_NOCLDWAIT = 2;		 /* Don't create zombie on child death.  */
        const SA_SIGINFO   = 4;  	 /* Invoke signal-catching function with
                                        three arguments instead of one.  */
        const SA_ONSTACK   = 0x08000000; /* Use signal stack by using `sa_restorer'. */
        const SA_RESTART   = 0x10000000; /* Restart syscall on signal return.  */
        const SA_NODEFER   = 0x40000000; /* Don't automatically block the signal when
                                            its handler is being executed.  */
        const SA_RESETHAND = 0x80000000; /* Reset to SIG_DFL on entry to handler.  */
    }
}
