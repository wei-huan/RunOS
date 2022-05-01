// use super::signal::SignalFlags;
use super::context::TaskContext;
use super::kernel_stack::{kstack_alloc, KernelStack};
use super::{pid_alloc, PidHandle};
use crate::config::TRAP_CONTEXT;
use crate::fs::{FileClass, FileDescripter, Stdin, Stdout};
use crate::mm::{kernel_token, translated_refmut, AddrSpace, PhysPageNum, VirtAddr, KERNEL_SPACE};
use crate::trap::{user_trap_handler, TrapContext};
use alloc::string::String;
use alloc::sync::{Arc, Weak};
use alloc::vec;
use alloc::vec::Vec;
use spin::{Mutex, MutexGuard};

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
    inner: Mutex<TaskControlBlockInner>,
}

pub type FDTable = Vec<Option<FileDescripter>>;
pub struct TaskControlBlockInner {
    pub entry_point: usize, // 用户程序入口点 exec会改变
    pub trap_cx_ppn: PhysPageNum,
    pub ustack_bottom: usize,
    pub heap_start: usize,
    pub heap_pointer: usize,
    pub task_cx: TaskContext,
    pub task_status: TaskStatus,
    pub addrspace: AddrSpace,
    pub parent: Option<Weak<TaskControlBlock>>,
    pub children: Vec<Arc<TaskControlBlock>>,
    pub exit_code: i32,
    pub fd_table: FDTable,
    pub current_path: String,
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
    pub fn get_work_path(&self) -> String {
        self.current_path.clone()
    }
}

impl TaskControlBlock {
    pub fn acquire_inner_lock(&self) -> MutexGuard<TaskControlBlockInner> {
        self.inner.lock()
    }
    // only for initproc
    pub fn new(elf_data: &[u8]) -> Self {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (addrspace, heap_start, ustack_base, entry_point) =
            AddrSpace::create_user_space(elf_data);
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
            inner: Mutex::new(TaskControlBlockInner {
                entry_point,
                trap_cx_ppn,
                ustack_bottom: ustack_base,
                heap_start,
                heap_pointer: heap_start,
                task_cx: TaskContext::goto_trap_return(kernel_stack_top),
                task_status: TaskStatus::Ready,
                addrspace,
                parent: None,
                children: Vec::new(),
                exit_code: 0,
                fd_table: vec![
                    // 0 -> stdin
                    Some(FileDescripter::new(
                        false,
                        FileClass::Abstr(Arc::new(Stdin)),
                    )),
                    // 1 -> stdout
                    Some(FileDescripter::new(
                        false,
                        FileClass::Abstr(Arc::new(Stdout)),
                    )),
                    // 2 -> stderr
                    Some(FileDescripter::new(
                        false,
                        FileClass::Abstr(Arc::new(Stdout)),
                    )),
                ],
                current_path: String::from("/"),
            }),
        };
        // prepare TrapContext in user space
        let trap_cx = task.acquire_inner_lock().get_trap_cx();
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            ustack_base,
            kernel_token(),
            kernel_stack_top,
            user_trap_handler as usize,
        );
        task
    }
    pub fn exec(&self, elf_data: &[u8], args: Vec<String>) {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (addr_space, heap_start, mut user_sp, entry_point) =
            AddrSpace::create_user_space(elf_data);
        let trap_cx_ppn = addr_space
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();

        // // **********  argv[] *************** //
        // let mut argv: Vec<usize> = (0..=args.len()).collect();
        // argv[args.len()] = 0;
        // for i in 0..args.len() {
        //     user_sp -= args[i].len() + 1;
        //     // println!("user_sp {:#X}", user_sp);
        //     argv[i] = user_sp;
        //     let mut p = user_sp;
        //     // write chars to [user_sp, user_sp + len]
        //     for c in args[i].as_bytes() {
        //         *translated_refmut(addr_space.get_token(), p as *mut u8) = *c;
        //         // println!("({})",*c as char);
        //         p += 1;
        //     }
        //     *translated_refmut(addr_space.get_token(), p as *mut u8) = 0;
        // }
        // // make the user_sp aligned to 8B for k210 platform
        // user_sp -= user_sp % core::mem::size_of::<usize>();
        // // println!("user_sp: {:#X}", user_sp);

        // ////////////// *argv [] //////////////////////
        // user_sp -= (args.len() + 1) * core::mem::size_of::<usize>();
        // // println!("user_sp: {:#X}", user_sp);
        // *translated_refmut(
        //     addr_space.get_token(),
        //     (user_sp + core::mem::size_of::<usize>() * (args.len())) as *mut usize,
        // ) = 0;
        // for i in 0..args.len() {
        //     *translated_refmut(
        //         addr_space.get_token(),
        //         (user_sp + core::mem::size_of::<usize>() * i) as *mut usize,
        //     ) = argv[i];
        // }

        // // ************** argc *************** //
        // user_sp -= core::mem::size_of::<usize>();
        // // println!("user_sp: {:#X}", user_sp);
        // *translated_refmut(addr_space.get_token(), user_sp as *mut usize) = args.len();

        // push arguments on user stack
        user_sp -= (args.len() + 1) * core::mem::size_of::<usize>();
        let argv_base = user_sp;
        let mut argv: Vec<_> = (0..=args.len())
            .map(|arg| {
                translated_refmut(
                    addr_space.get_token(),
                    (argv_base + arg * core::mem::size_of::<usize>()) as *mut usize,
                )
            })
            .collect();
        *argv[args.len()] = 0;
        for i in 0..args.len() {
            user_sp -= args[i].len() + 1;
            *argv[i] = user_sp;
            let mut p = user_sp;
            for c in args[i].as_bytes() {
                *translated_refmut(addr_space.get_token(), p as *mut u8) = *c;
                p += 1;
            }
            *translated_refmut(addr_space.get_token(), p as *mut u8) = 0;
        }
        // make the user_sp aligned to 8B for k210 platform
        user_sp -= user_sp % core::mem::size_of::<usize>();

        // **** access current TCB exclusively
        let mut inner = self.acquire_inner_lock();
        // set new entry_point
        inner.entry_point = entry_point;
        // substitute memory_set
        inner.addrspace = addr_space;
        // update trap_cx ppn
        inner.trap_cx_ppn = trap_cx_ppn;
        // update heap_start
        inner.heap_start = heap_start;
        // update heap_pointer
        inner.heap_pointer = heap_start;
        // initialize trap_cx
        let mut trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.lock().get_token(),
            self.kernel_stack.get_top(),
            user_trap_handler as usize,
        );
        trap_cx.x[10] = args.len();
        trap_cx.x[11] = argv_base;
        *inner.get_trap_cx() = trap_cx;
        // **** release current PCB
    }
    pub fn fork(self: &Arc<TaskControlBlock>) -> Arc<TaskControlBlock> {
        // ---- hold parent PCB lock
        let mut parent_inner = self.acquire_inner_lock();
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
        let mut new_fd_table: FDTable = Vec::new();
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
            inner: Mutex::new(TaskControlBlockInner {
                entry_point: parent_inner.entry_point,
                trap_cx_ppn,
                ustack_bottom: parent_inner.ustack_bottom,
                heap_start: parent_inner.heap_start,
                heap_pointer: parent_inner.heap_pointer,
                task_cx: TaskContext::goto_trap_return(kernel_stack_top),
                task_status: TaskStatus::Ready,
                addrspace,
                parent: Some(Arc::downgrade(self)),
                children: Vec::new(),
                exit_code: 0,
                fd_table: new_fd_table,
                current_path: parent_inner.current_path.clone(),
            }),
        });
        // add child
        parent_inner.children.push(task_control_block.clone());
        // modify kernel_sp in trap_cx
        // **** access child PCB exclusively
        let trap_cx = task_control_block.acquire_inner_lock().get_trap_cx();
        trap_cx.kernel_sp = kernel_stack_top;
        // return
        task_control_block
        // **** release child PCB
        // ---- release parent PCB
    }
    pub fn getpid(&self) -> usize {
        self.pid.0
    }
    pub fn get_parent(&self) -> Option<Arc<TaskControlBlock>> {
        let inner = self.acquire_inner_lock();
        inner.parent.as_ref().unwrap().upgrade()
    }
    // initproc 不会调用 sys_getppid
    pub fn getppid(&self) -> usize {
        self.get_parent().unwrap().pid.0
    }
}
