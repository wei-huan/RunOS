use crate::{
    cpus::Cpus,
    dt::TIMER_FREQ,
    timer::get_time,
    utils::{micros, time_parts},
};
use core::fmt;
use core::sync::atomic::{AtomicBool, Ordering};
use log::Level;

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

// static HART_FILTER: AtomicUsize = AtomicUsize::new(usize::MAX);
static USING: AtomicBool = AtomicBool::new(false);

struct MyLogger;

impl log::Log for MyLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        // let hart_id = crate::HART_ID.get();
        // if HART_FILTER.load(Ordering::Relaxed) & (1 << hart_id) == 0 {
        //     return false;
        // }
        let mut mod_path = metadata.target();
        mod_path = if mod_path == "MyOS" {
            "kernel"
        } else {
            mod_path.trim_start_matches("MyOS::")
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
        let cpu_id = Cpus::cpu_id();
        let freq = TIMER_FREQ.load(core::sync::atomic::Ordering::Relaxed);
        let curr_time = get_time();
        let (secs, ms, _) = time_parts(micros(curr_time, freq));
        let color = match record.level() {
            log::Level::Trace => WHITE,
            log::Level::Debug => GREEN,
            log::Level::Info => BLUE,
            log::Level::Warn => YELLOW,
            log::Level::Error => RED,
        };
        let clear = CLEAR;
        while USING.load(Ordering::SeqCst) {
            core::hint::spin_loop();
        }
        USING.store(true, Ordering::SeqCst);
        println!(
            "[{:>5}.{:<03}][{}{:>5}{} ][HART {}][{}] {}",
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
    log::set_logger(&MyLogger).expect("failed to init logging");
    log::set_max_level(log::LevelFilter::Trace);
}

/// Add escape sequence to print with color in Linux console
macro_rules! with_color {
    ($args: ident, $color_code: ident) => {{
        format_args!("\u{1B}[{}m{}\u{1B}[0m", $color_code as u8, $args)
    }};
}

fn print_in_color(args: fmt::Arguments, color_code: u8) {
    use crate::console::print;
    print(with_color!(args, color_code));
}

fn level_to_color_code(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn => 93,  // BrightYellow
        Level::Info => 34,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // BrightBlack
    }
}
