use super::context::TaskContext;
use super::process::ProcessControlBlock;
use crate::mm::PhysPageNum;
use crate::task::{kstack_alloc, tid_alloc, Arc, KernelStack, SignalFlags, TaskUserRes, TidHandle};
use crate::trap::TrapContext;
use spin::{Mutex, MutexGuard};

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    Ready,
    Running,
    Zombie,
}

pub struct TaskControlBlock {
    // immutable
    pub tid: TidHandle,
    pub kernel_stack: KernelStack,
    // mutable
    inner: Mutex<TaskControlBlockInner>,
}

pub struct TaskControlBlockInner {
    pub res: Option<TaskUserRes>,
    pub task_cx: TaskContext,
    pub trap_cx_ppn: PhysPageNum,
    pub task_status: TaskStatus,
    pub signal_mask: SignalFlags,
    // the signal which is being handling
    pub handling_sig: isize,
    // if the task is killed
    pub killed: bool,
    // if the task is frozen by a signal
    pub frozen: bool,
    pub trap_ctx_backup: Option<TrapContext>,
}

impl TaskControlBlockInner {
    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }
    fn get_status(&self) -> TaskStatus {
        self.task_status
    }
    pub fn is_zombie(&self) -> bool {
        self.get_status() == TaskStatus::Zombie
    }
}

impl TaskControlBlock {
    pub fn gettid(&self) -> usize {
        self.tid.0
    }
    pub fn acquire_inner_lock(&self) -> MutexGuard<TaskControlBlockInner> {
        self.inner.lock()
    }
    // only for initproc
    pub fn new(process: Arc<ProcessControlBlock>, lid: usize, is_alloc_user_res: bool) -> Self {
        let tid_handle = tid_alloc();
        let res = TaskUserRes::new(process, lid, is_alloc_user_res);
        let trap_cx_ppn = res.trap_cx_ppn();
        let kernel_stack = kstack_alloc();
        let kernel_stack_top = kernel_stack.get_top();
        let task = Self {
            tid: tid_handle,
            kernel_stack,
            inner: Mutex::new(TaskControlBlockInner {
                res: Some(res),
                trap_cx_ppn,
                task_cx: TaskContext::goto_trap_return(kernel_stack_top),
                task_status: TaskStatus::Ready,
                signal_mask: SignalFlags::empty(),
                handling_sig: -1,
                killed: false,
                frozen: false,
                trap_ctx_backup: None,
            }),
        };
        task
    }
    // pub fn getpid(&self) -> usize {
    //     self.pid.0
    // }
    // pub fn get_parent(&self) -> Option<Arc<TaskControlBlock>> {
    //     let inner = self.acquire_inner_lock();
    //     inner.parent.as_ref().unwrap().upgrade()
    // }
    // initproc won't call sys_getppid
    // pub fn getppid(&self) -> usize {
    //     self.get_parent().unwrap().pid.0
    // }
}
