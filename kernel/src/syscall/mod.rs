#![allow(unused)]

mod errorno;
mod fs;
mod process;
mod sysinfo;
mod syslog;
mod utsname;
mod futex;

use crate::cpu::{current_process, current_task};
use crate::task::SignalAction;
use crate::timer::{TimeVal, Times};

pub use errorno::*;
use fs::*;
use process::*;
use sysinfo::*;
use syslog::*;
use utsname::*;
use futex::*;

const SYSCALL_GETCWD: usize = 17;
const SYSCALL_DUP: usize = 23;
const SYSCALL_DUP3: usize = 24;
const SYSCALL_FCNTL: usize = 25;
const SYSCALL_IOCTL: usize = 29;
const SYSCALL_MKDIRAT: usize = 34;
const SYSCALL_UNLINKAT: usize = 35;
const SYSCALL_LINKAT: usize = 37;
const SYSCALL_UMOUNT2: usize = 39;
const SYSCALL_MOUNT: usize = 40;
const SYSCALL_STATFS: usize = 43;
const SYSCALL_FSTATFS: usize = 44;
const SYSCALL_FACCESSAT: usize = 48;
const SYSCALL_CHDIR: usize = 49;
const SYSCALL_OPENAT: usize = 56;
const SYSCALL_CLOSE: usize = 57;
const SYSCALL_PIPE: usize = 59;
const SYSCALL_GETDENTS64: usize = 61;
const SYSCALL_LSEEK: usize = 62;
const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_READV: usize = 65;
const SYSCALL_WRITEV: usize = 66;
const SYSCALL_PREAD: usize = 67;
const SYSCALL_PWRITE: usize = 68;
const SYSCALL_SENDFILE: usize = 71;
const SYSCALL_FSTATAT: usize = 79;
const SYSCALL_FSTAT: usize = 80;
const SYSCALL_UTIMENSAT: usize = 88;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_EXIT_GRUOP: usize = 94;
const SYSCALL_SET_TID_ADDRESS: usize = 96;
const SYSCALL_FUTEX: usize = 98;
const SYSCALL_SET_ROBUST_LIST: usize = 99;
const SYSCALL_GET_ROBUST_LIST: usize = 100;
const SYSCALL_NANOSLEEP: usize = 101;
const SYSCALL_CLOCK_GETTIME: usize = 113;
const SYSCALL_SYSLOG: usize = 116;
const SYSCALL_SCHED_YIELD: usize = 124;
const SYSCALL_KILL: usize = 129;
const SYSCALL_TKILL: usize = 130;
const SYSCALL_SIGACTION: usize = 134;
const SYSCALL_SIGPROCMASK: usize = 135;
const SYSCALL_RT_SIGTIMEDWAIT: usize = 137;
const SYSCALL_SIGRETURN: usize = 139;
const SYSCALL_TIMES: usize = 153;
const SYSCALL_UNAME: usize = 160;
const SYSCALL_GET_TIMEOFDAY: usize = 169;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_GETPPID: usize = 173;
const SYSCALL_GETUID: usize = 174;
const SYSCALL_GETEUID: usize = 175;
const SYSCALL_GETGID: usize = 176;
const SYSCALL_GETEGID: usize = 177;
const SYSCALL_GETTID: usize = 178;
const SYSCALL_SYSINFO: usize = 179;
const SYSCALL_SOCKET: usize = 198;
const SYSCALL_SOCKETPAIR: usize = 199;
const SYSCALL_BIND: usize = 200;
const SYSCALL_LISTEN: usize = 201;
const SYSCALL_ACCEPT: usize = 202;
const SYSCALL_CONNECT: usize = 203;
const SYSCALL_GETSOCKNAME: usize = 204;
const SYSCALL_GETPAIRNAME: usize = 205;
const SYSCALL_SENDTO: usize = 206;
const SYSCALL_RECVFROM: usize = 207;
const SYSCALL_SETSOCKOPT: usize = 208;
const SYSCALL_GETSOCKOPT: usize = 209;
const SYSCALL_SHUTDOWN: usize = 210;
const SYSCALL_SBRK: usize = 213;
const SYSCALL_BRK: usize = 214;
const SYSCALL_MUNMAP: usize = 215;
const SYSCALL_CLONE: usize = 220;
const SYSCALL_EXECVE: usize = 221;
const SYSCALL_MMAP: usize = 222;
const SYSCALL_MPROTECT: usize = 226;
const SYSCALL_WAIT4: usize = 260;
const SYSCALL_PRLIMIT: usize = 261;
const SYSCALL_MEMBARRIER: usize = 283;

pub fn syscall(syscall_id: usize, args: [usize; 6]) -> isize {
    // let pid = current_process().unwrap().getpid();
    // let lid = current_task()
    //     .unwrap()
    //     .acquire_inner_lock()
    //     .res
    //     .as_ref()
    //     .unwrap()
    //     .lid;
    // if pid >= 3 && syscall_id != SYSCALL_READ && syscall_id != SYSCALL_WRITE {
    //     log::debug!("process{} thread{} syscall: {}", pid, lid, syscall_id);
    // }
    match syscall_id {
        SYSCALL_GETCWD => sys_getcwd(args[0] as *mut u8, args[1] as usize),
        SYSCALL_DUP => sys_dup(args[0]),
        SYSCALL_DUP3 => sys_dup3(args[0], args[1]),
        SYSCALL_FCNTL => sys_fcntl(),
        SYSCALL_IOCTL => sys_ioctl(),
        SYSCALL_MKDIRAT => sys_mkdir(args[0] as isize, args[1] as *const u8, args[2] as u32),
        SYSCALL_UNLINKAT => sys_unlinkat(args[0] as i32, args[1] as *const u8, args[2] as u32),
        SYSCALL_UMOUNT2 => sys_umount(args[0] as *const u8, args[1] as usize),
        SYSCALL_MOUNT => sys_mount(
            args[0] as *const u8,
            args[1] as *const u8,
            args[2] as *const u8,
            args[3] as usize,
            args[4] as *const u8,
        ),
        SYSCALL_STATFS => sys_statfs(args[0] as _, args[1] as _),
        SYSCALL_FACCESSAT => sys_faccessat(args[0], args[1] as *const u8, args[2], 0),
        SYSCALL_CHDIR => sys_chdir(args[0] as *const u8),
        SYSCALL_OPENAT => sys_open_at(args[0] as _, args[1] as _, args[2] as _, args[3] as _),
        SYSCALL_CLOSE => sys_close(args[0]),
        SYSCALL_PIPE => sys_pipe(args[0] as *mut u32, args[1] as usize),
        SYSCALL_GETDENTS64 => {
            sys_getdents64(args[0] as isize, args[1] as *mut u8, args[2] as usize)
        }
        SYSCALL_LSEEK => sys_lseek(args[0], args[1] as _, args[2] as _),
        SYSCALL_READ => sys_read(args[0], args[1] as *const u8, args[2]),
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_READV => sys_readv(args[0], args[1] as *const crate::fs::IOVec, args[2]),
        SYSCALL_WRITEV => sys_writev(args[0], args[1] as *const crate::fs::IOVec, args[2]),
        SYSCALL_PREAD => sys_pread(args[0], args[1] as _, args[2], args[3]),
        SYSCALL_SENDFILE => sys_sendfile(
            args[0] as isize,
            args[1] as isize,
            args[2] as *mut usize,
            args[3] as usize,
        ),
        SYSCALL_FSTATAT => sys_fstatat(args[0] as isize, args[1] as *mut u8, args[2] as *mut u8),
        SYSCALL_FSTAT => sys_fstat(args[0] as isize, args[1] as *mut u8),
        SYSCALL_UTIMENSAT => sys_utimensat(args[0], args[1] as *const u8, args[2], args[3] as u32),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_EXIT_GRUOP => sys_exit_group(args[0] as i32),
        SYSCALL_SET_ROBUST_LIST => 0,
        SYSCALL_GET_ROBUST_LIST => 0,
        SYSCALL_SET_TID_ADDRESS => sys_set_tid_address(args[0] as _),
        SYSCALL_FUTEX => 0,
        SYSCALL_NANOSLEEP => sys_sleep(unsafe { &*(args[0] as *const TimeVal) }, unsafe {
            &mut *(args[1] as *mut TimeVal)
        }),
        SYSCALL_CLOCK_GETTIME => sys_clock_get_time(args[0] as usize, args[1] as *mut u64),
        SYSCALL_SYSLOG => sys_syslog(args[0] as isize, args[1] as *const u8, args[2] as isize),
        SYSCALL_SCHED_YIELD => sys_yield(),
        SYSCALL_KILL => sys_kill(args[0], args[1] as _),
        SYSCALL_TKILL => sys_tkill(args[0], args[1] as _),
        SYSCALL_SIGACTION => sys_sigaction(
            args[0] as i32,
            args[1] as *const SignalAction,
            args[2] as *mut SignalAction,
        ),
        SYSCALL_SIGPROCMASK => sys_sigprocmask(args[0] as _, args[1] as _, args[2] as _),
        SYSCALL_RT_SIGTIMEDWAIT => 0,
        SYSCALL_SIGRETURN => sys_sigretrun(),
        SYSCALL_TIMES => sys_times(unsafe { &mut *(args[0] as *mut Times) }),
        SYSCALL_UNAME => sys_uname(args[0] as *mut u8),
        SYSCALL_GET_TIMEOFDAY => sys_get_time(args[0] as *mut TimeVal),
        SYSCALL_GETPID => sys_getpid(),
        SYSCALL_GETPPID => sys_getppid(),
        SYSCALL_GETUID => sys_getuid(),
        SYSCALL_GETEUID => sys_geteuid(),
        SYSCALL_GETGID => sys_getgid(),
        SYSCALL_GETEGID => sys_getegid(),
        SYSCALL_GETTID => sys_gettid(),
        SYSCALL_SYSINFO => sys_sysinfo(args[0] as *mut u8),
        SYSCALL_SOCKET => 0,
        SYSCALL_SOCKETPAIR => 0,
        SYSCALL_BIND => 0,
        SYSCALL_LISTEN => 0,
        SYSCALL_ACCEPT => 0,
        SYSCALL_CONNECT => 0,
        SYSCALL_GETSOCKNAME => 0,
        SYSCALL_GETPAIRNAME => 0,
        SYSCALL_SENDTO => 0,
        SYSCALL_RECVFROM => 0,
        SYSCALL_SETSOCKOPT => 0,
        SYSCALL_GETSOCKOPT => 0,
        SYSCALL_SBRK => sys_sbrk(args[0] as isize),
        SYSCALL_BRK => sys_brk(args[0]),
        SYSCALL_CLONE => sys_clone(
            args[0] as _,
            args[1] as _,
            args[2] as _,
            args[3] as _,
            args[4] as _,
        ),
        SYSCALL_MUNMAP => sys_munmap(args[0] as usize, args[1] as usize),
        SYSCALL_EXECVE => sys_exec(args[0] as *const u8, args[1] as *const usize),
        SYSCALL_MMAP => sys_mmap(
            args[0] as usize,
            args[1] as usize,
            args[2] as usize,
            args[3] as usize,
            args[4] as isize,
            args[5] as usize,
        ),
        SYSCALL_MPROTECT => sys_mprotect(args[0], args[1], args[2]),
        SYSCALL_WAIT4 => sys_wait4(args[0] as isize, args[1] as *mut i32, args[2] as isize), //sys_waitpid(args[0] as isize, args[1] as *mut i32),
        SYSCALL_PRLIMIT => sys_prlimit(args[0] as _, args[1] as _, args[2] as _, args[3] as _),
        SYSCALL_MEMBARRIER => 0,
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
