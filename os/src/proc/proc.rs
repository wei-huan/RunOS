use lazy_static::*;
use spin::Mutex;
use alloc::sync::Arc;

lazy_static!{
    pub static ref PROCESS: Mutex<Process> = Mutex::new(Process::new());
}

pub struct Process {
    pid: usize
}

impl Process {
    pub fn new() -> Self {
        Self{
            pid: 0
        }
    }

    pub fn get_pid(&self) -> usize {
        self.pid
    }

    pub fn set_pid(&mut self, pid: usize) {
        self.pid = pid;
    }
}
