.global _start
_start:
_test_case_sc_d_after_success_sc_d:
    la a0, n0         # a0 holds address of memory location n0
    la a1, n1         # a1 holds address of memory location n1
    lr.d a2, (a0)
    sc.d a3, a2, (a0)
    bnez a3, fail     # sc.d must success
    sc.d a3, a2, (a0)
    beqz a3, fail     # sc.d must fail
_test_case_sc_d_after_failure_sc_d:
    la a0, n0         # a0 holds address of memory location n0
    la a1, n1         # a1 holds address of memory location n1
    lr.d a2, (a0)
    sc.d a3, a2, (a1)
    beqz a3, fail     # sc.d must success
    sc.d a3, a2, (a0)
    beqz a3, fail     # sc.d must fail
_test_case_sc_w_after_success_sc_w:
    la a0, n0         # a0 holds address of memory location n0
    la a1, n1         # a1 holds address of memory location n1
    lr.w a2, (a0)
    sc.w a3, a2, (a0)
    bnez a3, fail     # sc.d must success
    sc.w a3, a2, (a0)
    beqz a3, fail     # sc.d must fail
_test_case_sc_w_after_failure_sc_w:
    la a0, n0         # a0 holds address of memory location n0
    la a1, n1         # a1 holds address of memory location n1
    lr.w a2, (a0)
    sc.w a3, a2, (a1)
    beqz a3, fail     # sc.d must success
    sc.w a3, a2, (a0)
    beqz a3, fail     # sc.d must fail
done:
    li a0, 0
    li a7, 93
    ecall
fail:
    li a0, 1
    li a7, 93
    ecall
.section .data
n0:
    .dword 4      # Initialize to 4
n1:
    .dword 4      # Initialize to 4
