    .section .text.entry
    .globl _start
    .extern CPU_NUMS
_start:
    mv tp, a0
    add t0, a0, 1
    slli t0, t0, 13
    la sp, boot_stack
    add sp, sp, t0
    call os_main

    .section .data.stack
boot_stack:
    # 8K 启动栈大小 * CPU_NUMS
    .space 4096 * 2 * 8
boot_stack_top:
