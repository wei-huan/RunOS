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

use crate::config::SIGRETURN_TRAMPOLINE;
use crate::cpu::{current_process, current_task, take_current_task};
use crate::mm::{translated_byte_buffer, translated_refmut, UserBuffer};
use crate::scheduler::{
    add_task, remove_from_tid2task, save_current_and_back_to_schedule, INITPROC,
};
use crate::syscall::futex_wake;
use alloc::sync::Arc;
use core::mem::size_of;

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
    save_current_and_back_to_schedule(task_cx_ptr);
    // log::debug!("back to suspend");
}

pub fn block_current_and_run_next() {
    let task = current_task().unwrap();
    // println!("futex_wait8");
    let mut task_inner = task.acquire_inner_lock();
    // println!("futex_wait9");
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    task_inner.task_status = TaskStatus::Block;
    drop(task_inner);
    drop(task);
    // println!("futex_wait10");
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
    // println!("exit0");
    let process = current_process().unwrap();
    // take from Processor
    let task = take_current_task().unwrap();
    let mut task_inner = task.acquire_inner_lock();
    // println!("exit1");
    // do futex_wake if clear_child_tid is set
    if let Some(p) = &task_inner.clear_child_tid {
        *translated_refmut(
            process.acquire_inner_lock().get_user_token(),
            p.addr as *mut u32,
        ) = 0;
        futex_wake(p.addr, 1);
    }
    // println!("exit2");
    // remove from pid2task
    remove_from_tid2task(task.gettid());
    // get local id in process to check if main thread
    let lid = task_inner.get_lid();
    // record exit code
    task_inner.exit_code = exit_code;
    task_inner.res = None;
    drop(task_inner);
    drop(task);
    // main thread exit or exit group
    if lid == 0 || is_group {
        let mut initproc_inner = INITPROC.acquire_inner_lock();
        let mut process_inner = process.acquire_inner_lock();
        for child in process_inner.children.iter() {
            child.acquire_inner_lock().parent = Some(Arc::downgrade(&INITPROC));
            initproc_inner.children.push(child.clone());
        }
        drop(initproc_inner);
        process_inner.is_zombie = true;
        // record exit code
        process_inner.exit_code = exit_code;
        // deallocate user res (including tid/trap_cx/ustack) of all threads
        // it has to be done before we dealloc the whole memory_set
        // otherwise they will be deallocated twice
        for task in process_inner.tasks.iter() {
            let mut task_inner = task.acquire_inner_lock();
            task_inner.res = None;
        }

        process_inner.children.clear();
        // drop inner
        process_inner.addrspace.recycle_data_pages();
        // drop file descriptors
        process_inner.fd_table.clear();
    }
    // **** release current TCB
    // drop task manually to maintain rc correctly
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

pub fn current_add_signal(signal: usize) {
    let task = current_task().unwrap();
    let mut task_inner = task.acquire_inner_lock();
    task_inner.signals.add_sig(signal);
}

fn call_kernel_signal_handler(signal: usize) {
    let task = current_task().unwrap();
    let mut task_inner = task.acquire_inner_lock();
    match signal {
        SIGSTOP => {
            task_inner.frozen = true;
            task_inner.signals.clear_sig(SIGSTOP);
        }
        SIGCONT => {
            if task_inner.signals.contains_sig(SIGCONT) {
                task_inner.signals.clear_sig(SIGCONT);
                task_inner.frozen = false;
            }
        }
        _ => {
            task_inner.killed = true;
        }
    }
}

fn call_user_signal_handler(sig: usize) {
    let task = current_task().unwrap();
    let mut task_inner = task.acquire_inner_lock();
    let process = current_process().unwrap();
    let process_inner = process.acquire_inner_lock();
    let handler = process_inner.signal_actions[&(sig as u32)].sa_handler;
    // change current mask
    task_inner.signal_mask = process_inner.signal_actions[&(sig as u32)].sa_mask;
    // handle flag
    task_inner.handling_sig = sig as isize;
    task_inner.signals.clear_sig(sig);
    // backup trapframe
    let mut trap_ctx = task_inner.get_trap_cx();
    task_inner.trap_ctx_backup = Some(*trap_ctx);
    // modify trapframe
    trap_ctx.sepc = handler;
    extern "C" {
        fn __sigreturn();
        fn __uservec();
    }
    //put ra
    trap_ctx.x[1] = __sigreturn as usize - __uservec as usize + SIGRETURN_TRAMPOLINE;
    // put args (a0)
    trap_ctx.x[10] = sig;
    if process_inner.signal_actions[&(sig as u32)]
        .sa_flags
        .contains(SAFlags::SA_SIGINFO)
    {
        let token = process_inner.get_user_token();
        trap_ctx.x[2] -= size_of::<UContext>(); // sp -= sizeof(ucontext)
        trap_ctx.x[12] = trap_ctx.x[2]; // a2  = sp
        // log::debug!(
        //     "sighandler prepare: sp = {:#x?}, a2 = {:#x?}",
        //     trap_ctx.x[2],
        //     trap_ctx.x[12]
        // );
        let mut userbuf = UserBuffer::new(translated_byte_buffer(
            token,
            trap_ctx.x[2] as *const u8,
            size_of::<UContext>(),
        ));
        let mut ucontext = UContext::new();
        *ucontext.mc_pc() = trap_ctx.sepc;
        userbuf.write(ucontext.as_bytes()); // copy ucontext to userspace
    }
    // log::debug!("sig{} handler address {:#X?}", sig, handler);
}

fn check_pending_signals() {
    for sig in 1..(NSIG + 1) {
        let task = current_task().unwrap();
        let task_inner = task.acquire_inner_lock();
        let process = current_process().unwrap();
        let process_inner = process.acquire_inner_lock();
        if task_inner.signals.contains_sig(sig) && (!task_inner.signal_mask.contains_sig(sig)) {
            log::debug!(
                "process {}, task {} contains_sig {}",
                process.getpid(),
                task.gettid(),
                sig
            );
            if task_inner.handling_sig == -1 {
                drop(task_inner);
                drop(task);
                drop(process_inner);
                drop(process);
                if sig == SIGKILL || sig == SIGSTOP || sig == SIGCONT {
                    // signal is a kernel signal
                    call_kernel_signal_handler(sig);
                } else {
                    // signal is a user signal
                    call_user_signal_handler(sig);
                    return;
                }
            } else {
                if !process_inner.signal_actions[&(task_inner.handling_sig as u32)]
                    .sa_mask
                    .contains_sig(sig)
                {
                    log::warn!("can't be here");
                    drop(task_inner);
                    drop(task);
                    drop(process_inner);
                    drop(process);
                    if sig == SIGKILL || sig == SIGSTOP || sig == SIGCONT {
                        // signal is a kernel signal
                        call_kernel_signal_handler(sig);
                    } else {
                        // signal is a user signal
                        call_user_signal_handler(sig);
                        return;
                    }
                }
            }
        }
    }
}

pub fn handle_signals() {
    // 禁止信号嵌套
    let task = current_task().unwrap();
    let task_inner = task.acquire_inner_lock();
    if task_inner.handling_sig != -1 {
        println!("now handle_signals return");
        return;
    }
    drop(task_inner);
    drop(task);
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
