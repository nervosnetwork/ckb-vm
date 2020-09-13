.global _start
_start:
  li a0, 0xffffffff
  li a1, 0xffffffff
  li a2, 0xffffffffffffffff
  packw a0, a0, a1
  beq a0, a2, ok
  li a0, 1
  li a7, 93
  ecall
ok:
  li a0, 0
  li a7, 93
  ecall
