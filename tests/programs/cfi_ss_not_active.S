.include "cfi_macro.h"

gnu_property_cfi_ss 0

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
    exit_imm 0
