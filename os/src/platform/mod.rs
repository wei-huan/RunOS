#[cfg(not(any(feature = "platform-k210")))]
mod qemu;
#[cfg(feature = "platform-k210")]
mod k210;

pub enum ExitStatus<'a> {
    Ok,
    Error(&'a dyn core::fmt::Display),
}

#[cfg(not(any(feature = "platform-k210")))]
pub fn exit(status: ExitStatus) -> ! {
    qemu::exit(match status {
        ExitStatus::Ok => qemu::ExitStatus::Pass,
        ExitStatus::Error(_) => qemu::ExitStatus::Fail(1),
    })
}

#[cfg(not(any(feature = "platform-k210")))]
pub use qemu::{BlockDeviceImpl, MMIO};
#[cfg(feature = "platform-k210")]
pub use k210::{BlockDeviceImpl, MMIO};
// #[cfg(feature = "platform-k210")]
