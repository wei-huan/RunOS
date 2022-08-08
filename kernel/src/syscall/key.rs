use crate::cpu::{current_task, current_user_token};
use crate::mm::{
    translated_ref, translated_refmut, translated_str, PTEFlags, VirtAddr, VirtPageNum,
};
use crate::scheduler::{add_task, pid2task};
use crate::syscall::ESRCH;
use crate::task::{
    exit_current_and_run_next, suspend_current_and_run_next, SignalAction, SignalFlags, MAX_SIG,
};

type KeySerial = i32;

pub fn sys_add_key(
    type_: *const u8,
    description: *const u8,
    payload: usize,
    plen: usize,
    keyring: KeySerial,
) -> isize {
    let token = current_user_token();
    let type_str = translated_str(token, type_);
    let description_str = translated_str(token, description);
    log::debug!(
        "sys_add_key type: {}, description: {}, payload: {}, plen: {}, keyring: {}",
        type_str,
        description_str,
        payload,
        plen,
        keyring
    );
    0
}

pub fn sys_request_key(
    type_: *const u8,
    description: *const u8,
    callout_info: *const u8,
    dest_keyring: KeySerial,
) -> isize {
    let token = current_user_token();
    let type_str = translated_str(token, type_);
    let description_str = translated_str(token, description);
    let callout_info_str = translated_str(token, callout_info);
    log::debug!(
        "sys_add_key type: {}, description: {}, callout_info_str: {}, dest_keyring: {}",
        type_str,
        description_str,
        callout_info_str,
        dest_keyring
    );
    0
}
