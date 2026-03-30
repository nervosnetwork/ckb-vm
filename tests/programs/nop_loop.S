  .global _start
_start:
  li a0, 1024
  li a1, 1
.loop:
  nop
  nop
  nop
  nop
  nop
  nop
  nop
  nop
  nop
  nop
  nop
  nop
  nop
  nop
  nop
  nop
  nop
  nop
  nop
  sub a0, a0, a1
  bnez a0, .loop
  li a1, 0
  li a2, 0
  li a3, 0
  li a4, 0
  li a5, 0
  li a7, 93
  ecall
