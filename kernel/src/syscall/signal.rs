use crate::{
    cpu::{current_task, current_user_token},
    mm::{translated_ref, translated_refmut},
    scheduler::pid2task,
    syscall::{EINVAL, EPERM, ESRCH},
    task::{SigSet, SignalAction, NSIG, SIGKILL, SIGQUIT, SIGSTOP, SIGTRAP},
};

fn valid_signal(signum: i32) -> bool {
    signum >= 1 && (signum <= (NSIG as i32))
}

pub fn sys_kill(pid: isize, signum: i32) -> isize {
    log::debug!("sys_kill pid: {} signum: {}", pid, signum);
    // don't support sending to every process
    if pid == -1 {
        return -EINVAL;
    }
    if let Some(task) = pid2task(pid as usize) {
        if !valid_signal(signum) {
            return -EINVAL;
        }
        let mut inner = task.acquire_inner_lock();
        inner.signals.add_sig(signum);
        0
    } else {
        return -ESRCH;
    }
}

pub fn sys_tkill(tid: usize, signum: i32) -> isize {
    log::debug!("sys_tkill tid: {} signum: {}", tid, signum);
    0
}

pub fn sys_tgkill(tgid: usize, pid: usize, signum: i32) -> isize {
    log::debug!("sys_tgkill tgid: {} pid: {}, signum: {}", tgid, pid, signum);
    0
}

const SIG_BLOCK: isize = 0;
const SIG_UNBLOCK: isize = 1;
const SIG_SETMASK: isize = 2;

pub fn sys_sigprocmask(how: isize, set_ptr: *const SigSet, oldset_ptr: *mut SigSet) -> isize {
    log::trace!(
        "sys_sigprocmask how: {},  set_ptr: {},  oldset_ptr: {}",
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
    } else {
        return -ESRCH;
    }
    0
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

fn check_sigaction_error(signal: i32) -> bool {
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
    // log::debug!(
    //     "sys_sigaction signum {}, action {:#X?}, old_action {:#X?}",
    //     signum,
    //     action as usize,
    //     old_action as usize
    // );
    if !valid_signal(signum) || check_sigaction_error(signum) {
        log::warn!("here invalid sigaction num {}", signum);
        return -EINVAL;
    }
    let token = current_user_token();
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    if old_action as usize != 0 {
        if let Some(old) = inner.signal_actions.get(&signum) {
            *translated_refmut(token, old_action) = (*old).clone();
        } else {
            let sigact_old = translated_refmut(token, old_action);
            sigact_old.sa_handler = 0;
            sigact_old.sa_mask = SigSet::default();
        }
    }
    if action as usize != 0 {
        let new_action = *translated_ref(token, action);
        inner.signal_actions.insert(signum, new_action);
        // log::debug!(
        //     "new_action handler {:#X?}, mask {:#X?}, sa_flags {:#X?}, sa_restorer {:#X?}",
        //     new_action.sa_handler,
        //     new_action.sa_mask,
        //     new_action.sa_flags,
        //     new_action.sa_restorer
        // );
    }
    return 0;
}
