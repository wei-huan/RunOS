extern crate alloc;
use super::address::{PhysAddr, PhysPageNum};
use crate::config::MEMORY_END;
use alloc::collections::VecDeque;

extern "C" {
    fn ekernel();
}

pub struct Frame {
    pub ppn: PhysPageNum,
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
    fn drop(&mut self) {}
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
    recycled: VecDeque<Frame>,
}

// impl FrameAllocator for FIFOFrameAllocator {
//     fn new() -> Self;
//     fn alloc(&mut self) -> Option<Frame>;
//     fn dealloc(&mut self, frame: Frame);
// }

pub fn frame_test() {
    unsafe { ((0x80480000) as *mut u8).write_volatile(255) };
    let b: u8 = unsafe { ((0x80480000) as *mut u8).read_volatile() };
    let a = Frame::new(PhysPageNum::from(0x80480));
    println!("b {}", b);
    let c: u8 = unsafe { ((0x80480000) as *mut u8).read_volatile() };
    println!("c {}", c);
    println!("frame test pass");
}
