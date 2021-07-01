.global _start
_start:
  lui t0, 0x80000
  addiw t0, t0, -1

  lui t1, 0x80000
  nop
  addiw t1, t1, -1

  bne t0, t1, fail

  li a0, 0
  li a7, 93
  ecall
fail:
  li a0, 1
  li a7, 93
  ecall
