#![no_std]
#![no_main]

extern crate alloc;
#[macro_use]
extern crate user;

#[no_mangle]
fn main() -> i32 {
    // use user::{exec, fork, wait, yield_};
    // println!("Init Process");
    // if fork() == 0 {
    //     exec("user_shell\0", &[0 as *const u8]);
    // } else {
    //     loop {
    //         // println!("Init Process Waiting");
    //         let mut exit_code: i32 = 0;
    //         let pid = wait(&mut exit_code);
    //         if pid == -1 {
    //             yield_();
    //             continue;
    //         }
    //         println!(
    //             "[initproc] Released a zombie process, pid={}, exit_code={}",
    //             pid, exit_code,
    //         );
    //     }
    // }
    // 0

    use alloc::string::String;
    use user::console::read_line;
    println!("Init Process");
    let mut line: String = String::new();
    read_line(&mut line).unwrap();
    println!("{}", line);
    0
}
