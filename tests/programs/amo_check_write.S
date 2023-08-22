.global _start
_start:

    la a0, n0
    li a1, 1
    amoadd.w a2, a1, (a0)

    ld a3, (a0)
    bne a3, a1, fail
    bne a2, zero, fail

done:
    li a0, 0
    li a7, 93
    ecall
fail:
    li a0, 1
    li a7, 93
    ecall

.section .data
n0:
    .dword 0
