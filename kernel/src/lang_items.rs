#[cfg(feature = "opensbi")]
use crate::opensbi::shutdown;
#[cfg(feature = "rustsbi")]
use crate::rustsbi::shutdown;
use core::panic::PanicInfo;
use crate::cpu::current_kstack_top;
use core::arch::asm;

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

unsafe fn backtrace() {
    let mut fp: usize;
    let stop = current_kstack_top();
    asm!("mv {}, s0", out(reg) fp);
    println!("---START BACKTRACE---");
    for i in 0..10 {
        if fp == stop {
            break;
        }
        println!("#{}:ra={:#x}", i, *((fp - 8) as *const usize));
        fp = *((fp - 16) as *const usize);
    }
    println!("---END   BACKTRACE---");
}

