  .global _start
_start:
  li a0, 7
  li a0, 10
  auipc a1, 0
  addi a1, a1, -4
  lh a2, 0(a1)
  auipc a3, 0
  addi a3, a3, 10
  sh a2, 0(a3)
  li a0, 11
  li a7, 93
  ecall
