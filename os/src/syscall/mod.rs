mod fs;
mod sync;
mod process;
mod thread;

use fs::*;
use process::*;
// use sync::*;
// use thread::*;

const SYSCALL_OPEN: usize = 56;
const SYSCALL_CLOSE: usize = 57;
// const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_GET_TIME: usize = 169;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_OPEN => sys_open(args[0] as *const u8, args[1] as u32),
        SYSCALL_CLOSE => sys_close(args[0]),
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_GET_TIME => sys_get_time(),
        _ => 0
    }
}
