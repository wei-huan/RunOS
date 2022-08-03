use crate::config::page_aligned_up;
use crate::cpu::{current_process, current_task, current_user_token};
use crate::dt::TIMER_FREQ;
use crate::fs::{open, DiskInodeType, OpenFlags};
use crate::mm::{
    translated_ref, translated_refmut, translated_str, PTEFlags, VirtAddr, VirtPageNum,
};
use crate::scheduler::{add_task, pid2process, tid2task};
use crate::syscall::{EINVAL, ESRCH};
use crate::task::{
    exit_current_and_run_next, suspend_current_and_run_next, SignalAction, SignalFlags, MAX_SIG,
};
use crate::timer::*;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use bitflags::*;
use core::sync::atomic::Ordering;

pub fn sys_exit(exit_code: i32) -> ! {
    exit_current_and_run_next(exit_code, false);
    panic!("Unreachable in sys_exit!");
}

pub fn sys_exit_group(exit_code: i32) -> ! {
    exit_current_and_run_next(exit_code, true);
    panic!("Unreachable in sys_exit_group!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time(time_val: *mut TimeVal) -> isize {
    get_time_val(time_val)
}

pub fn sys_times(times: *mut Times) -> isize {
    let token = current_user_token();
    let sec = get_time_us() as i64;
    *translated_refmut(token, times) = Times {
        tms_utime: sec,
        tms_stime: sec,
        tms_cutime: sec,
        tms_cstime: sec,
    };
    0
}

// struct timespec {
//     time_t   tv_sec;        /* seconds */
//     long     tv_nsec;       /* nanoseconds */
// };
pub fn sys_clock_get_time(_clk_id: usize, tp: *mut u64) -> isize {
    if tp as usize == 0 {
        return 0;
    }
    let timer_freq = TIMER_FREQ.load(Ordering::Acquire);
    let token = current_user_token();
    let ticks = get_time();
    let sec = (ticks / timer_freq) as u64;
    let nsec = ((ticks % timer_freq) * (NSEC_PER_SEC / timer_freq)) as u64;
    *translated_refmut(token, tp) = sec;
    *translated_refmut(token, unsafe { tp.add(1) }) = nsec;
    0
}

pub fn sys_set_tid_address(ptr: *mut u32) -> isize {
    // log::debug!("sys_set_tid_address, ptr: {:#X?}", ptr);
    let token = current_user_token();
    *translated_refmut::<u32>(token, ptr) = current_task().unwrap().gettid() as u32;
    current_task().unwrap().gettid() as isize
}

pub fn sys_getpid() -> isize {
    current_process().unwrap().getpid() as isize
}

pub fn sys_getppid() -> isize {
    current_process().unwrap().getppid() as isize
}

pub fn sys_getuid() -> isize {
    0 // root user
}

pub fn sys_geteuid() -> isize {
    0 // root user
}

pub fn sys_getgid() -> isize {
    0 // root group
}

pub fn sys_getegid() -> isize {
    0 // root group
}

// For user, tid is pid in kernel
pub fn sys_gettid() -> isize {
    current_task().unwrap().gettid() as isize
}

//  __clone(func, stack, flags, arg, ptid, tls, ctid)
//            a0,    a1,    a2,  a3,   a4,  a5,   a6
// 子进程返回到 func 在用户态实现
//  syscall(SYS_clone, flags, stack, ptid, tls, ctid)
pub fn sys_fork(
    _flags: usize,
    stack_ptr: usize,
    ptid_ptr: *const u32,
    _newtls: usize,
    ctid_ptr: *const u32,
) -> isize {
    let current_process = current_process().unwrap();
    let token = current_user_token();
    log::debug!("sys_fork ptid: {}, ctid: {}", ptid_ptr as usize, ctid_ptr as usize);
    // if ptid_ptr as usize != 0 {
    //     let ptid = *translated_ref(token, ptid_ptr);
    //     log::debug!("ptid: {}", ptid);
    // }
    // if ctid_ptr as usize != 0 {
    //     let ctid = *translated_ref(token, ctid_ptr);
    //     log::debug!("ctid: {}", ctid);
    // }
    let new_process = current_process.fork();
    let new_task = new_process.acquire_inner_lock().get_task(0);
    let trap_cx = new_task.acquire_inner_lock().get_trap_cx();
    // set new stack
    if stack_ptr != 0 {
        trap_cx.set_sp(stack_ptr);
    }
    let new_pid = new_process.getpid();
    // modify trap context of new_task, because it returns immediately after switching
    // we do not have to move to next instruction since we have done it before
    // for child process, fork returns 0
    trap_cx.x[10] = 0;
    // let child process run first
    // suspend_current_and_run_next();
    println!("here_5");
    new_pid as isize
}

pub fn sys_sleep(time_req: &TimeVal, time_remain: &mut TimeVal) -> isize {
    let (mut sec, mut usec) = get_time_sec_usec();
    let token = current_user_token();
    let req_sec = *translated_ref(token, &(time_req.sec));
    let req_usec = *translated_ref(token, &(time_req.usec));
    let (end_sec, end_usec) = (req_sec + sec, req_usec + usec);
    // println!("end_sec: {}", end_sec);
    // println!("end_usec: {}", end_usec);
    loop {
        (sec, usec) = get_time_sec_usec();
        // println!("sec: {}", sec);
        // println!("usec: {}", usec);
        if (sec < end_sec) || (usec < end_usec) {
            *translated_refmut(token, time_remain) = TimeVal { sec: 0, usec: 0 };
            suspend_current_and_run_next();
        } else {
            return 0;
        }
    }
}

pub fn sys_exec(path: *const u8, mut args: *const usize) -> isize {
    log::debug!("sys_exec");
    let token = current_user_token();
    let path = translated_str(token, path);
    let mut args_vec: Vec<String> = Vec::new();
    loop {
        let arg_str_ptr = *translated_ref(token, args);
        if arg_str_ptr == 0 {
            break;
        }
        args_vec.push(translated_str(token, arg_str_ptr as *const u8));
        // log::debug!("arg{}: {}",0, args_vec[0]);
        unsafe {
            args = args.add(1);
        }
    }
    let current_process = current_process().unwrap();
    let process_inner = current_process.acquire_inner_lock();
    if let Some(app_inode) = open(
        process_inner.current_path.as_str(),
        path.as_str(),
        OpenFlags::RDONLY,
        DiskInodeType::File,
    ) {
        drop(process_inner);
        let all_data = app_inode.read_all();
        let argc = args_vec.len();
        log::debug!("before current_process.exec");
        current_process.exec(all_data.as_slice(), args_vec);
        log::debug!("after task.exec, now return");
        argc as isize
    } else {
        -1
    }
}

/// If there is not a child process whose pid is same as given, return -1.
/// Else if there is a child process but it is still running, return -2.
pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize {
    let current_process = current_process().unwrap();
    // find a child process

    // ---- access current PCB exclusively
    let mut process_inner = current_process.acquire_inner_lock();
    if !process_inner
        .children
        .iter()
        .any(|p| pid == -1 || pid as usize == p.getpid())
    {
        return -1;
        // ---- release current PCB
    }
    let pair = process_inner.children.iter().enumerate().find(|(_, p)| {
        // ++++ temporarily access child PCB exclusively
        p.acquire_inner_lock().is_zombie() && (pid == -1 || pid as usize == p.getpid())
        // ++++ release child PCB
    });
    if let Some((idx, _)) = pair {
        let child = process_inner.children.remove(idx);
        // confirm that child will be deallocated after being removed from children list
        assert_eq!(Arc::strong_count(&child), 1);
        let found_pid = child.getpid();
        // ++++ temporarily access child PCB exclusively
        let exit_code = child.acquire_inner_lock().exit_code;
        // ++++ release child PCB
        if (exit_code_ptr as usize) != 0 {
            *translated_refmut(process_inner.addrspace.token(), exit_code_ptr) = exit_code << 8;
        }
        found_pid as isize
    } else {
        -2
    }
    // ---- release current PCB automatically
}

/// If there is not any adopt child process active on cpu or waiting in any
/// ready_queue, return 0 to let init_proc know it is waiting for itself to exit
/// Else If there is not a child process whose pid is same as given, return -1.
/// Else if there is a child process but it is still running, suspend_current_and_run_next.
pub fn sys_wait4(pid: isize, wstatus: *mut i32, option: isize) -> isize {
    if option != 0 {
        panic! {"Extended option not support yet..."};
    }
    loop {
        let current_process = current_process().unwrap();
        // No any child process waiting
        // if !have_ready_task() && !task.acquire_inner_lock().have_children() && {

        // }
        // find a child process
        // ---- access current PCB exclusively
        let mut inner = current_process.acquire_inner_lock();
        if !inner
            .children
            .iter()
            .any(|p| pid == -1 || pid as usize == p.getpid())
        {
            return -1;
            // ---- release current PCB
        }
        let pair = inner.children.iter().enumerate().find(|(_, p)| {
            // ++++ temporarily access child PCB exclusively
            p.acquire_inner_lock().is_zombie() && (pid == -1 || pid as usize == p.getpid())
            // ++++ release child PCB
        });
        if let Some((idx, _)) = pair {
            let child = inner.children.remove(idx);
            // confirm that child will be deallocated after being removed from children list
            assert_eq!(Arc::strong_count(&child), 1);
            let found_pid = child.getpid();
            // ++++ temporarily access child PCB exclusively
            let exit_code = child.acquire_inner_lock().exit_code;
            // ++++ release child PCB
            let ret_status = exit_code << 8;
            if (wstatus as usize) != 0 {
                *translated_refmut(inner.addrspace.token(), wstatus) = ret_status;
            }
            return found_pid as isize;
        } else {
            // let wait_pid = task.getpid();
            // if wait_pid >= 1 {
            //     log::debug!("Not yet, pid {} still wait", wait_pid);
            // }
            drop(inner);
            drop(current_process);
            suspend_current_and_run_next();
        }
        // ---- release current PCB automatically
    }
}

/// On success, returns the new program break
/// On failure, the system call returns the current break.
pub fn sys_brk(mut brk_addr: usize) -> isize {
    let current_process = current_process().unwrap();
    let mut inner = current_process.acquire_inner_lock();
    // log::debug!(
    //     "sys_brk: {:#X?}, start: {:#X?}, current_break: {:#X?}",
    //     brk_addr,
    //     inner.heap_start,
    //     inner.heap_pointer
    // );
    let heap_start = inner.heap_start;
    brk_addr = page_aligned_up(brk_addr);
    if brk_addr != 0 {
        // 还未分配堆，直接创建 heap section
        if inner.heap_pointer == heap_start {
            inner
                .addrspace
                .alloc_heap_section(heap_start, brk_addr - heap_start);
        }
        // 已经有堆，扩展
        else {
            let (_, top) = inner.addrspace.get_section_range(".heap");
            let top_vpn: VirtPageNum = VirtAddr::from(top).into();
            let new_top_vpn: VirtPageNum = VirtAddr::from(brk_addr).floor().into();
            if top_vpn != new_top_vpn {
                // 如果超出界限，需要分配新的页
                // 如果缩小到的新虚拟页号变小，需要回收页
                inner.addrspace.modify_section_end(".heap", new_top_vpn);
            }
        }
        inner.heap_pointer = brk_addr;
    }
    return inner.heap_pointer as isize;
}

// sets the end of the data segment to the value
// increment can be negative
// return heap size
// todo test, No test yet
pub fn sys_sbrk(increment: isize) -> isize {
    log::debug!("sys_sbrk");
    let current_process = current_process().unwrap();
    let mut inner = current_process.acquire_inner_lock();
    let heap_start = inner.heap_start;
    if increment == 0 {
        return (inner.heap_pointer - heap_start) as isize;
    } else {
        // 还未分配堆，直接创建 heap section
        if inner.heap_pointer == heap_start {
            // 还没分配时增量如果是负数就不分配了, 直接返回0
            if increment < 0 {
                return 0;
            }
            inner
                .addrspace
                .alloc_heap_section(heap_start, increment as usize);
            inner.heap_pointer = heap_start + increment as usize;
            return (inner.heap_pointer - heap_start) as isize;
        }
        // 回收 heap 段
        if inner.heap_pointer as isize + increment <= inner.heap_start as isize {
            // todo 回收 .heap 段
            inner.addrspace.dealloc_heap_section();
            return 0;
        } else {
            let (_, top) = inner.addrspace.get_section_range(".heap");
            let top_vpn: VirtPageNum = VirtAddr::from(top).into();
            let new_top = (inner.heap_pointer as isize + increment) as usize;
            let new_top_vpn: VirtPageNum = VirtAddr::from(new_top).floor().into();
            if top_vpn != new_top_vpn {
                // 如果超出界限，需要分配新的页
                // 如果缩小到的新虚拟页号变小，需要回收页
                inner.addrspace.modify_section_end(".heap", new_top_vpn);
            }
            inner.heap_pointer = new_top;
            return (inner.heap_pointer - heap_start) as isize;
        }
    }
}

pub fn sys_mmap(
    start: usize,
    length: usize,
    prot: usize,
    flags: usize,
    fd: isize,
    offset: usize,
) -> isize {
    // log::debug!("sys_mmap start: {:#X?}, length: {:#X?}, fd: {}", start, length, fd);
    let current_process = current_process().unwrap();
    let res = current_process.mmap(start, length, prot, flags, fd, offset);
    // log::debug!("sys_mmap leave");
    res
}

pub fn sys_munmap(start: usize, length: usize) -> isize {
    log::trace!("sys_unmmap");
    let current_process = current_process().unwrap();
    current_process.munmap(start, length)
}

const RLIMIT_CPU: usize = 0;
const RLIMIT_FSIZE: usize = 1;
const RLIMIT_DATA: usize = 2;
const RLIMIT_STACK: usize = 3;
const RLIMIT_NOFILE: usize = 7;
const RLIMIT_AS: usize = 9;

pub struct RLimit {
    pub rlim_cur: usize, /* Soft limit */
    pub rlim_max: usize, /* Hard limit (ceiling for rlim_cur) */
}

const FDMAX: usize = 10;

pub fn sys_prlimit(pid: usize, res: usize, rlim: *const RLimit, old_rlim: *mut RLimit) -> isize {
    // log::debug!("sys_prlimit res: {}", res);
    0
}

// just add sig to main thread
pub fn sys_kill(pid: usize, signum: i32) -> isize {
    let process_option = pid2process(pid);
    if process_option.is_none() {
        return -ESRCH;
    }
    let process = process_option.unwrap();
    if let Some(flag) = SignalFlags::from_bits(1 << signum) {
        // insert the signal if legal
        let mut process_inner = process.acquire_inner_lock();
        let task = process_inner.get_task(0);
        let mut task_inner = task.acquire_inner_lock();
        if task_inner.signals.contains(flag) {
            return -EINVAL;
        }
        task_inner.signals.insert(flag);
        0
    } else {
        -EINVAL
    }
}

pub fn sys_tkill(tid: usize, signum: u32) -> isize {
    if let Some(task) = tid2task(tid) {
        if let Some(flag) = SignalFlags::from_bits(1 << signum) {
            let mut task_inner = task.acquire_inner_lock();
            if task_inner.signals.contains(flag) {
                return -EINVAL;
            }
            task_inner.signals.insert(flag);
            0
        } else {
            -EINVAL
        }
    } else {
        -ESRCH
    }
}

const SIG_BLOCK: isize = 0;
const SIG_UNBLOCK: isize = 1;
const SIG_SETMASK: isize = 2;

pub fn sys_sigprocmask(how: isize, set_ptr: *const u128, oldset_ptr: *mut u128) -> isize {
    log::trace!(
        "sys_sigprocmask how: {},  set_ptr: {},  oldset_ptr: {}",
        how,
        set_ptr as usize,
        oldset_ptr as usize
    );
    let token = current_user_token();
    if let Some(task) = current_task() {
        let mut inner = task.acquire_inner_lock();
        let old_mask = inner.signal_mask.bits();
        if oldset_ptr as usize != 0 {
            *translated_refmut::<u128>(token, oldset_ptr) = old_mask as u128;
        }
        if set_ptr as usize != 0 {
            let new_mask = *translated_ref::<u128>(token, set_ptr) as u32;
            match how {
                SIG_BLOCK => {
                    inner.signal_mask = SignalFlags::from_bits(new_mask | old_mask).unwrap()
                }
                SIG_UNBLOCK => {
                    inner.signal_mask = SignalFlags::from_bits(old_mask & (!new_mask)).unwrap()
                }
                SIG_SETMASK => inner.signal_mask = SignalFlags::from_bits(new_mask).unwrap(),
                _ => return -1,
            };
        }
        0
    } else {
        -ESRCH
    }
}

pub fn sys_sigretrun() -> isize {
    log::debug!("sys_sigretrun");
    if let Some(task) = current_task() {
        let mut inner = task.acquire_inner_lock();
        inner.handling_sig = -1;
        // restore the trap context
        let trap_ctx = inner.get_trap_cx();
        *trap_ctx = inner.trap_ctx_backup.unwrap();
        0
    } else {
        -1
    }
}

fn check_sigaction_error(signal: SignalFlags, action: usize, old_action: usize) -> bool {
    if action == 0
        || old_action == 0
        || signal == SignalFlags::SIGKILL
        || signal == SignalFlags::SIGSTOP
    {
        true
    } else {
        false
    }
}

pub fn sys_sigaction(
    signum: i32,
    action: *const SignalAction,
    old_action: *mut SignalAction,
) -> isize {
    log::trace!("sys_sigaction");
    let token = current_user_token();
    let process = current_process().unwrap();
    let mut inner = process.acquire_inner_lock();
    if signum as usize > MAX_SIG {
        return -1;
    }
    if let Some(flag) = SignalFlags::from_bits(1 << signum) {
        if check_sigaction_error(flag, action as usize, old_action as usize) {
            return -1;
        }
        let old_kernel_action = inner.signal_actions.table[signum as usize];
        if old_kernel_action.mask != SignalFlags::from_bits(40).unwrap() {
            *translated_refmut(token, old_action) = old_kernel_action;
        } else {
            let mut ref_old_action = *translated_refmut(token, old_action);
            ref_old_action.handler = old_kernel_action.handler;
        }
        let ref_action = translated_ref(token, action);
        inner.signal_actions.table[signum as usize] = *ref_action;
        return 0;
    } else {
        return -EINVAL;
    }
}

pub fn sys_mprotect(addr: usize, len: usize, prot: usize) -> isize {
    let flags = PTEFlags::from_bits((prot << 1) as u8).unwrap();
    log::trace!(
        "sys_mprotect addr: {:#X} len: {:#X} flags: {:?}",
        addr,
        len,
        flags
    );
    let start = VirtPageNum::from(VirtAddr::from(addr).floor());
    let end = VirtPageNum::from(VirtAddr::from(addr + len).ceil());
    let current_process = current_process().unwrap();
    let mut inner = current_process.acquire_inner_lock();
    for vpn in start..end {
        inner.addrspace.set_pte_flags(vpn, flags);
    }
    0
}
