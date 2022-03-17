use core::arch::asm;
use log::*;

pub fn idle_process() {
    info!("No Process");
    loop {
        unsafe {asm!("wfi")};
    }
}
