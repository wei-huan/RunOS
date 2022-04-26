use crate::sync::interrupt_on;
use core::arch::asm;

pub fn idle_task() -> ! {
    interrupt_on();
    log::trace!("No Process");
    loop {
        unsafe { asm!("wfi") };
    }
}
