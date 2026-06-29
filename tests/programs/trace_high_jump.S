.global _start
_start:
  auipc t0, 0
  srli t1, t0, 32
  bnez t1, high_path
  li t2, 1
  slli t2, t2, 32
  add t0, t0, t2
  jalr x0, t0, 0

high_path:
  li a0, 42
  li a7, 93
  ecall
