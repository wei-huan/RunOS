use riscv::register::sstatus;

// enable device interrupts
pub fn intr_on() {
    unsafe {
        sstatus::set_sie();
    }
}
// disable device interrupts
pub fn intr_off() {
    unsafe {
        sstatus::clear_sie();
    }
}

// are device interrupts enabled?
pub fn intr_get() -> bool {
    sstatus::read().sie()
}
