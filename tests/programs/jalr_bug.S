.global _start
_start:
  auipc ra, 0
  addi ra, ra, 18
  jalr ra
  li a0, -1
  li a7, 93
  ecall
1:
  li a0, 0
  li a7, 93
  ecall
