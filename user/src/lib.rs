#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;
#[macro_use]
extern crate bitflags;

use alloc::vec::Vec;
#[macro_use]
pub mod console;
mod lang_items;
mod syscall;

// use alloc::vec::Vec;
use buddy_system_allocator::LockedHeap;
use syscall::*;

const AT_FDCWD: i32 = -100;

/// 用户堆空间设置为 32 KB 即 8 个页面
const USER_HEAP_SIZE: usize = 4096 * 8;

static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

// 给buddy_system_allocator使用的，这个值大于32即可, 应该是每次分配的最小单位
pub const HEAP_ALLOCATE_MIN_SIZE: usize = 32;

#[global_allocator]
static HEAP: LockedHeap<HEAP_ALLOCATE_MIN_SIZE> = LockedHeap::empty();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

#[linkage = "weak"]
#[no_mangle]
fn main(_argc: usize, _argv: &[&str]) -> i32 {
    panic!("Cannot find main!");
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start(argc: usize, argv: usize) -> ! {
    unsafe {
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
    let mut v: Vec<&'static str> = Vec::new();
    for i in 0..argc {
        let str_start =
            unsafe { ((argv + i * core::mem::size_of::<usize>()) as *const usize).read_volatile() };
        let len = (0usize..)
            .find(|i| unsafe { ((str_start + *i) as *const u8).read_volatile() == 0 })
            .unwrap();
        v.push(
            core::str::from_utf8(unsafe {
                core::slice::from_raw_parts(str_start as *const u8, len)
            })
            .unwrap(),
        );
    }
    exit(main(argc, v.as_slice()));
}

bitflags! {
    pub struct OpenFlags: u32 {
        const RDONLY = 0;
        const WRONLY = 1 << 0;
        const RDWR = 1 << 1;
        const CREATE = 1 << 9;
        const TRUNC = 1 << 10;
    }
}

pub fn dup(fd: usize) -> isize {
    sys_dup(fd)
}
pub fn chdir(path: &str) -> isize {
    sys_chdir(path)
}
pub fn unlink(path: &str) -> isize {
    sys_unlinkat(AT_FDCWD, path, 0)
}
pub fn mkdir(path: &str) -> isize {
    sys_mkdir(path)
}
pub fn open(path: &str, flags: OpenFlags) -> isize {
    sys_open(path, flags.bits)
}
pub fn close(fd: usize) -> isize {
    sys_close(fd)
}
pub fn pipe(pipe_fd: &mut [usize]) -> isize {
    sys_pipe(pipe_fd)
}
pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    sys_read(fd, buf)
}
pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
pub fn exit(exit_code: i32) -> ! {
    sys_exit(exit_code);
}
pub fn yield_() -> isize {
    sys_yield()
}
pub fn getpid() -> isize {
    sys_getpid()
}
pub fn fork() -> isize {
    sys_fork()
}
pub fn exec(path: &str, args: &[*const u8]) -> isize {
    sys_execve(path, args)
}
pub fn wait(exit_code: &mut i32) -> isize {
    loop {
        match sys_waitpid(-1, exit_code as *mut _) {
            -2 => {
                yield_();
            }
            // -1 or a real pid
            exit_pid => return exit_pid,
        }
    }
}
pub fn waitpid(pid: usize, exit_code: &mut i32) -> isize {
    loop {
        match sys_waitpid(pid as isize, exit_code as *mut _) {
            -2 => {
                yield_();
            }
            // -1 or a real pid
            exit_pid => return exit_pid,
        }
    }
}
bitflags! {
    pub struct SignalFlags: i32 {
        const SIGINT    = 1 << 2;
        const SIGILL    = 1 << 4;
        const SIGABRT   = 1 << 6;
        const SIGFPE    = 1 << 8;
        const SIGSEGV   = 1 << 11;
    }
}

// pub fn kill(pid: usize, signal: i32) -> isize {
//     sys_kill(pid, signal)
// }

#[derive(Copy, Clone, Debug, Default)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub fn sleep(period_us: usize) {
    let mut time = TimeVal::default();
    sys_get_time(&mut time);
    let start = time.sec * 1000000 + time.usec;
    let mut cur_time = start;
    while cur_time < (start + period_us) {
        sys_yield();
        sys_get_time(&mut time);
        cur_time = time.sec * 1000000 + time.usec;
    }
}
