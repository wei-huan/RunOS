mod context;
mod trap;

pub use context::TrapContext;
pub use trap::{set_kernel_trap_entry, trap_return, user_trap_handler};

use crate::sync::interrupt_on;

pub fn init() {
    set_kernel_trap_entry();
    interrupt_on();
}
