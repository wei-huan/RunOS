// use super::signal::SignalFlags;
use super::context::TaskContext;
use super::kernel_stack::{kstack_alloc, KernelStack};
use super::{pid_alloc, PidHandle};
use crate::config::TRAP_CONTEXT;
use crate::fs::{File, Stdin, Stdout};
use crate::trap::{user_trap_handler, TrapContext};
use crate::{
    mm::{kernel_token, AddrSpace, PhysPageNum, VirtAddr, KERNEL_SPACE, translated_refmut},
    sync::UPSafeCell,
};
use alloc::sync::{Arc, Weak};
use alloc::vec;
use alloc::vec::Vec;
use alloc::string::String;
use core::cell::RefMut;

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    Ready,
    Running,
    Zombie,
}

pub struct TaskControlBlock {
    // immutable
    pub pid: PidHandle,
    pub kernel_stack: KernelStack,
    // mutable
    inner: UPSafeCell<TaskControlBlockInner>,
}

pub struct TaskControlBlockInner {
    pub entry_point: usize, // 用户程序入口点 exec会改变
    pub trap_cx_ppn: PhysPageNum,
    pub ustack_bottom: usize,
    pub uheap_base: usize,
    pub uheap_pt: usize,
    pub task_cx: TaskContext,
    pub task_status: TaskStatus,
    pub addrspace: AddrSpace,
    pub parent: Option<Weak<TaskControlBlock>>,
    pub children: Vec<Arc<TaskControlBlock>>,
    pub exit_code: i32,
    pub fd_table: Vec<Option<Arc<dyn File + Send + Sync>>>,
}

impl TaskControlBlockInner {
    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }
    pub fn get_user_token(&self) -> usize {
        self.addrspace.get_token()
    }
    fn get_status(&self) -> TaskStatus {
        self.task_status
    }
    pub fn is_zombie(&self) -> bool {
        self.get_status() == TaskStatus::Zombie
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

impl TaskControlBlock {
    pub fn inner_exclusive_access(&self) -> RefMut<'_, TaskControlBlockInner> {
        self.inner.exclusive_access()
    }
    pub fn new(elf_data: &[u8]) -> Self {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (addrspace, ustack_base, uheap_base, entry_point) = AddrSpace::create_user_space(elf_data);
        let trap_cx_ppn = addrspace
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        // allocate a pid
        let pid_handle = pid_alloc();
        let kernel_stack = kstack_alloc();
        let kernel_stack_top = kernel_stack.get_top();
        let task = Self {
            pid: pid_handle,
            kernel_stack,
            inner: unsafe {
                UPSafeCell::new(TaskControlBlockInner {
                    entry_point,
                    trap_cx_ppn,
                    ustack_bottom: ustack_base,
                    uheap_base: uheap_base,
                    uheap_pt: uheap_base,
                    task_cx: TaskContext::goto_trap_return(kernel_stack_top),
                    task_status: TaskStatus::Ready,
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
                })
            },
        };
        // prepare TrapContext in user space
        let trap_cx = task.inner_exclusive_access().get_trap_cx();
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            ustack_base,
            kernel_token(),
            kernel_stack_top,
            user_trap_handler as usize,
        );
        task
    }
    // pub fn reset_trap_context(&mut self) {
    //     let inner = self.inner_exclusive_access();
    //     let trap_cx = inner.get_trap_cx();
    //     *trap_cx = TrapContext::app_init_context(
    //         self.entry_point,
    //         inner.ustack_bottom,
    //         kernel_token(),
    //         self.kernel_stack.get_top(),
    //         user_trap_handler as usize,
    //     );
    // }
    // pub fn reset_task_context(&mut self) {
    //     let mut inner = self.inner_exclusive_access();
    //     let kernel_stack_top = self.kernel_stack.get_top();
    //     inner.task_cx = TaskContext::goto_trap_return(kernel_stack_top);
    // }
    pub fn exec(&self, elf_data: &[u8], args: Vec<String>) {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (addr_space, mut user_sp, uheap_base, entry_point) = AddrSpace::create_user_space(elf_data);
        let trap_cx_ppn = addr_space
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();

        // **********  argv[] *************** //
        let mut argv: Vec<usize> = (0..=args.len()).collect();
        argv[args.len()] = 0;
        for i in 0..args.len() {
            user_sp -= args[i].len() + 1;
            // println!("user_sp {:#X}", user_sp);
            argv[i] = user_sp;
            let mut p = user_sp;
            // write chars to [user_sp, user_sp + len]
            for c in args[i].as_bytes() {
                *translated_refmut(addr_space.get_token(), p as *mut u8) = *c;
                // println!("({})",*c as char);
                p += 1;
            }
            *translated_refmut(addr_space.get_token(), p as *mut u8) = 0;
        }
        // make the user_sp aligned to 8B for k210 platform
        user_sp -= user_sp % core::mem::size_of::<usize>();
        // println!("user_sp: {:#X}", user_sp);

        ////////////// *argv [] //////////////////////
        user_sp -= (args.len() + 1) * core::mem::size_of::<usize>();
        // println!("user_sp: {:#X}", user_sp);
        let argv_base = user_sp;
        *translated_refmut(addr_space.get_token(), (user_sp + core::mem::size_of::<usize>() * (args.len())) as *mut usize) = 0;
        for i in 0..args.len() {
            *translated_refmut(addr_space.get_token(), (user_sp + core::mem::size_of::<usize>() * i) as *mut usize) = argv[i] ;
        }

        // ************** argc *************** //
        user_sp -= core::mem::size_of::<usize>();
        // println!("user_sp: {:#X}", user_sp);
        *translated_refmut(addr_space.get_token(), user_sp as *mut usize) = args.len();

        // **** access current TCB exclusively
        let mut inner = self.inner_exclusive_access();
        // set new entry_point
        inner.entry_point = entry_point;
        // substitute memory_set
        inner.addrspace = addr_space;
        // update trap_cx ppn
        inner.trap_cx_ppn = trap_cx_ppn;
        // update user heap
        inner.uheap_base = uheap_base;
        inner.uheap_pt = uheap_base;
        // initialize trap_cx
        let trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.lock().get_token(),
            self.kernel_stack.get_top(),
            user_trap_handler as usize,
        );
        *inner.get_trap_cx() = trap_cx;
        // **** release current PCB
    }
    pub fn fork(self: &Arc<TaskControlBlock>) -> Arc<TaskControlBlock> {
        // ---- hold parent PCB lock
        let mut parent_inner = self.inner_exclusive_access();
        // copy user space(include trap context)
        let addrspace = AddrSpace::from_existed_user(&parent_inner.addrspace);
        let trap_cx_ppn = addrspace
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        // alloc a pid and a kernel stack in kernel space
        let pid_handle = pid_alloc();
        let kernel_stack = kstack_alloc();
        let kernel_stack_top = kernel_stack.get_top();
        // copy fd table
        let mut new_fd_table: Vec<Option<Arc<dyn File + Send + Sync>>> = Vec::new();
        for fd in parent_inner.fd_table.iter() {
            if let Some(file) = fd {
                new_fd_table.push(Some(file.clone()));
            } else {
                new_fd_table.push(None);
            }
        }
        let task_control_block = Arc::new(TaskControlBlock {
            pid: pid_handle,
            kernel_stack,
            inner: unsafe {
                UPSafeCell::new(TaskControlBlockInner {
                    entry_point: parent_inner.entry_point,
                    trap_cx_ppn,
                    ustack_bottom: parent_inner.ustack_bottom,
                    uheap_base: parent_inner.uheap_base,
                    uheap_pt: parent_inner.uheap_pt,
                    task_cx: TaskContext::goto_trap_return(kernel_stack_top),
                    task_status: TaskStatus::Ready,
                    addrspace,
                    parent: Some(Arc::downgrade(self)),
                    children: Vec::new(),
                    exit_code: 0,
                    fd_table: new_fd_table,
                })
            },
        });
        // add child
        parent_inner.children.push(task_control_block.clone());
        // modify kernel_sp in trap_cx
        // **** access child PCB exclusively
        let trap_cx = task_control_block.inner_exclusive_access().get_trap_cx();
        trap_cx.kernel_sp = kernel_stack_top;
        // return
        task_control_block
        // **** release child PCB
        // ---- release parent PCB
    }
    pub fn getpid(&self) -> usize {
        self.pid.0
    }
}
