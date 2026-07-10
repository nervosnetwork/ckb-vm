#define CKB_VM_ASM_GENERATE_LABEL_TABLES 1
#include "cdefinitions_generated.h"

#define MACHINE     a0
#define INVOKE_DATA a1
#define MEMORY_ADDR a2
#define MEMORY_SIZE a3
#define TRACE       a4
#define INST_PC     a5
#define INST_ARGS   a6
#define INST_HI32   a7
#define RD          s0
#define RS1         s1
#define RS2         s2
#define RS3         s3
#define RS4         s4
#define IMMEDIATE   s5
#define ERROR_ARG0  s6

#define CALL_INITED_MEMORY call inited_memory@plt

#define CHECK_READ(version, address_reg, length, tmp1, tmp2, tmp3) \
  srli   tmp1, address_reg, CKB_VM_ASM_MEMORY_FRAME_SHIFTS; \
  ld     tmp2, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_LAST_READ_FRAME(MACHINE); \
  bne    tmp1, tmp2, 1f; \
  addi   tmp2, address_reg, (length) - 1; \
  srli   tmp2, tmp2, CKB_VM_ASM_MEMORY_FRAME_SHIFTS; \
  beq    tmp1, tmp2, 3f; \
1: \
  mv     ERROR_ARG0, address_reg; \
  bgeu   ERROR_ARG0, MEMORY_SIZE, .exit_out_of_bound; \
  addi   ERROR_ARG0, address_reg, length; \
.if version < 1; \
  bgeu   ERROR_ARG0, MEMORY_SIZE, .exit_out_of_bound; \
.else; \
  bgtu   ERROR_ARG0, MEMORY_SIZE, .exit_out_of_bound; \
.endif; \
  sd     tmp1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_LAST_READ_FRAME(MACHINE); \
  ld     tmp2, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FRAMES_PTR(MACHINE); \
  add    tmp2, tmp2, tmp1; \
  lbu    tmp3, 0(tmp2); \
  bnez   tmp3, 2f; \
  li     tmp3, 1; \
  sb     tmp3, 0(tmp2); \
  PREPCALL; \
  mv     a1, MACHINE; \
  mv     a0, tmp1; \
  CALL_INITED_MEMORY; \
  POSTCALL; \
2: \
  addi   tmp1, address_reg, (length) - 1; \
  srli   tmp1, tmp1, CKB_VM_ASM_MEMORY_FRAME_SHIFTS; \
  ld     tmp2, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FRAMES_PTR(MACHINE); \
  add    tmp2, tmp2, tmp1; \
  lbu    tmp3, 0(tmp2); \
  bnez   tmp3, 3f; \
  li     tmp3, 1; \
  sb     tmp3, 0(tmp2); \
  PREPCALL; \
  mv     a1, MACHINE; \
  mv     a0, tmp1; \
  CALL_INITED_MEMORY; \
  POSTCALL; \
3:

#define CHECK_READ_VERSION0(address_reg, length, tmp1, tmp2, tmp3) \
  CHECK_READ(0, address_reg, length, tmp1, tmp2, tmp3)

#define CHECK_READ_VERSION1(address_reg, length, tmp1, tmp2, tmp3) \
  CHECK_READ(1, address_reg, length, tmp1, tmp2, tmp3)

#define CHECK_WRITE(address_reg, length, tmp1, tmp2, tmp3, tmp4) \
  srli   tmp1, address_reg, CKB_VM_ASM_RISCV_PAGE_SHIFTS; \
  ld     tmp2, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_LAST_WRITE_PAGE(MACHINE); \
  bne    tmp1, tmp2, 1f; \
  addi   tmp2, address_reg, (length) - 1; \
  srli   tmp2, tmp2, CKB_VM_ASM_RISCV_PAGE_SHIFTS; \
  beq    tmp1, tmp2, 3f; \
1: \
  sd     tmp1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_LAST_WRITE_PAGE(MACHINE); \
  ld     tmp2, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FLAGS_SIZE(MACHINE); \
  mv     ERROR_ARG0, address_reg; \
  bgeu   tmp1, tmp2, .exit_out_of_bound; \
  ld     tmp2, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FLAGS_PTR(MACHINE); \
  add    tmp4, tmp2, tmp1; \
  lbu    tmp3, 0(tmp4); \
  andi   tmp2, tmp3, CKB_VM_ASM_MEMORY_FLAG_WXORX_BIT; \
  li     tmp3, CKB_VM_ASM_MEMORY_FLAG_WRITABLE; \
  mv     ERROR_ARG0, tmp1; \
  bne    tmp2, tmp3, .exit_invalid_permission; \
  lbu    tmp3, 0(tmp4); \
  ori    tmp3, tmp3, CKB_VM_ASM_MEMORY_FLAG_DIRTY; \
  sb     tmp3, 0(tmp4); \
  mv     tmp2, tmp1; \
  srli   tmp1, tmp1, CKB_VM_ASM_MEMORY_FRAME_PAGE_SHIFTS; \
  ld     tmp4, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FRAMES_PTR(MACHINE); \
  add    tmp4, tmp4, tmp1; \
  lbu    tmp3, 0(tmp4); \
  bnez   tmp3, 2f; \
  li     tmp3, 1; \
  sb     tmp3, 0(tmp4); \
  PREPCALL; \
  mv     a1, MACHINE; \
  mv     a0, tmp1; \
  CALL_INITED_MEMORY; \
  POSTCALL; \
  srli   tmp2, address_reg, CKB_VM_ASM_RISCV_PAGE_SHIFTS; \
2: \
  addi   tmp1, tmp2, 1; \
  slli   tmp1, tmp1, CKB_VM_ASM_RISCV_PAGE_SHIFTS; \
  addi   tmp3, address_reg, length; \
  mv     ERROR_ARG0, tmp3; \
  bleu   tmp3, tmp1, 3f; \
  srli   tmp1, tmp1, CKB_VM_ASM_RISCV_PAGE_SHIFTS; \
  ld     tmp2, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FLAGS_SIZE(MACHINE); \
  bgeu   tmp1, tmp2, .exit_out_of_bound; \
  ld     tmp2, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FLAGS_PTR(MACHINE); \
  add    tmp4, tmp2, tmp1; \
  lbu    tmp3, 0(tmp4); \
  andi   tmp2, tmp3, CKB_VM_ASM_MEMORY_FLAG_WXORX_BIT; \
  li     tmp3, CKB_VM_ASM_MEMORY_FLAG_WRITABLE; \
  mv     ERROR_ARG0, tmp1; \
  bne    tmp2, tmp3, .exit_invalid_permission; \
  lbu    tmp3, 0(tmp4); \
  ori    tmp3, tmp3, CKB_VM_ASM_MEMORY_FLAG_DIRTY; \
  sb     tmp3, 0(tmp4); \
  srli   tmp1, tmp1, CKB_VM_ASM_MEMORY_FRAME_PAGE_SHIFTS; \
  ld     tmp4, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FRAMES_PTR(MACHINE); \
  add    tmp4, tmp4, tmp1; \
  lbu    tmp3, 0(tmp4); \
  bnez   tmp3, 3f; \
  li     tmp3, 1; \
  sb     tmp3, 0(tmp4); \
  PREPCALL; \
  mv     a1, MACHINE; \
  mv     a0, tmp1; \
  CALL_INITED_MEMORY; \
  POSTCALL; \
3:

#define DECODE_I \
  andi   RS1, INST_HI32, 0xff; \
  srai   IMMEDIATE, INST_HI32, 8

#define DECODE_R \
  andi   RS1, INST_HI32, 0xff; \
  srli   RS2, INST_HI32, 8; \
  andi   RS2, RS2, 0xff

#define DECODE_R4 \
  DECODE_R; \
  srli   RS3, INST_HI32, 16; \
  andi   RS3, RS3, 0xff

#define DECODE_R5 \
  DECODE_R4; \
  srli   RS4, INST_HI32, 24; \
  andi   RS4, RS4, 0xff

#define DECODE_S \
  mv     RS2, RD; \
  andi   RS1, INST_HI32, 0xff; \
  srai   IMMEDIATE, INST_HI32, 8

#define DECODE_U \
  mv     IMMEDIATE, INST_HI32

#define LOAD_PC(reg1, tmp1, tmp2) \
  ld     reg1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_PC(MACHINE); \
  lbu    tmp1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_VERSION(MACHINE); \
  li     tmp2, 2; \
  bge    tmp1, tmp2, 1f; \
  slli   reg1, reg1, 32; \
  srli   reg1, reg1, 32; \
1:

#define LOAD_REGISTER_ADDRESS(reg1, reg2) \
  slli   reg2, reg1, 3; \
  add    reg2, reg2, MACHINE

#define LOAD_RESERVATION_ADDRESS CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_LOAD_RESERVATION_ADDRESS(MACHINE)

#define LOAD_RD(tmp1) \
  LOAD_REGISTER_ADDRESS(RD, tmp1); \
  ld     RD, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(tmp1)

#define LOAD_RS1(tmp1) \
  LOAD_REGISTER_ADDRESS(RS1, tmp1); \
  ld     RS1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(tmp1)

#define LOAD_RS2(tmp1) \
  LOAD_REGISTER_ADDRESS(RS2, tmp1); \
  ld     RS2, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(tmp1)

#define LOAD_RS3(tmp1) \
  LOAD_REGISTER_ADDRESS(RS3, tmp1); \
  ld     RS3, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(tmp1)

#define LOAD_RS4(tmp1) \
  LOAD_REGISTER_ADDRESS(RS4, tmp1); \
  ld     RS4, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(tmp1)

#define NEXT_INST \
  ld     t0, (INST_ARGS); \
  ld     t1, (INST_PC); \
  addi   INST_ARGS, INST_ARGS, 16; \
  addi   INST_PC, INST_PC, 16; \
  srli   RD, t0, 8; \
  andi   RD, RD, 0xff; \
  srai   INST_HI32, t0, 32; \
  jr     t1

#define NEXT_INST_V2 \
  sd     zero, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(MACHINE); \
  NEXT_INST

#define PREPCALL \
  addi   sp, sp, -64; \
  sd     a0, 0(sp); \
  sd     a1, 8(sp); \
  sd     a2, 16(sp); \
  sd     a3, 24(sp); \
  sd     a4, 32(sp); \
  sd     a5, 40(sp); \
  sd     a6, 48(sp); \
  sd     a7, 56(sp);

#define POSTCALL \
  ld     a0, 0(sp); \
  ld     a1, 8(sp); \
  ld     a2, 16(sp); \
  ld     a3, 24(sp); \
  ld     a4, 32(sp); \
  ld     a5, 40(sp); \
  ld     a6, 48(sp); \
  ld     a7, 56(sp); \
  addi   sp, sp, 64

#define WRITE_RD(reg1, tmp1) \
  LOAD_REGISTER_ADDRESS(RD, tmp1); \
  sd     reg1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(tmp1); \
  sd     zero, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(MACHINE)

#define WRITE_RD_V2(reg1, tmp1) \
  LOAD_REGISTER_ADDRESS(RD, tmp1); \
  sd     reg1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(tmp1)

#define WRITE_RS1(reg1, tmp1) \
  LOAD_REGISTER_ADDRESS(RS1, tmp1); \
  sd     reg1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(tmp1)

#define WRITE_RS2(reg1, tmp1) \
  LOAD_REGISTER_ADDRESS(RS2, tmp1); \
  sd     reg1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(tmp1)

#define WRITE_RS3(reg1, tmp1) \
  LOAD_REGISTER_ADDRESS(RS3, tmp1); \
  sd     reg1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(tmp1)

.globl ckb_vm_asm_execute
ckb_vm_asm_execute:
.CKB_VM_ASM_EXECUTE:
  addi   sp, sp, -64
  sd     ra, 0(sp)
  sd     s0, 8(sp)
  sd     s1, 16(sp)
  sd     s2, 24(sp)
  sd     s3, 32(sp)
  sd     s4, 40(sp)
  sd     s5, 48(sp)
  sd     s6, 56(sp)
  ld     MEMORY_ADDR, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY_PTR(MACHINE)
  ld     MEMORY_SIZE, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY_SIZE(MACHINE)
.CKB_VM_ASM_LABEL_OP_CUSTOM_TRACE_END:
  LOAD_PC(t0, t1, t2)
  srli   t1, t0, 2
  ld     t2, CKB_VM_ASM_INVOKE_DATA_OFFSET_FIXED_TRACE_MASK(INVOKE_DATA)
  and    t1, t1, t2
  li     t2, CKB_VM_ASM_FIXED_TRACE_STRUCT_SIZE
  mul    t1, t1, t2
  ld     TRACE, CKB_VM_ASM_INVOKE_DATA_OFFSET_FIXED_TRACES(INVOKE_DATA)
  add    TRACE, TRACE, t1
  ld     t2, CKB_VM_ASM_TRACE_OFFSET_ADDRESS(TRACE)
  bne    t2, t0, .exit_trace
.trace_loaded:
  lwu    t3, CKB_VM_ASM_TRACE_OFFSET_LENGTH(TRACE)
  beqz   t3, .exit_trace
  ld     t1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_CYCLES(MACHINE)
  ld     t0, CKB_VM_ASM_TRACE_OFFSET_CYCLES(TRACE)
  add    t1, t1, t0
  bltu   t1, t0, .exit_cycles_overflow
  ld     t0, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MAX_CYCLES(MACHINE)
  bgtu   t1, t0, .exit_max_cycles_exceeded
  sd     t1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_CYCLES(MACHINE)
  ld     t2, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_PC(MACHINE)
  add    t2, t2, t3
  sd     t2, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_PC(MACHINE)
  addi   INST_PC, TRACE, CKB_VM_ASM_TRACE_OFFSET_THREADS
  addi   INST_ARGS, INST_PC, 8
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_CUSTOM_ASM_TRACE_JUMP:
  ld     TRACE, -16(INST_ARGS)
  ld     t2, CKB_VM_ASM_TRACE_OFFSET_ADDRESS(TRACE)
  j      .trace_loaded
.prepare_trace:
  ld     t0, CKB_VM_ASM_INVOKE_DATA_OFFSET_PAUSE(INVOKE_DATA)
  lbu    t0, (t0)
  bnez   t0, .exit_pause
  j      .CKB_VM_ASM_LABEL_OP_CUSTOM_TRACE_END
.CKB_VM_ASM_LABEL_OP_ADDI:
  DECODE_I
  LOAD_RS1(t0)
  add    RS1, RS1, IMMEDIATE
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_ADD:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  add    RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_ADDIW:
  DECODE_I
  LOAD_RS1(t0)
  addw   RS1, RS1, IMMEDIATE
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_ADDW:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  addw   RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_AND:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  and    RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_ANDI:
  DECODE_I
  LOAD_RS1(t0)
  and    RS1, RS1, IMMEDIATE
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_AUIPC:
  DECODE_U
  ld     RS1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_PC(MACHINE)
  addi   RS1, RS1, -4
  add    RS1, RS1, IMMEDIATE
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_BEQ:
  DECODE_S
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  beq    RS1, RS2, .i_branch_success
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_BGE:
  DECODE_S
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  bge    RS1, RS2, .i_branch_success
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_BGEU:
  DECODE_S
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  bgeu   RS1, RS2, .i_branch_success
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_BLT:
  DECODE_S
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  blt    RS1, RS2, .i_branch_success
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_BLTU:
  DECODE_S
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  bltu   RS1, RS2, .i_branch_success
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_BNE:
  DECODE_S
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  bne    RS1, RS2, .i_branch_success
  NEXT_INST
.i_branch_success:
  ld     t0, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_PC(MACHINE)
  /* Loading flag of current instruction */
  ld     t1, -16(INST_ARGS)
  srai   t1, t1, 24
  andi   t1, t1, 0x0f
  slli   t1, t1, 1
  sub    t0, t0, t1
  add    t0, t0, IMMEDIATE
  sd     t0, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_PC(MACHINE)
  j      .prepare_trace
.CKB_VM_ASM_LABEL_OP_DIV:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  div    RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_DIVU:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  divu   RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_DIVUW:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  divuw  RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_DIVW:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  divw   RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_EBREAK:
  li     a0, CKB_VM_ASM_RET_EBREAK
  j      .exit
.CKB_VM_ASM_LABEL_OP_ECALL:
  li     a0, CKB_VM_ASM_RET_ECALL
  j      .exit
.CKB_VM_ASM_LABEL_OP_FENCE:
.CKB_VM_ASM_LABEL_OP_FENCEI:
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_JAL:
  DECODE_U
  ld     RS1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_PC(MACHINE);
  WRITE_RD(RS1, t0)
  /* Loading instruction flags */
  ld     t1, -16(INST_ARGS)
  srai   t1, t1, 24
  andi   t1, t1, 0x0f
  slli   t1, t1, 1
  sub    RS1, RS1, t1
  add    RS1, RS1, IMMEDIATE
  sd     RS1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_PC(MACHINE)
  j      .prepare_trace
.CKB_VM_ASM_LABEL_OP_JALR_VERSION0:
  DECODE_I
  ld     t1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_PC(MACHINE)
  WRITE_RD(t1, t0)
  LOAD_RS1(t0)
  add    t1, RS1, IMMEDIATE
  andi   t1, t1, -2
  sd     t1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_PC(MACHINE)
  j      .prepare_trace
.CKB_VM_ASM_LABEL_OP_JALR_VERSION1:
  DECODE_I
  LOAD_RS1(t0)
  ld     t1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_PC(MACHINE)
  WRITE_RD(t1, t0)
  add    t2, RS1, IMMEDIATE
  andi   t2, t2, -2
  sd     t2, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_PC(MACHINE)
  j      .prepare_trace
.CKB_VM_ASM_LABEL_OP_LB_VERSION0:
  DECODE_I
  LOAD_RS1(t0)
  add    RS1, RS1, IMMEDIATE
  CHECK_READ_VERSION0(RS1, 1, t0, t1, t2)
  add    t0, MEMORY_ADDR, RS1
  lb     RS1, 0(t0)
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_LB_VERSION1:
  DECODE_I
  LOAD_RS1(t0)
  add    RS1, RS1, IMMEDIATE
  CHECK_READ_VERSION1(RS1, 1, t0, t1, t2)
  add    t0, MEMORY_ADDR, RS1
  lb     RS1, 0(t0)
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_LBU_VERSION0:
  DECODE_I
  LOAD_RS1(t0)
  add    RS1, RS1, IMMEDIATE
  CHECK_READ_VERSION0(RS1, 1, t0, t1, t2)
  add    t0, MEMORY_ADDR, RS1
  lbu    RS1, 0(t0)
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_LBU_VERSION1:
  DECODE_I
  LOAD_RS1(t0)
  add    RS1, RS1, IMMEDIATE
  CHECK_READ_VERSION1(RS1, 1, t0, t1, t2)
  add    t0, MEMORY_ADDR, RS1
  lbu    RS1, 0(t0)
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_LD_VERSION0:
  DECODE_I
  LOAD_RS1(t0)
  add    RS1, RS1, IMMEDIATE
  CHECK_READ_VERSION0(RS1, 8, t0, t1, t2)
  add    t0, MEMORY_ADDR, RS1
  ld     RS1, 0(t0)
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_LD_VERSION1:
  DECODE_I
  LOAD_RS1(t0)
  add    RS1, RS1, IMMEDIATE
  CHECK_READ_VERSION1(RS1, 8, t0, t1, t2)
  add    t0, MEMORY_ADDR, RS1
  ld     RS1, 0(t0)
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_LH_VERSION0:
  DECODE_I
  LOAD_RS1(t0)
  add    RS1, RS1, IMMEDIATE
  CHECK_READ_VERSION0(RS1, 2, t0, t1, t2)
  add    t0, MEMORY_ADDR, RS1
  lh     RS1, 0(t0)
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_LH_VERSION1:
  DECODE_I
  LOAD_RS1(t0)
  add    RS1, RS1, IMMEDIATE
  CHECK_READ_VERSION1(RS1, 2, t0, t1, t2)
  add    t0, MEMORY_ADDR, RS1
  lh     RS1, 0(t0)
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_LHU_VERSION0:
  DECODE_I
  LOAD_RS1(t0)
  add    RS1, RS1, IMMEDIATE
  CHECK_READ_VERSION0(RS1, 2, t0, t1, t2)
  add    t0, MEMORY_ADDR, RS1
  lhu    RS1, 0(t0)
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_LHU_VERSION1:
  DECODE_I
  LOAD_RS1(t0)
  add    RS1, RS1, IMMEDIATE
  CHECK_READ_VERSION1(RS1, 2, t0, t1, t2)
  add    t0, MEMORY_ADDR, RS1
  lhu    RS1, 0(t0)
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_LUI:
.CKB_VM_ASM_LABEL_OP_CUSTOM_LOAD_IMM:
  DECODE_U
  WRITE_RD(IMMEDIATE, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_CUSTOM_LOAD_UIMM:
  DECODE_U
  slli   IMMEDIATE, IMMEDIATE, 32
  srli   IMMEDIATE, IMMEDIATE, 32
  WRITE_RD(IMMEDIATE, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_LW_VERSION0:
  DECODE_I
  LOAD_RS1(t0)
  add    RS1, RS1, IMMEDIATE
  CHECK_READ_VERSION0(RS1, 4, t0, t1, t2)
  add    t0, MEMORY_ADDR, RS1
  lw     RS1, 0(t0)
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_LW_VERSION1:
  DECODE_I
  LOAD_RS1(t0)
  add    RS1, RS1, IMMEDIATE
  CHECK_READ_VERSION1(RS1, 4, t0, t1, t2)
  add    t0, MEMORY_ADDR, RS1
  lw     RS1, 0(t0)
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_LWU_VERSION0:
  DECODE_I
  LOAD_RS1(t0)
  add    RS1, RS1, IMMEDIATE
  CHECK_READ_VERSION0(RS1, 4, t0, t1, t2)
  add    t0, MEMORY_ADDR, RS1
  lwu    RS1, 0(t0)
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_LWU_VERSION1:
  DECODE_I
  LOAD_RS1(t0)
  add    RS1, RS1, IMMEDIATE
  CHECK_READ_VERSION1(RS1, 4, t0, t1, t2)
  add    t0, MEMORY_ADDR, RS1
  lwu    RS1, 0(t0)
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_MUL:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  mul    RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_MULH:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  mulh   RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_MULHSU:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  mulhsu RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_MULHU:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  mulhu  RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_MULW:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  mulw   RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_OR:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  or     RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_ORI:
  DECODE_I
  LOAD_RS1(t0)
  or     RS1, RS1, IMMEDIATE
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_REM:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  rem    RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_REMU:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  remu   RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_REMUW:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  remuw  RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_REMW:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  remw   RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SB:
  DECODE_S
  LOAD_RS1(t0)
  add    RS1, RS1, IMMEDIATE
  CHECK_WRITE(RS1, 1, t0, t1, t2, t3)
  LOAD_RS2(t0)
  add    t0, MEMORY_ADDR, RS1
  sb     RS2, 0(t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SD:
  DECODE_S
  LOAD_RS1(t0)
  add    RS1, RS1, IMMEDIATE
  CHECK_WRITE(RS1, 8, t0, t1, t2, t3)
  LOAD_RS2(t0)
  add    t0, MEMORY_ADDR, RS1
  sd     RS2, 0(t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SH:
  DECODE_S
  LOAD_RS1(t0)
  add    RS1, RS1, IMMEDIATE
  CHECK_WRITE(RS1, 2, t0, t1, t2, t3)
  LOAD_RS2(t0)
  add    t0, MEMORY_ADDR, RS1
  sh     RS2, 0(t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SLL:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  sll    RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SLLI:
  DECODE_I
  LOAD_RS1(t0)
  sll    RS1, RS1, IMMEDIATE
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SLLIW:
  DECODE_I
  LOAD_RS1(t0)
  sllw   RS1, RS1, IMMEDIATE
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SLLW:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  andi   t0, RS2, 0x1f
  sllw   RS1, RS1, t0
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SLT:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  slt    RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SLTI:
  DECODE_I
  LOAD_RS1(t0)
  slt    RS1, RS1, IMMEDIATE
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SLTIU:
  DECODE_I
  LOAD_RS1(t0)
  sltu   RS1, RS1, IMMEDIATE
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SLTU:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  sltu   RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SRA:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  sra    RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SRAI:
  DECODE_I
  LOAD_RS1(t0)
  sra    RS1, RS1, IMMEDIATE
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SRAIW:
  DECODE_I
  LOAD_RS1(t0)
  sraw   RS1, RS1, IMMEDIATE
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SRAW:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  andi   RS2, RS2, 0x1f
  sraw   RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SRL:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  srl    RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SRLI:
  DECODE_I
  LOAD_RS1(t0)
  srl    RS1, RS1, IMMEDIATE
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SRLIW:
  DECODE_I
  LOAD_RS1(t0)
  srlw   RS1, RS1, IMMEDIATE
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SRLW:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  andi   RS2, RS2, 0x1f
  srlw   RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SUB:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  sub    RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SUBW:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  subw   RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SW:
  DECODE_S
  LOAD_RS1(t0)
  add    RS1, RS1, IMMEDIATE
  CHECK_WRITE(RS1, 4, t0, t1, t2, t3)
  LOAD_RS2(t0)
  add    t0, MEMORY_ADDR, RS1
  sw     RS2, 0(t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_XOR:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  xor    RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_XORI:
  DECODE_I
  LOAD_RS1(t0)
  xor    RS1, RS1, IMMEDIATE
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_LR_W:
  DECODE_R
  LOAD_RS1(t0)
  CHECK_READ_VERSION1(RS1, 4, t0, t1, t2)
  sd     RS1, LOAD_RESERVATION_ADDRESS
  add    t0, MEMORY_ADDR, RS1
  lw     RS1, 0(t0)
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SC_W:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  CHECK_WRITE(RS1, 4, t0, t1, t2, t3)
  ld     t0, LOAD_RESERVATION_ADDRESS
  li     t1, 0xffffffffffffffff
  sd     t1, LOAD_RESERVATION_ADDRESS
  li     RS3, 1
  bne    RS1, t0, 1f
  add    t1, MEMORY_ADDR, RS1
  sw     RS2, 0(t1)
  li     RS3, 0
1:
  WRITE_RD(RS3, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_AMOSWAP_W:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  CHECK_WRITE(RS1, 4, t0, t1, t2, t3)
  add    t0, MEMORY_ADDR, RS1
  lw     t1, 0(t0)
  sw     RS2, 0(t0)
  mv     RS1, t1
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_AMOADD_W:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  CHECK_WRITE(RS1, 4, t0, t1, t2, t3)
  add    t0, MEMORY_ADDR, RS1
  lw     t1, 0(t0)
  addw   t2, t1, RS2
  sw     t2, 0(t0)
  mv     RS1, t1
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_AMOXOR_W:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  CHECK_WRITE(RS1, 4, t0, t1, t2, t3)
  add    t0, MEMORY_ADDR, RS1
  lw     t1, 0(t0)
  xor    t2, t1, RS2
  sw     t2, 0(t0)
  mv     RS1, t1
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_AMOAND_W:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  CHECK_WRITE(RS1, 4, t0, t1, t2, t3)
  add    t0, MEMORY_ADDR, RS1
  lw     t1, 0(t0)
  and    t2, t1, RS2
  sw     t2, 0(t0)
  mv     RS1, t1
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_AMOOR_W:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  CHECK_WRITE(RS1, 4, t0, t1, t2, t3)
  add    t0, MEMORY_ADDR, RS1
  lw     t1, 0(t0)
  or     t2, t1, RS2
  sw     t2, 0(t0)
  mv     RS1, t1
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_AMOMIN_W:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  CHECK_WRITE(RS1, 4, t0, t1, t2, t3)
  add    t0, MEMORY_ADDR, RS1
  lw     t1, 0(t0)
  sext.w t2, t1
  sext.w t3, RS2
  blt    t2, t3, 1f
  mv     t1, RS2
1:
  sw     t1, 0(t0)
  sext.w RS1, t2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_AMOMAX_W:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  CHECK_WRITE(RS1, 4, t0, t1, t2, t3)
  add    t0, MEMORY_ADDR, RS1
  lw     t1, 0(t0)
  sext.w t2, t1
  sext.w t3, RS2
  bge    t2, t3, 1f
  mv     t1, RS2
1:
  sw     t1, 0(t0)
  sext.w RS1, t2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_AMOMINU_W:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  CHECK_WRITE(RS1, 4, t0, t1, t2, t3)
  add    t0, MEMORY_ADDR, RS1
  lwu    t1, 0(t0)
  slli   t2, t1, 32
  srli   t2, t2, 32
  slli   t3, RS2, 32
  srli   t3, t3, 32
  bltu   t2, t3, 1f
  mv     t1, RS2
1:
  sw     t1, 0(t0)
  sext.w RS1, t2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_AMOMAXU_W:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  CHECK_WRITE(RS1, 4, t0, t1, t2, t3)
  add    t0, MEMORY_ADDR, RS1
  lwu    t1, 0(t0)
  slli   t2, t1, 32
  srli   t2, t2, 32
  slli   t3, RS2, 32
  srli   t3, t3, 32
  bgeu   t2, t3, 1f
  mv     t1, RS2
1:
  sw     t1, 0(t0)
  sext.w RS1, t2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_LR_D:
  DECODE_R
  LOAD_RS1(t0)
  CHECK_READ_VERSION1(RS1, 8, t0, t1, t2)
  sd     RS1, LOAD_RESERVATION_ADDRESS
  add    t0, MEMORY_ADDR, RS1
  ld     RS1, 0(t0)
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SC_D:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  CHECK_WRITE(RS1, 8, t0, t1, t2, t3)
  ld     t0, LOAD_RESERVATION_ADDRESS
  li     t1, 0xffffffffffffffff
  sd     t1, LOAD_RESERVATION_ADDRESS
  li     RS3, 1
  bne    RS1, t0, 1f
  add    t1, MEMORY_ADDR, RS1
  sd     RS2, 0(t1)
  li     RS3, 0
1:
  WRITE_RD(RS3, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_AMOSWAP_D:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  CHECK_WRITE(RS1, 8, t0, t1, t2, t3)
  add    t0, MEMORY_ADDR, RS1
  ld     t1, 0(t0)
  sd     RS2, 0(t0)
  mv     RS1, t1
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_AMOADD_D:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  CHECK_WRITE(RS1, 8, t0, t1, t2, t3)
  add    t0, MEMORY_ADDR, RS1
  ld     t1, 0(t0)
  add    t2, t1, RS2
  sd     t2, 0(t0)
  mv     RS1, t1
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_AMOXOR_D:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  CHECK_WRITE(RS1, 8, t0, t1, t2, t3)
  add    t0, MEMORY_ADDR, RS1
  ld     t1, 0(t0)
  xor    t2, t1, RS2
  sd     t2, 0(t0)
  mv     RS1, t1
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_AMOAND_D:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  CHECK_WRITE(RS1, 8, t0, t1, t2, t3)
  add    t0, MEMORY_ADDR, RS1
  ld     t1, 0(t0)
  and    t2, t1, RS2
  sd     t2, 0(t0)
  mv     RS1, t1
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_AMOOR_D:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  CHECK_WRITE(RS1, 8, t0, t1, t2, t3)
  add    t0, MEMORY_ADDR, RS1
  ld     t1, 0(t0)
  or     t2, t1, RS2
  sd     t2, 0(t0)
  mv     RS1, t1
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_AMOMIN_D:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  CHECK_WRITE(RS1, 8, t0, t1, t2, t3)
  add    t0, MEMORY_ADDR, RS1
  ld     t1, 0(t0)
  mv     t2, t1
  blt    t1, RS2, 1f
  mv     t2, RS2
1:
  sd     t2, 0(t0)
  mv     RS1, t1
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_AMOMAX_D:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  CHECK_WRITE(RS1, 8, t0, t1, t2, t3)
  add    t0, MEMORY_ADDR, RS1
  ld     t1, 0(t0)
  mv     t2, t1
  bge    t1, RS2, 1f
  mv     t2, RS2
1:
  sd     t2, 0(t0)
  mv     RS1, t1
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_AMOMINU_D:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  CHECK_WRITE(RS1, 8, t0, t1, t2, t3)
  add    t0, MEMORY_ADDR, RS1
  ld     t1, 0(t0)
  mv     t2, t1
  bltu   t1, RS2, 1f
  mv     t2, RS2
1:
  sd     t2, 0(t0)
  mv     RS1, t1
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_AMOMAXU_D:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  CHECK_WRITE(RS1, 8, t0, t1, t2, t3)
  add    t0, MEMORY_ADDR, RS1
  ld     t1, 0(t0)
  mv     t2, t1
  bgeu   t1, RS2, 1f
  mv     t2, RS2
1:
  sd     t2, 0(t0)
  mv     RS1, t1
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_ADDUW:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  slli   RS1, RS1, 32
  srli   RS1, RS1, 32
  add    RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SH1ADD:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  slli   RS1, RS1, 1
  add    RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SH1ADDUW:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  slli   RS1, RS1, 32
  srli   RS1, RS1, 31
  add    RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SH2ADD:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  slli   RS1, RS1, 2
  add    RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SH2ADDUW:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  slli   RS1, RS1, 32
  srli   RS1, RS1, 30
  add    RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SH3ADD:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  slli   RS1, RS1, 3
  add    RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SH3ADDUW:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  slli   RS1, RS1, 32
  srli   RS1, RS1, 29
  add    RS1, RS1, RS2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SLLIUW:
  DECODE_I
  LOAD_RS1(t0)
  slli   RS1, RS1, 32
  srli   RS1, RS1, 32
  sll    RS1, RS1, IMMEDIATE
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_ANDN:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  not    t0, RS2
  and    RS1, RS1, t0
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_ORN:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  not    t0, RS2
  or     RS1, RS1, t0
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_XNOR:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  xor    RS1, RS1, RS2
  not    RS1, RS1
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_CLZ:
  DECODE_R
  LOAD_RS1(t0)
  beqz   RS1, .clz_zero
  li     t0, 0
  li     t1, 0x8000000000000000
1:
  and    t2, RS1, t1
  bnez   t2, 2f
  addi   t0, t0, 1
  srli   t1, t1, 1
  j      1b
2:
  mv     RS1, t0
  WRITE_RD(RS1, t0)
  NEXT_INST
.clz_zero:
  li     RS1, 64
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_CLZW:
  DECODE_R
  LOAD_RS1(t0)
  slli   RS1, RS1, 32
  srli   RS1, RS1, 32
  beqz   RS1, .clzw_zero
  li     t0, 0
  li     t1, 0x80000000
1:
  and    t2, RS1, t1
  bnez   t2, 2f
  addi   t0, t0, 1
  srli   t1, t1, 1
  j      1b
2:
  mv     RS1, t0
  WRITE_RD(RS1, t0)
  NEXT_INST
.clzw_zero:
  li     RS1, 32
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_CTZ:
  DECODE_R
  LOAD_RS1(t0)
  beqz   RS1, .ctz_zero
  li     t0, 0
1:
  andi   t1, RS1, 1
  bnez   t1, 2f
  addi   t0, t0, 1
  srli   RS1, RS1, 1
  j      1b
2:
  mv     RS1, t0
  WRITE_RD(RS1, t0)
  NEXT_INST
.ctz_zero:
  li     RS1, 64
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_CTZW:
  DECODE_R
  LOAD_RS1(t0)
  slli   RS1, RS1, 32
  srli   RS1, RS1, 32
  beqz   RS1, .ctzw_zero
  li     t0, 0
1:
  andi   t1, RS1, 1
  bnez   t1, 2f
  addi   t0, t0, 1
  srli   RS1, RS1, 1
  j      1b
2:
  mv     RS1, t0
  WRITE_RD(RS1, t0)
  NEXT_INST
.ctzw_zero:
  li     RS1, 32
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_CPOP:
  DECODE_R
  LOAD_RS1(t0)
  li     t0, 0
  beqz   RS1, 2f
1:
  andi   t1, RS1, 1
  add    t0, t0, t1
  srli   RS1, RS1, 1
  bnez   RS1, 1b
2:
  mv     RS1, t0
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_CPOPW:
  DECODE_R
  LOAD_RS1(t0)
  slli   RS1, RS1, 32
  srli   RS1, RS1, 32
  li     t0, 0
  beqz   RS1, 2f
1:
  andi   t1, RS1, 1
  add    t0, t0, t1
  srli   RS1, RS1, 1
  bnez   RS1, 1b
2:
  mv     RS1, t0
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_MAX:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  bge    RS1, RS2, 1f
  mv     RS1, RS2
1:
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_MAXU:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  bgeu   RS1, RS2, 1f
  mv     RS1, RS2
1:
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_MIN:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  ble    RS1, RS2, 1f
  mv     RS1, RS2
1:
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_MINU:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  bleu   RS1, RS2, 1f
  mv     RS1, RS2
1:
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SEXTB:
  DECODE_R
  LOAD_RS1(t0)
  slli   RS1, RS1, 56
  srai   RS1, RS1, 56
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_SEXTH:
  DECODE_R
  LOAD_RS1(t0)
  slli   RS1, RS1, 48
  srai   RS1, RS1, 48
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_ZEXTH:
  DECODE_R
  LOAD_RS1(t0)
  slli   RS1, RS1, 48
  srli   RS1, RS1, 48
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_ROL:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  andi   RS2, RS2, 63
  sll    t0, RS1, RS2
  li     t1, 64
  sub    t1, t1, RS2
  srl    t1, RS1, t1
  or     RS1, t0, t1
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_ROLW:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  slli   RS1, RS1, 32
  srli   RS1, RS1, 32
  andi   RS2, RS2, 31
  sllw   t0, RS1, RS2
  li     t1, 32
  sub    t1, t1, RS2
  srlw   t1, RS1, t1
  or     RS1, t0, t1
  sext.w RS1, RS1
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_ROR:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  andi   RS2, RS2, 63
  srl    t0, RS1, RS2
  li     t1, 64
  sub    t1, t1, RS2
  sll    t1, RS1, t1
  or     RS1, t0, t1
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_RORI:
  DECODE_I
  LOAD_RS1(t0)
  andi   t1, IMMEDIATE, 63
  srl    t0, RS1, t1
  li     t2, 64
  sub    t2, t2, t1
  sll    t2, RS1, t2
  or     RS1, t0, t2
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_RORIW:
  DECODE_I
  LOAD_RS1(t0)
  slli   RS1, RS1, 32
  srli   RS1, RS1, 32
  andi   t1, IMMEDIATE, 31
  srlw   t0, RS1, t1
  li     t2, 32
  sub    t2, t2, t1
  sllw   t2, RS1, t2
  or     RS1, t0, t2
  sext.w RS1, RS1
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_RORW:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  slli   RS1, RS1, 32
  srli   RS1, RS1, 32
  andi   RS2, RS2, 31
  srlw   t0, RS1, RS2
  li     t1, 32
  sub    t1, t1, RS2
  sllw   t1, RS1, t1
  or     RS1, t0, t1
  sext.w RS1, RS1
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_ORCB:
  DECODE_R
  LOAD_RS1(t0)
  li     t0, 0
  li     t2, 0xff
  and    t1, RS1, t2
  beqz   t1, 1f
  or     t0, t0, t2
1:
  slli   t2, t2, 8
  and    t1, RS1, t2
  beqz   t1, 2f
  or     t0, t0, t2
2:
  slli   t2, t2, 8
  and    t1, RS1, t2
  beqz   t1, 3f
  or     t0, t0, t2
3:
  slli   t2, t2, 8
  and    t1, RS1, t2
  beqz   t1, 4f
  or     t0, t0, t2
4:
  slli   t2, t2, 8
  and    t1, RS1, t2
  beqz   t1, 5f
  or     t0, t0, t2
5:
  slli   t2, t2, 8
  and    t1, RS1, t2
  beqz   t1, 6f
  or     t0, t0, t2
6:
  slli   t2, t2, 8
  and    t1, RS1, t2
  beqz   t1, 7f
  or     t0, t0, t2
7:
  slli   t2, t2, 8
  and    t1, RS1, t2
  beqz   t1, 8f
  or     t0, t0, t2
8:
  mv     RS1, t0
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_REV8:
  DECODE_R
  LOAD_RS1(t0)
  andi   t1, RS1, 0xff
  slli   t0, t1, 56
  srli   t1, RS1, 8
  andi   t1, t1, 0xff
  slli   t1, t1, 48
  or     t0, t0, t1
  srli   t1, RS1, 16
  andi   t1, t1, 0xff
  slli   t1, t1, 40
  or     t0, t0, t1
  srli   t1, RS1, 24
  andi   t1, t1, 0xff
  slli   t1, t1, 32
  or     t0, t0, t1
  srli   t1, RS1, 32
  andi   t1, t1, 0xff
  slli   t1, t1, 24
  or     t0, t0, t1
  srli   t1, RS1, 40
  andi   t1, t1, 0xff
  slli   t1, t1, 16
  or     t0, t0, t1
  srli   t1, RS1, 48
  andi   t1, t1, 0xff
  slli   t1, t1, 8
  or     t0, t0, t1
  srli   t1, RS1, 56
  or     RS1, t0, t1
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_BCLR:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  li     t0, 1
  sll    t0, t0, RS2
  not    t0, t0
  and    RS1, RS1, t0
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_BCLRI:
  DECODE_I
  LOAD_RS1(t0)
  li     t0, 1
  sll    t0, t0, IMMEDIATE
  not    t0, t0
  and    RS1, RS1, t0
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_BEXT:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  srl    RS1, RS1, RS2
  andi   RS1, RS1, 1
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_BEXTI:
  DECODE_I
  LOAD_RS1(t0)
  srl    RS1, RS1, IMMEDIATE
  andi   RS1, RS1, 1
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_BINV:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  li     t0, 1
  sll    t0, t0, RS2
  xor    RS1, RS1, t0
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_BINVI:
  DECODE_I
  LOAD_RS1(t0)
  li     t0, 1
  sll    t0, t0, IMMEDIATE
  xor    RS1, RS1, t0
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_BSET:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  li     t0, 1
  sll    t0, t0, RS2
  or     RS1, RS1, t0
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_BSETI:
  DECODE_I
  LOAD_RS1(t0)
  li     t0, 1
  sll    t0, t0, IMMEDIATE
  or     RS1, RS1, t0
  WRITE_RD(RS1, t0)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_CLMUL:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  li     t0, 0
  li     t1, 0
1:
  andi   t2, RS2, 1
  sll    t3, RS1, t1
  beqz   t2, 2f
  xor    t0, t0, t3
2:
  addi   t1, t1, 1
  srli   RS2, RS2, 1
  bnez   RS2, 1b
  WRITE_RD(t0, t1)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_CLMULH:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  li     t0, 0
  li     t1, 1
  srli   RS2, RS2, 1
1:
  beqz   RS2, 3f
  andi   t2, RS2, 1
  beqz   t2, 2f
  li     t3, 64
  sub    t3, t3, t1
  srl    t3, RS1, t3
  xor    t0, t0, t3
2:
  addi   t1, t1, 1
  srli   RS2, RS2, 1
  j      1b
3:
  WRITE_RD(t0, t1)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_CLMULR:
  DECODE_R
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  li     t0, 0
  li     t1, 0
1:
  li     t4, 64
  bge    t1, t4, 3f
  andi   t2, RS2, 1
  beqz   t2, 2f
  li     t3, 63
  sub    t3, t3, t1
  srl    t3, RS1, t3
  xor    t0, t0, t3
2:
  addi   t1, t1, 1
  srli   RS2, RS2, 1
  j      1b
3:
  WRITE_RD(t0, t1)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_FAR_JUMP_ABS:
  DECODE_U
  ld     t0, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_PC(MACHINE)
  li     t1, 1
  slli   t1, t1, 3
  add    t1, t1, MACHINE
  sd     t0, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(t1)
  andi   t0, IMMEDIATE, -2
  sd     t0, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_PC(MACHINE)
  j      .prepare_trace
.CKB_VM_ASM_LABEL_OP_FAR_JUMP_REL:
  DECODE_U
  ld     t0, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_PC(MACHINE)
  li     t1, 1
  slli   t1, t1, 3
  add    t1, t1, MACHINE
  sd     t0, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(t1)
  ld     t1, -16(INST_ARGS)
  srai   t1, t1, 24
  andi   t1, t1, 0x0f
  slli   t1, t1, 1
  sub    t0, t0, t1
  add    t0, t0, IMMEDIATE
  andi   t0, t0, -2
  sd     t0, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_PC(MACHINE)
  j      .prepare_trace
.CKB_VM_ASM_LABEL_OP_WIDE_MUL:
  DECODE_R4
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  mul    t0, RS1, RS2
  mulh   t1, RS1, RS2
  WRITE_RS3(t0, t2)
  WRITE_RD(t1, t2)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_WIDE_MULU:
  DECODE_R4
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  mul    t0, RS1, RS2
  mulhu  t1, RS1, RS2
  WRITE_RS3(t0, t2)
  WRITE_RD(t1, t2)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_WIDE_MULSU:
  DECODE_R4
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  mul    t0, RS1, RS2
  mulhsu t1, RS1, RS2
  WRITE_RS3(t0, t2)
  WRITE_RD(t1, t2)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_WIDE_DIV:
  DECODE_R4
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  div    t0, RS1, RS2
  rem    t1, RS1, RS2
  WRITE_RS3(t1, t2)
  WRITE_RD(t0, t2)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_WIDE_DIVU:
  DECODE_R4
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  divu   t0, RS1, RS2
  remu   t1, RS1, RS2
  WRITE_RS3(t1, t2)
  WRITE_RD(t0, t2)
  NEXT_INST
.CKB_VM_ASM_LABEL_OP_ADC:
  DECODE_R
  LOAD_REGISTER_ADDRESS(RD, t3)
  ld     t0, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(t3)
  LOAD_REGISTER_ADDRESS(RS1, t3)
  ld     t1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(t3)
  LOAD_REGISTER_ADDRESS(RS2, t3)
  ld     t2, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(t3)
  add    t3, t0, t1
  sltu   t4, t3, t0
  add    t0, t3, t2
  sltu   t1, t0, t3
  or     t2, t4, t1
  WRITE_RS1(t2, t3)
  WRITE_RS2(t1, t3)
  WRITE_RD_V2(t0, t3)
  NEXT_INST_V2
.CKB_VM_ASM_LABEL_OP_SBB:
  DECODE_R4
  LOAD_REGISTER_ADDRESS(RD, t3)
  ld     t0, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(t3)
  LOAD_REGISTER_ADDRESS(RS1, t3)
  ld     t1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(t3)
  LOAD_REGISTER_ADDRESS(RS2, t3)
  ld     t2, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(t3)
  sltu   t3, t0, t1
  sub    t4, t0, t1
  sltu   t5, t4, t2
  sub    t0, t4, t2
  or     t1, t3, t5
  WRITE_RS2(t5, t2)
  WRITE_RS3(t3, t2)
  WRITE_RS1(t1, t2)
  WRITE_RD_V2(t0, t2)
  NEXT_INST_V2
.CKB_VM_ASM_LABEL_OP_ADCS:
  DECODE_R4
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  add    t0, RS1, RS2
  sltu   t1, t0, RS1
  WRITE_RD_V2(t0, t2)
  WRITE_RS3(t1, t2)
  NEXT_INST_V2
.CKB_VM_ASM_LABEL_OP_SBBS:
  DECODE_R4
  LOAD_RS1(t0)
  LOAD_RS2(t0)
  sltu   t1, RS1, RS2
  sub    t0, RS1, RS2
  WRITE_RD_V2(t0, t2)
  WRITE_RS3(t1, t2)
  NEXT_INST_V2
.CKB_VM_ASM_LABEL_OP_ADD3A:
  DECODE_R5
  LOAD_REGISTER_ADDRESS(RD, t0)
  ld     t0, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(t0)
  LOAD_REGISTER_ADDRESS(RS1, t1)
  ld     t1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(t1)
  LOAD_REGISTER_ADDRESS(RS4, t2)
  ld     t2, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(t2)
  add    t3, t0, t1
  sltu   t4, t3, t0
  add    t5, t4, t2
  WRITE_RD_V2(t3, t0)
  WRITE_RS2(t4, t0)
  WRITE_RS3(t5, t0)
  NEXT_INST_V2
.CKB_VM_ASM_LABEL_OP_ADD3B:
  DECODE_R5
  LOAD_REGISTER_ADDRESS(RS1, t0)
  ld     t0, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(t0)
  LOAD_REGISTER_ADDRESS(RS2, t1)
  ld     t1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(t1)
  LOAD_REGISTER_ADDRESS(RS4, t2)
  ld     t2, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(t2)
  add    t3, t0, t1
  sltu   t4, t3, t0
  add    t5, t4, t2
  WRITE_RD_V2(t3, t0)
  WRITE_RS1(t4, t0)
  WRITE_RS3(t5, t0)
  NEXT_INST_V2
.CKB_VM_ASM_LABEL_OP_ADD3C:
  DECODE_R5
  LOAD_REGISTER_ADDRESS(RS1, t0)
  ld     t0, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(t0)
  LOAD_REGISTER_ADDRESS(RS2, t1)
  ld     t1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(t1)
  LOAD_REGISTER_ADDRESS(RS4, t2)
  ld     t2, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(t2)
  add    t3, t0, t1
  sltu   t4, t3, t0
  add    t5, t2, t4
  WRITE_RD_V2(t3, t0)
  WRITE_RS3(t5, t0)
  NEXT_INST_V2
.CKB_VM_ASM_LABEL_OP_UNLOADED:
  li     a0, CKB_VM_ASM_RET_DECODE_TRACE
  j      .exit
.exit_cycles_overflow:
  li     a0, CKB_VM_ASM_RET_CYCLES_OVERFLOW
  j      .exit
.exit_invalid_permission:
  sd     ERROR_ARG0, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_ERROR_ARG0(MACHINE)
  li     a0, CKB_VM_ASM_RET_INVALID_PERMISSION
  j      .exit
.exit_max_cycles_exceeded:
  li     a0, CKB_VM_ASM_RET_MAX_CYCLES_EXCEEDED
  j      .exit
.exit_pause:
  li     a0, CKB_VM_ASM_RET_PAUSE
  j      .exit
.exit_trace:
  li     a0, CKB_VM_ASM_RET_DECODE_TRACE
  j      .exit
.exit_out_of_bound:
  sd     ERROR_ARG0, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_ERROR_ARG0(MACHINE)
  li     a0, CKB_VM_ASM_RET_OUT_OF_BOUND
  j      .exit
.exit:
  ld     ra, 0(sp)
  ld     s0, 8(sp)
  ld     s1, 16(sp)
  ld     s2, 24(sp)
  ld     s3, 32(sp)
  ld     s4, 40(sp)
  ld     s5, 48(sp)
  ld     s6, 56(sp)
  addi   sp, sp, 64
  ret
