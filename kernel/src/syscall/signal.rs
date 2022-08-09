use crate::{
    cpu::{current_task, current_user_token},
    mm::{translated_ref, translated_refmut},
    scheduler::pid2task,
    syscall::ESRCH,
    task::{SignalAction, SignalFlags, MAX_SIG},
};

pub fn sys_kill(pid: usize, signum: i32) -> isize {
    log::debug!("sys_kill pid: {} signum: {}", pid, signum);
    if let Some(task) = pid2task(pid) {
        if let Some(flag) = SignalFlags::from_bits(1 << signum) {
            // insert the signal if legal
            let mut task_ref = task.acquire_inner_lock();
            if task_ref.signals.contains(flag) {
                return -1;
            }
            task_ref.signals.insert(flag);
            0
        } else {
            -1
        }
    } else {
        -1
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

pub fn sys_sigprocmask(how: isize, set_ptr: *const u128, oldset_ptr: *mut u128) -> isize {
    log::trace!(
        "sys_sigprocmask how: {},  set_ptr: {},  oldset_ptr: {}",
        how,
        set_ptr as usize,
        oldset_ptr as usize
    );
    let token = current_user_token();
    if let Some(task) = current_task() {
        let mut inner = task.acquire_inner_lock();
        let old_mask = inner.signal_mask.bits();
        if oldset_ptr as usize != 0 {
            *translated_refmut::<u128>(token, oldset_ptr) = old_mask as u128;
        }
        if set_ptr as usize != 0 {
            let new_mask = *translated_ref::<u128>(token, set_ptr) as u32;
            match how {
                SIG_BLOCK => {
                    inner.signal_mask = SignalFlags::from_bits(new_mask | old_mask).unwrap()
                }
                SIG_UNBLOCK => {
                    inner.signal_mask = SignalFlags::from_bits(old_mask & (!new_mask)).unwrap()
                }
                SIG_SETMASK => inner.signal_mask = SignalFlags::from_bits(new_mask).unwrap(),
                _ => return -1,
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

fn check_sigaction_error(signal: SignalFlags) -> bool {
    if signal == SignalFlags::SIGKILL || signal == SignalFlags::SIGSTOP {
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
        "sys_sigaction signum {}, action {:#X?}, old_action {:#X?}",
        signum,
        action as usize,
        old_action as usize
    );
    if signum as usize > MAX_SIG {
        return -1;
    }
    let token = current_user_token();
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    if let Some(flag) = SignalFlags::from_bits(1 << signum) {
        if check_sigaction_error(flag) {
            println!("here1");
            return -1;
        }
        let old_kernel_action = inner.signal_actions[&signum];
        if old_kernel_action.mask != SignalFlags::from_bits(40).unwrap() && old_action as usize != 0
        {
            *translated_refmut(token, old_action) = old_kernel_action;
        }
        if action as usize != 0 {
            let ref_action = translated_ref(token, action);
            inner.signal_actions[&signum] = *ref_action;
        }
    }
    println!("here0");
    return 0;
}
