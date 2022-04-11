    .section .text.entry
    .globl _start
_start:
    mv tp, a0
    add t0, a0, 1
    slli t0, t0, 16
    la sp, boot_stack
    add sp, sp, t0
    call os_main

#     .section .text
#     .globl _warm_start
# _warm_start:
#     mv tp, a0
#     add t0, a0, 1
#     slli t0, t0, 16
#     la sp, boot_stack
#     add sp, sp, t0
#     call os_sub_main

    .section .bss.stack
boot_stack:
    .globl boot_stack
    # 64K 启动栈大小 * CPU_NUMS
    .space 4096 * 16 * 4
boot_stack_top:
    .globl boot_stack_top
