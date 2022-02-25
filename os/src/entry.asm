    .section .text.entry
    .globl _start
_start:
    mv tp, a0
    add t0, a0, 1
    slli t0, t0, 16
    la sp, boot_stack
    add sp, sp, t0
    call os_main

    .section .data.stack
boot_stack:
    .space 4096 * 16 * 4
boot_stack_top:
