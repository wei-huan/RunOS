#![no_std]
#![no_main]

#[macro_use]
extern crate user;

use user::get_time;

#[no_mangle]
pub fn main() -> i32 {
    get_time();
    println!("Hello world from user mode program 1!");
    0
}
