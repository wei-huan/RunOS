#![allow(unused)]

use core::arch::asm;

const SBI_SET_TIMER_EID: usize = 0;
const SBI_CONSOLE_PUTCHAR_EID: usize = 1;
const SBI_CONSOLE_GETCHAR_EID: usize = 2;
const SBI_CLEAR_IPI_EID: usize = 3;
const SBI_SEND_IPI_EID: usize = 4;
const SBI_REMOTE_FENCE_I_EID: usize = 5;
const SBI_REMOTE_SFENCE_VMA_EID: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID_EID: usize = 7;
const SBI_SHUTDOWN_EID: usize = 8;

const SBI_SET_TIMER_FID: usize = 0;
const SBI_CONSOLE_PUTCHAR_FID: usize = 0;
const SBI_CONSOLE_GETCHAR_FID: usize = 0;
const SBI_CLEAR_IPI_FID: usize = 0;
const SBI_SEND_IPI_FID: usize = 0;
const SBI_REMOTE_FENCE_I_FID: usize = 0;
const SBI_REMOTE_SFENCE_VMA_FID: usize = 0;
const SBI_REMOTE_SFENCE_VMA_ASID_FID: usize = 0;
const SBI_SHUTDOWN_FID: usize = 0;

#[inline(always)]
fn opensbi_call(
    ext: usize,
    fid: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> (usize, usize) {
    let mut ret0 = arg0;
    let mut ret1 = arg1;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") arg0 => ret0,
            inlateout("x11") arg1 => ret1,
            in("x12") arg2,
            in("x13") arg3,
            in("x14") arg4,
            in("x15") arg5,
            in("x16") fid,
            in("x17") ext,
        );
    }
    (ret0, ret1)
}

pub fn console_putchar(c: usize) {
    opensbi_call(SBI_CONSOLE_PUTCHAR_EID, SBI_CONSOLE_PUTCHAR_FID, c, 0, 0, 0, 0, 0);
}

pub fn console_getchar() -> usize {
    opensbi_call(SBI_CONSOLE_GETCHAR_EID, SBI_CONSOLE_GETCHAR_FID, 0, 0, 0, 0, 0, 0).1
}

pub fn shutdown() -> ! {
    println!("I am dead");
    opensbi_call(SBI_SHUTDOWN_EID, SBI_SHUTDOWN_FID, 0, 0, 0, 0, 0, 0);
    panic!("It should shutdown!");
}

pub fn clear_ipi() -> (usize, usize) {
    opensbi_call(SBI_CLEAR_IPI_EID, SBI_CLEAR_IPI_FID, 0, 0, 0, 0, 0, 0)
}

pub fn send_ipi(cpus_mask: usize) -> (usize, usize){
    opensbi_call(SBI_SEND_IPI_EID, SBI_SEND_IPI_FID, cpus_mask, 0, 0, 0, 0, 0)
}
