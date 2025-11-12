
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
.4byte 2          // GNU_PROPERTY_RISCV_FEATURE_1_CFI_SS
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
    ssrdp t0

    li t1, 0x10000
    beq t0, t1, 1f
    exit_imm 1

1:
    // Loop 0x10000/8 = 0x2000 times
    li t2, 0x2000
    li t0, 0

loop_start:
    beqz t2, loop_end
    sspush t0
    addi t2, t2, -1
    j loop_start

loop_end:
    ssrdp t0
    // shadow stack pointer becomes zero which is invalid
    bne t0, x0, 1f
    exit_imm 2

1:
    exit_imm 0
