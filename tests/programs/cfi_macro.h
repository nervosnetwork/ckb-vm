.macro gnu_property_cfi_ss flags
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
.4byte \flags
.balign 8
ndesc_end:
.endm

.macro exit_imm val
    li a0, \val
    li a7, 93
    ecall
.endm

