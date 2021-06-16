.global _start
_start:
  li a0, 1
  li a1, 2
  li a2, 4
  li a3, 8
  li sp, 0x200000

  sd a0, 0(sp)
  sd a1, 8(sp)
  sd a3, 24(sp)
  sd a2, 16(sp)
  ld t0, 0(sp)
  ld t1, 8(sp)
  ld t3, 24(sp)
  ld t2, 16(sp)
  bne a0, t0, fail
  bne a1, t1, fail
  bne a2, t2, fail
  bne a3, t3, fail

  ld t0, 8(sp)
  ld t0, 0(sp)
  bne a0, t0, fail

  li a0, 0
  li a7, 93
  ecall
fail:
  li a0, 1
  li a7, 93
  ecall
