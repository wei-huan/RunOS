use crate::cpu::{hart_id, take_my_cpu};
use crate::sync::interrupt_on;
use riscv::asm::wfi;

pub static mut TIME_TO_SCHEDULE: [bool; 4] = [false; 4];

#[allow(unused)]
pub fn idle_task() {
    // statistics
    let mut cpu = take_my_cpu();
    // cpu idle count + 1
    cpu.idle_cnt += 1;
    // let idle_cnt: f32 = cpu.idle_cnt as f32;
    // let task_cnt: f32 = cpu.task_cnt as f32;
    let idle_cnt = cpu.idle_cnt;
    let task_cnt = cpu.task_cnt;
    drop(cpu);
    // log::debug!("idle_cnt: {}", idle_cnt);
    // log::debug!("task_cnt: {}", task_cnt);
    // println!("idle_cnt: {}", idle_cnt);
    // println!("task_cnt: {}", task_cnt);
    // let cpu_usage: f32 = task_cnt / (idle_cnt + task_cnt);
    // println!("CPU Usage: {:<3}", cpu_usage);
    interrupt_on();
    log::trace!("NP"); // No Process
    unsafe {
        while !TIME_TO_SCHEDULE[hart_id()] {
            wfi()
        }
        TIME_TO_SCHEDULE[hart_id()] = false;
    }
}
