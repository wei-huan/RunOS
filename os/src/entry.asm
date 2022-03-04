    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top
    call os_main

    .section .data.stack
boot_stack:
    .space 4096 * 16 * 4
boot_stack_top:


