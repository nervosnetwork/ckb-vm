.global _start
_start:
  li a3, 0x100b4
  li a1, 1
  amoswap.d a4, a1, (a3)
  li a0, 0
  li a7, 93
  ecall
