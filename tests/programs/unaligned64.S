  .global _start
_start:
  li a2, 0x10000
  ld a1, 0(a2)
  bne a1, x0, invalid
  li a0, 0
  li a7, 93
  ecall
invalid:
  li a0, 1
  li a7, 93
  ecall
