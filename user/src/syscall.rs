#![allow(unused)]
use alloc::string::String;
use core::arch::asm;

use crate::TimeVal;

const SYSCALL_GETCWD: usize = 17;
const SYSCALL_DUP: usize = 23;
const SYSCALL_DUP3: usize = 24;
const SYSCALL_MKDIRAT: usize = 34;
const SYSCALL_UNLINKAT: usize = 35;
const SYSCALL_LINKAT: usize = 37;
const SYSCALL_UMOUNT2: usize = 39;
const SYSCALL_MOUNT: usize = 40;
const SYSCALL_CHDIR: usize = 49;
const SYSCALL_OPENAT: usize = 56;
const SYSCALL_CLOSE: usize = 57;
const SYSCALL_PIPE: usize = 59;
const SYSCALL_GETDENTS64: usize = 61;
const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_FSTAT: usize = 80;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_NANOSLEEP: usize = 101;
const SYSCALL_SCHED_YIELD: usize = 124;
const SYSCALL_TIMES: usize = 153;
const SYSCALL_UNAME: usize = 160;
const SYSCALL_GET_TIMEOFDAY: usize = 169;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_GETPPID: usize = 172;
const SYSCALL_BRK: usize = 214;
const SYSCALL_MUNMAP: usize = 215;
const SYSCALL_CLONE: usize = 220;
const SYSCALL_EXECVE: usize = 221;
const SYSCALL_MMAP: usize = 222;
const SYSCALL_WAIT4: usize = 260;

fn syscall(id: usize, args: [usize; 6]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x13") args[3],
            in("x14") args[4],
            in("x15") args[5],
            in("x17") id
        );
    }
    ret
}

pub fn sys_dup(fd: usize) -> isize {
    syscall(SYSCALL_DUP, [fd, 0, 0, 0, 0, 0])
}

pub fn sys_chdir(path: &str) -> isize {
    syscall(SYSCALL_CHDIR, [path.as_ptr() as usize, 0, 0, 0, 0, 0])
}

pub fn sys_unlinkat(fd: i32, path: &str, flags: u32) -> isize {
    syscall(
        SYSCALL_UNLINKAT,
        [fd as usize, path.as_ptr() as usize, flags as usize, 0, 0, 0],
    )
}

pub fn sys_open(path: &str, flags: u32) -> isize {
    let path_str = String::new() + path + "\0";
    syscall(
        SYSCALL_OPENAT,
        [
            (-100 as isize) as usize,
            path_str.as_ptr() as usize,
            flags as usize,
            0,
            0,
            0,
        ],
    )
}

pub fn sys_mkdir(path: &str) -> isize {
    let path_str = String::new() + path + "\0";
    syscall(
        SYSCALL_MKDIRAT,
        [
            (-100 as isize) as usize,
            path_str.as_ptr() as usize,
            0,
            0,
            0,
            0,
        ],
    )
}

pub fn sys_close(fd: usize) -> isize {
    syscall(SYSCALL_CLOSE, [fd, 0, 0, 0, 0, 0])
}

pub fn sys_pipe(pipe: &mut [usize]) -> isize {
    syscall(SYSCALL_PIPE, [pipe.as_mut_ptr() as usize, 0, 0, 0, 0, 0])
}

pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    syscall(
        SYSCALL_READ,
        [fd, buffer.as_mut_ptr() as usize, buffer.len(), 0, 0, 0],
    )
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(
        SYSCALL_WRITE,
        [fd, buffer.as_ptr() as usize, buffer.len(), 0, 0, 0],
    )
}

pub fn sys_exit(exit_code: i32) -> ! {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0, 0, 0, 0]);
    panic!("sys_exit never returns!");
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_SCHED_YIELD, [0, 0, 0, 0, 0, 0])
}

// pub fn sys_kill(pid: usize, signal: i32) -> isize {
//     syscall(SYSCALL_KILL, [pid, signal as usize, 0, 0, 0, 0])
// }

pub fn sys_get_time(time: &mut TimeVal) -> isize {
    syscall(
        SYSCALL_GET_TIMEOFDAY,
        [time as *mut TimeVal as usize, 0, 0, 0, 0, 0],
    )
}

pub fn sys_getpid() -> isize {
    syscall(SYSCALL_GETPID, [0, 0, 0, 0, 0, 0])
}

pub fn sys_fork() -> isize {
    syscall(SYSCALL_CLONE, [0, 0, 0, 0, 0, 0])
}

pub fn sys_execve(path: &str, args: &[*const u8]) -> isize {
    syscall(
        SYSCALL_EXECVE,
        [path.as_ptr() as usize, args.as_ptr() as usize, 0, 0, 0, 0],
    )
}

pub fn sys_waitpid(pid: isize, exit_code: *mut i32) -> isize {
    syscall(
        SYSCALL_WAIT4,
        [pid as usize, exit_code as usize, 0, 0, 0, 0],
    )
}
