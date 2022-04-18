#![no_std]
#![no_main]

#[macro_use]
extern crate user;

use user::{exec, fork, wait, yield_};

#[no_mangle]
fn main() -> i32 {
    println!("Init Process");
    if fork() == 0 {
        // println!("exec user_shell");
        exec("user_shell\0", &[0 as *const u8]);
    } else {
        loop {
            let mut exit_code: i32 = 0;
            let pid = wait(&mut exit_code);
            if pid == -1 {
                yield_();
                continue;
            }
            println!(
                "[initproc] Released a zombie process, pid={}, exit_code={}",
                pid, exit_code,
            );
        }
    }
    0
}
