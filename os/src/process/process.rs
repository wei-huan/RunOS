use super::pid::RecycleAllocator;
use super::signal::SignalFlags;
use super::kernelstack::KernelStack;
use super::context::ProcessContext;
use super::{pid_alloc, PidHandle};
use crate::fs::{File, Stdin, Stdout};
use crate::{
    mm::{AddrSpace, KERNEL_SPACE, PhysPageNum},
    sync::UPSafeCell,
};
use alloc::sync::{Arc, Weak};
use alloc::vec::Vec;
// use spin::{Condvar, Mutex, Semaphore};

#[derive(Copy, Clone, PartialEq)]
pub enum ProcessStatus {
    Ready,
    Running,
    Blocking,
}

pub struct ProcessControlBlock {
    // immutable
    pub pid: PidHandle,
    pub kstack: KernelStack,
    // mutable
    inner: UPSafeCell<ProcessControlBlockInner>,
}

pub struct ProcessControlBlockInner {
    pub exit_code: i32,
    pub is_zombie: bool,
    pub ustack_base: usize,
    pub addrspace: AddrSpace,
    pub signals: SignalFlags,
    pub proc_cx: ProcessContext,
    pub proc_cx_ppn: PhysPageNum,
    pub proc_status: ProcessStatus,
    pub children: Vec<Arc<ProcessControlBlock>>,
    pub parent: Option<Weak<ProcessControlBlock>>,
    pub fd_table: Vec<Option<Arc<dyn File + Send + Sync>>>,
    // pub mutex_list: Vec<Option<Arc<dyn Mutex>>>,
    // pub semaphore_list: Vec<Option<Arc<Semaphore>>>,
    // pub condvar_list: Vec<Option<Arc<Condvar>>>,
}
