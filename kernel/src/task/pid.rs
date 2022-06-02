use super::recycle_allocator::RecycleAllocator;
use lazy_static::*;
use spin::Mutex;

lazy_static! {
    static ref PID_ALLOCATOR: Mutex<RecycleAllocator> = Mutex::new(RecycleAllocator::new());
}

pub struct PidHandle(pub usize);

pub fn pid_alloc() -> PidHandle {
    PidHandle(PID_ALLOCATOR.lock().alloc())
}

impl Drop for PidHandle {
    fn drop(&mut self) {
        // println!("pid {} recycled", self.0);
        PID_ALLOCATOR.lock().dealloc(self.0);
    }
}
