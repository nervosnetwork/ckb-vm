.global _start
_start:
  li a3, 123
  li a2, 0xFFFFFF00
  sd a3, 0(a2)
  li a1, 0
  li a7, 93
  ecall
