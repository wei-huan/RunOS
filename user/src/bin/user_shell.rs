#![no_std]
#![no_main]
#![allow(clippy::println_empty_string)]

extern crate alloc;

#[macro_use]
extern crate user;

use alloc::string::String;
use alloc::vec::Vec;
use user::console::read_line;
use user::{exec, fork, waitpid};

#[no_mangle]
pub fn main() -> i32 {
    let mut line: String = String::new();
    loop {
        print!("shell >> ");
        read_line(&mut line).unwrap();
        if line.as_str() == "exit_shell\0" {
            return 0;
        }
        // autorun
        else if line.as_str() == "run_testsuites\0" {
            let mut testsuits: Vec<&str> = Vec::new();
            testsuits.push("times\0");
            testsuits.push("gettimeofday\0");
            testsuits.push("sleep\0");
            testsuits.push("brk\0");
            testsuits.push("clone\0");
            testsuits.push("close\0");
            testsuits.push("dup2\0");
            testsuits.push("dup\0");
            testsuits.push("execve\0");
            testsuits.push("exit\0");
            testsuits.push("fork\0");
            testsuits.push("fstat\0");
            testsuits.push("getcwd\0");
            testsuits.push("getdents\0");
            testsuits.push("getpid\0");
            testsuits.push("getppid\0");
            testsuits.push("mkdir_\0");
            testsuits.push("mmap\0");
            testsuits.push("munmap\0");
            testsuits.push("mount\0");
            testsuits.push("openat\0");
            testsuits.push("open\0");
            testsuits.push("pipe\0");
            testsuits.push("read\0");
            testsuits.push("umount\0");
            testsuits.push("uname\0");
            testsuits.push("wait\0");
            testsuits.push("waitpid\0");
            testsuits.push("write\0");
            testsuits.push("yield\0");
            testsuits.push("unlink\0");
            testsuits.push("chdir\0");
            for programname in testsuits.iter() {
                let pid = fork();
                let mut exit_code = 0;
                if pid == 0 {
                    // child process
                    if exec(programname, &[0 as *const u8]) == -1 {
                        println!("Error when executing run_testsuites!1");
                        return -4;
                    }
                    unreachable!();
                } else {
                    waitpid(pid as usize, &mut exit_code);
                }
            }
        } else {
            let pid = fork();
            if pid == 0 {
                // child process
                if exec(line.as_str(), &[0 as *const u8]) == -1 {
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
        }
        line.clear();
    }
}
