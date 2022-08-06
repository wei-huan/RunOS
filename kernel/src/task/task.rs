use crate::mm::PhysPageNum;
use crate::scheduler::{add_task, insert_into_tid2task};
use crate::task::{
    kstack_alloc, tid_alloc, Arc, KernelStack, ProcessControlBlock, TaskContext, TaskUserRes,
    TidHandle,
};
use crate::trap::TrapContext;
use alloc::sync::Weak;
use spin::{Mutex, MutexGuard};

use super::SigSet;

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    Ready,
    Block,
    Running,
}
#[derive(Debug)]
pub struct ClearChildTid {
    pub ctid: u32,
    pub addr: usize,
}

pub struct TaskControlBlock {
    // immutable
    pub tid: TidHandle,
    pub process: Weak<ProcessControlBlock>,
    pub kernel_stack: KernelStack,
    // mutable
    inner: Mutex<TaskControlBlockInner>,
}

pub struct TaskControlBlockInner {
    pub res: Option<TaskUserRes>,
    pub task_cx: TaskContext,
    pub trap_cx_ppn: PhysPageNum,
    pub task_status: TaskStatus,
    pub exit_code: i32,
    pub signals: SigSet,
    pub signal_mask: SigSet,
    // the signal which is being handling
    pub handling_sig: isize,
    // if the task is killed
    pub killed: bool,
    // if the task is frozen by a signal
    pub frozen: bool,
    pub trap_ctx_backup: Option<TrapContext>,
    pub clear_child_tid: Option<ClearChildTid>,
}

impl TaskControlBlockInner {
    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }
    #[allow(unused)]
    fn get_status(&self) -> TaskStatus {
        self.task_status
    }
    pub fn get_lid(&self) -> usize {
        self.res.as_ref().unwrap().lid
    }
}

impl TaskControlBlock {
    pub fn gettid(&self) -> usize {
        self.tid.0
    }
    pub fn acquire_inner_lock(&self) -> MutexGuard<TaskControlBlockInner> {
        self.inner.lock()
    }
    // only for main thread
    pub fn new(process: Arc<ProcessControlBlock>, lid: usize, is_alloc_user_res: bool) -> Self {
        let tid_handle = tid_alloc();
        let res = TaskUserRes::new(process.clone(), lid, is_alloc_user_res);
        let trap_cx_ppn = res.trap_cx_ppn();
        let kernel_stack = kstack_alloc();
        let kernel_stack_top = kernel_stack.get_top();
        let task = Self {
            tid: tid_handle,
            kernel_stack,
            process: Arc::downgrade(&process),
            inner: Mutex::new(TaskControlBlockInner {
                res: Some(res),
                trap_cx_ppn,
                task_cx: TaskContext::goto_trap_return(kernel_stack_top),
                task_status: TaskStatus::Ready,
                exit_code: 0,
                signals: SigSet::default(),
                signal_mask: SigSet::default(),
                handling_sig: -1,
                killed: false,
                frozen: false,
                trap_ctx_backup: None,
                clear_child_tid: None,
            }),
        };
        task
    }
    pub fn clone_thread(
        self: &Arc<TaskControlBlock>,
        process: Arc<ProcessControlBlock>,
        _flags: u32,
    ) -> Arc<TaskControlBlock> {
        let kernel_stack = kstack_alloc();
        let kernel_stack_top = kernel_stack.get_top();
        let tid = tid_alloc();
        let lid = process.acquire_inner_lock().tasks.len();
        let res = TaskUserRes::new(process.clone(), lid, true);
        let trap_cx_ppn = res.trap_cx_ppn();
        let self_inner = self.acquire_inner_lock();
        // create new thread
        let task = Arc::new(TaskControlBlock {
            tid,
            process: Arc::downgrade(&process),
            kernel_stack,
            inner: Mutex::new(TaskControlBlockInner {
                res: Some(res),
                trap_cx_ppn,
                task_cx: TaskContext::goto_trap_return(kernel_stack_top),
                task_status: TaskStatus::Ready,
                exit_code: self_inner.exit_code,
                signals: self_inner.signals.clone(),
                signal_mask: self_inner.signal_mask.clone(),
                handling_sig: -1,
                killed: false,
                frozen: false,
                trap_ctx_backup: None,
                clear_child_tid: None,
            }),
        });
        let task_inner = task.acquire_inner_lock();
        let trap_cx = task_inner.get_trap_cx();
        // copy trap_cx from the parent thread
        *trap_cx = *self_inner.get_trap_cx();
        // modify kstack_top in trap_cx of this thread
        trap_cx.kernel_sp = kernel_stack_top;
        drop(self_inner);
        drop(task_inner);
        // add new thread to process
        let mut process_inner = process.acquire_inner_lock();
        process_inner.tasks.push(task.clone());
        drop(process_inner);
        insert_into_tid2task(task.gettid(), task.clone());
        add_task(task.clone());
        task
    }
    // pub fn getpid(&self) -> usize {
    //     self.pid.0
    // }
    // pub fn get_parent(&self) -> Option<Arc<TaskControlBlock>> {
    //     let inner = self.acquire_inner_lock();
    //     inner.parent.as_ref().unwrap().upgrade()
    // }
    // pub fn getppid(&self) -> usize {
    //     self.get_parent().unwrap().pid.0
    // }
}
