mod sdcard;
mod virtio_blk;

pub use sdcard::SDCardWrapper;
pub use virtio_blk::VirtIOBlock;

use crate::platform::BlockDeviceImpl;
use alloc::sync::Arc;
use lazy_static::*;
use runfs::BlockDevice;

lazy_static! {
    pub static ref BLOCK_DEVICE: Arc<dyn BlockDevice> = {
        let b = Arc::new(BlockDeviceImpl::new());
        // println!("here block device");
        b
    };
}
