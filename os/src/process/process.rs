use super::signal::SignalFlags;
use super::kernelstack::{kstack_alloc};
use super::kernelstack::KernelStack;
use super::context::ProcessContext;
use super::{pid_alloc, PidHandle};
use crate::fs::{File, Stdin, Stdout};
use crate::{
    mm::{AddrSpace, KERNEL_SPACE, PhysPageNum},
    sync::UPSafeCell,
};
use alloc::sync::{Arc, Weak};
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefMut;
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

impl ProcessControlBlockInner {
    #[allow(unused)]
    pub fn get_user_token(&self) -> usize {
        self.addrspace.get_token()
    }
    pub fn alloc_fd(&mut self) -> usize {
        if let Some(fd) = (0..self.fd_table.len()).find(|fd| self.fd_table[*fd].is_none()) {
            fd
        } else {
            self.fd_table.push(None);
            self.fd_table.len() - 1
        }
    }
}

impl ProcessControlBlock {
    pub fn inner_exclusive_access(&self) -> RefMut<'_, ProcessControlBlockInner> {
        self.inner.exclusive_access()
    }

    pub fn new(elf_data: &[u8]) -> Arc<Self> {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (addrspace, ustack_base, entry_point) = AddrSpace::from_elf(elf_data);
        // allocate a pid
        let pid_handle = pid_alloc();
        let process = Arc::new(Self {
            pid: pid_handle,
            inner: unsafe {
                UPSafeCell::new(ProcessControlBlockInner {
                    is_zombie: false,
                    addrspace,
                    parent: None,
                    children: Vec::new(),
                    exit_code: 0,
                    fd_table: vec![
                        // 0 -> stdin
                        Some(Arc::new(Stdin)),
                        // 1 -> stdout
                        Some(Arc::new(Stdout)),
                        // 2 -> stderr
                        Some(Arc::new(Stdout)),
                    ],
                    signals: SignalFlags::empty(),
                    // tasks: Vec::new(),
                    // task_res_allocator: RecycleAllocator::new(),
                    // mutex_list: Vec::new(),
                    // semaphore_list: Vec::new(),
                    // condvar_list: Vec::new(),
                })
            },
        });
        let kstack = kstack_alloc();
        let kstack_top = kstack.get_top();
        process
    }

    pub fn getpid(&self) -> usize {
        self.pid.0
    }
}
