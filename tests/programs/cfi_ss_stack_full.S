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
    // Loop 0x10000/8 - 1 = 0x1FFF times
    li t2, 0x1FFF
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
