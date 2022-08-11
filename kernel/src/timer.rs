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

#[derive(Copy, Clone, Debug)]
pub struct TimeSpec {
    pub sec: u64,
    pub nsec: u64,
}

#[derive(Copy, Clone, Debug)]
pub struct TimeVal {
    pub sec: u64,
    pub usec: u64,
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

pub fn get_time_ms() -> usize {
    let timer_freq = TIMER_FREQ.load(Ordering::Acquire);
    time::read() / (timer_freq / MSEC_PER_SEC)
}

pub fn get_time_us() -> usize {
    let timer_freq = TIMER_FREQ.load(Ordering::Acquire);
    // time::read() / (timer_freq / USEC_PER_SEC)
    time::read() * 10 / (timer_freq / 100000)
}

pub fn get_time_ns() -> usize {
    let timer_freq = TIMER_FREQ.load(Ordering::Acquire);
    // (time::read() / (timer_freq / USEC_PER_SEC)) * MSEC_PER_SEC
    time::read() * 10000 / (timer_freq / 100000)
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

pub fn get_time_sec_nsec() -> (u64, u64) {
    let timer_freq = TIMER_FREQ.load(Ordering::Acquire) as u64;
    let ticks = get_time() as u64;
    let sec = ticks / timer_freq;
    let nsec = ((ticks % timer_freq) * (NSEC_PER_SEC as u64)) / timer_freq;
    // println!("sec: {}", sec);
    // println!("nsec: {}", usec);
    (sec, nsec)
}

pub fn get_time_val(time_val: *mut TimeVal) -> isize {
    let token = current_user_token();
    let (sec, usec) = get_time_sec_usec();
    *translated_refmut(token, time_val) = TimeVal { sec, usec };
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
