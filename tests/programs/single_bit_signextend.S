.global _start
_start:
  li a0, 0
  li a1, 31
  li a2, 0xffffffff80000000
  sbsetw a0, a0, a1
  bne a0, a2, fail

  li a0, 0xffffffffffffffff
  li a1, 31
  li a2, 0x000000007fffffff
  sbclrw a0, a0, a1
  bne a0, a2, fail

  li a0, 0xffffffffffffffff
  li a1, 30
  li a2, 0xffffffffbfffffff
  sbclrw a0, a0, a1
  bne a0, a2, fail

  li a0, 0xffffffffffffffff
  li a1, 31
  li a2, 0x000000007fffffff
  sbinvw a0, a0, a1
  bne a0, a2, fail

  li a0, 0
  li a1, 31
  li a2, 0xffffffff80000000
  sbinvw a0, a0, a1
  bne a0, a2, fail

  li a0, 0
  li a2, 0xffffffff80000000
  sbsetiw a0, a0, 31
  bne a0, a2, fail

  li a0, 0xffffffffffffffff
  li a2, 0x000000007fffffff
  sbclriw a0, a0, 31
  bne a0, a2, fail

  li a0, 0xffffffffffffffff
  li a2, 0xffffffffbfffffff
  sbclriw a0, a0, 30
  bne a0, a2, fail

  li a0, 0xffffffffffffffff
  li a2, 0x000000007fffffff
  sbinviw a0, a0, 31
  bne a0, a2, fail

  li a0, 0
  li a2, 0xffffffff80000000
  sbinviw a0, a0, 31
  bne a0, a2, fail

  li a0, 0
  li a7, 93
  ecall
fail:
  li a0, 1
  li a7, 93
  ecall
