mod finfo;
mod inode;
mod mount;
mod pipe;
mod stdio;

use crate::mm::UserBuffer;
use alloc::sync::Arc;

#[derive(Clone)]
pub struct FileDescripter {
    cloexec: bool,
    pub fclass: FileClass,
}

impl FileDescripter {
    pub fn new(cloexec: bool, fclass: FileClass) -> Self {
        Self { cloexec, fclass }
    }

    pub fn set_cloexec(&mut self, flag: bool) {
        self.cloexec = flag;
    }

    pub fn get_cloexec(&self) -> bool {
        self.cloexec
    }
}

#[derive(Clone)]
pub enum FileClass {
    File(Arc<OSInode>),
    Abstr(Arc<dyn File + Send + Sync>),
}

pub trait File: Send + Sync {
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
    fn read(&self, buf: UserBuffer) -> usize;
    fn write(&self, buf: UserBuffer) -> usize;
}

pub use finfo::{Dirent, FdSet, Kstat, NewStat, DT_DIR, DT_REG, DT_UNKNOWN, *};
pub use inode::{
    /* find_par_inode_id, */ ch_dir, clear_cache, init_rootfs, list_apps, list_files, open,
    DiskInodeType, OSInode, OpenFlags,
};
pub use mount::MNT_TABLE;
pub use pipe::{make_pipe, Pipe};
pub use stdio::{Stdin, Stdout};
