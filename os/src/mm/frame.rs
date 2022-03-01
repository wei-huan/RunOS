extern crate alloc;
use super::address::{PhysPageNum};
use crate::config::MEMORY_END;
use alloc::collections::VecDeque;
use lazy_static::*;

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
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn dealloc(&mut self, ppn: PhysPageNum);
}

struct FIFOFrameAllocator {
    start: usize,
    end: usize,
    current: usize,
    recycled: VecDeque<PhysPageNum>,
}

impl FrameAllocator for FIFOFrameAllocator {
    fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            current: 0,
            recycled: VecDeque::<PhysPageNum>::new(),
        }
    }

    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(ppn) = self.recycled.pop_front() {
            Some(ppn.into())
        } else {
            if self.current < self.end {
                self.current += 1;
                Some((self.current - 1).into())
            } else {
                return None
            }
        }
    }

    fn dealloc(&mut self, ppn: PhysPageNum) {
        let a: usize = ppn.into();
        if a < self.start {
            panic!("ppn smaller than start");
        } else {
            self.recycled.push_back(ppn);
        }
    }
}

pub fn frame_test() {
    unsafe { ((0x80480000) as *mut u8).write_volatile(255) };
    let b: u8 = unsafe { ((0x80480000) as *mut u8).read_volatile() };
    Frame::new(PhysPageNum::from(0x80480));
    assert_eq!(b, 255);
    let c: u8 = unsafe { ((0x80480000) as *mut u8).read_volatile() };
    assert_eq!(c, 0);
    println!("frame test pass");
}

type FrameAllocatorImpl = FIFOFrameAllocator;

lazy_static !{
    pub static ref FRAME_ALLOCATOR: Mutex<FrameAllocatorImpl> = Mutex::new(FrameAllocatorImpl);
}

pub fn frame_alloc_test() {

}
