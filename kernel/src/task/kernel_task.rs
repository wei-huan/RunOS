use core::arch::asm;

pub fn idle_task() {
    log::debug!("No Process");
    loop {
        unsafe {asm!("wfi")};
    }
}
