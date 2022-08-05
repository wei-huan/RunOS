mod action;
mod aux;
mod context;
mod id;
mod idle_task;
mod process;
mod signal;
mod task;

pub use action::*;
pub use aux::*;
pub use context::*;
pub use id::*;
pub use idle_task::*;
pub use process::*;
pub use signal::*;
pub use task::*;

use crate::cpu::{current_process, current_task, take_current_task};
use crate::mm::translated_refmut;
use crate::scheduler::{
    add_task, remove_from_pid2process, remove_from_tid2task, save_current_and_back_to_schedule,
    INITPROC,
};
use crate::syscall::futex_wake;
use alloc::sync::Arc;
use alloc::vec::Vec;

pub fn suspend_current_and_run_next() {
    // There must be an application running.
    let task = current_task().unwrap();
    // ---- access current TCB exclusively
    let mut task_inner = task.acquire_inner_lock();
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    // Change status to Ready
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);
    drop(task);
    // jump to scheduling cycle
    // log::debug!("suspend 1");
    save_current_and_back_to_schedule(task_cx_ptr);
    // log::debug!("back to suspend");
}

pub fn block_current_and_run_next() {
    let task = take_current_task().unwrap();
    let mut task_inner = task.acquire_inner_lock();
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    task_inner.task_status = TaskStatus::Block;
    drop(task_inner);
    drop(task);
    save_current_and_back_to_schedule(task_cx_ptr);
}

pub fn unblock_task(task: Arc<TaskControlBlock>) {
    let mut task_inner = task.acquire_inner_lock();
    assert!(task_inner.task_status == TaskStatus::Block);
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);
    add_task(task);
}

/// 将当前任务退出重新加入就绪队列，并调度新的任务
pub fn exit_current_and_run_next(exit_code: i32, is_group: bool) {
    let process = current_process().unwrap();
    let mut process_inner = process.acquire_inner_lock();
    // take task from Processor
    let task = take_current_task().unwrap();
    let mut task_inner = task.acquire_inner_lock();

    // do futex_wake if clear_child_tid is set
    if let Some(p) = &task_inner.clear_child_tid {
        *translated_refmut(process_inner.get_user_token(), p.addr as *mut u32) = 0;
        futex_wake(p.addr, 1);
    }
    // dealloc task user resource
    let lid = task.getlid();
    process_inner.dealloc_task_user_res(lid);
    // remove from pid2task
    let tid = task.gettid();
    remove_from_tid2task(tid);
    // record exit code
    task_inner.exit_code = exit_code;

    drop(task_inner);
    drop(task);
    // main thread exit or exit group
    if lid == 0 || is_group {
        process_inner.is_zombie = true;
        // record exit code
        process_inner.exit_code = exit_code;
        {
            let mut initproc_inner = INITPROC.acquire_inner_lock();
            for child in process_inner.children.iter() {
                child.acquire_inner_lock().parent = Some(Arc::downgrade(&INITPROC));
                initproc_inner.children.push(child.clone());
            }
        }
        process_inner.children.clear();

        remove_from_pid2process(process.getpid());
        // deallocate user res (including tid/trap_cx/ustack) of all threads
        // it has to be done before we dealloc the whole memory_set
        // otherwise they will be deallocated twice
        let mut lid_vec = Vec::new();
        for task in process_inner.tasks.iter() {
            lid_vec.push(task.getlid());
        }
        for lid in lid_vec {
            process_inner.dealloc_task_user_res(lid);
        }
        // drop address space
        process_inner.addrspace.recycle_data_pages();
        // drop file descriptors
        process_inner.fd_table.clear();
    }
    // **** release current PCB
    drop(process_inner);
    // drop process manually to maintain rc correctly
    drop(process);
    // jump to schedule cycle
    // we do not have to save task context
    let mut _unused = TaskContext::zero_init();
    save_current_and_back_to_schedule(&mut _unused as *mut _);
}

pub fn check_signals_error_of_current() -> Option<(i32, &'static str)> {
    let task = current_task().unwrap();
    let task_inner = task.acquire_inner_lock();
    task_inner.signals.check_error()
}

pub fn current_add_signal(signal: SignalFlags) {
    let task = current_task().unwrap();
    let mut task_inner = task.acquire_inner_lock();
    task_inner.signals |= signal;
}

fn call_kernel_signal_handler(signal: SignalFlags) {
    let task = current_task().unwrap();
    let mut task_inner = task.acquire_inner_lock();
    match signal {
        SignalFlags::SIGSTOP => {
            task_inner.frozen = true;
            task_inner.signals ^= SignalFlags::SIGSTOP;
        }
        SignalFlags::SIGCONT => {
            if task_inner.signals.contains(SignalFlags::SIGCONT) {
                task_inner.signals ^= SignalFlags::SIGCONT;
                task_inner.frozen = false;
            }
        }
        _ => {
            task_inner.killed = true;
        }
    }
}

fn call_user_signal_handler(sig: usize, signal: SignalFlags) {
    let task = current_task().unwrap();
    let mut task_inner = task.acquire_inner_lock();
    let process = current_process().unwrap();
    let process_inner = process.acquire_inner_lock();
    let handler = process_inner.signal_actions.table[sig].handler;
    // change current mask
    task_inner.signal_mask = process_inner.signal_actions.table[sig].mask;
    // handle flag
    task_inner.handling_sig = sig as isize;
    task_inner.signals ^= signal;
    // backup trapframe
    let mut trap_ctx = task_inner.get_trap_cx();
    task_inner.trap_ctx_backup = Some(*trap_ctx);
    // modify trapframe
    trap_ctx.sepc = handler;
    // put args (a0)
    trap_ctx.x[10] = sig;
}

fn check_pending_signals() {
    for sig in 0..(MAX_SIG + 1) {
        let task = current_task().unwrap();
        let task_inner = task.acquire_inner_lock();
        let process = current_process().unwrap();
        let process_inner = process.acquire_inner_lock();
        let signal = SignalFlags::from_bits(1 << sig).unwrap();
        if task_inner.signals.contains(signal) && (!task_inner.signal_mask.contains(signal)) {
            if task_inner.handling_sig == -1 {
                drop(task_inner);
                drop(task);
                drop(process_inner);
                drop(process);
                if signal == SignalFlags::SIGKILL
                    || signal == SignalFlags::SIGSTOP
                    || signal == SignalFlags::SIGCONT
                    || signal == SignalFlags::SIGDEF
                {
                    // signal is a kernel signal
                    call_kernel_signal_handler(signal);
                } else {
                    // signal is a user signal
                    call_user_signal_handler(sig, signal);
                    return;
                }
            } else {
                if !process_inner.signal_actions.table[task_inner.handling_sig as usize]
                    .mask
                    .contains(signal)
                {
                    drop(task_inner);
                    drop(task);
                    drop(process_inner);
                    drop(process);
                    if signal == SignalFlags::SIGKILL
                        || signal == SignalFlags::SIGSTOP
                        || signal == SignalFlags::SIGCONT
                        || signal == SignalFlags::SIGDEF
                    {
                        // signal is a kernel signal
                        call_kernel_signal_handler(signal);
                    } else {
                        // signal is a user signal
                        call_user_signal_handler(sig, signal);
                        return;
                    }
                }
            }
        }
    }
}

pub fn handle_signals() {
    check_pending_signals();
    loop {
        let task = current_task().unwrap();
        let task_inner = task.acquire_inner_lock();
        let frozen_flag = task_inner.frozen;
        let killed_flag = task_inner.killed;
        drop(task_inner);
        drop(task);
        if (!frozen_flag) || killed_flag {
            break;
        }
        check_pending_signals();
        suspend_current_and_run_next()
    }
}
