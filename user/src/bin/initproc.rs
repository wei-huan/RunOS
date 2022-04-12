#![no_std]
#![no_main]

#[macro_use]
extern crate user;

use user::fork;

#[no_mangle]
fn main() -> i32 {
    if fork() == 0 {
        println!("Son Ok");
        // exec("user_shell\0");
    } else {
        println!("Father Ok");
        loop{}
        // loop {
        //     let mut exit_code: i32 = 0;
        //     let pid = wait(&mut exit_code);
        //     if pid == -1 {
        //         yield_();
        //         continue;
        //     }
        //     println!(
        //         "[initproc] Released a zombie process, pid={}, exit_code={}",
        //         pid, exit_code,
        //     );
        // }
    }
    0
}
