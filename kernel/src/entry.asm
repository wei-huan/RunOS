    .section .text.entry
    .globl _start
_start:
    mv tp, a0
    add t0, a0, 1
    slli t0, t0, 13
    la sp, boot_stack
    add sp, sp, t0
    call os_main

    .section .bss.stack
    .globl boot_stack
    .globl boot_stack_top
boot_stack:
    .globl boot_stack
    # 8K 启动栈大小 * CPU_NUMS
    .space 4096 * 2 * 2
boot_stack_top:
    .globl boot_stack_top
