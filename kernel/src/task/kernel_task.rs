use crate::sync::interrupt_on;
use core::arch::asm;

pub fn idle_task() -> ! {
    interrupt_on();
    log::debug!("No Process");
    loop {
        unsafe { asm!("wfi") };
    }
}
