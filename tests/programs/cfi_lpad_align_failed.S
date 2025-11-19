

.include "cfi_macro.h"

gnu_property_cfi_ss 1

.text
.global _start
_start:
    la t1, func
    c.jalr t1
    
.globl func
.type func,@function
# no alignment
# .balign 4
func:
  lpad 0
  ret
