#![no_std]
#![no_main]
#![allow(clippy::println_empty_string)]

extern crate alloc;

#[macro_use]
extern crate user;

use alloc::string::String;
use user::console::read_line;
use user::{exec, fork, waitpid};

#[no_mangle]
pub fn main() -> i32 {
    println!("Rust user shell");
    let mut line: String = String::new();
    loop {
        print!(">> ");
        read_line(&mut line).unwrap();
        // println!("123 {}", line);
        if line == "exit\0" {
            break;
        }
        let pid = fork();
        if pid == 0 {
            // child process
            if exec(line.as_str()) == -1 {
                println!("Error when executing!");
                return -4;
            }
            unreachable!();
        } else {
            let mut exit_code: i32 = 0;
            let exit_pid = waitpid(pid as usize, &mut exit_code);
            assert_eq!(pid, exit_pid);
            println!("Shell: Process {} exited with code {}", pid, exit_code);
        }
        line.clear();
    }
    0
}
