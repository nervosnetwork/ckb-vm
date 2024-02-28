.global _start
_start:
_test_case_sc_after_snapshot:
    la a0, n0         # a0 holds address of memory location n0
    lr.d a2, (a0)
    nop
    nop
    nop
    nop
    sc.d a3, a2, (a0)
    bnez a3, fail     # sc.d must success
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
    .dword 4      # Initialize to 4
