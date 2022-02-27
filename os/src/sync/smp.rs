use core::cell::{Cell, UnsafeCell};
use core::sync::atomic::{AtomicBool, Ordering};

#[repr(C, align(1))]
// pub struct AtomicBool { /* fields omitted */ }
// pub fn compare_exchange(
//     &self,
//     current: bool,
//     new: bool,
//     success: Ordering,
//     failure: Ordering
// ) -> Result<bool, bool>

pub struct Mutex<T> {
    locked: AtomicBool, // Is the lock held?
    data: UnsafeCell<T>, // actual data
}

pub struct MutexGuard<'a, T: 'a> {
    mutex: &'a Mutex<T>,
}

impl<T> Mutex<T> {
    pub fn lock(&self) -> MutexGuard<'_, T> {
        loop {
            if !self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
                break MutexGuard {
                    mutex: self
                }
            }
        }
    }
}
