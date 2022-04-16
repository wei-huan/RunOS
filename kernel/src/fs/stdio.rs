use super::File;
use crate::mm::{UserBuffer};
use crate::task::suspend_current_and_run_next;
use lazy_static::*;
use spin::Mutex;
#[cfg(feature = "rustsbi")]
use crate::rustsbi::console_getchar;
#[cfg(feature = "opensbi")]
use crate::opensbi::console_getchar;

//use crate::task::get_core_id;

// 这个模块的两个宏应该公开
// 如果制造实例的时候，给定了stdout，那么就会打印到这个stdout里面
// use embedded_hal::serial::{Read, Write};
// use nb::block;


pub struct Stdin;

pub struct Stdout;

lazy_static!{
    pub static ref STDOUTLOCK:Mutex<usize> = Mutex::new(0);
    pub static ref STDINLOCK:Mutex<usize> = Mutex::new(0);
}

impl File for Stdin {
    fn readable(&self) -> bool { true }
    fn writable(&self) -> bool { false }
    fn read(&self, mut user_buf: UserBuffer) -> usize {
        //assert_eq!(user_buf.len(), 1);
        let lock = STDINLOCK.lock();
        // busy loop
        let mut c: usize;
        let mut count = 0;
        if user_buf.len() > 1{
            return 0;
        }
        loop {
            c = console_getchar();
            if c == 0 {
                drop(user_buf);
                suspend_current_and_run_next();
            } else {
                break;
            }
        }
        let ch = c as u8;
        unsafe {
            user_buf.buffers[0].as_mut_ptr().write_volatile(ch);
            //user_buf.write_at(count, ch);
        }
        return 1
        /*
        loop {
            if count == user_buf.len(){
                break;
            }
            loop {
                c = console_getchar();
                if c == 0 {
                    suspend_current_and_run_next();
                    continue;
                } else {
                    break;
                }
            }
            let ch = c as u8;
            if ch as char == '\n' {
                break;
            }
            unsafe {
                //user_buf.buffers[0].as_mut_ptr().write_volatile(ch);
                user_buf.write_at(count, ch);
            }
            count += 1;
        }
        count*/
    }
    fn write(&self, _user_buf: UserBuffer) -> usize {
        panic!("Cannot write to stdin!");
    }
}

impl File for Stdout {
    fn readable(&self) -> bool { false }
    fn writable(&self) -> bool { true }
    fn read(&self, _user_buf: UserBuffer) -> usize{
        panic!("Cannot read from stdout!");
    }
    fn write(&self, user_buf: UserBuffer) -> usize {
        let lock = STDOUTLOCK.lock();
        for buffer in user_buf.buffers.iter() {
            print!("{}", core::str::from_utf8(*buffer).unwrap());
        }
        user_buf.len()
    }
}
