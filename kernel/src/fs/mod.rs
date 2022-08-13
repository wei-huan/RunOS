mod dev;
mod finfo;
mod inode;
mod mount;
mod pipe;
mod stdio;

use crate::mm::UserBuffer;
use alloc::sync::Arc;

#[derive(Clone)]
pub enum FileClass {
    File(Arc<OSInode>),
    Abstr(Arc<dyn File + Send + Sync>),
}

pub trait File: Send + Sync {
    fn readable(&self) -> bool; // general authority
    fn writable(&self) -> bool;
    fn read(&self, buf: UserBuffer) -> usize;
    fn write(&self, buf: UserBuffer) -> usize;
    fn read_available(&self) -> bool; // checking at a specific time for if there is something to reach or if being blocked
    fn write_available(&self) -> bool; // checking at a specific time for if there is something to reach or if being blocked
}

pub use dev::open_device_file;
pub use finfo::*;
pub use inode::{ch_dir, clear_cache, init_rootfs, list_rootfs, open, OSInode, OpenFlags};
pub use mount::MNT_TABLE;
pub use pipe::{make_pipe, Pipe};
pub use stdio::{Stdin, Stdout};
