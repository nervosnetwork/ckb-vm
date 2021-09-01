.global _start
_start:
  li a0, 1
  li a1, 1
  li a2, 0

  mulh a3, a0, a1
  mul zero, a0, a1
  bne a2, zero, fail

  mulhu a3, a0, a1
  mul zero, a0, a1
  bne a2, zero, fail

  mulhsu a3, a0, a1
  mul zero, a0, a1
  bne a2, zero, fail

  li a0, 0
  li a7, 93
  ecall
fail:
  li a0, 1
  li a7, 93
  ecall
