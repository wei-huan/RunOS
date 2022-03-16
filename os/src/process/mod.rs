mod context;
mod kernelprocess;
mod kernelstack;
mod manager;
mod pid;
mod process;
mod recyclealloc;
mod signal;
mod switch;

pub use switch::__first_switch;
pub use kernelprocess::idle_process;
pub use manager::fetch_process;
pub use pid::{pid_alloc, PidHandle};
pub use process::{ProcessControlBlock, ProcessStatus};
pub use context::ProcessContext;

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
