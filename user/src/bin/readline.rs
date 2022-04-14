#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user;

#[no_mangle]
pub fn main() -> i32 {
    println!("Rust user shell");
    println!(">> ");
    0
}
