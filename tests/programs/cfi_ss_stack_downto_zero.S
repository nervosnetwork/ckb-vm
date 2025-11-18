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
    // Loop 0x10000/8 = 0x2000 times
    li t2, 0x2000
    li t0, 0

loop_start:
    beqz t2, loop_end
    sspush t0
    addi t2, t2, -1
    j loop_start

loop_end:
    exit_imm 0

1:
    exit_imm 0
