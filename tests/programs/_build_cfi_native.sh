# use specific llvm version to avoid mixed tooling
MC=llvm-mc-21
LD=ld.lld-21

build() {
    local src=$1
    local output=$2
    local obj="${output}.o"
    $MC --filetype=obj --triple=riscv64 -mattr=+experimental-zicfiss,+experimental-zicfilp,+zcmop "$src" -o "$obj" && \
    $LD "$obj" -o "$output" --fatal-warnings
    local result=$?
    rm -f "$obj"
    return $result
}

build cfi_ss_success.S cfi_ss_success
build cfi_ss_not_active.S cfi_ss_not_active
build cfi_ss_stack_downto_zero.S cfi_ss_stack_downto_zero
build cfi_ss_stack_full.S cfi_ss_stack_full
build cfi_lpad_unlabeled.S cfi_lpad_unlabeled
build cfi_lpad_not_active.S cfi_lpad_not_active
build cfi_lpad_unlabeled_failed.S cfi_lpad_unlabeled_failed
build cfi_lpad_func_sig.S cfi_lpad_func_sig
build cfi_lpad_func_sig_zero.S cfi_lpad_func_sig_zero
build cfi_lpad_func_sig_failed.S cfi_lpad_func_sig_failed
build cfi_ss_only_pop.S cfi_ss_only_pop
build cfi_ss_popchk_failed.S cfi_ss_popchk_failed

