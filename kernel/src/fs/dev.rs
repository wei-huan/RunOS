use alloc::sync::Arc;

use crate::{fs::File, mm::UserBuffer};

use super::OpenFlags;

#[derive(Default)]
struct DevZero;

impl File for DevZero {
    fn readable(&self) -> bool {
        true
    }
    fn writable(&self) -> bool {
        true
    }
    // fill length size zero
    fn read(&self, mut user_buf: UserBuffer) -> usize {
        user_buf.clear()
    }
    // write no meaning
    fn write(&self, user_buf: UserBuffer) -> usize {
        user_buf.len()
    }
    fn available(&self) -> bool {
        true
    }
}

#[derive(Default)]
pub struct DevNull;

impl File for DevNull {
    fn readable(&self) -> bool {
        true
    }
    fn writable(&self) -> bool {
        true
    }
    // fill length size zero
    fn read(&self, mut user_buf: UserBuffer) -> usize {
        0
    }
    // write to a hole
    fn write(&self, user_buf: UserBuffer) -> usize {
        user_buf.len()
    }
    fn available(&self) -> bool {
        true
    }
}

pub fn open_device_file(filename: &str, flags: OpenFlags) -> Option<Arc<dyn File + Send + Sync>> {
    if flags.contains(OpenFlags::DIRECTROY) {
        return None
    };
    match filename {
        "zero" => Some(Arc::new(DevZero::default())),
        "null" => Some(Arc::new(DevNull::default())),
        _ => None,
    }
}
