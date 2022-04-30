use crate::cpu::{current_task, current_user_token};
use crate::fs::{open, DiskInodeType, OpenFlags};
use crate::mm::{translated_ref, translated_refmut, translated_str};
use crate::scheduler::add_task;
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};
use crate::timer::{get_time_sec_usec, get_time_us, get_time_val, TimeVal, Times};
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

pub fn sys_exit(exit_code: i32) -> ! {
    exit_current_and_run_next(exit_code);
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

pub fn sys_getpid() -> isize {
    current_task().unwrap().getpid() as isize
}

pub fn sys_getppid() -> isize {
    current_task().unwrap().getppid() as isize
}

pub fn sys_fork() -> isize {
    let current_task = current_task().unwrap();
    let new_task = current_task.fork();
    let new_pid = new_task.pid.0;
    // modify trap context of new_task, because it returns immediately after switching
    let trap_cx = new_task.acquire_inner_lock().get_trap_cx();
    // we do not have to move to next instruction since we have done it before
    // for child process, fork returns 0
    trap_cx.x[10] = 0;
    // add new task to scheduler
    add_task(new_task);
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
    // log::debug!("sys_exec");
    let token = current_user_token();
    let path = translated_str(token, path);
    let mut args_vec: Vec<String> = Vec::new();
    loop {
        let arg_str_ptr = *translated_ref(token, args);
        if arg_str_ptr == 0 {
            break;
        }
        args_vec.push(translated_str(token, arg_str_ptr as *const u8));
        // println!("arg{}: {}",0, args_vec[0]);
        unsafe {
            args = args.add(1);
        }
    }
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    // log::debug!("sys_after");
    if let Some(app_inode) = open(
        inner.current_path.as_str(),
        path.as_str(),
        OpenFlags::RDONLY,
        DiskInodeType::File,
    ) {
        drop(inner);
        let all_data = app_inode.read_all();
        let task = current_task().unwrap();
        task.exec(all_data.as_slice(), args_vec);
        0
    } else {
        -1
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
            *translated_refmut(inner.addrspace.get_token(), exit_code_ptr) = exit_code << 8;
        }
        found_pid as isize
    } else {
        -2
    }
    // ---- release current PCB automatically
}

/// If there is not a child process whose pid is same as given, return -1.
/// Else if there is a child process but it is still running, return -2.
pub fn sys_wait4(pid: isize, wstatus: *mut i32, option: isize) -> isize {
    if option != 0 {
        panic! {"Extended option not support yet..."};
    }
    loop {
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
            let ret_status = exit_code << 8;
            if (wstatus as usize) != 0 {
                *translated_refmut(inner.addrspace.get_token(), wstatus) = ret_status;
            }
            return found_pid as isize;
        } else {
            // let wait_pid = task.getpid();
            // if wait_pid >= 1 {
            //     log::trace!("Not yet, pid {} still wait", wait_pid);
            // }
            drop(inner);
            drop(task);
            suspend_current_and_run_next();
        }
        // ---- release current PCB automatically
    }
}

pub fn sys_brk(brk_addr: usize) -> isize{
    0
}

pub fn sys_munmap() -> isize{
    0
}

pub fn sys_mmap() -> isize{
    0
}
