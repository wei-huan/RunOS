#[cfg(not(any(feature = "platform-k210")))]
mod qemu;
#[cfg(feature = "platform-k210")]
mod k210;


#[cfg(not(any(feature = "platform-k210")))]
pub use qemu::{BlockDeviceImpl, MMIO};
#[cfg(feature = "platform-k210")]
pub use k210::{BlockDeviceImpl, MMIO};
