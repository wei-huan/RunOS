use crate::{
    cpu::cpu_id,
    dt::TIMER_FREQ,
    timer::get_time,
    utils::{micros, time_parts},
};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use log::{max_level, Level, LevelFilter};

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
        let hart_id = cpu_id();
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
        _mod_path = if _mod_path == "MyOS" {
            "kernel"
        } else {
            _mod_path.trim_start_matches("MyOS::")
        };
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let mut mod_path = record
            .module_path_static()
            .or_else(|| record.module_path())
            .unwrap_or("<n/a>");
        mod_path = if mod_path == "MyOS" {
            "kernel"
        } else {
            mod_path.trim_start_matches("MyOS::")
        };
        let cpu_id = cpu_id();
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
        while USING.load(Ordering::SeqCst) {
            core::hint::spin_loop();
        }
        USING.store(true, Ordering::SeqCst);
        println!(
            "[{:>5}.{:<03}][ {}{:>5}{} ][HART {}][{}] {}",
            secs,
            ms,
            color,
            record.level(),
            clear,
            cpu_id,
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
    log::set_max_level(LevelFilter::Trace);
}

fn set_hart_filter(hart_id: usize) {
    HART_FILTER.store(hart_id, Ordering::Relaxed);
}
