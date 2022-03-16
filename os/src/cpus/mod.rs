mod cpu;
mod cpus;

pub use cpu::{Cpu, current_user_token, current_trap_cx};
pub use cpus::{cpu_id, CPUS};

use crate::process::{fetch_process, idle_process, ProcessContext, ProcessStatus, __first_switch};
// use crate::trap::user_trap_return;
use log::info;

pub fn run_processes() {
    loop {
        let mut cpu = CPUS[cpu_id()].exclusive_access();
        if let Some(process) = fetch_process() {
            info!("1");
            let mut process_inner = process.inner_exclusive_access();
            info!("2");
            let next_proc_cx_ptr = &process_inner.proc_cx as *const ProcessContext;
            info!("3");
            process_inner.proc_status = ProcessStatus::Running;
            info!("4");
            drop(process_inner);
            info!("5");
            // release coming task TCB manually
            cpu.set_current(Some(process));
            // release processor manually
            // drop(cpu);
            info!("6");
            unsafe {
                __first_switch(next_proc_cx_ptr);
            }
            info!("7");
            // user_trap_return();
        } else {
            idle_process();
            // access coming task TCB exclusively
        }
    }
}
