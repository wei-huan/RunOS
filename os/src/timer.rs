use crate::opensbi::set_timer;
use crate::dt::TIMER_FREQ;
use riscv::register::time;
use core::sync::atomic::{Ordering};

const MSEC_PER_SEC: usize = 1000;
const TICKS_PER_SEC: usize = 100;

pub fn get_time() -> usize {
    time::read()
}

// 得到时钟频率的方式不是很好
#[allow(unused)]
pub fn get_time_ms() -> usize {
    let timer_freq = TIMER_FREQ.load(Ordering::Acquire);
    time::read() / (timer_freq / MSEC_PER_SEC)
}

// 得到时钟频率的方式不是很好
#[allow(unused)]
pub fn set_next_trigger() {
    let timer_freq = TIMER_FREQ.load(Ordering::Acquire);
    set_timer(get_time() + timer_freq / TICKS_PER_SEC);
}
