.global _start
_start:
  li a0, 0x7fffffffffffffff
  li a1, 1
  add a0, a1, a0
  sltu a1, a0, a1
  li t0, 0x8000000000000000
  bne a0, t0, fail
  li t0, 0
  bne a1, t0, fail

  li a0, 0x7fffffffffffffff
  li a1, 1
  add a0, a0, a1
  sltu a1, a0, a1
  li t0, 0x8000000000000000
  bne a0, t0, fail
  li t0, 0
  bne a1, t0, fail

  li a0, 0xffffffffffffffff
  li a1, 1
  add a0, a1, a0
  sltu a1, a0, a1
  li t0, 0x0
  bne a0, t0, fail
  li t0, 1
  bne a1, t0, fail

  li a0, 0xffffffffffffffff
  li a1, 1
  add a0, a1, a0
  sltu a0, a0, a1
  li t0, 1
  bne a0, t0, fail

  li a0, 0xffffffffffffffff
  li a1, 1
  add a0, a0, a1
  sltu a0, a0, a1
  li t0, 1
  bne a0, t0, fail

  li a1, 0xfffffffffffffff0
  li a2, 2
  add a0, a1, a2
  sltu a3, a0, a1
  li t0, 0xfffffffffffffff2
  bne a0, t0, fail
  li t0, 0
  bne a3, t0, fail

  li a0, 0
  li a7, 93
  ecall
fail:
  li a0, 1
  li a7, 93
  ecall
