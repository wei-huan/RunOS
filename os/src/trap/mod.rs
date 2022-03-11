mod trap;

use crate::sync;
use trap::set_kernel_trap_entry;

pub fn init() {
    sync::interrupt_on();
    set_kernel_trap_entry();
}
