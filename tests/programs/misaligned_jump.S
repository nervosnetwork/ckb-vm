.global _start
_start:
  li a7, 93
  la s2, 1f + 1
  jalr x0, s2, 0
  li a6, 17
1:
  li a5, 45
  la s3, 2f
  jalr x0, s3, 1
  li a4, 5
  li a3, 0
2:
  li a2, 19
  li a1, 0
  ecall
