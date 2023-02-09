/* This test case is contributed by @mohanson */
.global _start
_start:
  auipc ra, 0x80000
  jalr ra, ra, -0x7ff
