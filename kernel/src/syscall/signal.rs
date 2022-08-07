use crate::cpu::{current_process, current_task, current_user_token};
use crate::mm::{
    translated_ref, translated_refmut, translated_str, PTEFlags, VirtAddr, VirtPageNum,
};
use crate::scheduler::{pid2process, tid2task};
use crate::syscall::{EINVAL, EPERM, ESRCH};
use crate::task::{SigSet, SignalAction, NSIG, SIGKILL, SIGSTOP};

// just add sig to main thread
pub fn sys_kill(pid: usize, signum: i32) -> isize {
    log::debug!("sys_kill pid: {}, signum: {}", pid, signum);
    let process_option = pid2process(pid);
    if process_option.is_none() {
        return -ESRCH;
    }
    let process = process_option.unwrap();
    if signum > 0 && signum as usize <= NSIG {
        // insert the signal if legal
        let mut process_inner = process.acquire_inner_lock();
        let task = process_inner.get_task(0);
        let mut task_inner = task.acquire_inner_lock();
        task_inner.signals.add_sig(signum as usize);
        0
    } else {
        -EINVAL
    }
}

pub fn sys_tkill(tid: usize, signum: i32) -> isize {
    log::debug!("sys_tkill tid: {}, signum: {}", tid, signum);
    if let Some(task) = tid2task(tid) {
        if signum > 0 && signum as usize <= NSIG {
            let mut task_inner = task.acquire_inner_lock();
            task_inner.signals.add_sig(signum as usize);
            0
        } else {
            -EINVAL
        }
    } else {
        -ESRCH
    }
}

const SIG_BLOCK: isize = 0;
const SIG_UNBLOCK: isize = 1;
const SIG_SETMASK: isize = 2;

pub fn sys_sigprocmask(how: isize, set_ptr: *const SigSet, oldset_ptr: *mut SigSet) -> isize {
    log::debug!(
        "sys_sigprocmask how: {}, set_ptr: {:#X?}, oldset_ptr: {:#X?}",
        how,
        set_ptr as usize,
        oldset_ptr as usize
    );
    let token = current_user_token();
    if let Some(task) = current_task() {
        let mut inner = task.acquire_inner_lock();
        let old_mask = inner.signal_mask;
        if oldset_ptr as usize != 0 {
            *translated_refmut(token, oldset_ptr) = old_mask;
        }
        if set_ptr as usize != 0 {
            match how {
                SIG_BLOCK => {
                    let block_signals = *translated_ref(token, set_ptr);
                    inner.signal_mask.block_with_other(block_signals);
                }
                SIG_UNBLOCK => {
                    let unblock_signals = *translated_ref(token, set_ptr);
                    inner.signal_mask.unblock_with_other(unblock_signals);
                }
                SIG_SETMASK => {
                    inner.signal_mask = *translated_ref(token, set_ptr);
                }
                _ => return -EPERM,
            };
        }
        0
    } else {
        -ESRCH
    }
}

pub fn sys_sigretrun() -> isize {
    log::debug!("sys_sigretrun");
    if let Some(task) = current_task() {
        let mut inner = task.acquire_inner_lock();
        inner.handling_sig = -1;
        // restore the trap context
        let trap_ctx = inner.get_trap_cx();
        *trap_ctx = inner.trap_ctx_backup.unwrap();
        0
    } else {
        -1
    }
}

fn check_sigaction_error(signal: usize) -> bool {
    if signal == SIGKILL || signal == SIGSTOP {
        true
    } else {
        false
    }
}

pub fn sys_sigaction(
    signum: i32,
    action: *const SignalAction,
    old_action: *mut SignalAction,
) -> isize {
    log::debug!(
        "sys_sigaction signum: {}, action: {:#X?}, old_action: {:#X?}",
        signum,
        action as usize,
        old_action as usize
    );
    let token = current_user_token();
    let process = current_process().unwrap();
    let mut inner = process.acquire_inner_lock();
    if signum > 0 && signum as usize <= NSIG {
        if check_sigaction_error(signum as usize) {
            return -EINVAL;
        }
        if let Some(old) = inner.signal_actions.get(&(signum as u32)) {
            if old_action as usize != 0 {
                *translated_refmut(token, old_action) = (*old).clone();
            }
        } else {
            if old_action as usize != 0 {
                let sigact_old = translated_refmut(token, old_action);
                sigact_old.sa_handler = 0;
                sigact_old.sa_mask = SigSet::default();
            }
        }
        if action as usize != 0 {
            inner
                .signal_actions
                .insert(signum as u32, (*translated_ref(token, action)).clone());
        }
        return 0;
    } else {
        return -EINVAL;
    }
}
