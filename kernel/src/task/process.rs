use super::id::RecycleAllocator;
use super::TaskControlBlock;
use super::{pid_alloc, PidHandle};
use super::{SignalActions, SignalFlags};
use crate::config::MMAP_BASE;
use crate::fs::{File, FileClass, FileDescripter, Stdin, Stdout};
use crate::hart_id;
use crate::mm::{
    translated_byte_buffer, translated_refmut, AddrSpace, MMapFlags, MapPermission, UserBuffer,
    VirtAddr, KERNEL_SPACE,
};
use crate::scheduler::{add_task, insert_into_pid2process};
use crate::syscall::{EBADF, ENOENT, EPERM};
use crate::task::{AuxHeader, AT_EXECFN, AT_NULL, AT_RANDOM};
use crate::trap::{user_trap_handler, TrapContext};
use alloc::string::String;
use alloc::sync::{Arc, Weak};
use alloc::vec;
use alloc::vec::Vec;
use bitflags::*;
use spin::{Mutex, MutexGuard};

bitflags! {
    pub struct CloneFlags: u32 {
        const CSIGNAL              = 0xff;
        const CLONE_VM             = 0x00000100;
        const CLONE_FS             = 0x00000200;
        const CLONE_FILES          = 0x00000400;
        const CLONE_SIGHAND        = 0x00000800;
        const CLONE_PIDFD          = 0x00001000;
        const CLONE_PTRACE         = 0x00002000;
        const CLONE_VFORK          = 0x00004000;
        const CLONE_PARENT         = 0x00008000;
        const CLONE_THREAD         = 0x00010000;
        const CLONE_NEWNS          = 0x00020000;
        const CLONE_SYSVSEM        = 0x00040000;
        const CLONE_SETTLS         = 0x00080000;
        const CLONE_PARENT_SETTID  = 0x00100000;
        const CLONE_CHILD_CLEARTID = 0x00200000;
        const CLONE_DETACHED       = 0x00400000;
        const CLONE_UNTRACED       = 0x00800000;
        const CLONE_CHILD_SETTID   = 0x01000000;
        const CLONE_NEWCGROUP      = 0x02000000;
        const CLONE_NEWUTS         = 0x04000000;
        const CLONE_NEWIPC         = 0x08000000;
        const CLONE_NEWUSER        = 0x10000000;
        const CLONE_NEWPID         = 0x20000000;
        const CLONE_NEWNET         = 0x40000000;
        const CLONE_IO             = 0x80000000;
    }
}

pub struct ProcessControlBlock {
    // immutable
    pub pid: PidHandle,
    // mutable
    inner: Mutex<ProcessControlBlockInner>,
}

pub type FDTable = Vec<Option<FileDescripter>>;
pub struct ProcessControlBlockInner {
    pub is_zombie: bool,
    pub address_space: AddrSpace,
    pub parent: Option<Weak<ProcessControlBlock>>,
    pub children: Vec<Arc<ProcessControlBlock>>,
    pub exit_code: i32,
    pub fd_table: FDTable,
    pub heap_start: usize,
    pub heap_pointer: usize,
    pub current_path: String,
    // mmap area
    pub mmap_area_hint: usize,
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
    pub tasks: Vec<Option<Arc<TaskControlBlock>>>,
    pub task_res_allocator: RecycleAllocator,
}

impl ProcessControlBlockInner {
    #[allow(unused)]
    pub fn get_user_token(&self) -> usize {
        self.address_space.token()
    }

    pub fn alloc_fd(&mut self) -> usize {
        if let Some(fd) = (0..self.fd_table.len()).find(|fd| self.fd_table[*fd].is_none()) {
            fd
        } else {
            self.fd_table.push(None);
            self.fd_table.len() - 1
        }
    }

    pub fn alloc_tid(&mut self) -> usize {
        self.task_res_allocator.alloc()
    }

    pub fn dealloc_tid(&mut self, tid: usize) {
        self.task_res_allocator.dealloc(tid)
    }

    pub fn thread_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn get_task(&self, tid: usize) -> Arc<TaskControlBlock> {
        self.tasks[tid].as_ref().unwrap().clone()
    }
}

impl ProcessControlBlock {
    pub fn acquire_inner_lock(&self) -> MutexGuard<ProcessControlBlockInner> {
        self.inner.lock()
    }
    // only for initproc
    pub fn new(elf_data: &[u8]) -> Arc<Self> {
        println!("here new0");
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (address_space, heap_start, ustack_base, entry_point, _) =
            AddrSpace::create_user_space(elf_data);
        println!("here new1");
        // allocate a pid
        let pid_handle = pid_alloc();
        let process = Arc::new(Self {
            pid: pid_handle,
            inner: Mutex::new(ProcessControlBlockInner {
                is_zombie: false,
                address_space,
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
                heap_start,
                heap_pointer: heap_start,
                current_path: String::from("/"),
                mmap_area_hint: MMAP_BASE,
                signals: SignalFlags::empty(),
                signal_mask: SignalFlags::empty(),
                handling_sig: -1,
                signal_actions: SignalActions::default(),
                killed: false,
                frozen: false,
                tasks: Vec::new(),
                task_res_allocator: RecycleAllocator::new(),
            }),
        });
        println!("here new2");
        // create a main thread, we should allocate ustack and trap_cx here
        let task = Arc::new(TaskControlBlock::new(
            Arc::clone(&process),
            ustack_base,
            true,
        ));
        println!("here new3");
        // prepare trap_cx of main thread
        let task_inner = task.acquire_inner_lock();
        let trap_cx = task_inner.get_trap_cx();
        let ustack_top = task_inner.res.as_ref().unwrap().ustack_top();
        let kstack_top = task.kstack.get_top();
        drop(task_inner);
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            ustack_top,
            KERNEL_SPACE.lock().token(),
            kstack_top,
            user_trap_handler as usize,
            hart_id(),
        );
        println!("here new4");
        // add main thread to the process
        let mut process_inner = process.acquire_inner_lock();
        process_inner.tasks.push(Some(Arc::clone(&task)));
        drop(process_inner);
        insert_into_pid2process(process.getpid(), Arc::clone(&process));
        // add main thread to scheduler
        add_task(task);
        println!("here new 100");
        process
    }

    /// Only support processes with a single thread.
    pub fn exec(self: &Arc<Self>, elf_data: &[u8], args: Vec<String>) {
        assert_eq!(self.acquire_inner_lock().thread_count(), 1);
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (address_space, heap_start, ustack_base, entry_point, mut auxv) =
            AddrSpace::create_user_space(elf_data);

        let new_token = address_space.token();

        // substitute memory_set
        self.acquire_inner_lock().address_space = address_space;
        self.acquire_inner_lock().heap_start = heap_start;
        self.acquire_inner_lock().heap_pointer = heap_start;
        self.acquire_inner_lock().mmap_area_hint = MMAP_BASE;
        // then we alloc user resource for main thread again
        // since memory_set has been changed
        let task = self.acquire_inner_lock().get_task(0);
        let mut task_inner = task.acquire_inner_lock();
        task_inner.res.as_mut().unwrap().ustack_base = ustack_base;
        task_inner.res.as_mut().unwrap().alloc_user_res();
        task_inner.trap_cx_ppn = task_inner.res.as_mut().unwrap().trap_cx_ppn();

        let mut user_sp = task_inner.res.as_mut().unwrap().ustack_top();
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
        env.push(String::from("LD_LIBRARY_PATH=/"));
        let mut envp: Vec<usize> = (0..=env.len()).collect();
        envp[env.len()] = 0;

        for i in 0..env.len() {
            user_sp -= env[i].len() + 1;
            envp[i] = user_sp;
            let mut p = user_sp;
            // write chars to [user_sp, user_sp + len]
            for c in env[i].as_bytes() {
                *translated_refmut(new_token, p as *mut u8) = *c;
                p += 1;
            }
            *translated_refmut(new_token, p as *mut u8) = 0;
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
                *translated_refmut(new_token, p as *mut u8) = *c;
                // print!("({})",*c as char);
                p += 1;
            }
            *translated_refmut(new_token, p as *mut u8) = 0;
        }
        // make the user_sp aligned to 8B for k210 platform
        user_sp -= user_sp % core::mem::size_of::<usize>();

        ////////////// platform String ///////////////////
        let platform = "RISC-V64";
        user_sp -= platform.len() + 1;
        user_sp -= user_sp % core::mem::size_of::<usize>();
        let mut p = user_sp;
        for c in platform.as_bytes() {
            *translated_refmut(new_token, p as *mut u8) = *c;
            p += 1;
        }
        *translated_refmut(new_token, p as *mut u8) = 0;

        ////////////// rand bytes ///////////////////
        user_sp -= 16;
        p = user_sp;
        auxv.push(AuxHeader {
            aux_type: AT_RANDOM,
            value: user_sp,
        });
        for i in 0..0xf {
            *translated_refmut(new_token, p as *mut u8) = i as u8;
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
            *translated_refmut(new_token, addr as *mut usize) = auxv[i].aux_type;
            *translated_refmut(
                new_token,
                (addr + core::mem::size_of::<usize>()) as *mut usize,
            ) = auxv[i].value;
        }

        ////////////// *envp [] //////////////////////
        user_sp -= (env.len() + 1) * core::mem::size_of::<usize>();
        let envp_base = user_sp;
        *translated_refmut(
            new_token,
            (user_sp + core::mem::size_of::<usize>() * (env.len())) as *mut usize,
        ) = 0;
        for i in 0..env.len() {
            *translated_refmut(
                new_token,
                (user_sp + core::mem::size_of::<usize>() * i) as *mut usize,
            ) = envp[i];
        }

        ////////////// *argv [] //////////////////////
        user_sp -= (args.len() + 1) * core::mem::size_of::<usize>();
        let argv_base = user_sp;
        *translated_refmut(
            new_token,
            (user_sp + core::mem::size_of::<usize>() * (args.len())) as *mut usize,
        ) = 0;
        for i in 0..args.len() {
            *translated_refmut(
                new_token,
                (user_sp + core::mem::size_of::<usize>() * i) as *mut usize,
            ) = argv[i];
        }

        ////////////// argc //////////////////////
        user_sp -= core::mem::size_of::<usize>();
        *translated_refmut(new_token, user_sp as *mut usize) = args.len();

        // initialize trap_cx
        let mut trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.lock().token(),
            task.kstack.get_top(),
            user_trap_handler as usize,
            hart_id(),
        );

        trap_cx.x[10] = args.len();
        trap_cx.x[11] = argv_base;
        trap_cx.x[12] = envp_base;
        trap_cx.x[13] = auxv_base;

        *task_inner.get_trap_cx() = trap_cx;
    }

    pub fn clone_thread(
        self: &Arc<Self>,
        parent_task: Arc<TaskControlBlock>,
        flags: CloneFlags,
        stack: usize,
        newtls: usize,
    ) -> Arc<TaskControlBlock> {
        // only the main thread can create a sub-thread
        assert_eq!(parent_task.acquire_inner_lock().gettid(), 0);
        // create main thread of child process
        let task = Arc::new(TaskControlBlock::new(
            Arc::clone(self),
            parent_task
                .acquire_inner_lock()
                .res
                .as_ref()
                .unwrap()
                .ustack_base(),
            // mention that we allocate a new kstack / ustack / trap_cx here
            true,
        ));
        let task_inner = task.acquire_inner_lock();
        let trap_cx = task_inner.get_trap_cx();

        // copy trap_cx from the parent thread
        *trap_cx = *parent_task.acquire_inner_lock().get_trap_cx();

        // modify kstack_top in trap_cx of this thread
        trap_cx.kernel_sp = task.kstack.get_top();
        // sys_fork return value ...
        if stack != 0 {
            trap_cx.set_sp(stack);
        }
        trap_cx.x[10] = 0;
        if flags.contains(CloneFlags::CLONE_SETTLS) {
            trap_cx.x[4] = newtls;
        }

        drop(task_inner);
        // add this thread to scheduler
        add_task(Arc::clone(&task));

        task
    }

    /// Only support processes with a single thread.
    pub fn fork(
        self: &Arc<Self>,
        flags: CloneFlags,
        stack_ptr: usize,
        newtls: usize,
    ) -> Arc<ProcessControlBlock> {
        let mut parent = self.acquire_inner_lock();
        assert_eq!(parent.thread_count(), 1);
        // clone parent's memory_set completely including trampoline/ustacks/trap_cxs
        let address_space = AddrSpace::from_existed_user(&parent.address_space);
        // alloc a pid
        let pid = pid_alloc();
        // copy fd table
        let mut new_fd_table = Vec::new();
        for fd in parent.fd_table.iter() {
            if let Some(file) = fd {
                new_fd_table.push(Some(file.clone()));
            } else {
                new_fd_table.push(None);
            }
        }

        // create child process pcb
        let child = Arc::new(Self {
            pid,
            inner: Mutex::new(ProcessControlBlockInner {
                is_zombie: false,
                address_space,
                parent: Some(Arc::downgrade(self)),
                children: Vec::new(),
                exit_code: 0,
                fd_table: new_fd_table,
                heap_start: parent.heap_start,
                heap_pointer: parent.heap_pointer,
                current_path: parent.current_path.clone(),
                mmap_area_hint: parent.mmap_area_hint,
                signals: SignalFlags::empty(),
                // inherit the signal_mask and signal_action
                signal_mask: parent.signal_mask,
                handling_sig: -1,
                signal_actions: parent.signal_actions.clone(),
                killed: false,
                frozen: false,
                tasks: Vec::new(),
                task_res_allocator: RecycleAllocator::new(),
            }),
        });
        // add child
        parent.children.push(Arc::clone(&child));
        // create main thread of child process
        let task = Arc::new(TaskControlBlock::new(
            Arc::clone(&child),
            parent
                .get_task(0)
                .acquire_inner_lock()
                .res
                .as_ref()
                .unwrap()
                .ustack_base(),
            // here we do not allocate trap_cx or ustack again
            // but mention that we allocate a new kstack here
            false,
        ));
        // attach task to child process
        let mut child_inner = child.acquire_inner_lock();
        child_inner.tasks.push(Some(Arc::clone(&task)));
        drop(child_inner);
        // modify kstack_top in trap_cx of this thread
        let task_inner = task.acquire_inner_lock();
        let trap_cx = task_inner.get_trap_cx();
        trap_cx.kernel_sp = task.kstack.get_top();

        if stack_ptr != 0 {
            trap_cx.set_sp(stack_ptr);
        }

        // for child process, fork returns 0
        trap_cx.x[10] = 0;

        if flags.contains(CloneFlags::CLONE_SETTLS) {
            trap_cx.x[4] = newtls;
        }

        drop(task_inner);
        insert_into_pid2process(child.getpid(), Arc::clone(&child));
        // add this thread to scheduler
        add_task(task);
        child
    }

    pub fn getpid(&self) -> usize {
        self.pid.0
    }
    pub fn getppid(&self) -> usize {
        self.get_parent().unwrap().pid.0
    }
    pub fn get_parent(&self) -> Option<Arc<ProcessControlBlock>> {
        let inner = self.acquire_inner_lock();
        inner.parent.as_ref().unwrap().upgrade()
    }
    pub fn mmap(
        &self,
        mut start: usize,
        length: usize,
        prot: usize,
        flags: usize,
        fd: isize,
        offset: usize,
    ) -> isize {
        let mut inner = self.acquire_inner_lock();
        let token = inner.get_user_token();
        // prot << 1 is equal to meaning of MapPermission
        // TODO: Real Implementation of Adjust MMap Section permission
        let mmap_perm = MapPermission::from_bits((prot << 1) as u8).unwrap()
            | MapPermission::U
            | MapPermission::W
            | MapPermission::R
            | MapPermission::X;
        let mmap_flag = MMapFlags::from_bits(flags).unwrap();
        log::debug!(
            "start {:#X}, length: {:#X}, fd: {:#X}, offset: {:#X}, flags: {:?}, mmap_flag: {:?}",
            start,
            length,
            fd,
            offset,
            mmap_perm,
            mmap_flag
        );
        /* mmap section */
        // need hint
        if start == 0 {
            start = inner.mmap_area_hint;
            log::trace!("mmap need hint start before map: {:#X}", start);
            inner.mmap_area_hint = inner
                .address_space
                .create_mmap_section(start, length, mmap_perm)
                .into();
            log::debug!(
                "mmap need hint start after map: {:#X}",
                inner.mmap_area_hint
            );
        }
        // another mmaping already exist there, need to picks a new address depending on the hint
        else if inner
            .address_space
            .is_mmap_section_exist(VirtAddr::from(start).floor())
        {
            // Don't interpret addr as a hint: place the mapping at exactly that address.
            if mmap_flag.contains(MMapFlags::MAP_FIXED) {
                // TODO: Real Implementation of Adjust MMap Section
                // inner
                //     .address_space
                //     .remove_mmap_area_with_overlap(VirtAddr::from(start).floor());
                // inner.mmap_area_hint = inner
                //     .address_space
                //     .create_mmap_section(start - 0x1000, length, mmap_perm)
                //     .into();
            } else {
                start = inner.mmap_area_hint;
                log::trace!("mmap need to pick new place start before map: {:#X}", start);
                inner.mmap_area_hint = inner
                    .address_space
                    .create_mmap_section(start, length, mmap_perm)
                    .into();
                log::trace!(
                    "mmap need to pick new place start after map: {:#X}",
                    inner.mmap_area_hint
                );
            }
        }
        // no conflict, just map it
        else {
            log::trace!("mmap start before map: {:#X}", start);
            inner.mmap_area_hint = inner
                .address_space
                .create_mmap_section(start, length, mmap_perm)
                .into();
            log::trace!("mmap hint after map: {:#X}", inner.mmap_area_hint);
        }

        /* File Content Copy */
        let fd_table = inner.fd_table.clone();
        let mmap_flag = MMapFlags::from_bits(flags).unwrap();
        if fd < 0 || mmap_flag.contains(MMapFlags::MAP_ANONYMOUS) {
            log::trace!("mmap here no need file");
            return start as isize;
        }
        if fd as usize >= fd_table.len() {
            return -EBADF;
        }
        if let Some(file) = &fd_table[fd as usize] {
            match &file.fclass {
                FileClass::File(f) => {
                    if !f.readable() {
                        return -EPERM;
                    }
                    f.set_offset(offset);
                    log::trace! {"The va_start is {:#?}, offset of file is {:#X?}, file_size: {:#X?}", VirtAddr::from(start), offset, f.get_size()};
                    let read_len = f.read(UserBuffer::new(translated_byte_buffer(
                        token, start as _, length,
                    )));
                    log::trace! {"read {:#X?} bytes", read_len};
                    return start as isize;
                }
                _ => {
                    return -ENOENT;
                }
            };
        } else {
            return -ENOENT;
        };
    }

    pub fn munmap(&self, start: usize, _length: usize) -> isize {
        let mut inner = self.acquire_inner_lock();
        inner
            .address_space
            .remove_mmap_area_with_start_vpn(VirtAddr::from(start).into());
        0
    }
}
