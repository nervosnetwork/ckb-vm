.global _start
_start:
  li a0, 0xf011f8508554579d
  li a1, 0xf011f8500554579d
  sbinvi a0, a0, 0x1f
  bne a0, a1, fail
  li a0, 0
  li a7, 93
  ecall
fail:
  li a0, 1
  li a7, 93
  ecall
