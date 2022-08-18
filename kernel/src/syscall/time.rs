use crate::{
    cpu::{current_task, current_user_token},
    mm::{translated_ref, translated_refmut},
    syscall::EINVAL,
    timer::ITimerVal,
};

const ITIMER_REAL: i32 = 0;
const ITIMER_VIRTUAL: i32 = 1;
const ITIMER_PROF: i32 = 2;

pub fn sys_setitimer(which: i32, new_value: *const ITimerVal, old_value: *mut ITimerVal) -> isize {
    // log::debug!(
    //     "sys_setitimer which: {}, new_value: {:#X?}, old_value: {:#X?}",
    //     which,
    //     new_value as usize,
    //     old_value as usize
    // );
    let token = current_user_token();
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    let ret: isize = match which {
        ITIMER_REAL..=ITIMER_PROF => {
            if old_value as usize != 0 {
                *translated_refmut(token, old_value) = inner.timer[which as usize];
            }
            if new_value as usize != 0 {
                inner.timer[which as usize] = *translated_ref(token, new_value);
            }
            0
        }
        _ => -EINVAL,
    };
    ret
}
