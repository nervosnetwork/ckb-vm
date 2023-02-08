.global _start
_start:
  li t1, 0x7fffffffffffffff
  li t2, 1
  sub t0, t1, t2
  sltu t3, t1, t2
  li s1, 0x7ffffffffffffffe
  li s0, 1
  bne t0, s1, fail
  li s1, 0
  li s0, 2
  bne t3, s1, fail

  li t1, 100
  li t2, 211
  sub t0, t1, t2
  sltu t3, t1, t2
  li s1, -111
  li s0, 3
  bne t0, s1, fail
  li s1, 1
  li s0, 4
  bne t3, s1, fail

  li t1, 0x7fffffffffffffff
  li t2, 1
  sub t0, t1, t2
  sltu t0, t1, t2
  li s1, 0
  li s0, 5
  bne t0, s1, fail

  li t0, 123123412312
  li t1, 0x7fffffffffffffff
  li t2, 1
  sub zero, t1, t2
  sltu t3, t1, t2
  li s1, 123123412312
  li s0, 6
  bne t0, s1, fail
  li s1, 0
  li s0, 7
  bne t3, s1, fail
  li s1, 0
  li s0, 8
  bne zero, s1, fail

  li t0, 32423423
  li t1, 0x7fffffffffffffff
  li t2, 1
  li t3, 1237128937
  sub zero, t1, t2
  sltu zero, t1, t2
  li s1, 32423423
  li s0, 9
  bne t0, s1, fail
  li s1, 0x7fffffffffffffff
  li s0, 10
  bne t1, s1, fail
  li s1, 1
  li s0, 11
  bne t2, s1, fail
  li s1, 1237128937
  li s0, 12
  bne t3, s1, fail
  li s1, 0
  li s0, 13
  bne zero, s1, fail

success:
  li a0, 0
  li a7, 93
  ecall
fail:
  li a0, 1
  li a7, 93
  ecall
