.include "cfi_macro.h"

gnu_property_cfi_ss 2


.text
.global _start
_start:
    li ra, 42
    sspush ra
    li ra, 41
    sspopchk ra
    exit_imm 0

