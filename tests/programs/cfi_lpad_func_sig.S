
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
.4byte 4          // GNU_PROPERTY_RISCV_FEATURE_1_CFI_LP_FUNC_SIG
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
    //   is_lp_expected = ( (JALR || C.JR || C.JALR) &&
    //                    (rs1 != x1) && (rs1 != x5) && (rs1 != x7) ) ? 1 : 0;
    la t1, func
    li t2, 172032 // 42 << 12
    jalr ra, t1, 0
    exit_imm 0

.globl func
.type func,@function
func:
    lpad 42
    ret
