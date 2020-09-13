.global _start
_start:
  li a0, 0xffffffffffffffff
  clzw s8, a0
  clz a0, s8
  li a1, 64
  beq a1, a0, ok
  li a0, 1
  li a7, 93
  ecall
ok:
  li a0, 0
  li a7, 93
  ecall
