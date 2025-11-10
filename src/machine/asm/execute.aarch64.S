/*
 * ARM64/AArch64 implementation of RISC-V VM
 * Direct-threaded interpreter for Apple Silicon (M1/M2/M3)
 */
#define CKB_VM_ASM_GENERATE_LABEL_TABLES 1
#include "cdefinitions_generated.h"

#define INT64_MIN 0x8000000000000000
#define UINT64_MAX 0xffffffffffffffff

/* ARM64 calling convention: x0 is first argument */
#define ARG1 x0
#define ARG_RETw w0

/*
 * Register allocation for ARM64:
 * x19-x28 are callee-saved, so we use them for persistent values
 */
#define MACHINE x19
#define TRACE x20
#define INST_PC x21
#define INST_ARGS x22

/* Temporary registers (caller-saved)
 * Note: RD and RS2s share the same register, matching x86 behavior
 */
#define RD x9
#define RDw w9
#define RS1 x10
#define RS1w w10
#define RS2r x11
#define RS2rw w11
#define RS2s x9        /* Shares register with RD */
#define RS2sw w9
#define IMMEDIATE x12
#define IMMEDIATEw w12
#define TEMP1 x13
#define TEMP1w w13
#define TEMP2 x14
#define TEMP2w w14
#define TEMP3 x15
#define TEMP3w w15
#define TEMP4 x16
#define TEMP4w w16

/* Memory offset macros */
#define PC_ADDRESS \
  CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_PC

#define ZERO_ADDRESS \
  CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS

#define SP_ADDRESS \
  (CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS + CKB_VM_ASM_REGISTER_SP * 8)

/* Helper macros for register access
 * Loads value from machine register[idx] into dst
 */
#define LOAD_REG(dst, idx) \
  ldr dst, [MACHINE, idx, lsl #3]

/* Use assembly macros instead of C preprocessor for multi-line sequences */
.macro WRITE_RD reg
  str \reg, [MACHINE, RD, lsl #3]
  str xzr, [MACHINE]
.endm

.macro NEXT_INST
  ldr IMMEDIATE, [INST_ARGS], #8
  ubfx RDw, IMMEDIATEw, #8, #8
  asr IMMEDIATE, IMMEDIATE, #32
  ldr TEMP3, [INST_PC], #8
  br TEMP3
.endm

/* In x86, %rcx holds the packed immediate after shifting,
 * low byte is RS1, next byte is RS2 for R-type
 */
.macro DECODE_R
  and RS1w, IMMEDIATEw, #0xFF
  ubfx RS2rw, IMMEDIATEw, #8, #8
.endm

.macro DECODE_I
  and RS1w, IMMEDIATEw, #0xFF
  asr IMMEDIATE, IMMEDIATE, #8
.endm

.macro DECODE_S
  DECODE_I
.endm

.macro DECODE_U
.endm

.macro CHECK_WRITE_PERMISSION addr_reg temp1 temp2 temp2w length
  lsr \temp1, \addr_reg, #CKB_VM_ASM_RISCV_PAGE_SHIFTS
  mov TEMP3w, #CKB_VM_ASM_RISCV_PAGES
  cmp \temp1, TEMP3
  b.hs .exit_out_of_bound
  ldr TEMP3, =CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FLAGS
  add TEMP3, MACHINE, TEMP3
  add \temp2, TEMP3, \temp1
  ldrb \temp2w, [\temp2]
  and \temp2w, \temp2w, #CKB_VM_ASM_MEMORY_FLAG_WXORX_BIT
  cmp \temp2w, #CKB_VM_ASM_MEMORY_FLAG_WRITABLE
  b.ne .exit_invalid_permission
  add \temp1, \temp1, #1
  lsl \temp1, \temp1, #CKB_VM_ASM_RISCV_PAGE_SHIFTS
  add \temp2, \addr_reg, #\length
  cmp \temp2, \temp1
  b.ls 1f
  lsr \temp1, \temp1, #CKB_VM_ASM_RISCV_PAGE_SHIFTS
  mov TEMP3w, #CKB_VM_ASM_RISCV_PAGES
  cmp \temp1, TEMP3
  b.hs .exit_out_of_bound
  ldr TEMP3, =CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FLAGS
  add TEMP3, MACHINE, TEMP3
  add \temp2, TEMP3, \temp1
  ldrb \temp2w, [\temp2]
  and \temp2w, \temp2w, #CKB_VM_ASM_MEMORY_FLAG_WXORX_BIT
  cmp \temp2w, #CKB_VM_ASM_MEMORY_FLAG_WRITABLE
  b.ne .exit_invalid_permission
1:
.endm

#ifdef __APPLE__
.globl _ckb_vm_aarch64_execute
.p2align 2
_ckb_vm_aarch64_execute:
#else
.globl ckb_vm_aarch64_execute
.p2align 2
ckb_vm_aarch64_execute:
#endif
  /* Save callee-saved registers */
  stp x29, x30, [sp, #-80]!
  stp x19, x20, [sp, #16]
  stp x21, x22, [sp, #32]
  stp x23, x24, [sp, #48]
  stp x25, x26, [sp, #64]

  mov MACHINE, ARG1

.p2align 3
.CKB_VM_ASM_LABEL_OP_CUSTOM_TRACE_END:
.prepare_trace:
  /* Load PC and calculate trace slot */
  ldr x0, [MACHINE, #PC_ADDRESS]
  mov w1, w0
  lsr x2, x0, #9
  add x2, x2, x0
  lsr x2, x2, #1
  and x2, x2, #8191
  mov x3, #CKB_VM_ASM_TRACE_STRUCT_SIZE
  mul x2, x2, x3
  /* Load large offset into register using literal pool */
  ldr x3, =CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_TRACES
  add TRACE, MACHINE, x3
  add TRACE, TRACE, x2

  /* Check if trace is valid */
  ldr x3, [TRACE, #CKB_VM_ASM_TRACE_OFFSET_ADDRESS]
  cmp w1, w3
  b.ne .exit_trace
  ldrb w3, [TRACE, #CKB_VM_ASM_TRACE_OFFSET_LENGTH]
  cbz w3, .exit_trace

  /* Check cycles */
  ldr x0, [MACHINE, #CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_CYCLES]
  ldr x2, [TRACE, #CKB_VM_ASM_TRACE_OFFSET_CYCLES]
  add x0, x0, x2
  ldr x2, [MACHINE, #CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MAX_CYCLES]
  cmp x0, x2
  b.hi .exit_max_cycles_exceeded
  str x0, [MACHINE, #CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_CYCLES]

  /* Update PC and setup instruction pointers */
  ldr x2, [MACHINE, #PC_ADDRESS]
  add x2, x2, x3
  str x2, [MACHINE, #PC_ADDRESS]
  add INST_ARGS, TRACE, #CKB_VM_ASM_TRACE_OFFSET_INSTRUCTIONS
  add INST_PC, TRACE, #CKB_VM_ASM_TRACE_OFFSET_THREAD
  NEXT_INST

/* Arithmetic Instructions */
.p2align 3
.CKB_VM_ASM_LABEL_OP_ADDI:
.CKB_VM_ASM_LABEL_OP_RVC_ADDI:
  DECODE_I
  LOAD_REG(RS1, RS1)
  add RS1, RS1, IMMEDIATE
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_ADD:
.CKB_VM_ASM_LABEL_OP_RVC_ADD:
  DECODE_R
  LOAD_REG(RS1, RS1)
  LOAD_REG(RS2r, RS2r)
  add RS1, RS1, RS2r
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_ADDIW:
.CKB_VM_ASM_LABEL_OP_RVC_ADDIW:
  DECODE_I
  LOAD_REG(RS1, RS1)
  add RS1w, RS1w, IMMEDIATEw
  sxtw RS1, RS1w
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_ADDW:
.CKB_VM_ASM_LABEL_OP_RVC_ADDW:
  DECODE_R
  LOAD_REG(RS1, RS1)
  LOAD_REG(RS2r, RS2r)
  add RS1w, RS1w, RS2rw
  sxtw RS1, RS1w
  WRITE_RD RS1
  NEXT_INST

/* Logical Instructions */
.p2align 3
.CKB_VM_ASM_LABEL_OP_AND:
.CKB_VM_ASM_LABEL_OP_RVC_AND:
  DECODE_R
  LOAD_REG(RS1, RS1)
  LOAD_REG(RS2r, RS2r)
  and RS1, RS1, RS2r
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_ANDI:
.CKB_VM_ASM_LABEL_OP_RVC_ANDI:
  DECODE_I
  LOAD_REG(RS1, RS1)
  and RS1, RS1, IMMEDIATE
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_OR:
.CKB_VM_ASM_LABEL_OP_RVC_OR:
  DECODE_R
  LOAD_REG(RS1, RS1)
  LOAD_REG(RS2r, RS2r)
  orr RS1, RS1, RS2r
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_ORI:
  DECODE_I
  LOAD_REG(RS1, RS1)
  orr RS1, RS1, IMMEDIATE
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_XOR:
.CKB_VM_ASM_LABEL_OP_RVC_XOR:
  DECODE_R
  LOAD_REG(RS1, RS1)
  LOAD_REG(RS2r, RS2r)
  eor RS1, RS1, RS2r
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_XORI:
  DECODE_I
  LOAD_REG(RS1, RS1)
  eor RS1, RS1, IMMEDIATE
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_SUB:
.CKB_VM_ASM_LABEL_OP_RVC_SUB:
  DECODE_R
  LOAD_REG(RS1, RS1)
  LOAD_REG(RS2r, RS2r)
  sub RS1, RS1, RS2r
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_SUBW:
.CKB_VM_ASM_LABEL_OP_RVC_SUBW:
  DECODE_R
  LOAD_REG(RS1, RS1)
  LOAD_REG(RS2r, RS2r)
  sub RS1w, RS1w, RS2rw
  sxtw RS1, RS1w
  WRITE_RD RS1
  NEXT_INST

/* Upper Immediate */
.p2align 3
.CKB_VM_ASM_LABEL_OP_AUIPC:
  DECODE_U
  ldr RS1, [MACHINE, #PC_ADDRESS]
  sub RS1, RS1, #4
  add RS1, RS1, IMMEDIATE
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_LUI:
.CKB_VM_ASM_LABEL_OP_RVC_LI:
.CKB_VM_ASM_LABEL_OP_RVC_LUI:
.CKB_VM_ASM_LABEL_OP_CUSTOM_LOAD_IMM:
  DECODE_U
  WRITE_RD IMMEDIATE
  NEXT_INST

/* Branch Instructions */
.p2align 3
.CKB_VM_ASM_LABEL_OP_BEQ:
  DECODE_S
  LOAD_REG(RS1, RS1)
  LOAD_REG(RS2s, RD)
  cmp RS1, RS2s
  b.eq .i_branch_success
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_BNE:
  DECODE_S
  LOAD_REG(RS1, RS1)
  LOAD_REG(RS2s, RD)
  cmp RS1, RS2s
  b.ne .i_branch_success
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_BLT:
  DECODE_S
  LOAD_REG(RS1, RS1)
  LOAD_REG(RS2s, RD)
  cmp RS1, RS2s
  b.lt .i_branch_success
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_BGE:
  DECODE_S
  LOAD_REG(RS1, RS1)
  LOAD_REG(RS2s, RD)
  cmp RS1, RS2s
  b.ge .i_branch_success
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_BLTU:
  DECODE_S
  LOAD_REG(RS1, RS1)
  LOAD_REG(RS2s, RD)
  cmp RS1, RS2s
  b.lo .i_branch_success
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_BGEU:
  DECODE_S
  LOAD_REG(RS1, RS1)
  LOAD_REG(RS2s, RD)
  cmp RS1, RS2s
  b.hs .i_branch_success
  NEXT_INST

.i_branch_success:
  ldr RS1, [MACHINE, #PC_ADDRESS]
  sub RS1, RS1, #4
  add RS1, RS1, IMMEDIATE
  str RS1, [MACHINE, #PC_ADDRESS]
  b .prepare_trace

/* Jump Instructions */
.p2align 3
.CKB_VM_ASM_LABEL_OP_JAL:
  DECODE_U
  ldr RS1, [MACHINE, #PC_ADDRESS]
  WRITE_RD RS1
  sub RS1, RS1, #4
  add RS1, RS1, IMMEDIATE
  str RS1, [MACHINE, #PC_ADDRESS]
  b .prepare_trace

.p2align 3
.CKB_VM_ASM_LABEL_OP_JALR:
  DECODE_I
  ldr TEMP1, [MACHINE, #PC_ADDRESS]
  WRITE_RD TEMP1
  LOAD_REG(TEMP1, RS1)
  add TEMP1, TEMP1, IMMEDIATE
  and TEMP1, TEMP1, #-2
  str TEMP1, [MACHINE, #PC_ADDRESS]
  b .prepare_trace

/* Load Instructions */
.p2align 3
.CKB_VM_ASM_LABEL_OP_LB:
  DECODE_I
  LOAD_REG(RS1, RS1)
  add RS1, RS1, IMMEDIATE
  mov TEMP1, RS1
  ldr TEMP3, =CKB_VM_ASM_RISCV_MAX_MEMORY
  cmp TEMP1, TEMP3
  b.hs .exit_out_of_bound
  add TEMP1, TEMP1, #1
  cmp TEMP1, TEMP3
  b.hs .exit_out_of_bound
  ldr TEMP2, =CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY
  add TEMP1, MACHINE, TEMP2
  ldrsb RS1, [TEMP1, RS1]
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_LBU:
  DECODE_I
  LOAD_REG(RS1, RS1)
  add RS1, RS1, IMMEDIATE
  mov TEMP1, RS1
  ldr TEMP3, =CKB_VM_ASM_RISCV_MAX_MEMORY
  cmp TEMP1, TEMP3
  b.hs .exit_out_of_bound
  add TEMP1, TEMP1, #1
  ldr TEMP3, =CKB_VM_ASM_RISCV_MAX_MEMORY
  cmp TEMP1, TEMP3
  b.hs .exit_out_of_bound
  ldr TEMP2, =CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY
  add TEMP1, MACHINE, TEMP2
  ldrb RS1w, [TEMP1, RS1]
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_LH:
  DECODE_I
  LOAD_REG(RS1, RS1)
  add RS1, RS1, IMMEDIATE
  mov TEMP1, RS1
  ldr TEMP3, =CKB_VM_ASM_RISCV_MAX_MEMORY
  cmp TEMP1, TEMP3
  b.hs .exit_out_of_bound
  add TEMP1, TEMP1, #2
  ldr TEMP3, =CKB_VM_ASM_RISCV_MAX_MEMORY
  cmp TEMP1, TEMP3
  b.hs .exit_out_of_bound
  ldr TEMP2, =CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY
  add TEMP1, MACHINE, TEMP2
  ldrsh RS1, [TEMP1, RS1]
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_LHU:
  DECODE_I
  LOAD_REG(RS1, RS1)
  add RS1, RS1, IMMEDIATE
  mov TEMP1, RS1
  ldr TEMP3, =CKB_VM_ASM_RISCV_MAX_MEMORY
  cmp TEMP1, TEMP3
  b.hs .exit_out_of_bound
  add TEMP1, TEMP1, #2
  ldr TEMP3, =CKB_VM_ASM_RISCV_MAX_MEMORY
  cmp TEMP1, TEMP3
  b.hs .exit_out_of_bound
  ldr TEMP2, =CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY
  add TEMP1, MACHINE, TEMP2
  ldrh RS1w, [TEMP1, RS1]
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_LW:
.CKB_VM_ASM_LABEL_OP_RVC_LW:
  DECODE_I
  LOAD_REG(RS1, RS1)
  add RS1, RS1, IMMEDIATE
  mov TEMP1, RS1
  ldr TEMP3, =CKB_VM_ASM_RISCV_MAX_MEMORY
  cmp TEMP1, TEMP3
  b.hs .exit_out_of_bound
  add TEMP1, TEMP1, #4
  ldr TEMP3, =CKB_VM_ASM_RISCV_MAX_MEMORY
  cmp TEMP1, TEMP3
  b.hs .exit_out_of_bound
  ldr TEMP2, =CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY
  add TEMP1, MACHINE, TEMP2
  ldrsw RS1, [TEMP1, RS1]
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_LWU:
  DECODE_I
  LOAD_REG(RS1, RS1)
  add RS1, RS1, IMMEDIATE
  mov TEMP1, RS1
  ldr TEMP3, =CKB_VM_ASM_RISCV_MAX_MEMORY
  cmp TEMP1, TEMP3
  b.hs .exit_out_of_bound
  add TEMP1, TEMP1, #4
  ldr TEMP3, =CKB_VM_ASM_RISCV_MAX_MEMORY
  cmp TEMP1, TEMP3
  b.hs .exit_out_of_bound
  ldr TEMP2, =CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY
  add TEMP1, MACHINE, TEMP2
  ldr RS1w, [TEMP1, RS1]
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_LD:
.CKB_VM_ASM_LABEL_OP_RVC_LD:
  DECODE_I
  LOAD_REG(RS1, RS1)
  add RS1, RS1, IMMEDIATE
  mov TEMP1, RS1
  ldr TEMP3, =CKB_VM_ASM_RISCV_MAX_MEMORY
  cmp TEMP1, TEMP3
  b.hs .exit_out_of_bound
  add TEMP1, TEMP1, #8
  ldr TEMP3, =CKB_VM_ASM_RISCV_MAX_MEMORY
  cmp TEMP1, TEMP3
  b.hs .exit_out_of_bound
  ldr TEMP2, =CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY
  add TEMP1, MACHINE, TEMP2
  ldr RS1, [TEMP1, RS1]
  WRITE_RD RS1
  NEXT_INST

/* Store Instructions */
.p2align 3
.CKB_VM_ASM_LABEL_OP_SB:
  DECODE_S
  LOAD_REG(RS1, RS1)
  add RS1, RS1, IMMEDIATE
  CHECK_WRITE_PERMISSION RS1, TEMP1, RS2r, RS2rw, 1
  LOAD_REG(RS2s, RD)
  ldr TEMP2, =CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY
  add TEMP1, MACHINE, TEMP2
  strb RS2sw, [TEMP1, RS1]
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_SH:
  DECODE_S
  LOAD_REG(RS1, RS1)
  add RS1, RS1, IMMEDIATE
  CHECK_WRITE_PERMISSION RS1, TEMP1, RS2r, RS2rw, 2
  LOAD_REG(RS2s, RD)
  ldr TEMP2, =CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY
  add TEMP1, MACHINE, TEMP2
  strh RS2sw, [TEMP1, RS1]
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_SW:
.CKB_VM_ASM_LABEL_OP_RVC_SW:
  DECODE_S
  LOAD_REG(RS1, RS1)
  add RS1, RS1, IMMEDIATE
  CHECK_WRITE_PERMISSION RS1, TEMP1, RS2r, RS2rw, 4
  LOAD_REG(RS2s, RD)
  ldr TEMP2, =CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY
  add TEMP1, MACHINE, TEMP2
  str RS2sw, [TEMP1, RS1]
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_SD:
.CKB_VM_ASM_LABEL_OP_RVC_SD:
  DECODE_S
  LOAD_REG(RS1, RS1)
  add RS1, RS1, IMMEDIATE
  CHECK_WRITE_PERMISSION RS1, TEMP1, RS2r, RS2rw, 8
  LOAD_REG(RS2s, RD)
  ldr TEMP2, =CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY
  add TEMP1, MACHINE, TEMP2
  str RS2s, [TEMP1, RS1]
  NEXT_INST

/* Shift Instructions */
.p2align 3
.CKB_VM_ASM_LABEL_OP_SLL:
  DECODE_R
  LOAD_REG(TEMP1, RS1)
  LOAD_REG(TEMP2, RS2r)
  lsl TEMP1, TEMP1, TEMP2
  WRITE_RD TEMP1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_SLLI:
.CKB_VM_ASM_LABEL_OP_RVC_SLLI:
  DECODE_I
  LOAD_REG(TEMP1, RS1)
  lsl TEMP1, TEMP1, IMMEDIATE
  WRITE_RD TEMP1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_SLLIW:
  DECODE_I
  LOAD_REG(TEMP1, RS1)
  lsl TEMP1w, TEMP1w, IMMEDIATEw
  sxtw TEMP1, TEMP1w
  WRITE_RD TEMP1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_SLLW:
  DECODE_R
  LOAD_REG(TEMP1, RS1)
  LOAD_REG(TEMP2, RS2r)
  and TEMP2w, TEMP2w, #0x1F
  lsl TEMP1w, TEMP1w, TEMP2w
  sxtw TEMP1, TEMP1w
  WRITE_RD TEMP1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_SRL:
  DECODE_R
  LOAD_REG(TEMP1, RS1)
  LOAD_REG(TEMP2, RS2r)
  lsr TEMP1, TEMP1, TEMP2
  WRITE_RD TEMP1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_SRLI:
.CKB_VM_ASM_LABEL_OP_RVC_SRLI:
  DECODE_I
  LOAD_REG(TEMP1, RS1)
  lsr TEMP1, TEMP1, IMMEDIATE
  WRITE_RD TEMP1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_SRLIW:
  DECODE_I
  LOAD_REG(TEMP1, RS1)
  lsr TEMP1w, TEMP1w, IMMEDIATEw
  sxtw TEMP1, TEMP1w
  WRITE_RD TEMP1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_SRLW:
  DECODE_R
  LOAD_REG(TEMP1, RS1)
  LOAD_REG(TEMP2, RS2r)
  and TEMP2w, TEMP2w, #0x1F
  lsr TEMP1w, TEMP1w, TEMP2w
  sxtw TEMP1, TEMP1w
  WRITE_RD TEMP1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_SRA:
  DECODE_R
  LOAD_REG(TEMP1, RS1)
  LOAD_REG(TEMP2, RS2r)
  asr TEMP1, TEMP1, TEMP2
  WRITE_RD TEMP1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_SRAI:
.CKB_VM_ASM_LABEL_OP_RVC_SRAI:
  DECODE_I
  LOAD_REG(TEMP1, RS1)
  asr TEMP1, TEMP1, IMMEDIATE
  WRITE_RD TEMP1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_SRAIW:
  DECODE_I
  LOAD_REG(TEMP1, RS1)
  asr TEMP1w, TEMP1w, IMMEDIATEw
  sxtw TEMP1, TEMP1w
  WRITE_RD TEMP1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_SRAW:
  DECODE_R
  LOAD_REG(TEMP1, RS1)
  LOAD_REG(TEMP2, RS2r)
  and TEMP2w, TEMP2w, #0x1F
  asr TEMP1w, TEMP1w, TEMP2w
  sxtw TEMP1, TEMP1w
  WRITE_RD TEMP1
  NEXT_INST

/* Comparison Instructions */
.p2align 3
.CKB_VM_ASM_LABEL_OP_SLT:
  DECODE_R
  LOAD_REG(RS1, RS1)
  LOAD_REG(RS2r, RS2r)
  cmp RS1, RS2r
  cset RS1, lt
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_SLTI:
  DECODE_I
  LOAD_REG(RS1, RS1)
  cmp RS1, IMMEDIATE
  cset RS1, lt
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_SLTU:
  DECODE_R
  LOAD_REG(RS1, RS1)
  LOAD_REG(RS2r, RS2r)
  cmp RS1, RS2r
  cset RS1, lo
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_SLTIU:
  DECODE_I
  LOAD_REG(RS1, RS1)
  cmp RS1, IMMEDIATE
  cset RS1, lo
  WRITE_RD RS1
  NEXT_INST

/* Multiplication Instructions - ARM64 has built-in multiply */
.p2align 3
.CKB_VM_ASM_LABEL_OP_MUL:
  DECODE_R
  LOAD_REG(RS1, RS1)
  LOAD_REG(RS2r, RS2r)
  mul RS1, RS1, RS2r
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_MULH:
  DECODE_R
  LOAD_REG(RS1, RS1)
  LOAD_REG(RS2r, RS2r)
  smulh RS1, RS1, RS2r
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_MULHU:
  DECODE_R
  LOAD_REG(RS1, RS1)
  LOAD_REG(RS2r, RS2r)
  umulh RS1, RS1, RS2r
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_MULHSU:
  DECODE_R
  LOAD_REG(RS1, RS1)
  LOAD_REG(RS2r, RS2r)
  /* Mixed signed/unsigned multiply: handle manually */
  tbnz RS1, #63, .mulhsu_negative
  umulh RS1, RS1, RS2r
  WRITE_RD RS1
  NEXT_INST
.mulhsu_negative:
  neg TEMP1, RS1
  umulh TEMP1, TEMP1, RS2r
  mvn TEMP1, TEMP1
  mul TEMP2, RS1, RS2r
  cmp TEMP2, #0
  cinc TEMP1, TEMP1, eq
  WRITE_RD TEMP1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_MULW:
  DECODE_R
  LOAD_REG(RS1, RS1)
  LOAD_REG(RS2r, RS2r)
  mul RS1w, RS1w, RS2rw
  sxtw RS1, RS1w
  WRITE_RD RS1
  NEXT_INST

/* Division Instructions */
.p2align 3
.CKB_VM_ASM_LABEL_OP_DIV:
  DECODE_R
  LOAD_REG(RS1, RS1)
  LOAD_REG(RS2r, RS2r)
  /* Check for division by zero */
  cbz RS2r, .div_by_zero
  /* Check for overflow: INT64_MIN / -1 */
  mov x16, #INT64_MIN
  cmp RS1, x16
  b.ne .div_normal
  cmn RS2r, #1
  b.ne .div_normal
  /* Overflow case: return INT64_MIN */
  WRITE_RD RS1
  NEXT_INST
.div_by_zero:
  mov RS1, #-1
  WRITE_RD RS1
  NEXT_INST
.div_normal:
  sdiv RS1, RS1, RS2r
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_DIVU:
  DECODE_R
  LOAD_REG(RS2r, RS2r)
  cbz RS2r, .divu_by_zero
  LOAD_REG(RS1, RS1)
  udiv RS1, RS1, RS2r
  WRITE_RD RS1
  NEXT_INST
.divu_by_zero:
  mov RS2r, #-1
  WRITE_RD RS2r
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_DIVW:
  DECODE_R
  LOAD_REG(RS1, RS1)
  sxtw RS1, RS1w
  LOAD_REG(RS2r, RS2r)
  sxtw RS2r, RS2rw
  cbz RS2r, .divw_by_zero
  mov x16, #INT64_MIN
  cmp RS1, x16
  b.ne .divw_normal
  cmn RS2r, #1
  b.ne .divw_normal
  sxtw RS1, RS1w
  WRITE_RD RS1
  NEXT_INST
.divw_by_zero:
  mov RS1, #-1
  WRITE_RD RS1
  NEXT_INST
.divw_normal:
  sdiv RS1, RS1, RS2r
  sxtw RS1, RS1w
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_DIVUW:
  DECODE_R
  LOAD_REG(RS2r, RS2r)
  mov RS2rw, RS2rw
  cbz RS2r, .divuw_by_zero
  LOAD_REG(RS1, RS1)
  mov RS1w, RS1w
  udiv RS1, RS1, RS2r
  sxtw RS1, RS1w
  WRITE_RD RS1
  NEXT_INST
.divuw_by_zero:
  mov RS2r, #-1
  WRITE_RD RS2r
  NEXT_INST

/* Remainder Instructions */
.p2align 3
.CKB_VM_ASM_LABEL_OP_REM:
  DECODE_R
  LOAD_REG(RS1, RS1)
  LOAD_REG(RS2r, RS2r)
  cbz RS2r, .rem_by_zero
  mov x16, #INT64_MIN
  cmp RS1, x16
  b.ne .rem_normal
  cmn RS2r, #1
  b.ne .rem_normal
  /* Overflow case: return 0 */
  mov RS1, #0
  WRITE_RD RS1
  NEXT_INST
.rem_by_zero:
  WRITE_RD RS1
  NEXT_INST
.rem_normal:
  sdiv TEMP1, RS1, RS2r
  msub RS1, TEMP1, RS2r, RS1
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_REMU:
  DECODE_R
  LOAD_REG(RS2r, RS2r)
  cbz RS2r, .remu_by_zero
  LOAD_REG(RS1, RS1)
  udiv TEMP1, RS1, RS2r
  msub RS1, TEMP1, RS2r, RS1
  WRITE_RD RS1
  NEXT_INST
.remu_by_zero:
  LOAD_REG(RS1, RS1)
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_REMW:
  DECODE_R
  LOAD_REG(RS1, RS1)
  sxtw RS1, RS1w
  LOAD_REG(RS2r, RS2r)
  sxtw RS2r, RS2rw
  cbz RS2r, .remw_by_zero
  mov x16, #INT64_MIN
  cmp RS1, x16
  b.ne .remw_normal
  cmn RS2r, #1
  b.ne .remw_normal
  mov RS1, #0
  WRITE_RD RS1
  NEXT_INST
.remw_by_zero:
  sxtw RS1, RS1w
  WRITE_RD RS1
  NEXT_INST
.remw_normal:
  sdiv TEMP1, RS1, RS2r
  msub RS1, TEMP1, RS2r, RS1
  sxtw RS1, RS1w
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_REMUW:
  DECODE_R
  LOAD_REG(RS2r, RS2r)
  mov RS2rw, RS2rw
  cbz RS2r, .remuw_by_zero
  LOAD_REG(RS1, RS1)
  mov RS1w, RS1w
  udiv TEMP1, RS1, RS2r
  msub RS1, TEMP1, RS2r, RS1
  sxtw RS1, RS1w
  WRITE_RD RS1
  NEXT_INST
.remuw_by_zero:
  LOAD_REG(RS1, RS1)
  sxtw RS1, RS1w
  WRITE_RD RS1
  NEXT_INST

/* System Instructions */
.p2align 3
.CKB_VM_ASM_LABEL_OP_ECALL:
  DECODE_U
  mov ARG_RETw, #CKB_VM_ASM_RET_ECALL
  b .exit

.p2align 3
.CKB_VM_ASM_LABEL_OP_EBREAK:
.CKB_VM_ASM_LABEL_OP_RVC_EBREAK:
  DECODE_U
  mov ARG_RETw, #CKB_VM_ASM_RET_EBREAK
  b .exit

.p2align 3
.CKB_VM_ASM_LABEL_OP_FENCE:
.CKB_VM_ASM_LABEL_OP_FENCEI:
.CKB_VM_ASM_LABEL_OP_RVC_NOP:
.CKB_VM_ASM_LABEL_OP_RVC_SLLI64:
.CKB_VM_ASM_LABEL_OP_RVC_SRAI64:
.CKB_VM_ASM_LABEL_OP_RVC_SRLI64:
  DECODE_U
  NEXT_INST

/* Compressed Instructions - RVC */
.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_BEQZ:
  DECODE_S
  LOAD_REG(RS1, RS1)
  cbz RS1, .rvc_branch_success
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_BNEZ:
  DECODE_S
  LOAD_REG(RS1, RS1)
  cbnz RS1, .rvc_branch_success
  NEXT_INST

.rvc_branch_success:
  ldr RS1, [MACHINE, #PC_ADDRESS]
  sub RS1, RS1, #2
  add RS1, RS1, IMMEDIATE
  str RS1, [MACHINE, #PC_ADDRESS]
  b .prepare_trace

.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_J:
  DECODE_U
  ldr RS1, [MACHINE, #PC_ADDRESS]
  sub RS1, RS1, #2
  add RS1, RS1, IMMEDIATE
  str RS1, [MACHINE, #PC_ADDRESS]
  b .prepare_trace

.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_JAL:
  DECODE_U
  ldr RS1, [MACHINE, #PC_ADDRESS]
  add TEMP1, MACHINE, #(CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS + 1 * 8)
  str RS1, [TEMP1]
  sub RS1, RS1, #2
  add RS1, RS1, IMMEDIATE
  str RS1, [MACHINE, #PC_ADDRESS]
  b .prepare_trace

.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_JALR:
  DECODE_S
  ldr TEMP1, [MACHINE, #PC_ADDRESS]
  add TEMP2, MACHINE, #(CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS + 1 * 8)
  str TEMP1, [TEMP2]
  LOAD_REG(TEMP1, RS1)
  and TEMP1, TEMP1, #-2
  str TEMP1, [MACHINE, #PC_ADDRESS]
  b .prepare_trace

.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_JR:
  DECODE_S
  LOAD_REG(TEMP1, RS1)
  and TEMP1, TEMP1, #-2
  str TEMP1, [MACHINE, #PC_ADDRESS]
  b .prepare_trace

.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_ADDI16SP:
  DECODE_I
  ldr TEMP1, [MACHINE, #SP_ADDRESS]
  add TEMP1, TEMP1, IMMEDIATE
  str TEMP1, [MACHINE, #SP_ADDRESS]
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_ADDI4SPN:
  DECODE_U
  ldr RS1, [MACHINE, #SP_ADDRESS]
  add RS1, RS1, IMMEDIATE
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_LDSP:
  DECODE_U
  ldr RS1, [MACHINE, #SP_ADDRESS]
  add RS1, RS1, IMMEDIATE
  mov TEMP1, RS1
  ldr TEMP3, =CKB_VM_ASM_RISCV_MAX_MEMORY
  cmp TEMP1, TEMP3
  b.hs .exit_out_of_bound
  add TEMP1, TEMP1, #8
  ldr TEMP3, =CKB_VM_ASM_RISCV_MAX_MEMORY
  cmp TEMP1, TEMP3
  b.hs .exit_out_of_bound
  ldr TEMP2, =CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY
  add TEMP1, MACHINE, TEMP2
  ldr RS1, [TEMP1, RS1]
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_LWSP:
  DECODE_U
  ldr RS1, [MACHINE, #SP_ADDRESS]
  add RS1, RS1, IMMEDIATE
  mov TEMP1, RS1
  ldr TEMP3, =CKB_VM_ASM_RISCV_MAX_MEMORY
  cmp TEMP1, TEMP3
  b.hs .exit_out_of_bound
  add TEMP1, TEMP1, #4
  ldr TEMP3, =CKB_VM_ASM_RISCV_MAX_MEMORY
  cmp TEMP1, TEMP3
  b.hs .exit_out_of_bound
  ldr TEMP2, =CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY
  add TEMP1, MACHINE, TEMP2
  ldrsw RS1, [TEMP1, RS1]
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_MV:
  DECODE_R
  LOAD_REG(RS1, RS2r)
  WRITE_RD RS1
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_SDSP:
  DECODE_S
  ldr RS1, [MACHINE, #SP_ADDRESS]
  add RS1, RS1, IMMEDIATE
  CHECK_WRITE_PERMISSION RS1, TEMP1, RS2r, RS2rw, 8
  LOAD_REG(RS2s, RD)
  ldr TEMP2, =CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY
  add TEMP1, MACHINE, TEMP2
  str RS2s, [TEMP1, RS1]
  NEXT_INST

.p2align 3
.CKB_VM_ASM_LABEL_OP_RVC_SWSP:
  DECODE_S
  ldr RS1, [MACHINE, #SP_ADDRESS]
  add RS1, RS1, IMMEDIATE
  CHECK_WRITE_PERMISSION RS1, TEMP1, RS2r, RS2rw, 4
  LOAD_REG(RS2s, RD)
  ldr TEMP2, =CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY
  add TEMP1, MACHINE, TEMP2
  str RS2sw, [TEMP1, RS1]
  NEXT_INST

/* Exit conditions */
.p2align 3
.exit_out_of_bound:
  mov ARG_RETw, #CKB_VM_ASM_RET_OUT_OF_BOUND
  b .exit

.p2align 3
.exit_max_cycles_exceeded:
  mov ARG_RETw, #CKB_VM_ASM_RET_MAX_CYCLES_EXCEEDED
  b .exit

.exit_invalid_permission:
  mov ARG_RETw, #CKB_VM_ASM_RET_INVALID_PERMISSION
  b .exit

.p2align 3
.exit_trace:
.CKB_VM_ASM_LABEL_OP_UNLOADED:
  DECODE_U
  mov ARG_RETw, #CKB_VM_ASM_RET_DECODE_TRACE
  b .exit

.exit:
  /* Restore callee-saved registers - note MACHINE (x19) is still valid here */
  ldp x25, x26, [sp, #64]
  ldp x23, x24, [sp, #48]
  ldp x21, x22, [sp, #32]
  ldp x19, x20, [sp, #16]
  ldp x29, x30, [sp], #80
  ret

/* Label table is included from the #include at the top via CKB_VM_ASM_GENERATE_LABEL_TABLES */
