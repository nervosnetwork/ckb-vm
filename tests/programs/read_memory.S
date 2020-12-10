.global _start
_start:
  li a1, 0x300000
  lbu a0, 0(a1)
  li a7, 93
  ecall
