use crate::{
    cpus::Cpus,
    dt::TIMER_FREQ,
    timer::get_time,
    utils::{micros, time_parts},
};
use core::fmt;
use log::Level;
// use core::sync::atomic::AtomicUsize;

// static HART_FILTER: AtomicUsize = AtomicUsize::new(usize::MAX);

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
        let (secs, ms, _) = crate::utils::time_parts(crate::utils::micros(curr_time, freq));
        print_in_color(
            format_args!(
                "[{:>5}.{:<03}][{:>5}][HART {}][{}] {}\n",
                secs,
                ms,
                record.level(),
                cpu_id,
                mod_path,
                record.args()
            ),
            level_to_color_code(record.level()),
        );
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
