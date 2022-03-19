#![no_std]
#![no_main]

#[macro_use]
extern crate user;
extern crate alloc;

use alloc::string::String;
use user::console::read_line;

#[no_mangle]
pub fn main() -> i32 {
    let mut line = String::new();
    read_line(&mut line).expect("error");
    println!("Hello world from user mode program 0!");
    0
}

