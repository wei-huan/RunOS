#[cfg(feature = "board-k210")]
mod k210;
#[cfg(feature = "board-qemu")]
mod qemu;

#[cfg(feature = "board-k210")]
pub use k210::{CPU_NUM};
#[cfg(feature = "board-qemu")]
pub use qemu::{CPU_NUM};
