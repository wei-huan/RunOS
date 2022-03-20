#![no_std]
#![no_main]

#[macro_use]
extern crate user;
extern crate alloc;

use alloc::string::String;
use user::console::{getchar, read_line};

#[no_mangle]
pub fn main() -> i32 {
    // let mut line = String::new();
    // let c = getchar();
    // read_line(&mut line).expect("error");
    0
}

