.include "cfi_macro.h"

gnu_property_cfi_ss 2

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

    sspush t0
    sspopchk t0

    c.sspush ra
    sspopchk ra

    sspush t0
    c.sspopchk t0

    li ra, 42
    mv t0, ra
    .byte 0x73, 0x40, 0x10, 0xce # sspush   ra, avoid automatic conversion to compression instructions
    .byte 0x73, 0xc0, 0xc2, 0xcd # sspopchk t0, avoid automatic conversion to compression instructions

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
    # test ssamoswap.w version
    li t0, 100
    sspush t0
    ssrdp t3
    li t4, 200
    ssamoswap.w t5, t4, (t3)
    li t6, 100
    beq t5, t6, 1f
    exit_imm 7

1:
    exit_imm 0
