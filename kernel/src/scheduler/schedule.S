# .altmacro
# .macro SAVE_SNT n
#     sd s\n, (\n+3)*8(a0)
# .endm
#     .section .text
#     .globl __save_current_cx
#     .align 4
# __save_current_cx:
#     # __save_current_cx(
#     #     current_task_cx_ptr: *mut TaskContext,
#     # )
#     # save kernel stack of current task
#     sd sp, 8(a0)
#     # save ra & s0~s11 of current execution
#     sd ra, 0(a0)
#     .set n, 0
#     .rept 12
#         SAVE_SNT %n
#         .set n, n + 1
#     .endr
#     # save sstatus of current execution
#     csrr s0, sstatus
#     sd s0, 16(a0)
#     ret

.altmacro
.macro LOAD_SNT n
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
        LOAD_SNT %n
        .set n, n + 1
    .endr
    # restore kernel stack of next task
    ld sp, 8(a0)
    ret
