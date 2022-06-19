mod sdcard;
mod virtio_blk;

pub use virtio_blk::VirtIOBlock;
pub use sdcard::SDCardWrapper;

use alloc::sync::Arc;
use runfs::BlockDevice;
use lazy_static::*;
use crate::platform::BlockDeviceImpl;

lazy_static! {
    pub static ref BLOCK_DEVICE: Arc<dyn BlockDevice> = Arc::new(BlockDeviceImpl::new());
}

