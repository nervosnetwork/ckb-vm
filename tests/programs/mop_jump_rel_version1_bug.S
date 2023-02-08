/* This test case is contributed by @mohanson */
.global _start
_start:
  auipc ra, 0x80000
  jalr ra, ra, -0x7ff
  li a0, 1
  li a7, 93
  ecall
1:
  li a0, 0
  li a7, 93
  ecall
