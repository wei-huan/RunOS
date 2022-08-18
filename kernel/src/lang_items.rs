use crate::config::USER_STACK_BASE;
use crate::cpu::current_stack_top;
use crate::mm::translated_ref;
#[cfg(feature = "opensbi")]
use crate::opensbi::shutdown;
#[cfg(feature = "rustsbi")]
use crate::rustsbi::shutdown;
use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        log::error!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        log::error!("Panicked: {}", info.message().unwrap());
    }
    unsafe {
        backtrace();
    }
    shutdown()
}

const BACKTRACE_MAX_DEPTH: usize = 15;

pub unsafe fn backtrace() {
    let mut fp: usize;
    let stop = current_stack_top();
    // println!("stop: {:#X}", stop);
    asm!("mv {}, s0", out(reg) fp);
    // let mut sp: usize;
    // asm!("mv {}, sp", out(reg) sp);
    // println!("sp: {:#X}", sp);
    println!("---START BACKTRACE---");
    for i in 0..BACKTRACE_MAX_DEPTH {
        if fp == stop {
            break;
        }
        println!("#{}:ra={:#x}", i, *((fp - 8) as *const usize));
        fp = *((fp - 16) as *const usize);
    }
    println!("---END   BACKTRACE---");
}

#[allow(unused)]
pub unsafe fn user_backtrace(token: usize, s0: usize) {
    let mut fp = s0;
    println!("---START USER BACKTRACE---");
    for i in 0..BACKTRACE_MAX_DEPTH {
        println!("now fp = {:#x}", fp);
        if fp == USER_STACK_BASE {
            break;
        }
        if fp == 0 {
            println!("corrupted stack frame");
            break;
        }
        println!(
            "#{}:ra={:#x}",
            i,
            *(translated_ref(token, (fp - 8) as *const usize))
        );
        fp = *(translated_ref(token, (fp - 16) as *const usize));
    }
    println!("---END USER BACKTRACE---");
}
