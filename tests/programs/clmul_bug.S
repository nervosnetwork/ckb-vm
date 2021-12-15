.global _start
_start:
    li a0, 10
    li a1, 20
    clmul a0, a0, a1
    add a0, a0, 120
    li a7, 93
    ecall
