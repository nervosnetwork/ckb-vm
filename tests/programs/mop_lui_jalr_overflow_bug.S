.global _start
_start:
  lui ra, 0x80000
  jalr ra, ra, -1
