use crate::cpu::current_user_token;
use crate::mm::translated_refmut;
#[cfg(not(feature = "rustsbi"))]
use crate::opensbi::set_timer;
use crate::platform::CLOCK_FREQ;
#[cfg(feature = "rustsbi")]
use crate::rustsbi::set_timer;
use riscv::register::{sie, time};

const TICKS_PER_SEC: usize = 100;
const MSEC_PER_SEC: usize = 1000;
pub const USEC_PER_SEC: usize = 1000_000;
pub const NSEC_PER_SEC: usize = 1000_000_000;

#[derive(Copy, Clone, Debug, Default)]
pub struct TimeSpec {
    pub sec: u64,
    pub nsec: u64,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct TimeVal {
    pub sec: u64,
    pub usec: u64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ITimerVal {
    pub it_interval: TimeVal,
    pub it_value: TimeVal,
}

#[derive(Copy, Clone, Debug)]
pub struct Times {
    pub tms_utime: i64,
    pub tms_stime: i64,
    pub tms_cutime: i64,
    pub tms_cstime: i64,
}

pub fn get_time() -> usize {
    time::read()
}

#[allow(unused)]
pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}

pub fn get_time_us() -> usize {
    time::read() / (CLOCK_FREQ / USEC_PER_SEC)
}

pub fn get_time_ns() -> usize {
    time::read() * NSEC_PER_SEC / CLOCK_FREQ
}

#[allow(unused)]
pub fn get_time_sec() -> usize {
    time::read() / (CLOCK_FREQ)
}

pub fn get_time_sec_msec() -> (u64, u64) {
    let ticks = get_time();
    let sec = (ticks / CLOCK_FREQ) as u64;
    let msec = ((ticks % CLOCK_FREQ) * MSEC_PER_SEC / CLOCK_FREQ) as u64;
    // println!("sec: {}", sec);
    // println!("usec: {}", usec);
    (sec, msec)
}

pub fn get_time_sec_usec() -> (u64, u64) {
    let ticks = get_time();
    let sec = (ticks / CLOCK_FREQ) as u64;
    let usec = ((ticks % CLOCK_FREQ) * USEC_PER_SEC / CLOCK_FREQ) as u64;
    // println!("sec: {}", sec);
    // println!("usec: {}", usec);
    (sec, usec)
}

pub fn get_time_sec_nsec() -> (u64, u64) {
    let ticks = get_time();
    let sec = (ticks / CLOCK_FREQ) as u64;
    let nsec = ((ticks % CLOCK_FREQ) * NSEC_PER_SEC / CLOCK_FREQ) as u64;
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
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
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
