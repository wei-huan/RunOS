mod cpu;
mod cpus;

pub use cpu::{current_trap_cx, current_user_token, schedule, Cpu};
pub use cpus::{cpu_id, current_process, take_current_process, CPUS};

use crate::process::{fetch_process, idle_process, ProcessContext, ProcessStatus, __switch};
use crate::trap;

pub fn run_processes() {
    loop {
        let mut cpu = CPUS[cpu_id()].exclusive_access();
        if let Some(process) = fetch_process() {
            let idle_proc_cx_ptr = cpu.take_idle_proc_cx_ptr();
            // access coming process TCB exclusively
            let mut process_inner = process.inner_exclusive_access();
            let next_proc_cx_ptr = &process_inner.proc_cx as *const ProcessContext;
            process_inner.proc_status = ProcessStatus::Running;
            drop(process_inner);
            // release coming process TCB manually
            cpu.current = Some(process);
            // release processor manually
            drop(cpu);
            unsafe {
                __switch(idle_proc_cx_ptr, next_proc_cx_ptr);
            }
        } else {
            // trap::init();
            idle_process();
            // access coming task TCB exclusively
        }
    }
}
