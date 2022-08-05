use crate::cpu::{current_task, current_user_token};
use crate::mm::{
    translated_byte_buffer, translated_ref, translated_refmut, translated_str, UserBuffer,
};
use crate::syscall::{EAGAIN, ENOSYS, EPERM};
use crate::task::{block_current_and_run_next, unblock_task, TaskControlBlock, TaskStatus};
use crate::timer::{TimeSpec, NSEC_PER_SEC};
use alloc::collections::{BTreeMap, VecDeque};
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::{Lazy, RwLock};

const FUTEX_WAIT: usize = 0;
const FUTEX_WAKE: usize = 1;
const FUTEX_FD: usize = 2;
const FUTEX_REQUEUE: usize = 3;
const FUTEX_CMP_REQUEUE: usize = 4;
const FUTEX_WAKE_OP: usize = 5;
const FUTEX_LOCK_PI: usize = 6;
const FUTEX_UNLOCK_PI: usize = 7;
const FUTEX_TRYLOCK_PI: usize = 8;
const FUTEX_WAIT_BITSET: usize = 9;
const FUTEX_WAKE_BITSET: usize = 10;
const FUTEX_WAIT_REQUEUE_PI: usize = 11;
const FUTEX_CMP_REQUEUE_PI: usize = 12;

const FUTEX_PRIVATE_FLAG: usize = 128;
const FUTEX_CLOCK_REALTIME: usize = 256;
const FUTEX_CMD_MASK: usize = !(FUTEX_PRIVATE_FLAG | FUTEX_CLOCK_REALTIME);

const FUTEX_WAIT_PRIVATE: usize = (FUTEX_WAIT | FUTEX_PRIVATE_FLAG);
const FUTEX_WAKE_PRIVATE: usize = (FUTEX_WAKE | FUTEX_PRIVATE_FLAG);
const FUTEX_REQUEUE_PRIVATE: usize = (FUTEX_REQUEUE | FUTEX_PRIVATE_FLAG);
const FUTEX_CMP_REQUEUE_PRIVATE: usize = (FUTEX_CMP_REQUEUE | FUTEX_PRIVATE_FLAG);
const FUTEX_WAKE_OP_PRIVATE: usize = (FUTEX_WAKE_OP | FUTEX_PRIVATE_FLAG);
const FUTEX_LOCK_PI_PRIVATE: usize = (FUTEX_LOCK_PI | FUTEX_PRIVATE_FLAG);
const FUTEX_UNLOCK_PI_PRIVATE: usize = (FUTEX_UNLOCK_PI | FUTEX_PRIVATE_FLAG);
const FUTEX_TRYLOCK_PI_PRIVATE: usize = (FUTEX_TRYLOCK_PI | FUTEX_PRIVATE_FLAG);
const FUTEX_WAIT_BITSET_PRIVATE: usize = (FUTEX_WAIT_BITSET | FUTEX_PRIVATE_FLAG);
const FUTEX_WAKE_BITSET_PRIVATE: usize = (FUTEX_WAKE_BITSET | FUTEX_PRIVATE_FLAG);
const FUTEX_WAIT_REQUEUE_PI_PRIVATE: usize = (FUTEX_WAIT_REQUEUE_PI | FUTEX_PRIVATE_FLAG);
const FUTEX_CMP_REQUEUE_PI_PRIVATE: usize = (FUTEX_CMP_REQUEUE_PI | FUTEX_PRIVATE_FLAG);

pub struct FutexQueue(RwLock<VecDeque<Arc<TaskControlBlock>>>);

impl FutexQueue {
    fn new() -> Self {
        Self {
            0: RwLock::new(VecDeque::new()),
        }
    }
}

pub static FUTEX_QUEUE_MAP: Lazy<RwLock<BTreeMap<usize, FutexQueue>>> =
    Lazy::new(|| RwLock::new(BTreeMap::new()));

pub fn sys_futex(
    uaddr: *const u32,
    futex_op: usize,
    val: u32,
    timeout: *const TimeSpec,
    uaddr2: *const u32,
    val3: u32,
) -> isize {
    let mut flags = 0;
    let cmd = futex_op & FUTEX_CMD_MASK;
    log::debug!(
        "sys_futex uaddr {:#X}, op: {}, val: {}",
        uaddr as usize,
        cmd,
        val,
    );
    let token = current_user_token();
    if futex_op & FUTEX_CLOCK_REALTIME != 0 && cmd != FUTEX_WAIT {
        return -EPERM;
    }
    let ret = match cmd {
        FUTEX_WAIT => {
            let timeout = if timeout as usize != 0 {
                let timespec = *translated_ref(token, timeout);
                timespec.tv_sec * NSEC_PER_SEC + timespec.tv_nsec as usize
            } else {
                usize::MAX
            };
            futex_wait(uaddr as usize, val, timeout)
        }
        FUTEX_FD => {
            log::error!("FUTEX_FD not implement yet");
            EPERM
        }
        FUTEX_WAKE => futex_wake(uaddr as usize, val),
        FUTEX_REQUEUE => futex_requeue(uaddr as usize, val, uaddr2 as usize, timeout as u32),
        _ => EPERM,
    };
    return ret;
}

pub fn futex_wait(uaddr: usize, val: u32, timeout: usize) -> isize {
    let token = current_user_token();
    let uval = translated_ref(token, uaddr as *const u32);
    if *uval != val {
        return -EAGAIN;
    }

    let is_have_queue = FUTEX_QUEUE_MAP.read().contains_key(&uaddr);
    let mut futex_write = FUTEX_QUEUE_MAP.write();
    let futex_queue = if is_have_queue {
        futex_write.get(&uaddr).unwrap()
    } else {
        futex_write.insert(uaddr, FutexQueue::new());
        futex_write.get(&uaddr).unwrap()
    };
    // log::debug!(
    //     "futex_wait: uval: {:x?}, val: {:x?}, timeout: {}",
    //     uval, val, timeout
    // );
    let mut queue_write = futex_queue.0.write();
    let task = current_task().unwrap();
    queue_write.push_back(task.clone());
    drop(queue_write);
    drop(futex_write);
    block_current_and_run_next();
    return 0;
}

pub fn futex_wake(uaddr: usize, nr_wake: u32) -> isize {
    if !FUTEX_QUEUE_MAP.read().contains_key(&uaddr) {
        return 0;
    }
    let mut futex_write = FUTEX_QUEUE_MAP.write();
    let mut queue_write = futex_write.get(&uaddr).unwrap().0.write();

    let mut wakeup_queue = Vec::new();
    (0..nr_wake as usize).for_each(|_| {
        if let Some(task) = queue_write.pop_front() {
            wakeup_queue.push(task);
        }
    });
    drop(queue_write);

    for task in wakeup_queue.into_iter() {
        unblock_task(task);
    }
    return nr_wake as isize;
}

pub fn futex_requeue(uaddr: usize, nr_wake: u32, uaddr2: usize, nr_limit: u32) -> isize {
    if !FUTEX_QUEUE_MAP.read().contains_key(&uaddr) {
        return 0;
    }

    let mut futex_write = FUTEX_QUEUE_MAP.write();
    let mut queue_write = futex_write.get(&uaddr).unwrap().0.write();

    let mut wakeup_queue = Vec::new();
    let mut requeue_queue = Vec::new();

    (0..nr_wake as usize).for_each(|_| {
        if let Some(task) = queue_write.pop_front() {
            wakeup_queue.push(task);
        }
    });

    (0..nr_limit as usize).for_each(|_| {
        if let Some(task) = queue_write.pop_front() {
            requeue_queue.push(task);
        }
    });
    drop(queue_write);

    // wakeup sleeping tasks
    for task in wakeup_queue.into_iter() {
        unblock_task(task);
    }

    // requeue...
    if nr_limit == 0 {
        return nr_wake as isize;
    }

    let flag2 = FUTEX_QUEUE_MAP.read().contains_key(&uaddr2);
    let fq2 = if flag2 {
        futex_write.get(&uaddr2).unwrap()
    } else {
        futex_write.insert(uaddr2, FutexQueue::new());
        futex_write.get(&uaddr2).unwrap()
    };

    let mut queue2_write = fq2.0.write();

    for task in requeue_queue.into_iter() {
        queue2_write.push_back(task);
    }

    return nr_wake as isize;
}
