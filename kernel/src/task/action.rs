use crate::task::SigSet;
use bitflags::*;

bitflags! {
    #[derive(Default)]
    pub struct SAFlags: u64 {
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

/// Action for a signal
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct SignalAction {
    pub sa_handler: usize, // void (*__sighandler_t) (int)
    pub sa_mask: SigSet,
    pub sa_flags: SAFlags,
    pub sa_restorer: usize, // void __restorefn_t(void)
}
