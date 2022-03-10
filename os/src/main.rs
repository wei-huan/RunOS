#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod sbi;
mod logging;
mod lang_items;

use core::arch::global_asm;
use log::*;

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
    logging::init();
    info!("Hello, world!");
    warn!("Shit");
    error!("Fuck");
    debug!("Messy");
    trace!("Messy");
    panic!("Shit");
}
