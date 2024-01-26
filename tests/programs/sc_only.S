.global _start
_start:
    la a0, n0         # a0 holds address of memory location n0
    sc.d a3, a2, (a0)
    beqz a3, fail     # sc.d must fail
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
