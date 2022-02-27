use crate::cpus::{CPUS};
use crate::sync::{intr_get, IntrLock};
use core::cell::UnsafeCell;
use core::ops::{Deref, Drop};
use core::sync::atomic::{AtomicBool, Ordering};

// pub struct AtomicBool { /* fields omitted */ }
// pub fn compare_exchange(
//     &self,
//     current: bool,
//     new: bool,
//     success: Ordering,
//     failure: Ordering
// ) -> Result<bool, bool>

pub struct MutexGuard<'a, T: 'a> {
    mutex: &'a Mutex<T>,
    _intr_lock: IntrLock<'a>,
}

unsafe impl<T> Sync for Mutex<T> {}

unsafe impl<T> Send for Mutex<T> {}

impl<'a, T: 'a> MutexGuard<'a, T> {
    // Returns a reference to the original 'Mutex' object.
    pub fn mutex(&self) -> &'a Mutex<T> {
        self.mutex
    }

    pub fn is_holding(&self) -> bool {
        assert!(!intr_get(), "interrupts enabled");
        unsafe { self.mutex.is_holding() }
    }
}

impl<'a, T: 'a> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        assert!(self.is_holding(), "release {}", self.mutex.name);

        self.mutex.locked.store(false , Ordering::Release);
    }
}

impl<'a, T: 'a> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}


#[derive(Debug)]
pub struct Mutex<T> {
    name: &'static str,  // Name of lock
    locked: AtomicBool,  // Is the lock held?
    data: UnsafeCell<T>, // actual data
}

impl<T> Mutex<T> {
    pub const fn new(value: T, name: &'static str) -> Mutex<T> {
        Mutex {
            name: name,
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        let _intr_lock = CPUS.intr_lock(); // disable interrupts to avoid deadlock.

        loop {
            if !self
                .locked
                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_err()
            {
                break MutexGuard {
                    mutex: self,
                    _intr_lock,
                };
            }
            core::hint::spin_loop()
        }
    }

    // Check whether this cpu is holding the lock.
    // Interrupts must be off.
    pub unsafe fn is_holding(&self) -> bool {
        self.locked.load(Ordering::Relaxed) == CPUS.my_cpu().int_status
    }

    pub fn unlock(guard: MutexGuard<'_, T>) -> &'_ Mutex<T> {
        guard.mutex()
    }

    pub unsafe fn get_mut(&self) -> &mut T {
        &mut *self.data.get()
    }
}
