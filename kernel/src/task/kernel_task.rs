use core::arch::asm;
use log::*;

pub fn idle_task() -> !{
    info!("No Process");
    loop {
        unsafe {asm!("wfi")};
    }
}
