#![allow(unused)]
/// Hart State ManageMent Extension
use crate::opensbi::opensbi_call;
use crate::opensbi::SBIResult;

const SBI_HSM_EID: usize = 0x48534D;

const SBI_HART_START_FID: usize = 0;
const SBI_HART_STOP_FID: usize = 1;
const SBI_HART_STATUS_FID: usize = 2;

pub fn hart_start(hartid: usize, jump_addr: usize, privilege: usize) -> SBIResult<usize> {
    opensbi_call(
        SBI_HSM_EID,
        SBI_HART_START_FID,
        hartid,
        jump_addr,
        privilege,
        0,
        0,
        0,
    )
}

#[allow(unused)]
pub fn hart_stop(hartid: usize, start_addr: usize, privilege: usize) -> SBIResult<usize> {
    opensbi_call(
        SBI_HSM_EID,
        SBI_HART_STOP_FID,
        hartid,
        start_addr,
        privilege,
        0,
        0,
        0,
    )
}

#[allow(unused)]
pub fn hart_status(hartid: usize) -> SBIResult<usize> {
    opensbi_call(SBI_HSM_EID, SBI_HART_STATUS_FID, hartid, 0, 0, 0, 0, 0)
}
