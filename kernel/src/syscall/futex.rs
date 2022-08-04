use crate::cpu::{current_task, current_user_token};
use crate::mm::{
    translated_byte_buffer, translated_ref, translated_refmut, translated_str, UserBuffer,
};
use crate::syscall::{EAGAIN, ENOSYS, EPERM};
use crate::task::{block_current_and_run_next, unblock_task, TaskControlBlock, TaskStatus};
use crate::timer::USEC_PER_SEC;
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

pub struct FutexQueue {
    waiters: RwLock<usize>,
    chain: RwLock<VecDeque<Arc<TaskControlBlock>>>,
}

impl FutexQueue {
    pub fn new() -> Self {
        Self {
            waiters: RwLock::new(0),
            chain: RwLock::new(VecDeque::new()),
        }
    }
    pub fn waiters(&self) -> usize {
        *self.waiters.read()
    }
    pub fn waiters_inc(&self) {
        let mut waiters = self.waiters.write();
        *waiters += 1;
    }
    pub fn waiters_dec(&self) {
        let mut waiters = self.waiters.write();
        *waiters -= 1;
    }
}

pub static FUTEX_QUEUE: Lazy<RwLock<BTreeMap<usize, FutexQueue>>> =
    Lazy::new(|| RwLock::new(BTreeMap::new()));

pub fn sys_futex(
    uaddr: *const u32,
    op: usize,
    val: u32,
    timeout: *const u64,
    uaddr2: *const u32,
    val3: u32,
) -> isize {
    let mut flags = 0;
    let cmd = op & FUTEX_CMD_MASK;
    log::debug!(
        "sys_futex uaddr {:#X?}, op: {}, val: {}, timeout {:#X?}, uaddr2 {:#X?}, val3 {}",
        uaddr,
        cmd,
        val,
        timeout,
        uaddr2,
        val3
    );
    let token = current_user_token();
    if op & FUTEX_CLOCK_REALTIME != 0 {
        if cmd != FUTEX_WAIT {
            return -EPERM; // ENOSYS
        }
    }
    let ret = match cmd {
        FUTEX_WAIT => {
            let t = if timeout as usize != 0 {
                let sec = *translated_ref(token, timeout);
                let usec = *translated_ref(token, unsafe { timeout.add(1) });
                sec as usize * USEC_PER_SEC + usec as usize
            } else {
                usize::MAX // inf
            };
            futex_wait(uaddr as usize, val, t)
        }
        FUTEX_WAKE => futex_wake(uaddr as usize, val),
        FUTEX_REQUEUE => futex_requeue(uaddr as usize, val, uaddr2 as usize, timeout as u32),
        _ => ENOSYS,
    };
    return ret;
}

pub fn futex_wait(uaddr: usize, val: u32, timeout: usize) -> isize {
    // futex_wait_setup
    let flag = FUTEX_QUEUE.read().contains_key(&uaddr);
    let mut fq_writer = FUTEX_QUEUE.write();
    let fq = if flag {
        fq_writer.get(&uaddr).unwrap()
    } else {
        fq_writer.insert(uaddr, FutexQueue::new());
        fq_writer.get(&uaddr).unwrap()
    };
    fq.waiters_inc();
    let mut fq_lock = fq.chain.write();
    let token = current_user_token();
    let uval = translated_ref(token, uaddr as *const u32);
    // debug!(
    //     "futex_wait: uval: {:x?}, val: {:x?}, timeout: {}",
    //     uval, val, timeout
    // );
    if *uval != val {
        // Need to be atomic
        drop(fq_lock);
        fq.waiters_dec();
        if fq.waiters() == 0 {
            fq_writer.remove(&uaddr);
        }
        drop(fq_writer);
        return -EAGAIN;
    }

    // futex_wait_queue_me
    let task = current_task().unwrap();
    fq_lock.push_back(task.clone());
    drop(fq_lock);
    drop(fq_writer);

    block_current_and_run_next();
    return 0;
}

pub fn futex_wake(uaddr: usize, nr_wake: u32) -> isize {
    if !FUTEX_QUEUE.read().contains_key(&uaddr) {
        return 0;
    }
    let mut fq_writer = FUTEX_QUEUE.write();
    let fq = fq_writer.get(&uaddr).unwrap();
    let mut fq_lock = fq.chain.write();
    let waiters = fq.waiters();
    if waiters == 0 {
        return 0;
    }
    let nr_wake = nr_wake.min(waiters as u32);
    // debug!("futex_wake: uaddr: {:x?}, nr_wake: {:x?}", uaddr, nr_wake);

    let mut wakeup_queue = Vec::new();
    (0..nr_wake as usize).for_each(|_| {
        // 加入唤醒队列中，但需要等到释放完锁之后才能唤醒
        let task = fq_lock.pop_front().unwrap();
        wakeup_queue.push(task);
        fq.waiters_dec();
    });
    drop(fq_lock);

    if fq.waiters() == 0 {
        fq_writer.remove(&uaddr);
    }

    for task in wakeup_queue.into_iter() {
        unblock_task(task);
    }
    return nr_wake as isize;
}

pub fn futex_requeue(uaddr: usize, nr_wake: u32, uaddr2: usize, nr_limit: u32) -> isize {
    if !FUTEX_QUEUE.read().contains_key(&uaddr) {
        return 0;
    }
    let flag2 = FUTEX_QUEUE.read().contains_key(&uaddr2);

    let mut fq_writer = FUTEX_QUEUE.write();
    let fq = fq_writer.get(&uaddr).unwrap();
    let mut fq_lock = fq.chain.write();
    let waiters = fq.waiters();
    if waiters == 0 {
        return 0;
    }
    let nr_wake = nr_wake.min(waiters as u32);

    let mut wakeup_q = Vec::new();
    let mut requeue_q = Vec::new();

    (0..nr_wake as usize).for_each(|_| {
        // prepare to wake-up
        let task = fq_lock.pop_front().unwrap();
        wakeup_q.push(task);
        fq.waiters_dec();
    });

    let nr_limit = nr_limit.min(fq.waiters() as u32);
    (0..nr_limit as usize).for_each(|_| {
        // prepare to requeue
        let task = fq_lock.pop_front().unwrap();
        requeue_q.push(task);
        fq.waiters_dec();
    });
    drop(fq_lock);

    // wakeup sleeping tasks
    if fq.waiters() == 0 {
        fq_writer.remove(&uaddr);
    }
    for task in wakeup_q.into_iter() {
        unblock_task(task);
    }

    // requeue...
    if nr_limit == 0 {
        return nr_wake as isize;
    }

    let fq2 = if flag2 {
        fq_writer.get(&uaddr2).unwrap()
    } else {
        fq_writer.insert(uaddr2, FutexQueue::new());
        fq_writer.get(&uaddr2).unwrap()
    };

    let mut fq2_lock = fq2.chain.write();

    for task in requeue_q.into_iter() {
        fq2_lock.push_back(task);
        fq2.waiters_inc();
    }

    return nr_wake as isize;
}
