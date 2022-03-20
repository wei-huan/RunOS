#![no_std]
#![no_main]

#[macro_use]
extern crate user;
extern crate alloc;

use alloc::string::String;
use user::console::{getchar, read_line};

#[no_mangle]
pub fn main() -> i32 {
    // println!("please input:");
    // let c = getchar();
    // println!("get char: {}", c);
    let mut line = String::new();
    println!("please input:");
    read_line(&mut line).expect("error");
    println!("get line: {}", line);
    0
}

