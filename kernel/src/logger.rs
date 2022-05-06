#[cfg(not(feature = "rustsbi"))]
use crate::opensbi::{impl_id, impl_version, spec_version};
#[cfg(feature = "rustsbi")]
use crate::rustsbi::{impl_id, impl_version, spec_version};
use crate::{
    cpu::hart_id,
    dt::{CPU_NUMS, TIMER_FREQ},
    timer::get_time,
    utils::{micros, time_parts},
};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use log::*;

pub struct ColorEscape(pub &'static str);

impl core::fmt::Display for ColorEscape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}
pub const RED: ColorEscape = ColorEscape("\x1B[31m");
pub const BLUE: ColorEscape = ColorEscape("\x1B[34m");
pub const GREEN: ColorEscape = ColorEscape("\x1B[32m");
pub const YELLOW: ColorEscape = ColorEscape("\x1B[33m");
pub const WHITE: ColorEscape = ColorEscape("\x1B[37m");
pub const CLEAR: ColorEscape = ColorEscape("\x1B[0m");

static USING: AtomicBool = AtomicBool::new(false);
static HART_FILTER: AtomicUsize = AtomicUsize::new(usize::MAX);

struct MyLogger;

impl log::Log for MyLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        let hart_id = hart_id();
        let max_hart = HART_FILTER.load(Ordering::Relaxed);
        if max_hart < hart_id {
            return false;
        }
        let max_level = max_level();
        let level = metadata.level();
        if max_level < level {
            return false;
        }
        let mut _mod_path = metadata.target();
        _mod_path = if _mod_path == "RunOS" {
            "kernel"
        } else {
            _mod_path.trim_start_matches("RunOS::")
        };
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        while USING.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) == Ok(false) {
            core::hint::spin_loop();
        }
        let mut mod_path = record
            .module_path_static()
            .or_else(|| record.module_path())
            .unwrap_or("<n/a>");
        mod_path = if mod_path == "RunOS" {
            "kernel"
        } else {
            mod_path.trim_start_matches("RunOS::")
        };
        let hart_id = hart_id();
        let freq = TIMER_FREQ.load(core::sync::atomic::Ordering::Relaxed);
        let curr_time = get_time();
        let (secs, ms, _) = time_parts(micros(curr_time, freq));
        let color = match record.level() {
            Level::Trace => WHITE,
            Level::Debug => GREEN,
            Level::Info => BLUE,
            Level::Warn => YELLOW,
            Level::Error => RED,
        };
        let clear = CLEAR;
        println!(
            "[{:>5}.{:<03}][ {}{:>5}{} ][HART {}][{}] {}",
            secs,
            ms,
            color,
            record.level(),
            clear,
            hart_id,
            mod_path,
            record.args(),
        );
        while USING.compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst) == Ok(true) {
            core::hint::spin_loop();
        }
    }
    fn flush(&self) {}
}

pub fn init() {
    set_hart_filter(8);
    log::set_logger(&MyLogger).expect("failed to init logging");
    log::set_max_level(match option_env!("LOG") {
        Some("ERROR") => LevelFilter::Error,
        Some("WARN") => LevelFilter::Warn,
        Some("INFO") => LevelFilter::Info,
        Some("DEBUG") => LevelFilter::Debug,
        Some("TRACE") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });

    // log::set_max_level(LevelFilter::Debug);
}

fn set_hart_filter(hart_id: usize) {
    HART_FILTER.store(hart_id, Ordering::Relaxed);
}

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
