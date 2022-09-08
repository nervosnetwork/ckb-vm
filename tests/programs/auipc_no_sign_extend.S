  .global _start
_start:
  auipc gp, 0x0
  addi gp, gp, 6
  auipc a0, 0x7ffff
  sub a0, a0, gp
  lui gp, 0x7ffff
  beq a0, gp, ok
  li a0, 1
  li a7, 93
  ecall
ok:
  li a0, 0
  li a7, 93
  ecall
