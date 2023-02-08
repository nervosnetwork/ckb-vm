/* This test case is contributed by @mohanson */
.global _start
_start:
  lui a0, 0x4000
  jalr ra, a0, 1234
