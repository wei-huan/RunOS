#![no_std]

extern crate alloc;

mod bitmap;
mod block_cache;
mod block_dev;
mod myfs;
mod layout;
mod vfs;

// 扇区大小512字节
pub const BLOCK_SZ: usize = 512;
use bitmap::Bitmap;
use block_cache::{block_cache_sync_all, get_block_cache};
pub use block_dev::BlockDevice;
pub use myfs::MyFileSystem;
use layout::*;
pub use vfs::Inode;