mod context;
mod kernelprocess;
mod kernelstack;
mod manager;
mod pid;
mod process;
mod recyclealloc;
mod signal;
mod switch;

pub use context::ProcessContext;
pub use kernelprocess::idle_process;
pub use manager::fetch_process;
pub use pid::{pid_alloc, PidHandle};
pub use process::{ProcessControlBlock, ProcessStatus};
pub use switch::__switch;

use crate::cpu::schedule;
use crate::cpu::take_current_process;
use crate::fs::{open_file, OpenFlags, ROOT_INODE};
use alloc::sync::Arc;
use manager::add_process;

pub fn add_apps() {
    for app in ROOT_INODE.ls() {
        if let Some(app_inode) = open_file(app.as_str(), OpenFlags::RDONLY) {
            let elf_data = app_inode.read_all();
            let new_processs = ProcessControlBlock::new(elf_data.as_slice());
            add_process(Arc::new(new_processs));
        }
    }
}

pub fn exit_current_and_run_next(exit_code: i32) {
    // take from Processor
    let task = take_current_process().unwrap();
    // **** access current TCB exclusively
    let mut inner = task.inner_exclusive_access();
    // Change status to Ready
    inner.proc_status = ProcessStatus::Zombie;
    // Record exit code
    inner.exit_code = exit_code;
    // Delete children process
    inner.children.clear();
    // deallocate user space
    inner.addrspace.recycle_data_pages();
    drop(inner);
    // **** release current PCB
    // drop task manually to maintain rc correctly
    drop(task);
    // we do not have to save task context
    let mut _unused = ProcessContext::zero_init();
    schedule(&mut _unused as *mut _);
}
