use crate::config::{page_aligned_up, MMAP_BASE};
use crate::cpu::{current_task, current_user_token};
use crate::dt::TIMER_FREQ;
use crate::fs::{open, OpenFlags};
use crate::mm::{
    translated_ref, translated_refmut, translated_str, PTEFlags, VirtAddr, VirtPageNum,
    KERNEL_SPACE,
};
use crate::scheduler::{add_task, pid2task};
use crate::syscall::{ENOENT, ESRCH};
use crate::task::{
    exit_current_and_run_next, suspend_current_and_run_next, ClearChildTid, SigSet, SignalAction,
    NSIG,
};
use crate::timer::*;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use bitflags::*;
use core::sync::atomic::Ordering;

use super::{ECHILD, EFAULT};

pub fn sys_exit(exit_code: i32) -> ! {
    log::debug!("sys_exit");
    exit_current_and_run_next(exit_code);
    panic!("Unreachable in sys_exit!");
}

pub fn sys_exit_group(exit_code: i32) -> ! {
    log::debug!("sys_exit_group");
    exit_current_and_run_next(exit_code);
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

pub fn sys_clock_gettime(clock_id: i32, tp: *mut u64) -> isize {
    // log::debug!(
    //     "sys_clock_gettime clock_id: {}, tp: {:#X}",
    //     clock_id,
    //     tp as usize
    // );
    if tp as usize == 0 {
        return -EFAULT;
    }
    let token = current_user_token();
    let (sec, nsec) = get_time_sec_nsec();
    // let timespec = translated_refmut(token, tp);
    // timespec.sec = sec;
    // timespec.nsec = nsec;
    *translated_refmut(token, tp) = sec;
    *translated_refmut(token, unsafe { tp.add(1) }) = nsec;
    0
}

pub fn sys_set_tid_address(ptr: *mut u32) -> isize {
    log::debug!("sys_set_tid_address, ptr: {:#X?}", ptr);
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    let ctid = if let Some(p) = &inner.clear_child_tid {
        p.ctid
    } else {
        0
    };
    *translated_refmut(token, ptr) = ctid;
    task.getpid() as isize
}

pub fn sys_setpgid() -> isize {
    log::debug!("sys_setpgid");
    0
}

pub fn sys_getpgid() -> isize {
    log::debug!("sys_getpgid");
    0
}

pub fn sys_getpid() -> isize {
    log::debug!("sys_getpid");
    current_task().unwrap().getpid() as isize
}

pub fn sys_getppid() -> isize {
    // log::debug!("sys_getppid");
    current_task().unwrap().getppid() as isize
}

pub fn sys_getuid() -> isize {
    log::debug!("sys_getuid");
    0 // root user
}

pub fn sys_geteuid() -> isize {
    log::debug!("sys_geteuid");
    0 // root user
}

pub fn sys_getgid() -> isize {
    log::debug!("sys_getgid");
    0 // root group
}

pub fn sys_getegid() -> isize {
    log::debug!("sys_getegid");
    0 // root group
}

pub fn sys_gettid() -> isize {
    log::debug!("sys_gettid");
    current_task().unwrap().getpid() as isize
}

bitflags! {
    pub struct CloneFlags: u32 {
        const CSIGNAL		        = 0x000000ff;	/* signal mask to be sent at exit */
        const CLONE_VM	            = 0x00000100;	/* set if VM shared between processes */
        const CLONE_FS	            = 0x00000200;	/* set if fs info shared between processes */
        const CLONE_FILES	        = 0x00000400;	/* set if open files shared between processes */
        const CLONE_SIGHAND         = 0x00000800;	/* set if signal handlers and blocked signals shared */
        const CLONE_PIDFD	        = 0x00001000;	/* set if a pidfd should be placed in parent */
        const CLONE_PTRACE	        = 0x00002000;	/* set if we want to let tracing continue on the child too */
        const CLONE_VFORK	        = 0x00004000;	/* set if the parent wants the child to wake it up on mm_release */
        const CLONE_PARENT	        = 0x00008000;	/* set if we want to have the same parent as the cloner */
        const CLONE_THREAD	        = 0x00010000;	/* Same thread group? */
        const CLONE_NEWNS	        = 0x00020000;	/* New mount namespace group */
        const CLONE_SYSVSEM	        = 0x00040000;	/* share system V SEM_UNDO semantics */
        const CLONE_SETTLS	        = 0x00080000;	/* create a new TLS for the child */
        const CLONE_PARENT_SETTID	= 0x00100000;	/* set the TID in the parent */
        const CLONE_CHILD_CLEARTID	= 0x00200000;	/* clear the TID in the child */
        const CLONE_DETACHED		= 0x00400000;	/* Unused, ignored */
        const CLONE_UNTRACED		= 0x00800000;	/* set if the tracing process can't force CLONE_PTRACE on this clone */
        const CLONE_CHILD_SETTID	= 0x01000000;	/* set the TID in the child */
        const CLONE_NEWCGROUP		= 0x02000000;	/* New cgroup namespace */
        const CLONE_NEWUTS		    = 0x04000000;	/* New utsname namespace */
        const CLONE_NEWIPC		    = 0x08000000;	/* New ipc namespace */
        const CLONE_NEWUSER		    = 0x10000000;	/* New user namespace */
        const CLONE_NEWPID		    = 0x20000000;	/* New pid namespace */
        const CLONE_NEWNET		    = 0x40000000;	/* New network namespace */
        const CLONE_IO		        = 0x80000000;	/* Clone io context */
    }
}

//  __clone(func, stack, flags, arg, ptid, tls, ctid)
//            a0,    a1,    a2,  a3,   a4,  a5,   a6
// 子进程返回到 func 在用户态实现
//  syscall(SYS_clone, flags, stack, ptid, tls, ctid)
// TODO: deal with exit signal mask
pub fn sys_clone(
    flags: u32,
    stack_ptr: usize,
    ptid_ptr: *mut u32,
    newtls: usize,
    ctid_ptr: *mut u32,
) -> isize {
    let current_task = current_task().unwrap();
    let clone_flags = CloneFlags::from_bits(flags).unwrap();
    let token = current_user_token();
    // log::debug!(
    //     "sys_clone flags: {:?}, stack_ptr: {:#X?}, ptid: {:#X?}, newtls: {}, ctid: {:#X?}, exit_signal: {}",
    //     clone_flags,
    //     stack_ptr,
    //     ptid_ptr as usize,
    //     newtls,
    //     ctid_ptr as usize,
    //     (flags & 0xff) as usize
    // );
    let new_task = current_task.fork();
    if stack_ptr != 0 {
        let trap_cx = new_task.acquire_inner_lock().get_trap_cx();
        trap_cx.set_sp(stack_ptr);
    }
    let new_pid = new_task.pid.0;
    if clone_flags.contains(CloneFlags::CLONE_PARENT_SETTID) && ptid_ptr as usize != 0 {
        *translated_refmut(token, ptid_ptr) = new_pid as u32;
    }
    if clone_flags.contains(CloneFlags::CLONE_CHILD_CLEARTID) && ctid_ptr as usize != 0 {
        let mut new_task_inner = new_task.acquire_inner_lock();
        new_task_inner.clear_child_tid = Some(ClearChildTid {
            ctid: *translated_ref(token, ctid_ptr),
            addr: ctid_ptr as usize,
        });
    }
    // modify trap context of new_task, because it returns immediately after switching
    let trap_cx = new_task.acquire_inner_lock().get_trap_cx();
    // we do not have to move to next instruction since we have done it before
    // for child process, fork returns 0
    trap_cx.x[10] = 0;
    // add new task to scheduler
    add_task(new_task);
    // let child process run first
    suspend_current_and_run_next();
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
    let token = current_user_token();
    let mut path = translated_str(token, path);
    // log::debug!("sys_exec, path: {}", path);
    let mut args_vec: Vec<String> = Vec::new();
    if path.ends_with(".sh") {
        args_vec.push("/busybox".to_string());
        args_vec.push("sh".to_string());
        path = "/busybox".to_string();
    }
    loop {
        let arg_str_ptr = *translated_ref(token, args);
        if arg_str_ptr == 0 {
            break;
        }
        args_vec.push(translated_str(token, arg_str_ptr as *const u8));
        unsafe {
            args = args.add(1);
        }
    }
    let task = current_task().unwrap();
    let current_path = task.acquire_inner_lock().current_path.clone();
    if let Some(app_inode) = open(current_path.as_str(), path.as_str(), OpenFlags::RDONLY) {
        app_inode.mmap_to_kernel();
        let all_data =
            unsafe { core::slice::from_raw_parts_mut(MMAP_BASE as *mut u8, app_inode.get_size()) };
        task.exec(all_data, args_vec);
        KERNEL_SPACE
            .lock()
            .remove_mmap_area_with_start_vpn(VirtAddr::from(MMAP_BASE).floor());

        // let all_data = app_inode.read_all();
        // task.exec(all_data.as_slice(), args_vec);
        // let argc = args_vec.len();
        // argc as isize
        0
    } else {
        -ENOENT
    }
}

/// If there is not a child process whose pid is same as given, return -1.
/// Else if there is a child process but it is still running, return -2.
#[allow(unused)]
pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize {
    let task = current_task().unwrap();
    // find a child process

    // ---- access current PCB exclusively
    let mut inner = task.acquire_inner_lock();
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
        if (exit_code_ptr as usize) != 0 {
            *translated_refmut(inner.addrspace.token(), exit_code_ptr) = exit_code << 8;
        }
        found_pid as isize
    } else {
        -2
    }
    // ---- release current PCB automatically
}

const WNOHANG: isize = 0x00000001;
const WUNTRACED: isize = 0x00000002;
const WSTOPPED: isize = WUNTRACED;
const WEXITED: isize = 0x00000004;
const WCONTINUED: isize = 0x00000008;
const WNOWAIT: isize = 0x01000000;

/// If there is not any adopt child process active on cpu or waiting in any
/// ready_queue, return 0 to let init_proc know it is waiting for itself to exit
/// Else If there is not a child process whose pid is same as given, return -1.
/// Else if there is a child process but it is still running, suspend_current_and_run_next.
pub fn sys_wait4(pid: isize, wstatus: *mut i32, option: isize) -> isize {
    // log::debug!(
    //     "sys_wait4, pid: {}, wstatus: {:#X?}, option: {}",
    //     pid,
    //     wstatus,
    //     option
    // );
    loop {
        let task = current_task().unwrap();
        // No any child process waiting
        // find a child process
        // ---- access current PCB exclusively
        let mut inner = task.acquire_inner_lock();
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
            let ret_status = (exit_code & 0xff) << 8;
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
            drop(task);
            suspend_current_and_run_next();
        }
        // ---- release current PCB automatically
    }
}

/// On success, returns the new program break
/// On failure, the system call returns the current break.
pub fn sys_brk(mut brk_addr: usize) -> isize {
    let current_task = current_task().unwrap();
    let mut inner = current_task.acquire_inner_lock();
    // log::debug!(
    //     "sys_brk: brk_addr: {:#X?}, start: {:#X?}, current_break: {:#X?}",
    //     brk_addr,
    //     inner.heap_start,
    //     inner.heap_pointer
    // );
    let heap_start = inner.heap_start;
    brk_addr = page_aligned_up(brk_addr);
    if brk_addr != 0 {
        // create heap section
        if inner.heap_pointer == heap_start {
            inner
                .addrspace
                .alloc_heap_section(heap_start, brk_addr - heap_start);
        }
        // adjust heap section
        else {
            let (_, top_vpn) = inner.addrspace.get_section_range(".heap");
            let new_top_vpn: VirtPageNum = VirtAddr::from(brk_addr).floor();
            if top_vpn != new_top_vpn {
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
    log::debug!("sys_sbrk increment: {}", increment);
    let current_task = current_task().unwrap();
    let mut inner = current_task.acquire_inner_lock();
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
            let (_, end_vpn) = inner.addrspace.get_section_range(".heap");
            let new_top = (inner.heap_pointer as isize + increment) as usize;
            let new_top_vpn: VirtPageNum = VirtAddr::from(new_top).floor().into();
            if end_vpn != new_top_vpn {
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
    let task = current_task().unwrap();
    let res = task.mmap(start, length, prot, flags, fd, offset);
    // log::debug!("sys_mmap leave");
    res
}

pub fn sys_munmap(start: usize, length: usize) -> isize {
    log::debug!("sys_munmap, start: {:#X?}, length: {:#X?}", start, length);
    let task = current_task().unwrap();
    task.munmap(start, length)
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

pub fn sys_prlimit(pid: usize, res: usize, rlim: *const RLimit, old_rlim: *mut RLimit) -> isize {
    log::debug!("sys_prlimit res: {}", res);
    0
}

pub fn sys_mprotect(address: usize, length: usize, prot: usize) -> isize {
    let flags = PTEFlags::from_bits((prot << 1) as u8).unwrap();
    // log::debug!(
    //     "sys_mprotect address: {:#X} length: {:#X} flags: {:?}",
    //     address,
    //     length,
    //     flags
    // );
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    let start_vpn = VirtAddr::from(address).floor();
    let end_vpn = VirtAddr::from(address + length).ceil();
    for vpn in start_vpn..end_vpn {
        inner.addrspace.set_pte_flags(vpn, flags);
    }
    0
}
