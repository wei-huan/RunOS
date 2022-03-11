.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm
    # .section .text.trampoline
    # .globl __user_alltraps
    # .globl __user_restore
    # .align 2
# __user_alltraps:
# __user_restore:

    .extern kernel_trap_handler
    .section .text
    .globl __super_alltraps
    .globl __super_restore
    .align 2

__super_alltraps:
    .set n, 1
    .rept 31
        SAVE_GP %n
        .set n, n+1
    .endr
    j kernel_trap_handler

__super_restore:
    .set n, 1
    .rept 31
        LOAD_GP %n
        .set n, n+1
    .endr
    ret