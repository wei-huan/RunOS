#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[macro_use]
mod console;
mod sbi;
mod lang_items;
mod config;
mod mm;

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
    println!("Hello, world!");
    mm::whereis_heap();
    mm::init_heap();
    mm::heap_test();
    mm::addr_test();
    mm::frame_test();
    panic!("Shit");
}
