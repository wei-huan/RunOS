use core::fmt::{self, Write, Error};

const STDIN: usize = 0;
const STDOUT: usize = 1;

use super::{read, write};
use alloc::string::String;

struct Stdout;

impl Write for Stdout{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(STDOUT, s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    };
}

pub fn getchar() -> u8 {
    let mut c = [0u8; 1];
    read(STDIN, &mut c);
    c[0]
}

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;
const DL: u8 = 0x7fu8;
const BS: u8 = 0x08u8;
// const NONE: u8 = 0x00u8;

pub fn read_line(line: &mut String) -> Result<usize, Error> {
    let mut cnt: usize = 0;
    loop {
        // println!("Reading");
        let c = getchar();
        match c {
            // NONE => {
            //     continue;
            // }
            LF | CR => {
                println!("");
                if !line.is_empty() {
                    line.push('\0');
                    return Ok(cnt);
                }
            }
            BS | DL => {
                if !line.is_empty() {
                    print!("{}", BS as char);
                    print!(" ");
                    print!("{}", BS as char);
                    line.pop();
                    cnt -= 1;
                }
            }
            _ => {
                print!("{}", c as char);
                line.push(c as char);
                cnt += 1;
            }
        }
    }
}