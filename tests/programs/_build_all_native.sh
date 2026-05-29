set -ex

# RISC-V toolchain prefix - can be overridden via environment variable
# Examples: RISCV_PREFIX=riscv-none-elf sh _build_all_native.sh
RISCV_PREFIX="${RISCV_PREFIX:-riscv64-unknown-elf}"

# Extra flags for toolchains that default to 32-bit (e.g. xPack riscv-none-elf).
# Examples: RISCV_ASFLAGS="-mabi=lp64" RISCV_LDFLAGS="-m elf64lriscv" RISCV_CFLAGS="-march=rv64imc -mabi=lp64"
RISCV_CFLAGS="${RISCV_CFLAGS:-}"
RISCV_ASFLAGS="${RISCV_ASFLAGS:-}"
RISCV_LDFLAGS="${RISCV_LDFLAGS:-}"

RISCV_GCC="${RISCV_PREFIX}-gcc ${RISCV_CFLAGS}"
RISCV_AS="${RISCV_PREFIX}-as ${RISCV_ASFLAGS}"
RISCV_LD="${RISCV_PREFIX}-ld ${RISCV_LDFLAGS}"

# Compile a .c file to a binary. SOURCE defaults to OUTPUT.c.
# Usage: gcc_compile OUTPUT [SOURCE]
gcc_compile() {
    $RISCV_GCC -o "$1" "${2:-$1.c}"
}

# Assemble STEM.S, link to OUTPUT (defaults to STEM), then remove the object file.
# Usage: asm_link [-march=ARCH] [-T LINKER_SCRIPT] STEM [OUTPUT]
asm_link() {
    march="" lds=""
    while [ $# -gt 0 ]; do
        case "$1" in
            -march=*) march="$1"; shift ;;
            -T) lds="-T $2"; shift 2 ;;
            *) break ;;
        esac
    done
    stem="$1" output="${2:-$1}"
    $RISCV_AS $march -o "$stem.o" "$stem.S"
    $RISCV_LD $lds -o "$output" "$stem.o"
    rm "$stem.o"
}

gcc_compile resume2_load_data
gcc_compile alloc_many
asm_link -T amo_check_write.lds amo_check_write
asm_link -T amo_compare.lds amo_compare
asm_link amo_write_permission
# SKIP: andi
gcc_compile argv_null_test
gcc_compile big_binary
asm_link -march=rv64imc cadd_hints
asm_link ckbforks
# TODO: clzw_bug
# SKIP: decoder_instructions_cache_pc_out_of_bound_timeout
asm_link ebreak ebreak64
# SKIP: flat_crash_64
# SKIP: goblin_overflow_elf
# SKIP: invalid_file_offset64*
asm_link invalid_read invalid_read64
asm_link -march=rv64imc jalr_bug
asm_link jalr_bug_noc
asm_link jump0 jump0_64
# SKIP: load_elf_crash_64
# SKIP: load_elf_section_crash_64
# SKIP: load_malformed_elf_crash_64
# SKIP: minimal
asm_link misaligned_jump misaligned_jump64
asm_link mop_adc
asm_link mop_adcs
asm_link mop_sbbs
asm_link mop_add3
asm_link -march=rv64imc mop_far_jump
gcc_compile mop_ld_signextend_32
asm_link mop_ld_signextend_32_overflow_bug
asm_link mop_random_adc_sbb
asm_link mop_sbb
asm_link mop_wide_div_zero
gcc_compile mop_wide_divide
asm_link mop_wide_mul_zero
gcc_compile mop_wide_multiply
asm_link mulw mulw64
gcc_compile division division64
asm_link nop nop64
asm_link nop_loop
# SKIP: op_rvc_slli_crash_32
# SKIP: op_rvc_srai_crash_32
# SKIP: op_rvc_srli_crash_32
gcc_compile pause_resume
# TODO: pcnt
asm_link read_at_boundary read_at_boundary64
asm_link read_memory
gcc_compile reset_callee
gcc_compile reset_caller
asm_link rorw_in_end_of_aot_block
sh rvc_pageend.sh
# TODO: sbinvi_aot_load_imm_bug
asm_link -T sc_after_sc.lds sc_after_sc
asm_link -T sc_after_snapshot.lds sc_after_snapshot
asm_link -march=rv64ima -T sc_failed_no_write.lds sc_failed_no_write
asm_link -T sc_only.lds sc_only
# SKIP: simple
gcc_compile simple64 simple.c
asm_link sp_alignment_test
gcc_compile spawn
asm_link syscall syscall64
asm_link trace trace64
# SKIP: unaligned64
gcc_compile writable_page
${RISCV_PREFIX}-objdump -h writable_page > writable_page.dump
asm_link write_at_boundary write_at_boundary64
asm_link write_large_address write_large_address64
# asm_link -march=rv64i_zba_zbb_zbc clmul_bug
# asm_link -march=rv64i_zba_zbb_zbc orc_bug
asm_link -T zero_address.lds zero_address
asm_link mop_jump_rel_version1_bug
asm_link mop_jump_rel_version1_reg_not_updated_bug
asm_link mop_jump_abs_version1_reg_not_updated_bug
echo "done"
