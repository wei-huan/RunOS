#![no_std]
#![no_main]

#[macro_use]
extern crate user;

#[no_mangle]
pub fn main() {
    println!("Hello world from user mode program!");
}
