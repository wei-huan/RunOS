mod context;
mod trap;

pub use context::TrapContext;
pub use trap::set_kernel_trap_entry;
pub use trap::{user_trap_handler, trap_return};

use crate::sync;

pub fn init() {
    set_kernel_trap_entry();
    sync::interrupt_on();
}
