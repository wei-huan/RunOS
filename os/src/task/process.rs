// use super::signal::SignalFlags;
use super::context::ProcessContext;
use super::kernelstack::{kstack_alloc, KernelStack};
use super::{pid_alloc, PidHandle};
use crate::config::TRAP_CONTEXT;
use crate::fs::{File, Stdin, Stdout};
use crate::trap::{user_trap_handler, TrapContext};
use crate::{
    mm::{kernel_token, AddrSpace, PhysPageNum, VirtAddr},
    sync::UPSafeCell,
};
use alloc::sync::{Arc, Weak};
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefMut;

#[derive(Copy, Clone, PartialEq)]
pub enum ProcessStatus {
    Ready,
    Running,
    Zombie,
}

pub struct ProcessControlBlock {
    // immutable
    pub pid: PidHandle,
    pub kernel_stack: KernelStack,
    // mutable
    inner: UPSafeCell<ProcessControlBlockInner>,
}

pub struct ProcessControlBlockInner {
    pub trap_cx_ppn: PhysPageNum,
    pub base_size: usize,
    pub proc_cx: ProcessContext,
    pub proc_status: ProcessStatus,
    pub addrspace: AddrSpace,
    pub parent: Option<Weak<ProcessControlBlock>>,
    pub children: Vec<Arc<ProcessControlBlock>>,
    pub exit_code: i32,
    pub fd_table: Vec<Option<Arc<dyn File + Send + Sync>>>,
}

impl ProcessControlBlockInner {
    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }
    pub fn get_user_token(&self) -> usize {
        self.addrspace.get_token()
    }
    // fn get_status(&self) -> ProcessStatus {
    //     self.proc_status
    // }
    // pub fn is_zombie(&self) -> bool {
    //     self.get_status() == ProcessStatus::Zombie
    // }
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
    pub fn new(elf_data: &[u8]) -> Self {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (addrspace, ustack_base, entry_point) = AddrSpace::create_user_space(elf_data);
        let trap_cx_ppn = addrspace
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        // allocate a pid
        let pid_handle = pid_alloc();
        let kernel_stack = kstack_alloc();
        let kernel_stack_top = kernel_stack.get_top();
        let process = Self {
            pid: pid_handle,
            kernel_stack,
            inner: UPSafeCell::new(ProcessControlBlockInner {
                trap_cx_ppn,
                base_size: ustack_base,
                proc_cx: ProcessContext::goto_trap_return(kernel_stack_top),
                proc_status: ProcessStatus::Ready,
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
            }),
        };
        // prepare TrapContext in user space
        let trap_cx = process.inner_exclusive_access().get_trap_cx();
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            ustack_base,
            kernel_token(),
            kernel_stack_top,
            user_trap_handler as usize,
        );
        process
    }
    // pub fn getpid(&self) -> usize {
    //     self.pid.0
    // }
}
