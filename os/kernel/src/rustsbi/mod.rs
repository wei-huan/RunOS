/// API文档详情参考: https://github.com/riscv-non-isa/riscv-sbi-doc/blob/master/riscv-sbi.adoc#legacy-sbi-extension-extension-ids-0x00-through-0x0f
mod base;
mod hsm;
mod legacy;

pub use base::{impl_id, impl_version, spec_version};
pub use hsm::{hart_start, hart_status, hart_stop};
pub use legacy::{clear_ipi, console_getchar, console_putchar, send_ipi, set_timer, shutdown};

use core::arch::asm;

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

pub type SBIResult<T> = Result<T, SbiError>;

#[inline(always)]
pub fn rustsbi_call(
    eid: usize,
    fid: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> SBIResult<usize> {
    let mut error: isize;
    let mut value: usize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") arg0 => error,
            inlateout("x11") arg1 => value,
            in("x12") arg2,
            in("x13") arg3,
            in("x14") arg4,
            in("x15") arg5,
            in("x16") fid,
            in("x17") eid,
        );
    }
    match error {
        0..=isize::MAX => SBIResult::Ok(value),
        e => SBIResult::Err(SbiError::new(e)),
    }
}
