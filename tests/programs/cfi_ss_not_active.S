
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
.4byte 0          // GNU_PROPERTY_RISCV_FEATURE_1_CFI_SS not active
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
    // sspush and sspopchk work as May-Be-Operations
    sspush t0
    sspopchk t0
    sspopchk t0
    sspopchk t0

    li t0, 100
    // ssrdp can work, but returns zero (May-Be-Operations)
    ssrdp t0
    beqz t0, 1f
    exit_imm 1
    
1:
    // ssamoswap works as May-Be-Operations? TODO
    li t0, -1
    ssamoswap.d ra, x0, (t0)
    exit_imm 0 
