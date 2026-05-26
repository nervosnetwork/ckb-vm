.global _start
_start:
  li   t0, 5
  li   t1, 3
  li   t3, 2
  sub  t0, t1, t0
  sltu t2, t1, t3
  sub  t1, t0, t3
  sltu t3, t0, t1
  or   t0, t3, t2
  mv   a0, t0
  li   a7, 93
  ecall
