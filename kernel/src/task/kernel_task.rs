// use crate::cpu::take_my_cpu;
use crate::timer::enable_timer_interrupt;
use core::arch::asm;

pub fn idle_task() -> ! {
    enable_timer_interrupt();
    // statistics
    // let mut cpu = take_my_cpu();
    // cpu idle count + 1
    // cpu.idle_cnt += 1;
    // let idle_cnt: f32 = cpu.idle_cnt as f32;
    // let task_cnt: f32 = cpu.task_cnt as f32;
    // let idle_cnt = cpu.idle_cnt;
    // let task_cnt = cpu.task_cnt;
    // drop(cpu);
    // log::debug!("idle_cnt: {}", idle_cnt);
    // log::debug!("task_cnt: {}", task_cnt);
    // println!("idle_cnt: {}", idle_cnt);
    // println!("task_cnt: {}", task_cnt);
    // let cpu_usage: f32 = task_cnt / (idle_cnt + task_cnt);
    // println!("CPU Usage: {:<3}", cpu_usage);
    log::debug!("NP");  // No Process
    loop {
        unsafe { asm!("wfi") };
    }
}
