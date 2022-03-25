    .section .text.entry
    .globl _start
_start:
    mv tp, a0
    add t0, a0, 1
    slli t0, t0, 15
    la sp, boot_stack
    add sp, sp, t0
    call os_main

    .section .data.stack
boot_stack:
    # 16K 启动栈大小 * CPU_NUMS
    .space 4096 * 8 * 8
boot_stack_top:
