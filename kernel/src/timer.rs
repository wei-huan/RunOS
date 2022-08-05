use crate::cpu::current_user_token;
use crate::dt::TIMER_FREQ;
use crate::mm::translated_refmut;
#[cfg(not(feature = "rustsbi"))]
use crate::opensbi::set_timer;
#[cfg(feature = "rustsbi")]
use crate::rustsbi::set_timer;
use core::sync::atomic::Ordering;
use riscv::register::{sie, time};

const TICKS_PER_SEC: usize = 100;
const MSEC_PER_SEC: usize = 1000;
pub const USEC_PER_SEC: usize = 1000_000;
pub const NSEC_PER_SEC: usize = 1000_000_000;


#[repr(C)]
#[derive(Copy, Clone)]
pub struct TimeSpec {
    pub tv_sec: usize,  /* seconds */
    pub tv_nsec: usize, /* nanoseconds */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TimeVal {
    pub tv_sec: u64,   /* seconds */
    pub tv_usec: u64,  /* microseconds */
}

#[derive(Copy, Clone)]
pub struct Times {
    pub tms_utime: i64,
    pub tms_stime: i64,
    pub tms_cutime: i64,
    pub tms_cstime: i64,
}

pub fn get_time() -> usize {
    time::read()
}

// 得到时钟频率的方式不是很好
#[allow(unused)]
pub fn get_time_ms() -> usize {
    let timer_freq = TIMER_FREQ.load(Ordering::Acquire);
    time::read() / (timer_freq / MSEC_PER_SEC)
}

#[allow(unused)]
pub fn get_time_us() -> usize {
    let timer_freq = TIMER_FREQ.load(Ordering::Acquire);
    time::read() / (timer_freq / USEC_PER_SEC)
}

#[allow(unused)]
pub fn get_time_ns() -> usize {
    let timer_freq = TIMER_FREQ.load(Ordering::Acquire);
    (time::read() / (timer_freq / USEC_PER_SEC)) * MSEC_PER_SEC
}

#[allow(unused)]
pub fn get_time_sec() -> usize {
    let timer_freq = TIMER_FREQ.load(Ordering::Acquire);
    time::read() / (timer_freq)
}

pub fn get_time_sec_usec() -> (u64, u64) {
    let timer_freq = TIMER_FREQ.load(Ordering::Acquire);
    let ticks = get_time();
    let sec = (ticks / timer_freq) as u64;
    let usec = ((ticks % timer_freq) * USEC_PER_SEC / timer_freq) as u64;
    // println!("sec: {}", sec);
    // println!("usec: {}", usec);
    (sec, usec)
}

#[allow(unused)]
pub fn compare_time_sec_usec(t_sec: usize, t_usec: usize, f_sec: usize, f_usec: usize) -> bool {
    // Compare sec
    if t_sec > f_sec {
        return true;
    } else if t_sec < f_sec {
        return false;
    }
    //Compare usec
    if t_usec > f_usec {
        return true;
    } else if t_sec < f_usec {
        return false;
    }
    return true;
}

pub fn get_time_val(time_val: *mut TimeVal) -> isize {
    let token = current_user_token();
    let (tv_sec, tv_usec) = get_time_sec_usec();
    *translated_refmut(token, time_val) = TimeVal { tv_sec, tv_usec };
    0
}

// 得到时钟频率的方式不是很好
// 10毫秒一次中断
pub fn set_next_trigger() {
    let timer_freq = TIMER_FREQ.load(Ordering::Acquire);
    set_timer(get_time() + timer_freq / TICKS_PER_SEC);
}

#[inline(always)]
pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

#[inline(always)]
#[allow(unused)]
pub fn disable_timer_interrupt() {
    unsafe {
        sie::clear_stimer();
    }
}

pub fn init() {
    enable_timer_interrupt();
    set_next_trigger();
}
