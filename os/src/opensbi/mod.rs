mod base;
mod legacy;
mod timer;
mod ipi;
mod rfence;
mod hsm;

use core::arch::asm;
pub use hsm::{hart_start, hart_status, hart_stop};
pub use legacy::{console_getchar, console_putchar, shutdown, set_timer};
#[derive(Debug, Clone, Copy)]
pub enum SbiError {
    /// The SBI call failed
    Failed,
    /// The SBI call is not implemented or the functionality is not available
    NotSupported,
    /// An invalid parameter was passed
    InvalidParam,
    /// The SBI implementation has denied execution of the call functionality
    Denied,
    /// An invalid address was passed
    InvalidAddress,
    /// The resource is already available
    AlreadyAvailable,
}

impl SbiError {
    #[inline]
    fn new(n: isize) -> Self {
        match n {
            -1 => SbiError::Failed,
            -2 => SbiError::NotSupported,
            -3 => SbiError::InvalidParam,
            -4 => SbiError::Denied,
            -5 => SbiError::InvalidAddress,
            -6 => SbiError::AlreadyAvailable,
            n => unreachable!("bad SBI error return value: {}", n),
        }
    }
}

pub struct SBIRet(SbiError, usize);

#[inline(always)]
pub fn opensbi_call(
    ext: usize,
    fid: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> SBIRet {
    let mut ret0: isize;
    let mut ret1: usize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") arg0 => ret0,
            inlateout("x11") arg1 => ret1,
            in("x12") arg2,
            in("x13") arg3,
            in("x14") arg4,
            in("x15") arg5,
            in("x16") fid,
            in("x17") ext,
        );
    }
    SBIRet(SbiError::new(ret0), ret1)
}
