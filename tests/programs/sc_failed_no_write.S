# Regression test: a failed SC.W / SC.D must not modify memory.
#
# For each instruction, the test:
#   1. Issues the SC without a prior matching LR  -> must fail (rd != 0).
#   2. Reloads the target location and compares it to the static sentinel.
#   3. Exits with code 1 if the sentinel was overwritten, 0 on success.
.global _start
_start:

    # --- Test 1: failed SC.W must not write to val_w ---
    la      a0, val_w           # target address
    li      a2, 0x77777777      # value that must never reach memory
    sc.w    a3, a2, (a0)        # no prior lr.w -> reservation absent -> must fail
    beqz    a3, fail            # sanity: a3 == 0 would mean sc.w wrongly succeeded
    lw      a4, 0(a0)           # reload the word
    li      a5, 0x12345678      # expected sentinel (set in .data)
    bne     a4, a5, fail        # fail if memory was clobbered

    # --- Test 2: failed SC.D must not write to val_d ---
    la      a0, val_d           # target address
    li      a2, 0x77777777      # value that must never reach memory
    sc.d    a3, a2, (a0)        # no prior lr.d -> reservation absent -> must fail
    beqz    a3, fail            # sanity: a3 == 0 would mean sc.d wrongly succeeded
    ld      a4, 0(a0)           # reload the doubleword
    li      a5, 0x12345678      # expected sentinel (set in .data)
    bne     a4, a5, fail        # fail if memory was clobbered

done:
    li a0, 0
    li a7, 93
    ecall
fail:
    li a0, 1
    li a7, 93
    ecall

.section .data
val_w:
    .word 0x12345678
val_d:
    .dword 0x12345678
