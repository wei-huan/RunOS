mod interrupt;
mod up;

pub use up::UPSafeCell;
pub use interrupt::{IntrLock, interrupt_on, interrupt_off, interrupt_get};
