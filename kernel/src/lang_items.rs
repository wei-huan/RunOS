use crate::cpu::current_stack_top;
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
    // unsafe {
    //     backtrace();
    // }
    shutdown()
}

#[allow(unused)]
unsafe fn backtrace() {
    let mut fp: usize;
    let stop = current_stack_top();
    println!("stop: {:#X}", stop);
    asm!("mv {}, s0", out(reg) fp);
    let mut sp: usize;
    asm!("mv {}, sp", out(reg) sp);
    println!("sp: {:#X}", sp);
    println!("---START BACKTRACE---");
    for i in 0..15 {
        if fp == stop {
            break;
        }
        println!("#{}:ra={:#x}", i, *((fp - 8) as *const usize));
        fp = *((fp - 16) as *const usize);
    }
    println!("---END   BACKTRACE---");
}
