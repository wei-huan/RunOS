use core::arch::asm;

pub fn idle_process() {
    loop {
        unsafe {asm!("wfi")};
    }
}
