    .section .text
    .globl _start
_start:
    li a3, 123
    li a2, 0x3fffc
    sd a3, 0(a2)
    li a0, 0
    li a7, 93
    ecall
