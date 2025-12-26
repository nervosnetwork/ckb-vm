.include "cfi_macro.h"

gnu_property_cfi_ss 2


.text
.global _start
_start:
    li ra, 0
    sspopchk ra
    exit_imm 0

