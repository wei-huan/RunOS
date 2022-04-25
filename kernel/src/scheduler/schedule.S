.altmacro
.macro LOAD_S n
    ld s\n, (\n+3)*8(a0)
.endm

    .section .text
    .globl __schedule_new
    .align 4
__schedule_new:
    # __schedule_new(
    #     next_task_cx_ptr: *const TaskContext
    # )
    # restore sstatus of current execution
    ld s0, 16(a0)
    csrw sstatus, s0
    # restore ra & s0~s11 of next execution
    ld ra, 0(a0)
    .set n, 0
    .rept 12
        LOAD_S %n
        .set n, n + 1
    .endr
    # restore kernel stack of next task
    ld sp, 8(a0)
    fence.i
    ret
