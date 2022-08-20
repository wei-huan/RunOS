    .section .text.signaltrampoline
    .globl __sigreturn
    .align 2
__sigreturn:
    li a7, 139
    ecall
