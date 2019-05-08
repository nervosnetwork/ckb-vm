.global _start
_start:
  li s6, 1
  li s5, -1
  mulw s5, s6, s5
  li t6, -1
  beq s5, t6, ok
  li a0, 1
  li a7, 93
  ecall
ok:
  li a0, 0
  li a7, 93
  ecall
