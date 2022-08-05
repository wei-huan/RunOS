use crate::config::USER_STACK_SIZE;
use crate::mm::{PhysPageNum, VirtAddr};
use crate::scheduler::{add_task, insert_into_tid2task};
use crate::task::{
    kstack_alloc, tid_alloc, trap_cx_bottom_from_lid, ustack_bottom_from_lid, Arc, KernelStack,
    ProcessControlBlock, SignalFlags, TaskContext, TidHandle,
};
use crate::trap::TrapContext;
use alloc::sync::Weak;
use spin::{Mutex, MutexGuard};

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
    pub lid: usize, // local id in task group
    pub tid: TidHandle,
    pub process: Weak<ProcessControlBlock>,
    pub kernel_stack: KernelStack,
    // mutable
    inner: Mutex<TaskControlBlockInner>,
}

pub struct TaskControlBlockInner {
    pub is_alloc_user_res: bool,
    pub task_cx: TaskContext,
    pub trap_cx_ppn: PhysPageNum,
    pub task_status: TaskStatus,
    pub exit_code: i32,
    pub signals: SignalFlags,
    pub signal_mask: SignalFlags,
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
}

impl TaskControlBlock {
    pub fn gettid(&self) -> usize {
        self.tid.0
    }
    pub fn getlid(&self) -> usize {
        self.lid
    }
    pub fn acquire_inner_lock(&self) -> MutexGuard<TaskControlBlockInner> {
        self.inner.lock()
    }
    pub fn trap_cx_user_va(&self) -> usize {
        trap_cx_bottom_from_lid(self.lid)
    }
    pub fn ustack_top(&self) -> usize {
        ustack_bottom_from_lid(self.lid) + USER_STACK_SIZE
    }
    // only for main task
    pub fn new(process: Arc<ProcessControlBlock>, lid: usize, is_alloc_user_res: bool) -> Self {
        let tid_handle = tid_alloc();
        let trap_cx_ppn = if is_alloc_user_res == true {
            process.acquire_inner_lock().alloc_task_user_res(lid);
            process.acquire_inner_lock().trap_cx_ppn(lid)
        } else {
            process.acquire_inner_lock().trap_cx_ppn(0)
        };
        let kernel_stack = kstack_alloc();
        let kernel_stack_top = kernel_stack.get_top();
        let task = Self {
            lid,
            tid: tid_handle,
            kernel_stack,
            process: Arc::downgrade(&process),
            inner: Mutex::new(TaskControlBlockInner {
                is_alloc_user_res: is_alloc_user_res,
                trap_cx_ppn,
                task_cx: TaskContext::goto_trap_return(kernel_stack_top),
                task_status: TaskStatus::Ready,
                exit_code: 0,
                signals: SignalFlags::empty(),
                signal_mask: SignalFlags::empty(),
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
        let lid = process.lid_alloc();
        process.acquire_inner_lock().alloc_task_user_res(lid);
        let trap_cx_ppn = process.acquire_inner_lock().trap_cx_ppn(lid);
        let self_inner = self.acquire_inner_lock();
        // create new thread
        let task = Arc::new(TaskControlBlock {
            lid,
            tid,
            process: Arc::downgrade(&process),
            kernel_stack,
            inner: Mutex::new(TaskControlBlockInner {
                is_alloc_user_res: true,
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
}
