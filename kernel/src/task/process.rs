use crate::config::{page_aligned_up, FD_LIMIT, MMAP_BASE};
use crate::fs::{File, FileClass, Stdin, Stdout};
use crate::hart_id;
use crate::mm::{
    kernel_token, translated_byte_buffer, translated_refmut, AddrSpace, MMapFlags, MapPermission,
    UserBuffer, VirtAddr,
};
use crate::scheduler::{add_task, insert_into_pid2process, insert_into_tid2task};
use crate::syscall::{EBADF, ENOENT, EPERM};
use crate::task::{
    pid_alloc, AuxHeader, PidHandle, SignalActions, TaskControlBlock, AT_EXECFN, AT_NULL, AT_RANDOM,
};
use crate::trap::{user_trap_handler, TrapContext};
use alloc::string::String;
use alloc::sync::{Arc, Weak};
use alloc::vec;
use alloc::vec::Vec;
use spin::{Mutex, MutexGuard};

pub struct ProcessControlBlock {
    // immutable
    pub pid: PidHandle,
    // mutable
    inner: Mutex<ProcessControlBlockInner>,
}

pub type FDTable = Vec<Option<FileClass>>;
pub struct ProcessControlBlockInner {
    pub entry_point: usize,
    pub is_zombie: bool,
    pub addrspace: AddrSpace,
    pub parent: Option<Weak<ProcessControlBlock>>,
    pub children: Vec<Arc<ProcessControlBlock>>,
    pub tasks: Vec<Arc<TaskControlBlock>>,
    pub exit_code: i32,
    pub fd_table: FDTable,
    pub fd_limit: usize,
    pub current_path: String,
    pub heap_start: usize,
    pub heap_pointer: usize,
    pub mmap_area_hint: usize,
    pub signal_actions: SignalActions,
}

impl ProcessControlBlockInner {
    pub fn is_zombie(&self) -> bool {
        self.is_zombie
    }
    pub fn get_user_token(&self) -> usize {
        self.addrspace.token()
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
    pub fn thread_count(&self) -> usize {
        self.tasks.len()
    }
    pub fn get_task(&self, lid: usize) -> Arc<TaskControlBlock> {
        self.tasks[lid].clone()
    }
}

impl ProcessControlBlock {
    pub fn acquire_inner_lock(&self) -> MutexGuard<ProcessControlBlockInner> {
        self.inner.lock()
    }
    // only for initproc
    pub fn new(elf_data: &[u8]) -> Arc<Self> {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (addrspace, heap_start, _, entry_point, _) = AddrSpace::create_user_space(elf_data);
        // allocate a pid
        let pid_handle = pid_alloc();
        let process = Arc::new(Self {
            pid: pid_handle,
            inner: Mutex::new(ProcessControlBlockInner {
                entry_point,
                is_zombie: false,
                heap_start,
                heap_pointer: heap_start,
                addrspace,
                parent: None,
                children: Vec::new(),
                tasks: Vec::new(),
                exit_code: 0,
                fd_table: vec![
                    // 0 -> stdin
                    Some(FileClass::Abstr(Arc::new(Stdin))),
                    // 1 -> stdout
                    Some(FileClass::Abstr(Arc::new(Stdout))),
                    // 2 -> stderr
                    Some(FileClass::Abstr(Arc::new(Stdout))),
                ],
                fd_limit: FD_LIMIT,
                current_path: String::from("/"),
                mmap_area_hint: MMAP_BASE,
                signal_actions: SignalActions::default(),
            }),
        });
        let main_task = Arc::new(TaskControlBlock::new(process.clone(), 0, true));
        insert_into_tid2task(main_task.gettid(), main_task.clone());
        // prepare trap_cx of main thread
        let task_inner = main_task.acquire_inner_lock();
        let ustack_top = task_inner.res.as_ref().unwrap().ustack_top();
        let kstack_top = main_task.kernel_stack.get_top();
        let trap_cx = task_inner.get_trap_cx();
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            ustack_top,
            kernel_token(),
            kstack_top,
            user_trap_handler as usize,
            hart_id(),
        );
        // add main thread to the process
        let mut process_inner = process.acquire_inner_lock();
        process_inner.tasks.push(main_task.clone());

        drop(task_inner);
        drop(process_inner);
        insert_into_pid2process(process.getpid(), process.clone());
        // add main thread to scheduler
        add_task(main_task);
        process
    }

    /// Only support processes with a single thread.
    pub fn exec(self: &Arc<Self>, elf_data: &[u8], args: Vec<String>) {
        assert_eq!(
            self.acquire_inner_lock().thread_count(),
            1,
            "process can't execve with multithreads"
        );
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (addrspace, heap_start, _, entry_point, mut auxv) =
            AddrSpace::create_user_space(elf_data);
        let token = addrspace.token();
        // **** access current PCB exclusively
        let mut process_inner = self.acquire_inner_lock();
        // set new entry_point
        process_inner.entry_point = entry_point;
        // substitute addrspace
        process_inner.addrspace = addrspace;
        // update heap_start
        process_inner.heap_start = heap_start;
        // update heap_pointer
        process_inner.heap_pointer = heap_start;
        // update mmap hint
        process_inner.mmap_area_hint = MMAP_BASE;
        // fresh fd_table
        process_inner.fd_table = vec![
            // 0 -> stdin
            Some(FileClass::Abstr(Arc::new(Stdin))),
            // 1 -> stdout
            Some(FileClass::Abstr(Arc::new(Stdout))),
            // 2 -> stderr
            Some(FileClass::Abstr(Arc::new(Stdout))),
        ];
        // reset fd_limit
        process_inner.fd_limit = FD_LIMIT;
        // get main task and allocate user resource
        let task = process_inner.get_task(0);
        drop(process_inner);

        let mut task_inner = task.acquire_inner_lock();
        task_inner.res.as_mut().unwrap().alloc_user_res();
        task_inner.trap_cx_ppn = task_inner.res.as_mut().unwrap().trap_cx_ppn();

        let mut user_sp = task_inner.res.as_ref().unwrap().ustack_top();

        ////////////// *envp[] ///////////////////
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
                *translated_refmut(token, p as *mut u8) = *c;
                p += 1;
            }
            *translated_refmut(token, p as *mut u8) = 0;
        }
        // make the user_sp aligned to 8B for k210 platform
        user_sp -= user_sp % core::mem::size_of::<usize>();

        ////////////// *argv[] ///////////////////
        let mut argv: Vec<usize> = (0..=args.len()).collect();
        argv[args.len()] = 0;
        for i in 0..args.len() {
            user_sp -= args[i].len() + 1;
            argv[i] = user_sp;
            let mut p = user_sp;
            // write chars to [user_sp, user_sp + len]
            for c in args[i].as_bytes() {
                *translated_refmut(token, p as *mut u8) = *c;
                p += 1;
            }
            *translated_refmut(token, p as *mut u8) = 0;
        }
        // make the user_sp aligned to 8B for k210 platform
        user_sp -= user_sp % core::mem::size_of::<usize>();
        // println!("user_sp {:X}", user_sp);

        ////////////// *platform String ///////////////////
        let platform = "RISC-V64";
        user_sp -= platform.len() + 1;
        user_sp -= user_sp % core::mem::size_of::<usize>();
        let mut p = user_sp;
        for c in platform.as_bytes() {
            *translated_refmut(token, p as *mut u8) = *c;
            p += 1;
        }
        *translated_refmut(token, p as *mut u8) = 0;

        ////////////// *rand bytes ///////////////////
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

        ////////////// *padding //////////////////////
        user_sp -= user_sp % 16;

        ////////////// *auxv[] //////////////////////
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
        for i in 0..auxv.len() {
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

        ////////////// *argc //////////////////////
        user_sp -= core::mem::size_of::<usize>();
        *translated_refmut(token, user_sp as *mut usize) = args.len();
        // initialize trap_cx
        let mut trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            kernel_token(),
            task.kernel_stack.get_top(),
            user_trap_handler as usize,
            hart_id(),
        );
        trap_cx.x[10] = args.len();
        trap_cx.x[11] = argv_base;
        trap_cx.x[12] = envp_base;
        trap_cx.x[13] = auxv_base;
        *task_inner.get_trap_cx() = trap_cx;
    }
    pub fn fork(self: &Arc<ProcessControlBlock>, _flags: u32) -> Arc<ProcessControlBlock> {
        // assert_eq!(
        //     self.acquire_inner_lock().thread_count(),
        //     1,
        //     "process can't fork with multithread"
        // );
        // ---- hold parent PCB lock
        let mut parent_inner = self.acquire_inner_lock();
        // copy user space(include trap context)
        let addrspace = AddrSpace::from_existed_user(&parent_inner.addrspace);
        // alloc a pid
        let pid_handle = pid_alloc();
        // copy fd table
        let mut new_fd_table: FDTable = Vec::new();
        for fd in parent_inner.fd_table.iter() {
            if let Some(file) = fd {
                new_fd_table.push(Some(file.clone()));
            } else {
                new_fd_table.push(None);
            }
        }
        let child = Arc::new(ProcessControlBlock {
            pid: pid_handle,
            inner: Mutex::new(ProcessControlBlockInner {
                entry_point: parent_inner.entry_point,
                heap_start: parent_inner.heap_start,
                is_zombie: parent_inner.is_zombie,
                heap_pointer: parent_inner.heap_pointer,
                addrspace,
                parent: Some(Arc::downgrade(self)),
                children: Vec::new(),
                tasks: Vec::new(),
                exit_code: 0,
                fd_table: new_fd_table,
                fd_limit: parent_inner.fd_limit,
                current_path: parent_inner.current_path.clone(),
                mmap_area_hint: parent_inner.mmap_area_hint,
                signal_actions: parent_inner.signal_actions.clone(),
            }),
        });
        insert_into_pid2process(child.getpid(), child.clone());
        // add child
        parent_inner.children.push(child.clone());
        // create main thread of child process
        let task = Arc::new(TaskControlBlock::new(
            child.clone(),
            0,
            // here we do not allocate trap_cx or ustack again
            // but mention that we allocate a new kstack here
            false,
        ));
        insert_into_tid2task(task.gettid(), task.clone());
        // add task to child process
        let mut child_inner = child.acquire_inner_lock();
        child_inner.tasks.push(task.clone());
        drop(child_inner);

        // modify kernel_sp in trap_cx
        let task_inner = task.acquire_inner_lock();
        let trap_cx = task_inner.get_trap_cx();
        trap_cx.kernel_sp = task.kernel_stack.get_top();

        drop(task_inner);
        // add new task to scheduler
        add_task(task);
        // return
        child
        // **** release child PCB
        // ---- release parent PCB
    }
    pub fn getpid(&self) -> usize {
        self.pid.0
    }
    pub fn get_parent(&self) -> Option<Arc<ProcessControlBlock>> {
        let inner = self.acquire_inner_lock();
        inner.parent.as_ref().unwrap().upgrade()
    }
    // initproc won't call sys_getppid
    pub fn getppid(&self) -> usize {
        self.get_parent().unwrap().pid.0
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
        let mmap_perm = MapPermission::from_bits((prot << 1) as u8).unwrap() | MapPermission::U;
        let mmap_flag = MMapFlags::from_bits(flags).unwrap();
        log::trace!(
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
                .addrspace
                .create_mmap_section(start, length, mmap_perm)
                .into();
            log::trace!("mmap need hint hint after map: {:#X}", inner.mmap_area_hint);
        }
        // another mmaping already exist there, but need to place the mapping at exactly that address.
        else if inner.addrspace.is_mmap_section_conflict(start, length)
            && mmap_flag.contains(MMapFlags::MAP_FIXED)
        {
            // adjust mmap section
            log::trace!("start fixing mmap conflict");
            inner.addrspace.fix_mmap_section_conflict(start, length);

            // map mmap section at fixed place
            log::trace!("mmap at fixed place start before map: {:#X}", start);
            inner.mmap_area_hint = inner.mmap_area_hint.max(
                inner
                    .addrspace
                    .create_mmap_section(start, length, mmap_perm)
                    .into(),
            );
            log::trace!(
                "mmap at fixed place hint after map: {:#X}",
                inner.mmap_area_hint
            );
        }
        // no conflict, just map it
        else if mmap_flag.contains(MMapFlags::MAP_FIXED) {
            log::trace!("mmap start before map: {:#X}", start);
            inner.mmap_area_hint = inner.mmap_area_hint.max(
                inner
                    .addrspace
                    .create_mmap_section(start, length, mmap_perm)
                    .into(),
            );
            log::trace!("mmap hint after map: {:#X}", inner.mmap_area_hint);
        }
        // have conflict, but can pick a new place to map
        else {
            start = inner.mmap_area_hint;
            log::trace!("mmap need to pick new place start before map: {:#X}", start);
            inner.mmap_area_hint = inner.mmap_area_hint.max(
                inner
                    .addrspace
                    .create_mmap_section(start, length, mmap_perm)
                    .into(),
            );
            log::trace!(
                "mmap need to pick new place hint after map: {:#X}",
                inner.mmap_area_hint
            );
        }

        /* File Content Copy if need*/
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
            match &file {
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
        let start = page_aligned_up(start);
        let mut inner = self.acquire_inner_lock();
        inner
            .addrspace
            .remove_mmap_area_with_start_vpn(VirtAddr::from(start).into());
        0
    }
}
