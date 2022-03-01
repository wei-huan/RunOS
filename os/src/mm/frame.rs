extern crate alloc;
extern crate spin;
use super::address::{PhysAddr, PhysPageNum};
use crate::config::MEMORY_END;
// use crate::sync::Mutex;
use core::fmt::{self, Debug, Formatter};
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use lazy_static::*;
use spin::Mutex;

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

impl Debug for Frame {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Frame:PPN={:#x}", self.ppn.0))
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        frame_dealloc(self.ppn);
    }
}

trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn dealloc(&mut self, ppn: PhysPageNum);
}

pub struct FIFOFrameAllocator {
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
                return None;
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

impl FIFOFrameAllocator {
    pub fn init(&mut self, start: PhysPageNum, end: PhysPageNum) {
        self.start = start.0;
        self.end = end.0;
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

lazy_static! {
    pub static ref FRAME_ALLOCATOR: Mutex<FrameAllocatorImpl> =
        Mutex::new(FrameAllocatorImpl::new());
}

pub fn init_frame_allocator() {
    extern "C" {
        fn ekernel();
    }
    FRAME_ALLOCATOR.lock().init(
        PhysAddr::from(ekernel as usize).ceil(),
        PhysAddr::from(MEMORY_END).floor(),
    );
}

pub fn frame_alloc() -> Option<Frame>{
    FRAME_ALLOCATOR.lock().alloc().map(Frame::new)
}

pub fn frame_dealloc(ppn: PhysPageNum) {
    FRAME_ALLOCATOR.lock().dealloc(ppn);
}

#[allow(unused)]
pub fn frame_allocator_test() {
    let mut v: Vec<Frame> = Vec::new();
    for i in 0..10 {
        let frame = frame_alloc().unwrap();
        println!("{:?} i: {}", frame, i);
        // v.push(frame);
    }
    v.clear();
    for i in 0..5 {
        let frame = frame_alloc().unwrap();
        println!("{:?}", frame);
        v.push(frame);
    }
    drop(v);
    println!("frame_allocator_test passed!");
}
