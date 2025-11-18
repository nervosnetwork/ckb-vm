.include "cfi_macro.h"

gnu_property_cfi_ss 1

.text
.global _start
_start:
    la t1, func
    jalr ra, t1, 0

    la t1, func
    c.jalr t1

    la t1, exit
    c.jr t1
    
    exit_imm 1
    
.globl func
.type func,@function
# lpad checks alignment with 4
.balign 4
func:
  lpad 0
  ret

.global exit
.type func,@function
# lpad checks alignment with 4
.balign 4
exit:
    lpad 0
    exit_imm 0
