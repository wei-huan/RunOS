// use super::signal::SignalFlags;
use super::context::TaskContext;
use super::kernel_stack::{kstack_alloc, KernelStack};
use super::{pid_alloc, PidHandle, SignalActions, SignalFlags};
use crate::config::{MMAP_BASE, PAGE_SIZE, TRAP_CONTEXT};
use crate::fs::File;
use crate::fs::{FileClass, FileDescripter, Stdin, Stdout};
use crate::hart_id;
use crate::mm::{
    kernel_token, translated_byte_buffer, translated_refmut, AddrSpace, Permission, PhysPageNum,
    UserBuffer, VirtAddr, KERNEL_SPACE,
};
use crate::task::{AuxHeader, AT_EXECFN, AT_NULL, AT_RANDOM};
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
    // mmap area
    pub mmap_area_num: usize,
    pub mmap_area_top: usize,
    // signals
    pub signals: SignalFlags,
    pub signal_mask: SignalFlags,
    // the signal which is being handling
    pub handling_sig: isize,
    // Signal actions
    pub signal_actions: SignalActions,
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
    pub fn get_user_token(&self) -> usize {
        self.addrspace.token()
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
    // pub fn have_children(&self) -> bool {
    //     self.children.len() > 0
    // }
}

impl TaskControlBlock {
    pub fn acquire_inner_lock(&self) -> MutexGuard<TaskControlBlockInner> {
        self.inner.lock()
    }
    // only for initproc
    pub fn new(elf_data: &[u8]) -> Self {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (addrspace, heap_start, ustack_base, entry_point, _) =
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
                mmap_area_num: 0,
                mmap_area_top: MMAP_BASE,
                signals: SignalFlags::empty(),
                signal_mask: SignalFlags::empty(),
                handling_sig: -1,
                signal_actions: SignalActions::default(),
                killed: false,
                frozen: false,
                trap_ctx_backup: None,
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
            hart_id(),
        );
        task
    }
    // pub fn exec(&self, elf_data: &[u8], args: Vec<String>) {
    //     // memory_set with elf program headers/trampoline/trap context/user stack
    //     let (addr_space, heap_start, mut user_sp, mut entry_point) =
    //         AddrSpace::create_user_space(elf_data);
    //     log::debug!("entry point: 0x{:X?}", entry_point);
    //     log::debug!("heap_start: 0x{:X?}", heap_start);
    //     let trap_cx_ppn = addr_space
    //         .translate(VirtAddr::from(TRAP_CONTEXT).into())
    //         .unwrap()
    //         .ppn();

    //     // push arguments on user stack
    //     user_sp -= (args.len() + 1) * core::mem::size_of::<usize>();
    //     log::debug!("user_sp: 0x{:X?}", user_sp);
    //     let argv_base = user_sp;
    //     let mut argv: Vec<_> = (0..=args.len())
    //         .map(|arg| {
    //             translated_refmut(
    //                 addr_space.get_token(),
    //                 (argv_base + arg * core::mem::size_of::<usize>()) as *mut usize,
    //             )
    //         })
    //         .collect();
    //     *argv[args.len()] = 0;
    //     for i in 0..args.len() {
    //         user_sp -= args[i].len() + 1;
    //         *argv[i] = user_sp;
    //         let mut p = user_sp;
    //         for c in args[i].as_bytes() {
    //             *translated_refmut(addr_space.get_token(), p as *mut u8) = *c;
    //             p += 1;
    //         }
    //         *translated_refmut(addr_space.get_token(), p as *mut u8) = 0;
    //     }
    //     // make the user_sp aligned to 8B for k210 platform
    //     user_sp -= user_sp % core::mem::size_of::<usize>();

    //     // **** access current TCB exclusively
    //     let mut inner = self.acquire_inner_lock();
    //     // set new entry_point
    //     inner.entry_point = entry_point;
    //     // substitute memory_set
    //     inner.addrspace = addr_space;
    //     // update trap_cx ppn
    //     inner.trap_cx_ppn = trap_cx_ppn;
    //     // update heap_start
    //     inner.heap_start = heap_start;
    //     // update heap_pointer
    //     inner.heap_pointer = heap_start;
    //     // initialize trap_cx
    //     let mut trap_cx = TrapContext::app_init_context(
    //         entry_point,
    //         user_sp,
    //         KERNEL_SPACE.lock().get_token(),
    //         self.kernel_stack.get_top(),
    //         user_trap_handler as usize,
    //     );
    //     trap_cx.x[10] = args.len();
    //     trap_cx.x[11] = argv_base;
    //     *inner.get_trap_cx() = trap_cx;
    //     // **** release current PCB
    // }
    pub fn exec(self: &Arc<Self>, elf_data: &[u8], args: Vec<String>) {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (addrspace, heap_start, mut user_sp, entry_point, mut auxv) =
            AddrSpace::create_user_space(elf_data);
        let token = addrspace.token();
        let trap_cx_ppn = addrspace
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();

        ////////////// envp[] ///////////////////
        let mut env: Vec<String> = Vec::new();
        env.push(String::from("SHELL=/user_shell"));
        env.push(String::from("PWD=/"));
        env.push(String::from("USER=root"));
        env.push(String::from("MOTD_SHOWN=pam"));
        env.push(String::from("LANG=C.UTF-8"));
        env.push(String::from(
            "INVOCATION_ID=e9500a871cf044d9886a157f53826684",
        ));
        env.push(String::from("TERM=vt220"));
        env.push(String::from("SHLVL=2"));
        env.push(String::from("JOURNAL_STREAM=8:9265"));
        env.push(String::from("OLDPWD=/root"));
        env.push(String::from("_=busybox"));
        env.push(String::from("LOGNAME=root"));
        env.push(String::from("HOME=/"));
        env.push(String::from("PATH=/"));
        let mut envp: Vec<usize> = (0..=env.len()).collect();
        envp[env.len()] = 0;

        for i in 0..env.len() {
            user_sp -= env[i].len() + 1;
            envp[i] = user_sp;
            let mut p = user_sp;
            // write chars to [user_sp, user_sp + len]
            for c in env[i].as_bytes() {
                *translated_refmut(token, p as *mut u8) = *c;
                p += 1;
            }
            *translated_refmut(token, p as *mut u8) = 0;
        }
        // make the user_sp aligned to 8B for k210 platform
        user_sp -= user_sp % core::mem::size_of::<usize>();

        ////////////// argv[] ///////////////////
        let mut argv: Vec<usize> = (0..=args.len()).collect();
        argv[args.len()] = 0;
        for i in 0..args.len() {
            user_sp -= args[i].len() + 1;
            // println!("user_sp {:X}", user_sp);
            argv[i] = user_sp;
            let mut p = user_sp;
            // write chars to [user_sp, user_sp + len]
            for c in args[i].as_bytes() {
                *translated_refmut(token, p as *mut u8) = *c;
                // print!("({})",*c as char);
                p += 1;
            }
            *translated_refmut(token, p as *mut u8) = 0;
        }
        // make the user_sp aligned to 8B for k210 platform
        user_sp -= user_sp % core::mem::size_of::<usize>();

        ////////////// platform String ///////////////////
        let platform = "RISC-V64";
        user_sp -= platform.len() + 1;
        user_sp -= user_sp % core::mem::size_of::<usize>();
        let mut p = user_sp;
        for c in platform.as_bytes() {
            *translated_refmut(token, p as *mut u8) = *c;
            p += 1;
        }
        *translated_refmut(token, p as *mut u8) = 0;

        ////////////// rand bytes ///////////////////
        user_sp -= 16;
        p = user_sp;
        auxv.push(AuxHeader {
            aux_type: AT_RANDOM,
            value: user_sp,
        });
        for i in 0..0xf {
            *translated_refmut(token, p as *mut u8) = i as u8;
            p += 1;
        }

        ////////////// padding //////////////////////
        user_sp -= user_sp % 16;

        ////////////// auxv[] //////////////////////
        auxv.push(AuxHeader {
            aux_type: AT_EXECFN,
            value: argv[0],
        }); // file name
        auxv.push(AuxHeader {
            aux_type: AT_NULL,
            value: 0,
        }); // end
        user_sp -= auxv.len() * core::mem::size_of::<AuxHeader>();
        let auxv_base = user_sp;
        // println!("[auxv]: base 0x{:X}", auxv_base);
        for i in 0..auxv.len() {
            // println!("[auxv]: {:?}", auxv[i]);
            let addr = user_sp + core::mem::size_of::<AuxHeader>() * i;
            *translated_refmut(token, addr as *mut usize) = auxv[i].aux_type;
            *translated_refmut(token, (addr + core::mem::size_of::<usize>()) as *mut usize) =
                auxv[i].value;
        }

        ////////////// *envp [] //////////////////////
        user_sp -= (env.len() + 1) * core::mem::size_of::<usize>();
        let envp_base = user_sp;
        *translated_refmut(
            token,
            (user_sp + core::mem::size_of::<usize>() * (env.len())) as *mut usize,
        ) = 0;
        for i in 0..env.len() {
            *translated_refmut(
                token,
                (user_sp + core::mem::size_of::<usize>() * i) as *mut usize,
            ) = envp[i];
        }

        ////////////// *argv [] //////////////////////
        user_sp -= (args.len() + 1) * core::mem::size_of::<usize>();
        let argv_base = user_sp;
        *translated_refmut(
            token,
            (user_sp + core::mem::size_of::<usize>() * (args.len())) as *mut usize,
        ) = 0;
        for i in 0..args.len() {
            *translated_refmut(
                token,
                (user_sp + core::mem::size_of::<usize>() * i) as *mut usize,
            ) = argv[i];
        }

        ////////////// argc //////////////////////
        user_sp -= core::mem::size_of::<usize>();
        *translated_refmut(token, user_sp as *mut usize) = args.len();

        // **** access current TCB exclusively
        let mut inner = self.acquire_inner_lock();
        // set new entry_point
        inner.entry_point = entry_point;
        // substitute addrspace
        inner.addrspace = addrspace;
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
            KERNEL_SPACE.lock().token(),
            self.kernel_stack.get_top(),
            user_trap_handler as usize,
            hart_id(),
        );
        trap_cx.x[10] = args.len();
        trap_cx.x[11] = argv_base;
        trap_cx.x[12] = envp_base;
        trap_cx.x[13] = auxv_base;
        *inner.get_trap_cx() = trap_cx;
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
                mmap_area_num: parent_inner.mmap_area_num,
                mmap_area_top: parent_inner.mmap_area_top,
                signals: SignalFlags::empty(),
                // inherit the signal_mask and signal_action
                signal_mask: parent_inner.signal_mask,
                handling_sig: -1,
                signal_actions: parent_inner.signal_actions.clone(),
                killed: false,
                frozen: false,
                trap_ctx_backup: None,
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
    // 假设每次新 mmap 的区域和过往不重叠, 目前只支持创建一个mmap区域
    pub fn mmap(
        &self,
        mut start: usize,
        length: usize,
        prot: usize,
        _flags: usize,
        fd: isize,
        offset: usize,
    ) -> isize {
        let mut inner = self.acquire_inner_lock();
        let token = inner.get_user_token();
        inner.mmap_area_num += 1;
        // prot<<1 is equal to  meaning of Permission, 1<<4 means user
        let map_flags = (((prot & 0b111) << 1) + (1 << 4)) as u8;
        // log::debug!("mmap start: 0x{:#X}", start);
        // 如果没有 mmap section 就创建 mmap section
        if start == 0 {
            start = inner.mmap_area_top;
            log::debug!("mmap new start: 0x{:#X}", start);
            inner.mmap_area_top = VirtAddr::from(start + length).ceil().0 * PAGE_SIZE;
            log::debug!("mmap new start: 0x{:#X}", inner.mmap_area_top);
            inner.addrspace.create_mmap_section(
                start,
                length,
                Permission::from_bits(map_flags).unwrap(),
            );
        } else {
            inner.addrspace.create_mmap_section(
                start,
                length,
                Permission::from_bits(map_flags).unwrap(),
            );
        }
        let fd_table = inner.fd_table.clone();
        if fd < 0 || fd as usize >= fd_table.len() {
            return start as isize;
        }
        if let Some(file) = &fd_table[fd as usize] {
            match &file.fclass {
                FileClass::File(f) => {
                    f.set_offset(offset);
                    if !f.readable() {
                        return -1;
                    }
                    //println!{"The va_start is 0x{:X}, offset of file is {}", va_start.0, offset};
                    let read_len = f.read(UserBuffer::new(translated_byte_buffer(
                        token,
                        start as *const u8,
                        length,
                    )));
                    //println!{"read {} bytes", read_len};
                    return start as isize;
                }
                _ => {
                    return -1;
                }
            };
        } else {
            return -1;
        };
    }
    pub fn munmap(&self, start: usize, _length: usize) -> isize {
        let mut inner = self.acquire_inner_lock();
        inner
            .addrspace
            .remove_mmap_area_with_start_vpn(VirtAddr::from(start).into());
        0
    }
}
