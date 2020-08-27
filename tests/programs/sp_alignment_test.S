  .global _start
_start:
  andi a1, sp, 15
  li a2, 0
  beq a1, a2, ok
  li a0, 1
  li a7, 93
  ecall
ok:
  li a0, 0
  li a7, 93
  ecall
