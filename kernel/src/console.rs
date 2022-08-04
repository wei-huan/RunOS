#[cfg(feature = "rustsbi")]
use crate::rustsbi::console_putchar;
#[cfg(not(feature = "rustsbi"))]
use crate::opensbi::console_putchar;

use spin::Mutex;
use core::fmt::{self, Write};
use lazy_static::*;

pub struct Console;

lazy_static!{
    pub static ref CONSOLE_MUTEX: Mutex<Console> = Mutex::new(Console);
}

struct Stdout;

impl Write for Stdout{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    CONSOLE_MUTEX.lock();
    Stdout.write_fmt(args).unwrap();
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
