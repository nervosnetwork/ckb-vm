.global _start
_start:
  li a0, 1
  li a1, 1
  li a2, 0xffffffffffffffff
  sub a1, a0, a1
  sltu a3, a0, a1
  sub a0, a1, a2
  sltu a2, a1, a0
  or a1, a2, a3
  li t0, 1
  bne a0, t0, fail
  li t0, 1
  bne a1, t0, fail
  li t0, 1
  bne a2, t0, fail
  li t0, 0
  bne a3, t0, fail

  li a0, 0
  li a1, 1
  li a2, 1
  sub a1, a0, a1
  sltu a3, a0, a1
  sub a0, a1, a2
  sltu a2, a1, a0
  or a1, a2, a3
  li t0, 0xfffffffffffffffe
  bne a0, t0, fail
  li t0, 1
  bne a1, t0, fail
  li t0, 0
  bne a2, t0, fail
  li t0, 1
  bne a3, t0, fail

  li a0, 0
  li a7, 93
  ecall
fail:
  li a0, 1
  li a7, 93
  ecall
