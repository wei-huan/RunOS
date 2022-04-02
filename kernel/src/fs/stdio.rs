use super::File;
use crate::mm::UserBuffer;
use crate::task::suspend_current_and_run_next;
#[cfg(feature = "rustsbi")]
use crate::rustsbi::console_getchar;
#[cfg(feature = "opensbi")]
use crate::opensbi::console_getchar;

pub struct Stdin;
pub struct Stdout;

impl File for Stdin {
    fn readable(&self) -> bool {
        true
    }
    fn writable(&self) -> bool {
        false
    }
    fn read(&self, mut user_buf: UserBuffer) -> usize {
        assert_eq!(user_buf.len(), 1);
        // busy loop
        let c: usize;
        loop {
            c = console_getchar();
            if c == 0 {
                // log::debug!("Get char Suspend");
                drop(user_buf);
                suspend_current_and_run_next();
            } else {
                break;
            }
        }
        let ch = c as u8;
        unsafe {
            user_buf.buffers[0].as_mut_ptr().write_volatile(ch);
        }
        1
    }
    fn write(&self, _user_buf: UserBuffer) -> usize {
        panic!("Cannot write to stdin!");
    }
}

impl File for Stdout {
    fn readable(&self) -> bool {
        false
    }
    fn writable(&self) -> bool {
        true
    }
    fn read(&self, _user_buf: UserBuffer) -> usize {
        panic!("Cannot read from stdout!");
    }
    fn write(&self, user_buf: UserBuffer) -> usize {
        for buffer in user_buf.buffers.iter() {
            match core::str::from_utf8(*buffer) {
                Ok(s) => print!("{}", s),
                Err(e) => println!("{}", e)
            }
        }
        user_buf.len()
    }
}
