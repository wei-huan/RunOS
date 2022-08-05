#[cfg(not(feature = "rustsbi"))]
use crate::opensbi::console_putchar;
#[cfg(feature = "rustsbi")]
use crate::rustsbi::console_putchar;
use core::fmt::{self, Write};
use core::sync::atomic::{AtomicBool, Ordering};

static USING: AtomicBool = AtomicBool::new(false);

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    while USING.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) != Ok(false) {
        core::hint::spin_loop();
    }
    Stdout.write_fmt(args).unwrap();
    while USING.compare_exchange(true, false, Ordering::Acquire, Ordering::Relaxed) != Ok(true) {
        core::hint::spin_loop();
    }
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?))
    };
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?))
    };
}
