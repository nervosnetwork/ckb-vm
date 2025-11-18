.include "cfi_macro.h"

gnu_property_cfi_ss 4

.text
.global _start
_start:
    //   is_lp_expected = ( (JALR || C.JR || C.JALR) &&
    //                    (rs1 != x1) && (rs1 != x5) && (rs1 != x7) ) ? 1 : 0;
    la t1, func
    li t2, 142
    jalr ra, t1, 0
    exit_imm 0

.globl func
.type func,@function
func:
    // The checking is as following:
    // The ELP is LP_EXPECTED and the LPL is not zero and
    // the LPL does not match the expected landing pad label in bits 31:12 of the x7 register.
    // When it's zero, it's allowed.
    lpad 0
    ret
