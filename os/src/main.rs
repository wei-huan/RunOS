#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate spin;

#[cfg(feature = "board_k210")]
#[path = "boards/k210.rs"]
mod boards;
#[cfg(not(any(feature = "board_k210")))]
#[path = "boards/qemu.rs"]
mod boards;

#[macro_use]
mod console;
mod drivers;
mod lang_items;
mod rustsbi;
mod config;
mod sync;
mod mm;
mod trap;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe {
            (a as *mut u8).write_volatile(0)
        }
    })
}

#[no_mangle]
fn os_main() {
    clear_bss();
    mm::init();
    trap::init();
    println!("Hello, world!");
    panic!("Shit");
}
