mod logger;

use crate::dt::{CPU_NUMS, TIMER_FREQ};
#[cfg(not(feature = "rustsbi"))]
use crate::opensbi::{impl_id, impl_version, spec_version};
#[cfg(feature = "rustsbi")]
use crate::rustsbi::{impl_id, impl_version, spec_version};
use core::sync::atomic::Ordering;

pub use logger::init;

// extern "C" {
//     fn boot_stack();
//     fn boot_stack_top();
// }

pub fn show_basic_info() {
    let n_cpus = CPU_NUMS.load(Ordering::Relaxed);
    let timebase_frequency = TIMER_FREQ.load(Ordering::Relaxed);
    log::info!("=== Machine Info ===");
    log::info!(" Total CPUs: {}", n_cpus);
    log::info!(" Timer Clock: {}Hz", timebase_frequency);
    log::info!("=== SBI Implementation ===");
    let (impl_major, impl_minor) = {
        let version = impl_version();
        (version >> 16, version & 0xFFFF)
    };
    let (spec_major, spec_minor) = {
        let version = spec_version();
        (version.major, version.minor)
    };
    log::info!(
        " Implementor: {:?} (version: {}.{})",
        impl_id(),
        impl_major,
        impl_minor
    );
    log::info!(" Spec Version: {}.{}", spec_major, spec_minor);
    log::info!("=== RunOS Info ===");
    log::info!(" RunOS version {}", env!("CARGO_PKG_VERSION"));
    // log::info!(
    //     "Boot_Stack_0: [{:#X}, {:#X})",
    //     boot_stack as usize,
    //     boot_stack as usize + (boot_stack_top as usize - boot_stack as usize) / 4
    // );
    // log::info!(
    //     "Boot_Stack_1: [{:#X}, {:#X})",
    //     boot_stack as usize + (boot_stack_top as usize - boot_stack as usize) / 4,
    //     boot_stack as usize + (boot_stack_top as usize - boot_stack as usize) / 2
    // );
}
