#[cfg(feature = "board-k210")]
#[path = "boards/k210/mod.rs"]
mod k210;
#[cfg(feature = "board-qemu")]
#[path = "boards/qemu/mod.rs"]
mod qemu;

#[cfg(feature = "board-k210")]
pub use k210::{CPU_NUM};
#[cfg(feature = "board-qemu")]
pub use qemu::{CPU_NUM};
