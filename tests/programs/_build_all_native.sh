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

$RISCV_GCC -o resume2_load_data resume2_load_data.c
$RISCV_GCC -o alloc_many alloc_many.c
$RISCV_AS -o amo_check_write.o amo_check_write.S && $RISCV_LD -T amo_check_write.lds -o amo_check_write amo_check_write.o && rm amo_check_write.o
$RISCV_AS -o amo_compare.o amo_compare.S && $RISCV_LD -T amo_compare.lds -o amo_compare amo_compare.o && rm amo_compare.o
$RISCV_AS -o amo_write_permission.o amo_write_permission.S && $RISCV_LD -o amo_write_permission amo_write_permission.o && rm amo_write_permission.o
# SKIP: andi
$RISCV_GCC -o argv_null_test argv_null_test.c
$RISCV_GCC -o big_binary big_binary.c
$RISCV_AS -march=rv64imc -o cadd_hints.o cadd_hints.S && $RISCV_LD -o cadd_hints cadd_hints.o && rm cadd_hints.o
$RISCV_AS -o ckbforks.o ckbforks.S && $RISCV_LD -o ckbforks ckbforks.o && rm ckbforks.o
# TODO: clzw_bug
# SKIP: decoder_instructions_cache_pc_out_of_bound_timeout
$RISCV_AS -o ebreak.o ebreak.S && $RISCV_LD -o ebreak64 ebreak.o && rm ebreak.o
# SKIP: flat_crash_64
# SKIP: goblin_overflow_elf
# SKIP: invalid_file_offset64*
$RISCV_AS -o invalid_read.o invalid_read.S && $RISCV_LD -o invalid_read64 invalid_read.o && rm invalid_read.o
$RISCV_AS -march=rv64imc -o jalr_bug.o jalr_bug.S && $RISCV_LD -o jalr_bug jalr_bug.o && rm jalr_bug.o
$RISCV_AS -o jalr_bug_noc.o jalr_bug_noc.S && $RISCV_LD -o jalr_bug_noc jalr_bug_noc.o && rm jalr_bug_noc.o
$RISCV_AS -o jump0.o jump0.S && $RISCV_LD -o jump0_64 jump0.o && rm jump0.o
# SKIP: load_elf_crash_64
# SKIP: load_elf_section_crash_64
# SKIP: load_malformed_elf_crash_64
# SKIP: minimal
$RISCV_AS -o misaligned_jump.o misaligned_jump.S && $RISCV_LD -o misaligned_jump64 misaligned_jump.o && rm misaligned_jump.o
$RISCV_AS -o mop_adc.o mop_adc.S && $RISCV_LD -o mop_adc mop_adc.o && rm mop_adc.o
$RISCV_AS -o mop_adcs.o mop_adcs.S && $RISCV_LD -o mop_adcs mop_adcs.o && rm mop_adcs.o
$RISCV_AS -o mop_sbbs.o mop_sbbs.S && $RISCV_LD -o mop_sbbs mop_sbbs.o && rm mop_sbbs.o
$RISCV_AS -o mop_add3.o mop_add3.S && $RISCV_LD -o mop_add3 mop_add3.o && rm mop_add3.o
$RISCV_AS -march=rv64imc -o mop_far_jump.o mop_far_jump.S && $RISCV_LD -o mop_far_jump mop_far_jump.o && rm mop_far_jump.o
$RISCV_GCC -o mop_ld_signextend_32 mop_ld_signextend_32.c
$RISCV_AS -o mop_ld_signextend_32_overflow_bug.o mop_ld_signextend_32_overflow_bug.S && $RISCV_LD -o mop_ld_signextend_32_overflow_bug mop_ld_signextend_32_overflow_bug.o && rm mop_ld_signextend_32_overflow_bug.o
$RISCV_AS -o mop_random_adc_sbb.o mop_random_adc_sbb.S && $RISCV_LD -o mop_random_adc_sbb mop_random_adc_sbb.o && rm mop_random_adc_sbb.o
$RISCV_AS -o mop_sbb.o mop_sbb.S && $RISCV_LD -o mop_sbb mop_sbb.o && rm mop_sbb.o
$RISCV_AS -o mop_wide_div_zero.o mop_wide_div_zero.S && $RISCV_LD -o mop_wide_div_zero mop_wide_div_zero.o && rm mop_wide_div_zero.o
$RISCV_GCC -o mop_wide_divide mop_wide_divide.c
$RISCV_AS -o mop_wide_mul_zero.o mop_wide_mul_zero.S && $RISCV_LD -o mop_wide_mul_zero mop_wide_mul_zero.o && rm mop_wide_mul_zero.o
$RISCV_GCC -o mop_wide_multiply mop_wide_multiply.c
$RISCV_AS -o mulw.o mulw.S && $RISCV_LD -o mulw64 mulw.o && rm mulw.o
$RISCV_AS -o nop64.o nop.S && $RISCV_LD -o nop64 nop64.o && rm nop64.o
$RISCV_AS -o nop_loop.o nop_loop.S && $RISCV_LD -o nop_loop nop_loop.o && rm nop_loop.o
# SKIP: op_rvc_slli_crash_32
# SKIP: op_rvc_srai_crash_32
# SKIP: op_rvc_srli_crash_32
$RISCV_GCC -o pause_resume pause_resume.c
# TODO: pcnt
$RISCV_AS -o read_at_boundary.o read_at_boundary.S && $RISCV_LD -o read_at_boundary64 read_at_boundary.o && rm read_at_boundary.o
$RISCV_AS -o read_memory.o read_memory.S && $RISCV_LD -o read_memory read_memory.o && rm read_memory.o
$RISCV_GCC -o reset_callee reset_callee.c
$RISCV_GCC -o reset_caller reset_caller.c
$RISCV_AS -o rorw_in_end_of_aot_block.o rorw_in_end_of_aot_block.S && $RISCV_LD -o rorw_in_end_of_aot_block rorw_in_end_of_aot_block.o && rm rorw_in_end_of_aot_block.o
sh rvc_pageend.sh
# TODO: sbinvi_aot_load_imm_bug
$RISCV_AS -o sc_after_sc.o sc_after_sc.S && $RISCV_LD -T sc_after_sc.lds -o sc_after_sc sc_after_sc.o && rm sc_after_sc.o
$RISCV_AS -o sc_after_snapshot.o sc_after_snapshot.S && $RISCV_LD -T sc_after_snapshot.lds -o sc_after_snapshot sc_after_snapshot.o && rm sc_after_snapshot.o
$RISCV_AS -o sc_only.o sc_only.S && $RISCV_LD -T sc_only.lds -o sc_only sc_only.o && rm sc_only.o
# SKIP: simple
$RISCV_GCC -o simple64 simple.c
$RISCV_AS -o sp_alignment_test.o sp_alignment_test.S && $RISCV_LD -o sp_alignment_test sp_alignment_test.o && rm sp_alignment_test.o
$RISCV_GCC -o spawn spawn.c
$RISCV_AS -o syscall.o syscall.S && $RISCV_LD -o syscall64 syscall.o && rm syscall.o
$RISCV_AS -o trace.o trace.S && $RISCV_LD -o trace64 trace.o && rm trace.o
# SKIP: unaligned64
$RISCV_GCC -o writable_page writable_page.c && ${RISCV_PREFIX}-objdump -h writable_page > writable_page.dump
$RISCV_AS -o write_at_boundary.o write_at_boundary.S && $RISCV_LD -o write_at_boundary64 write_at_boundary.o && rm write_at_boundary.o
$RISCV_AS -o write_large_address.o write_large_address.S && $RISCV_LD -o write_large_address64 write_large_address.o && rm write_large_address.o
# $RISCV_AS -march=rv64i_zba_zbb_zbc clmul_bug.S -o clmul_bug.o && $RISCV_LD clmul_bug.o -o clmul_bug && rm clmul_bug.o
# $RISCV_AS -march=rv64i_zba_zbb_zbc orc_bug.S -o orc_bug.o && $RISCV_LD orc_bug.o -o orc_bug && rm orc_bug.o
$RISCV_AS -o zero_address.o zero_address.S && $RISCV_LD -T zero_address.lds -o zero_address zero_address.o && rm zero_address.o
$RISCV_AS -o mop_jump_rel_version1_bug.o mop_jump_rel_version1_bug.S && $RISCV_LD -o mop_jump_rel_version1_bug mop_jump_rel_version1_bug.o && rm mop_jump_rel_version1_bug.o
$RISCV_AS -o mop_jump_rel_version1_reg_not_updated_bug.o mop_jump_rel_version1_reg_not_updated_bug.S && $RISCV_LD -o mop_jump_rel_version1_reg_not_updated_bug mop_jump_rel_version1_reg_not_updated_bug.o && rm mop_jump_rel_version1_reg_not_updated_bug.o
$RISCV_AS -o mop_jump_abs_version1_reg_not_updated_bug.o mop_jump_abs_version1_reg_not_updated_bug.S && $RISCV_LD -o mop_jump_abs_version1_reg_not_updated_bug mop_jump_abs_version1_reg_not_updated_bug.o && rm mop_jump_abs_version1_reg_not_updated_bug.o
echo "done"
