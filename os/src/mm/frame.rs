extern crate alloc;

use alloc::collections::VecDeque;
use crate::config::MEMORY_END;
use super::address::{PhysPageNum, PhysAddr};

extern "C" {
    fn ekernel();
}

pub struct Frame {
    pub ppn: PhysPageNum
}

impl Frame {
    pub fn new(ppn: PhysPageNum) -> Self {
        // page cleaning
        let bytes_array = ppn.get_bytes_array();
        for i in bytes_array {
            *i = 0;
        }
        Self { ppn }
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
    }
}

trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<Frame>;
    fn dealloc(&mut self);
}

struct FIFOFrameAllocator {
    start: Frame,
    end: Frame,
    current: Frame,
    recycled: VecDeque<Frame>
}

// impl FrameAllocator for FIFOFrameAllocator {
//     fn new() -> Self;
//     fn alloc(&mut self) -> Option<Frame>;
//     fn dealloc(&mut self, frame: Frame);
// }

pub fn frame_test() {
    let a = Frame::new(PhysPageNum::from(0x80480));
    println!("frame test pass");
}
