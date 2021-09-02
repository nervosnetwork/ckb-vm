.global _start
_start:
  li a0, 10
  li a1, 3
  li a2, 0

  div a3, a0, a1
  rem zero, a0, a1
  bne a2, zero, fail

  divu a3, a0, a1
  rem zero, a0, a1
  bne a2, zero, fail

  li a0, 0
  li a7, 93
  ecall
fail:
  li a0, 1
  li a7, 93
  ecall
