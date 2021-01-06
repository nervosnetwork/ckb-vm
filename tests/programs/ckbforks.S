.section .ckb.forks
ver:
  .string "1.0.1"

.text
.global _start
_start:
  li a0, 1
  li a7, 93
  ecall
