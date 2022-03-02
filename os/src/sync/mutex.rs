// use core::cell::UnsafeCell;
// use core::hint::spin_loop;
// use core::ops::{Deref, DerefMut};
// use core::sync::atomic::{AtomicBool, Ordering};

// pub struct Mutex<T> {
//     lock: AtomicBool,
//     data: UnsafeCell<T>,
// }

// unsafe impl<T> Sync for Mutex<T> {}
// unsafe impl<T> Send for Mutex<T> {}

// pub struct MutexGuard<'a, T> {
//     lock: &'a AtomicBool,
//     data: &'a mut T,
// }

// impl<'a, T> Deref for MutexGuard<'a, T> {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         self.data
//     }
// }

// impl<'a, T> DerefMut for MutexGuard<'a, T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         self.data
//     }
// }

// impl<T> Mutex<T> {
//     pub fn new(data: T) -> Mutex<T> {
//         Mutex {
//             lock: AtomicBool::new(false),
//             data: UnsafeCell::new(data),
//         }
//     }
//     pub fn lock(&self) -> MutexGuard<T> {
//         while self
//             .lock
//             .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
//             == Ok(false)
//         {
//             spin_loop();
//         }
//         MutexGuard {
//             lock: &self.lock,
//             data: unsafe { &mut *self.data.get() },
//         }
//     }
//     fn unlock(&self) {
//         self.lock.store(false, Ordering::Release);
//     }
// }

// impl<'a, T> Drop for MutexGuard<'a, T> {
//     fn drop(&mut self) {
//         self.lock.unlock()
//     }
// }

// SPDX-License-Identifier: MPL-2.0
// SPDX-FileCopyrightText: 2021 The vanadinite developers
//
// This Source Code Form is subject to the terms of the Mozilla Public License,
// v. 2.0. If a copy of the MPL was not distributed with this file, You can
// obtain one at https://mozilla.org/MPL/2.0/.
pub trait DeadlockDetection {
    fn would_deadlock(metadata: usize) -> bool;
    fn gather_metadata() -> usize;
}

pub struct NoCheck;

impl DeadlockDetection for NoCheck {
    fn would_deadlock(_: usize) -> bool {
        false
    }

    fn gather_metadata() -> usize {
        0
    }
}

pub struct Immediate;

impl DeadlockDetection for Immediate {
    fn would_deadlock(_: usize) -> bool {
        true
    }

    fn gather_metadata() -> usize {
        0
    }
}

use core::{
    cell::UnsafeCell,
    marker::PhantomData,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

pub struct Mutex<T: Send, D: DeadlockDetection = NoCheck> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
    deadlock_detection: PhantomData<D>,
    deadlock_metadata: AtomicUsize,
}

impl<T: Send, D: DeadlockDetection> Mutex<T, D> {
    pub const fn new(data: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
            deadlock_detection: PhantomData,
            deadlock_metadata: AtomicUsize::new(0),
        }
    }

    pub fn with_lock<U>(&self, f: impl FnOnce(&mut T) -> U) -> U {
        self.acquire_lock();
        let ret = f(unsafe { &mut *self.data.get() });
        self.unlock();

        ret
    }

    #[track_caller]
    pub fn lock(&self) -> MutexGuard<'_, T, D> {
        self.acquire_lock();
        MutexGuard { lock: self }
    }

    pub fn try_lock(&self) -> Option<MutexGuard<'_, T, D>> {
        match self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
        {
            Ok(_) => {
                self.deadlock_metadata
                    .store(D::gather_metadata(), Ordering::Release);
                Some(MutexGuard { lock: self })
            }
            Err(_) => None,
        }
    }

    #[track_caller]
    fn acquire_lock(&self) {
        let mut spin_check_count = 100;

        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            if spin_check_count != 0
                && D::would_deadlock(self.deadlock_metadata.load(Ordering::Acquire))
            {
                panic!("Deadlock detected");
            }

            spin_check_count -= 1;
        }

        self.deadlock_metadata
            .store(D::gather_metadata(), Ordering::Release);
    }

    fn unlock(&self) {
        self.lock.store(false, Ordering::Release);
    }
}

unsafe impl<T: Send, D: DeadlockDetection> Send for Mutex<T, D> {}
unsafe impl<T: Send, D: DeadlockDetection> Sync for Mutex<T, D> {}

impl<T: Send, D: DeadlockDetection> core::fmt::Debug for Mutex<T, D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Mutex").finish_non_exhaustive()
    }
}

pub struct MutexGuard<'a, T: Send, D: DeadlockDetection> {
    lock: &'a Mutex<T, D>,
}

impl<T: Send, D: DeadlockDetection> core::ops::Deref for MutexGuard<'_, T, D> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: Send, D: DeadlockDetection> core::ops::DerefMut for MutexGuard<'_, T, D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T: Send, D: DeadlockDetection> Drop for MutexGuard<'_, T, D> {
    fn drop(&mut self) {
        self.lock.unlock()
    }
}
