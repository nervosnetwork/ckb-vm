
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
    li ra, 42
    sspush ra
    sspopchk ra

    sspush ra
    sspopchk ra

    sspush ra
    sspopchk ra

    li t0, 42
    beq ra, t0, 1f
    exit_imm 2

1:
    ssrdp t0

    li t1, 0x10000
    beq t0, t1, 1f
    exit_imm 3

1:
    // First push a value (100) onto shadow stack
    li t0, 100
    sspush t0
    
    ssrdp t3
    
    // Prepare new value (200) to swap
    li t4, 200
    
    // Perform atomic swap: load from [t3] into t5, store t4 to [t3]
    ssamoswap.d t5, t4, (t3)
    
    // Verify that t5 now contains the old value (100)
    li t6, 100
    beq t5, t6, 1f
    exit_imm 4
    
1:
    // Pop the value and verify it's the new value (200)
    li t0, 200
    sspopchk t0

1:
    // Verify shadow stack pointer is back to original
    ssrdp t0
    li t1, 0x10000
    beq t0, t1, 1f
    exit_imm 6

1:
    exit_imm 0
