.altmacro
.macro SAVE_SN n
    sd s\n, (\n+3)*8(a0)
.endm
.macro LOAD_SN n
    ld s\n, (\n+3)*8(a1)
.endm
    .section .text
    .globl __switch
    .align 4
__switch:
    # __switch(
    #     current_task_cx_ptr: *mut TaskContext,
    #     next_task_cx_ptr: *const TaskContext
    # )
    # save kernel stack of current task
    sd sp, 8(a0)
    # save ra & s0~s11 of current execution
    sd ra, 0(a0)
    .set n, 0
    .rept 12
        SAVE_SN %n
        .set n, n + 1
    .endr
    # save sstatus of current execution
    csrr s0, sstatus
    sd s0, 16(a0)
    # restore sstatus of current execution
    ld s0, 16(a1)
    csrw sstatus, s0
    # restore ra & s0~s11 of next execution
    ld ra, 0(a1)
    .set n, 0
    .rept 12
        LOAD_SN %n
        .set n, n + 1
    .endr
    # restore kernel stack of next task
    ld sp, 8(a1)
    ret
