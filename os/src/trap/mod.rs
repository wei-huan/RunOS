mod trap;
mod context;

use crate::sync;
use trap::set_kernel_trap_entry;

pub use trap::user_trap_return;
pub use context::TrapContext;

pub fn init() {
    sync::interrupt_on();
    set_kernel_trap_entry();
}
