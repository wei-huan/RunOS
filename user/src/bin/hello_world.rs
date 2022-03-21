#![no_std]
#![no_main]

#[macro_use]
extern crate user;

use user::get_time;

#[no_mangle]
pub fn main() -> i32 {
    let start = get_time();
    let mut end = get_time();
    while end - start <= 100 {
        end = get_time();
    }
    println!("Hello world from user mode program 0!");
    0
}
