use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};
use core::hint::spin_loop;
use core::ops::{Deref, DerefMut};

pub struct Mutex<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> {}
unsafe impl<T> Send for Mutex<T> {}

pub struct MutexGuard<'a, T> {
    lock: &'a AtomicBool,
    data: &'a mut T,
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, T>DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

impl<T> Mutex<T> {
    pub fn new(data: T) -> Mutex<T> {
        Mutex {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }
    pub fn lock(&self) -> MutexGuard<T> {
        while self.lock.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) == Ok(false) {
            spin_loop();
        }
        MutexGuard {
            lock: &self.lock,
            data: unsafe { &mut *self.data.get() }
        }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.store(false, Ordering::Release);
    }
}