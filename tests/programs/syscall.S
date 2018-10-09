  .global _start
_start:
  li a0, 4
  li a1, 5
  li a2, 6
  li a3, 7
  li a4, 8
  li a5, 9
  li a7, 1111
  ecall
  li a7, 93
  ecall
