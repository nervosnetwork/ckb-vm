.include "cfi_macro.h"

gnu_property_cfi_ss 1

.text
.global _start
_start:
    //   is_lp_expected = ( (JALR || C.JR || C.JALR) &&
    //                    (rs1 != x1) && (rs1 != x5) && (rs1 != x7) ) ? 1 : 0;
    la t1, func
    jalr ra, t1, 0
    exit_imm 0
    
.globl func
.type func,@function
func:
  // NO LPAD instruction, should fail
  ret
