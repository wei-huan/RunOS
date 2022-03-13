use super::{pid_alloc, PidHandle};
use crate::{
    mm::{AddrSpace, KERNEL_SPACE},
    sync::UPSafeCell,
};
use alloc::sync::{Arc, Weak};
use alloc::vec::Vec;
use crate::fs::{File, Stdin, Stdout};
use super::pid::RecycleAllocator;
// use spin::{Condvar, Mutex, Semaphore};

pub struct ProcessControlBlock {
    // immutable
    pub pid: PidHandle,
    // mutable
    inner: UPSafeCell<ProcessControlBlockInner>,
}

pub struct ProcessControlBlockInner {
    pub is_zombie: bool,
    pub addrspace: AddrSpace,
    pub parent: Option<Weak<ProcessControlBlock>>,
    pub children: Vec<Arc<ProcessControlBlock>>,
    pub exit_code: i32,
    pub fd_table: Vec<Option<Arc<dyn File + Send + Sync>>>,
    pub signals: SignalFlags,
    pub tasks: Vec<Option<Arc<TaskControlBlock>>>,
    pub task_res_allocator: RecycleAllocator,
    // pub mutex_list: Vec<Option<Arc<dyn Mutex>>>,
    // pub semaphore_list: Vec<Option<Arc<Semaphore>>>,
    // pub condvar_list: Vec<Option<Arc<Condvar>>>,
}
