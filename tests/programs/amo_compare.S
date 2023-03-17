.global _start
_start:

test_amominu_w:
    la a0, n0
    li a2, 0x00000001
    sw a2, (a0)
    li a3, 0x8000000000000000

    amominu.w a4, a3, (a0)
    li a5, 0x0000000000000001
    bne a4, a5, fail
    lw a6, (a0)
    li a7, 0x0000000000000000
    bne a6, a7, fail

test_amomaxu_w:
    la a0, n0
    li a2, 0x00000001
    sw a2, (a0)
    li a3, 0x8000000000000000

    amomaxu.w a4, a3, (a0)
    li a5, 0x0000000000000001
    bne a4, a5, fail
    lw a6, (a0)
    li a7, 0x0000000000000001
    bne a6, a7, fail

test_amomin_w:
    la a0, n0
    li a2, 0x00000001
    sw a2, (a0)
    li a3, 0x0000000080000000

    amomin.w a4, a3, (a0)
    li a5, 0x0000000000000001
    bne a4, a5, fail
    lw a6, (a0)
    li a7, 0xffffffff80000000
    bne a6, a7, fail

test_amomax_w:
    la a0, n0
    li a2, 0x00000001
    sw a2, (a0)
    li a3, 0x0000000080000000

    amomax.w a4, a3, (a0)
    li a5, 0x0000000000000001
    bne a4, a5, fail
    lw a6, (a0)
    li a7, 0x0000000000000001
    bne a6, a7, fail

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
