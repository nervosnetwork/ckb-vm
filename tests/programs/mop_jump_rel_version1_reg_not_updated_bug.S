/* This test case is contributed by @mohanson */
.global _start
_start:
  auipc a0, 0x4000
  jalr ra, a0, 1234
