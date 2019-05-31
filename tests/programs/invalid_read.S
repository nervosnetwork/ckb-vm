.global _start
_start:
  li a1, -1
  ld a2, 0(a1)
  li a0, 0
  li a7, 93
  ecall
