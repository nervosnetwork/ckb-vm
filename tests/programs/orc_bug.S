.global _start
_start:
    li a1, 0xffffffffffffee00
    orc.b a0, a1
    li a2, 0xffffffffffffff00
    beq a0, a2, ok
    li a0, 1
    li a7, 93
    ecall
ok:
    li a0, 0
    li a7, 93
    ecall
