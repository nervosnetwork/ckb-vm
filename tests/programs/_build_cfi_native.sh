set -ex
# use specific llvm version to avoid mixed tooling
MC=llvm-mc-21
LD=ld.lld-21

build_ss() {
    local src=$1
    local output=$2
    local obj="${output}.o"
    $MC --filetype=obj --triple=riscv64 -mattr=+experimental-zicfiss,+zcmop "$src" -o "$obj" && \
    $LD "$obj" -o "$output" --fatal-warnings
    local result=$?
    rm -f "$obj"
    return $result
}

build_ss cfi_success.S cfi_success
build_ss cfi_ss_not_active.S cfi_ss_not_active

