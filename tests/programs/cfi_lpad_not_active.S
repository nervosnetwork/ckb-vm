
.section ".note.gnu.property", "a"
.balign 8
.4byte 4
.4byte (ndesc_end - ndesc_begin)
.4byte 0x5        // NT_GNU_PROPERTY_TYPE_0
.asciz "GNU"
ndesc_begin:
.balign 8
.4byte 0xc0000000 // GNU_PROPERTY_RISCV_FEATURE_1_AND
.4byte 4
.4byte 0          // GNU_PROPERTY_RISCV_FEATURE_1_CFI_LP_UNLABELED not active
.balign 8
ndesc_end:

.macro exit_imm val
    li a0, \val
    li a7, 93
    ecall
.endm

.text
.global _start
_start:
    call func
    exit_imm 0
    
.globl func
.type func,@function
func:
  // With unlabeled not active, any lpad instruction is permitted and acts as a no-op.
  lpad 0
  lpad 42
  lpad 100
  ret
