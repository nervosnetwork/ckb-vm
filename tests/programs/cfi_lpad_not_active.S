.include "cfi_macro.h"

gnu_property_cfi_ss 0

.text
.global _start
_start:
    call func
    exit_imm 0
    
.globl func
.type func,@function
func:
  // With unlabeled not active, any lpad instruction is permitted and acts as a no-op.
  lpad 0
  lpad 42
  lpad 100
  ret
