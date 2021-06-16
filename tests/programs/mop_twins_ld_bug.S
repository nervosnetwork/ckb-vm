.global _start
_start:
  li sp, 0x200000
  li t0, 0x200008
  sd t0, 0(sp)
  li t0, 0x200010
  sd t0, 8(sp)
  sd zero, 16(sp)
  ld sp, 0(sp)
  ld t1, 8(sp)
  bne zero, t1, fail
  li a0, 0
  li a7, 93
  ecall
fail:
  li a0, 1
  li a7, 93
  ecall
