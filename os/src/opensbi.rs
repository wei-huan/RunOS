#![allow(unused)]

use core::arch::asm;

const SBI_GET_SPEC_VERSION_EID: usize = 0x10;
const SBI_GET_IMPL_ID_EID: usize = 0x10;
const SBI_GET_IMPL_VERSION_EID: usize = 0x10;
const SBI_SET_TIMER_EID: usize = 0;
const SBI_CONSOLE_PUTCHAR_EID: usize = 1;
const SBI_CONSOLE_GETCHAR_EID: usize = 2;
const SBI_CLEAR_IPI_EID: usize = 3;
const SBI_SEND_IPI_EID: usize = 4;
const SBI_REMOTE_FENCE_I_EID: usize = 5;
const SBI_REMOTE_SFENCE_VMA_EID: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID_EID: usize = 7;
const SBI_SHUTDOWN_EID: usize = 8;
const SBI_HART_START_EID: usize = 0x48534D;
const SBI_HART_STOP_EID: usize = 0x48534D;
const SBI_HART_STATUS_EID: usize = 0x48534D;

const SBI_GET_SPEC_VERSION_FID: usize = 0;
const SBI_GET_IMPL_ID_FID: usize = 1;
const SBI_GET_IMPL_VERSION_FID: usize = 2;
const SBI_SET_TIMER_FID: usize = 0;
const SBI_CONSOLE_PUTCHAR_FID: usize = 0;
const SBI_CONSOLE_GETCHAR_FID: usize = 0;
const SBI_CLEAR_IPI_FID: usize = 0;
const SBI_SEND_IPI_FID: usize = 0;
const SBI_REMOTE_FENCE_I_FID: usize = 0;
const SBI_REMOTE_SFENCE_VMA_FID: usize = 0;
const SBI_REMOTE_SFENCE_VMA_ASID_FID: usize = 0;
const SBI_SHUTDOWN_FID: usize = 0;
const SBI_HART_START_FID: usize = 0;
const SBI_HART_STOP_FID: usize = 1;
const SBI_HART_STATUS_FID: usize = 2;

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
fn opensbi_call(
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

/// SBI implementation name
#[derive(Debug, Clone, Copy)]
pub enum SbiImplId {
    #[allow(missing_docs)]
    BerkeleyBootLoader,
    #[allow(missing_docs)]
    OpenSbi,
    #[allow(missing_docs)]
    Xvisor,
    #[allow(missing_docs)]
    Kvm,
    #[allow(missing_docs)]
    RustSbi,
    #[allow(missing_docs)]
    Diosix,
    #[allow(missing_docs)]
    Other(usize),
}

impl From<usize> for SbiImplId {
    fn from(v: usize) -> Self {
        match v {
            0 => SbiImplId::BerkeleyBootLoader,
            1 => SbiImplId::OpenSbi,
            2 => SbiImplId::Xvisor,
            3 => SbiImplId::Kvm,
            4 => SbiImplId::RustSbi,
            5 => SbiImplId::Diosix,
            v => SbiImplId::Other(v),
        }
    }
}

impl From<SbiImplId> for usize{
    fn from(v: SbiImplId) -> usize {
        match v {
            SbiImplId::BerkeleyBootLoader => 0,
            SbiImplId::OpenSbi => 1,
            SbiImplId::Xvisor => 2,
            SbiImplId::Kvm => 3,
            SbiImplId::RustSbi => 4,
            SbiImplId::Diosix => 5,
            SbiImplId::Other(v) => v,
        }
    }
}

/// SBI specification version implemented by the SBI implementation
#[derive(Debug, Clone, Copy)]
pub struct SbiSpecVersion {
    /// Major version number
    pub major: usize,
    /// Minor version number
    pub minor: usize,
}

pub fn get_spec_version() -> SbiSpecVersion {
    let value = opensbi_call(
        SBI_GET_SPEC_VERSION_EID,
        SBI_GET_SPEC_VERSION_FID,
        0,
        0,
        0,
        0,
        0,
        0,
    )
    .1;
    SbiSpecVersion {
        major: (value >> 24) & 0x7f,
        minor: value & 0xff_ffff,
    }
}

pub fn impl_id() -> SbiImplId {
    let value = opensbi_call(SBI_GET_IMPL_ID_EID, SBI_GET_IMPL_ID_FID, 0, 0, 0, 0, 0, 0).1;
    value.into()
}

pub fn impl_version() -> usize {
    opensbi_call(
        SBI_GET_IMPL_VERSION_EID,
        SBI_GET_IMPL_VERSION_FID,
        0,
        0,
        0,
        0,
        0,
        0,
    )
    .1
}

pub fn set_timer(timer: usize) {
    opensbi_call(SBI_SET_TIMER_EID, SBI_SET_TIMER_FID, timer, 0, 0, 0, 0, 0);
}

pub fn console_putchar(c: usize) {
    opensbi_call(
        SBI_CONSOLE_PUTCHAR_EID,
        SBI_CONSOLE_PUTCHAR_FID,
        c,
        0,
        0,
        0,
        0,
        0,
    );
}

pub fn console_getchar() -> usize {
    opensbi_call(
        SBI_CONSOLE_GETCHAR_EID,
        SBI_CONSOLE_GETCHAR_FID,
        0,
        0,
        0,
        0,
        0,
        0,
    )
    .1
}

pub fn clear_ipi() -> SBIRet {
    opensbi_call(SBI_CLEAR_IPI_EID, SBI_CLEAR_IPI_FID, 0, 0, 0, 0, 0, 0)
}

pub fn send_ipi(cpu_id: usize) -> SBIRet {
    opensbi_call(SBI_SEND_IPI_EID, SBI_SEND_IPI_FID, cpu_id, 0, 0, 0, 0, 0)
}

pub fn remote_fence_i(cpu_id: usize) -> SBIRet {
    opensbi_call(
        SBI_REMOTE_FENCE_I_EID,
        SBI_REMOTE_FENCE_I_FID,
        cpu_id,
        0,
        0,
        0,
        0,
        0,
    )
}

pub fn remote_sfence_vma(hartid: usize, start: usize, size: usize) -> SBIRet {
    opensbi_call(
        SBI_REMOTE_SFENCE_VMA_EID,
        SBI_REMOTE_SFENCE_VMA_FID,
        hartid,
        start,
        size,
        0,
        0,
        0,
    )
}

pub fn remote_sfence_vma_asid(hartid: usize, start: usize, size: usize, asid: usize) -> SBIRet {
    opensbi_call(
        SBI_REMOTE_SFENCE_VMA_ASID_EID,
        SBI_REMOTE_SFENCE_VMA_ASID_FID,
        hartid,
        start,
        size,
        asid,
        0,
        0,
    )
}

pub fn shutdown() -> ! {
    println!("I am dead");
    opensbi_call(SBI_SHUTDOWN_EID, SBI_SHUTDOWN_FID, 0, 0, 0, 0, 0, 0);
    panic!("It should shutdown!");
}

pub fn hart_start(hartid: usize, jump_addr: usize, privilege: usize) -> SBIRet {
    opensbi_call(
        SBI_HART_START_EID,
        SBI_HART_START_FID,
        hartid,
        jump_addr,
        privilege,
        0,
        0,
        0,
    )
}

pub fn hart_stop(hartid: usize, start_addr: usize, privilege: usize) -> SBIRet {
    opensbi_call(
        SBI_HART_STOP_EID,
        SBI_HART_STOP_FID,
        hartid,
        start_addr,
        privilege,
        0,
        0,
        0,
    )
}

pub fn hart_status(hartid: usize) -> SBIRet {
    opensbi_call(
        SBI_HART_STATUS_EID,
        SBI_HART_STATUS_FID,
        hartid,
        0,
        0,
        0,
        0,
        0,
    )
}
