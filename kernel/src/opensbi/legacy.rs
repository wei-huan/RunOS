/// Legacy Extension
use crate::opensbi::opensbi_call;
use crate::opensbi::SBIResult;
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

#[allow(unused)]
pub fn set_timer(timer: usize) {
    opensbi_call(SBI_SET_TIMER_EID, SBI_SET_TIMER_FID, timer, 0, 0, 0, 0, 0);
}

#[allow(unused)]
pub fn console_putchar(c: usize) {
    opensbi_call(
        SBI_CONSOLE_PUTCHAR_EID,
        SBI_CONSOLE_PUTCHAR_FID,
        c,
        0,
        0,
        0,
        0,
        0,
    );
}

// pub fn console_getchar() -> usize {
//     let res = opensbi_call(
//         SBI_CONSOLE_GETCHAR_EID,
//         SBI_CONSOLE_GETCHAR_FID,
//         0,
//         0,
//         0,
//         0,
//         0,
//         0,
//     ).unwrap();
//     match res {
//         Failed => 0,
//         n => n as usize,
//     }
// }

pub fn console_getchar() -> usize {
    let mut err: isize = 0;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") err => err,
            in("x16") SBI_CONSOLE_GETCHAR_FID,
            in("x17") SBI_CONSOLE_GETCHAR_EID,
        );
    }
    // println!("dead");
    match err {
        -1 => 0,
        n => n as usize,
    }
}

#[allow(unused)]
pub fn clear_ipi() -> SBIResult<usize> {
    opensbi_call(SBI_CLEAR_IPI_EID, SBI_CLEAR_IPI_FID, 0, 0, 0, 0, 0, 0)
}

#[allow(unused)]
pub fn send_ipi(hart_id: usize) -> SBIResult<usize> {
    opensbi_call(SBI_SEND_IPI_EID, SBI_SEND_IPI_FID, hart_id, 0, 0, 0, 0, 0)
}

#[allow(unused)]
pub fn remote_fence_i(hartid: usize) -> SBIResult<usize> {
    opensbi_call(
        SBI_REMOTE_FENCE_I_EID,
        SBI_REMOTE_FENCE_I_FID,
        hartid,
        0,
        0,
        0,
        0,
        0,
    )
}

#[allow(unused)]
pub fn remote_sfence_vma(hartid: usize, start: usize, size: usize) -> SBIResult<usize> {
    opensbi_call(
        SBI_REMOTE_SFENCE_VMA_EID,
        SBI_REMOTE_SFENCE_VMA_FID,
        hartid,
        start,
        size,
        0,
        0,
        0,
    )
}

#[allow(unused)]
pub fn remote_sfence_vma_asid(
    hartid: usize,
    start: usize,
    size: usize,
    asid: usize,
) -> SBIResult<usize> {
    opensbi_call(
        SBI_REMOTE_SFENCE_VMA_ASID_EID,
        SBI_REMOTE_SFENCE_VMA_ASID_FID,
        hartid,
        start,
        size,
        asid,
        0,
        0,
    )
}

#[allow(unused)]
pub fn shutdown() -> ! {
    println!("I am dead");
    opensbi_call(SBI_SHUTDOWN_EID, SBI_SHUTDOWN_FID, 0, 0, 0, 0, 0, 0);
    panic!("It should shutdown!");
}
