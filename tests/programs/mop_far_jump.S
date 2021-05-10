.global _start
_start:
  auipc ra, 0x00
  jalr ra, ra, 0x08
  lui ra, 0x10
  jalr ra, ra, 0x90
  li a0, 1
  li a7, 93
  ecall
1:
  li a0, 0
  li a7, 93
  ecall
