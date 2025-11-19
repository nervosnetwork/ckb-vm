.include "cfi_macro.h"

gnu_property_cfi_ss 0

.text
.global _start
_start:
    li t0, -1
    ssamoswap.d ra, x0, (t0)
    exit_imm 0
