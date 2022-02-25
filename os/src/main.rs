#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[macro_use]
mod console;
mod boards;
mod mm;
mod sbi;
mod lang_items;
mod config;

use core::arch::global_asm;
use boards::CPU_NUM;
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
fn os_main(hartid: usize) {
    if hartid == 0 {
        clear_bss();

        for i in 1..CPU_NUM {
            let mask: usize = 1 << i;
            sbi::send_ipi(&mask as *const _ as usize);
        }
    }
    panic!("from hart{}", hartid);
}
