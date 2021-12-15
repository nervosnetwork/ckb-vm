set -ex

riscv64-unknown-elf-gcc -o alloc_many alloc_many.c
# SKIP: andi
riscv64-unknown-elf-gcc -o argv_null_test argv_null_test.c
riscv64-unknown-elf-as -march=rv64imc -o cadd_hints.o cadd_hints.S && riscv64-unknown-elf-ld -o cadd_hints cadd_hints.o && rm cadd_hints.o
riscv64-unknown-elf-as -o ckbforks.o ckbforks.S && riscv64-unknown-elf-ld -o ckbforks ckbforks.o && rm ckbforks.o
# TODO: clzw_bug
# SKIP: decoder_instructions_cache_pc_out_of_bound_timeout
riscv64-unknown-elf-as -o ebreak.o ebreak.S && riscv64-unknown-elf-ld -o ebreak64 ebreak.o && rm ebreak.o
# SKIP: flat_crash_64
# SKIP: goblin_overflow_elf
# SKIP: invalid_file_offset64*
riscv64-unknown-elf-as -o invalid_read.o invalid_read.S && riscv64-unknown-elf-ld -o invalid_read64 invalid_read.o && rm invalid_read.o
riscv64-unknown-elf-as -march=rv64imc -o jalr_bug.o jalr_bug.S && riscv64-unknown-elf-ld -o jalr_bug jalr_bug.o && rm jalr_bug.o
riscv64-unknown-elf-as -o jalr_bug_noc.o jalr_bug_noc.S && riscv64-unknown-elf-ld -o jalr_bug_noc jalr_bug_noc.o && rm jalr_bug_noc.o
riscv64-unknown-elf-as -o jump0.o jump0.S && riscv64-unknown-elf-ld -o jump0_64 jump0.o && rm jump0.o
# SKIP: load_elf_crash_64
# SKIP: load_elf_section_crash_64
# SKIP: load_malformed_elf_crash_64
# SKIP: minimal
riscv64-unknown-elf-as -o misaligned_jump.o misaligned_jump.S && riscv64-unknown-elf-ld -o misaligned_jump64 misaligned_jump.o && rm misaligned_jump.o
riscv64-unknown-elf-as -o mop_adc.o mop_adc.S && riscv64-unknown-elf-ld -o mop_adc mop_adc.o && rm mop_adc.o
riscv64-unknown-elf-as -march=rv64imc -o mop_far_jump.o mop_far_jump.S && riscv64-unknown-elf-ld -o mop_far_jump mop_far_jump.o && rm mop_far_jump.o
riscv64-unknown-elf-gcc -o mop_ld_signextend_32 mop_ld_signextend_32.c
riscv64-unknown-elf-as -o mop_ld_signextend_32_overflow_bug.o mop_ld_signextend_32_overflow_bug.S && riscv64-unknown-elf-ld -o mop_ld_signextend_32_overflow_bug mop_ld_signextend_32_overflow_bug.o && rm mop_ld_signextend_32_overflow_bug.o
riscv64-unknown-elf-as -o mop_random_adc_sbb.o mop_random_adc_sbb.S && riscv64-unknown-elf-ld -o mop_random_adc_sbb mop_random_adc_sbb.o && rm mop_random_adc_sbb.o
riscv64-unknown-elf-as -o mop_sbb.o mop_sbb.S && riscv64-unknown-elf-ld -o mop_sbb mop_sbb.o && rm mop_sbb.o
riscv64-unknown-elf-as -o mop_wide_div_zero.o mop_wide_div_zero.S && riscv64-unknown-elf-ld -o mop_wide_div_zero mop_wide_div_zero.o && rm mop_wide_div_zero.o
riscv64-unknown-elf-gcc -o mop_wide_divide mop_wide_divide.c
riscv64-unknown-elf-as -o mop_wide_mul_zero.o mop_wide_mul_zero.S && riscv64-unknown-elf-ld -o mop_wide_mul_zero mop_wide_mul_zero.o && rm mop_wide_mul_zero.o
riscv64-unknown-elf-gcc -o mop_wide_multiply mop_wide_multiply.c
riscv64-unknown-elf-as -o mulw.o mulw.S && riscv64-unknown-elf-ld -o mulw64 mulw.o && rm mulw.o
# SKIP: nop
# SKIP: op_rvc_slli_crash_32
# SKIP: op_rvc_srai_crash_32
# SKIP: op_rvc_srli_crash_32
# TODO: pcnt
riscv64-unknown-elf-as -o read_at_boundary.o read_at_boundary.S && riscv64-unknown-elf-ld -o read_at_boundary64 read_at_boundary.o && rm read_at_boundary.o
riscv64-unknown-elf-as -o read_memory.o read_memory.S && riscv64-unknown-elf-ld -o read_memory read_memory.o && rm read_memory.o
riscv64-unknown-elf-gcc -o reset_callee reset_callee.c
riscv64-unknown-elf-gcc -o reset_caller reset_caller.c
riscv64-unknown-elf-as -o rorw_in_end_of_aot_block.o rorw_in_end_of_aot_block.S && riscv64-unknown-elf-ld -o rorw_in_end_of_aot_block rorw_in_end_of_aot_block.o && rm rorw_in_end_of_aot_block.o
sh rvc_pageend.sh
# TODO: sbinvi_aot_load_imm_bug
# SKIP: simple
riscv64-unknown-elf-gcc -o simple64 simple.c
riscv64-unknown-elf-as -o sp_alignment_test.o sp_alignment_test.S && riscv64-unknown-elf-ld -o sp_alignment_test sp_alignment_test.o && rm sp_alignment_test.o
riscv64-unknown-elf-as -o syscall.o syscall.S && riscv64-unknown-elf-ld -o syscall64 syscall.o && rm syscall.o
riscv64-unknown-elf-as -o trace.o trace.S && riscv64-unknown-elf-ld -o trace64 trace.o && rm trace.o
# SKIP: unaligned64
riscv64-unknown-elf-gcc -o writable_page writable_page.c && riscv64-unknown-elf-objdump -h writable_page > writable_page.dump
riscv64-unknown-elf-as -o write_at_boundary.o write_at_boundary.S && riscv64-unknown-elf-ld -o write_at_boundary64 write_at_boundary.o && rm write_at_boundary.o
riscv64-unknown-elf-as -o write_large_address.o write_large_address.S && riscv64-unknown-elf-ld -o write_large_address64 write_large_address.o && rm write_large_address.o
# riscv64-unknown-elf-as -march=rv64i_zba_zbb_zbc clmul_bug.S -o clmul_bug.o && riscv64-unknown-elf-ld clmul_bug.o -o clmul_bug && rm clmul_bug.o
# riscv64-unknown-elf-as -march=rv64i_zba_zbb_zbc orc_bug.S -o orc_bug.o && riscv64-unknown-elf-ld orc_bug.o -o orc_bug && rm orc_bug.o
echo "done"
