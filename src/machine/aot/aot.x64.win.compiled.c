/*
** This file has been pre-processed with DynASM.
** https://luajit.org/dynasm.html
** DynASM version 1.4.0, DynASM x64 version 1.4.0
** DO NOT EDIT! The original file is in "src/machine/aot/aot.x64.c".
*/

#line 1 "src/machine/aot/aot.x64.c"
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "dasm_proto.h"
#include "dasm_x86.h"

#include "cdefinitions_generated.h"

#if (defined(_WIN32) != 1)
#error "Wrong DynASM flags used: pass -D WIN to dynasm.lua to generate windows specific file"
#endif

#define ERROR_INVALID_MEMORY_SIZE 0xFFFFFF00
#define ERROR_NOT_ENOUGH_LABELS 0xFFFFFF01
#define ERROR_INVALID_VALUE 0xFFFFFF02

//|.arch x64
#if DASM_VERSION != 10400
#error "Version mismatch between DynASM and included encoding engine"
#endif
#line 19 "src/machine/aot/aot.x64.c"
//|.section code
#define DASM_SECTION_CODE	0
#define DASM_MAXSECTION		1
#line 20 "src/machine/aot/aot.x64.c"
//|.globals lbl_
enum {
  lbl_check_write,
  lbl_check_read,
  lbl_exit,
  lbl__MAX
};
#line 21 "src/machine/aot/aot.x64.c"
//|.actionlist bf_actions
static const unsigned char bf_actions[2197] = {
  254,0,65,84,65,85,65,86,65,87,83,85,87,86,72,137,207,72,137,208,72,139,183,
  233,76,139,135,233,76,139,143,233,252,255,224,255,248,10,86,65,80,72,137,
  214,72,137,193,72,193,252,233,235,255,72,129,252,249,239,15,131,244,249,72,
  141,151,233,15,182,20,10,129,226,239,129,252,250,239,15,133,244,250,255,72,
  141,151,233,68,15,182,4,10,65,129,200,239,68,136,4,10,255,72,193,252,233,
  235,72,141,151,233,68,15,182,4,10,65,131,252,248,0,15,133,244,247,198,4,10,
  1,87,86,80,81,82,65,80,65,81,65,82,85,72,137,229,72,131,228,252,240,72,131,
  252,236,32,72,137,252,250,72,137,201,72,184,237,237,252,255,208,72,131,196,
  32,72,137,252,236,93,65,90,65,89,65,88,90,89,88,94,95,248,1,255,72,137,194,
  72,1,252,242,72,131,252,234,1,72,193,252,234,235,72,131,193,1,72,57,209,15,
  133,244,248,255,72,141,151,233,68,15,182,4,10,65,129,200,239,68,136,4,10,
  72,193,252,233,235,72,141,151,233,68,15,182,4,10,65,131,252,248,0,15,133,
  244,248,198,4,10,1,87,86,80,81,82,65,80,65,81,65,82,85,72,137,229,72,131,
  228,252,240,72,131,252,236,32,72,137,252,250,72,137,201,72,184,237,237,252,
  255,208,72,131,196,32,72,137,252,236,93,65,90,65,89,65,88,90,89,88,94,95,
  248,2,72,199,194,0,0,0,0,65,88,94,195,248,3,72,199,194,237,65,88,94,195,248,
  4,72,199,194,237,65,88,94,195,255,248,11,86,65,80,72,137,193,72,193,252,233,
  235,72,129,252,249,239,15,131,244,249,72,141,183,233,68,15,182,4,14,65,131,
  252,248,0,15,133,244,247,198,4,14,1,87,86,80,81,82,65,80,65,81,65,82,85,72,
  137,229,72,131,228,252,240,72,131,252,236,32,72,137,252,250,72,137,201,72,
  184,237,237,252,255,208,72,131,196,32,72,137,252,236,93,65,90,65,89,65,88,
  90,89,88,94,95,248,1,72,137,193,72,1,209,72,131,252,233,1,72,193,252,233,
  235,72,129,252,249,239,15,131,244,249,68,15,182,4,14,65,131,252,248,0,15,
  133,244,248,198,4,14,1,87,86,80,81,82,65,80,65,81,65,82,85,72,137,229,72,
  131,228,252,240,72,131,252,236,32,72,137,252,250,72,137,201,72,184,237,237,
  255,252,255,208,72,131,196,32,72,137,252,236,93,65,90,65,89,65,88,90,89,88,
  94,95,252,233,244,248,248,2,72,199,194,0,0,0,0,65,88,94,195,248,3,72,199,
  194,237,65,88,94,195,255,248,12,72,137,183,233,76,137,135,233,76,137,143,
  233,94,95,93,91,65,95,65,94,65,93,65,92,195,255,249,255,72,1,192,240,131,
  240,35,255,72,3,135,253,240,131,233,255,72,1,135,253,240,131,233,255,72,139,
  135,233,72,1,135,233,255,72,1,192,240,35,255,72,129,192,240,35,239,255,72,
  129,135,233,239,255,72,41,192,240,131,240,35,255,72,43,135,253,240,131,233,
  255,72,41,135,253,240,131,233,255,72,139,135,233,72,41,135,233,255,72,41,
  192,240,35,255,72,129,232,240,35,239,255,72,129,175,233,239,255,72,15,175,
  192,240,36,255,72,15,175,135,233,255,72,185,237,237,255,72,199,193,237,255,
  72,15,175,193,255,72,137,192,240,35,255,72,137,135,233,255,72,252,247,232,
  240,35,255,72,252,247,175,233,255,72,252,247,224,240,35,255,72,252,247,167,
  233,255,72,252,247,252,233,255,72,252,247,225,255,72,137,208,240,35,255,72,
  137,151,233,255,72,133,192,15,137,244,247,255,72,252,247,216,255,72,131,252,
  242,252,255,72,137,209,255,72,186,237,237,255,72,199,194,237,255,72,15,175,
  194,255,72,133,192,15,148,208,72,15,182,192,255,72,1,200,252,233,244,248,
  255,72,137,208,248,2,255,72,57,200,15,133,244,247,72,199,192,252,255,252,
  255,252,255,252,255,255,72,57,200,15,133,244,247,255,252,233,244,249,255,
  248,1,72,199,192,0,0,0,0,255,72,57,200,15,133,244,248,255,72,199,192,240,
  35,237,255,72,199,135,233,237,255,252,233,244,249,248,2,255,72,153,255,72,
  252,247,252,248,240,35,255,72,252,247,191,233,255,72,49,210,255,72,252,247,
  252,240,240,35,255,72,252,247,183,233,255,72,153,72,252,247,252,249,255,72,
  49,210,72,252,247,252,241,255,72,153,72,252,247,252,248,240,35,255,72,49,
  210,72,252,247,252,240,240,35,255,248,3,255,72,33,192,240,131,240,35,255,
  72,35,135,253,240,131,233,255,72,33,135,253,240,131,233,255,72,139,135,233,
  72,33,135,233,255,72,33,192,240,35,255,72,129,224,240,35,239,255,72,129,167,
  233,239,255,72,9,192,240,131,240,35,255,72,11,135,253,240,131,233,255,72,
  9,135,253,240,131,233,255,72,139,135,233,72,9,135,233,255,72,9,192,240,35,
  255,72,129,200,240,35,239,255,72,129,143,233,239,255,72,252,247,208,240,35,
  255,72,252,247,151,233,255,72,49,192,240,131,240,35,255,72,51,135,253,240,
  131,233,255,72,49,135,253,240,131,233,255,72,139,135,233,72,49,135,233,255,
  72,49,192,240,35,255,72,129,252,240,240,35,239,255,72,129,183,233,239,255,
  72,137,193,240,131,255,72,139,143,233,255,185,237,255,72,211,224,240,35,255,
  72,211,167,233,255,72,211,252,248,240,35,255,72,211,191,233,255,72,211,232,
  240,35,255,72,211,175,233,255,49,201,77,49,210,248,1,255,72,211,224,72,211,
  252,234,76,49,208,72,131,226,1,76,15,69,208,72,131,193,1,72,131,252,249,64,
  15,133,244,1,255,76,137,208,240,35,255,76,137,151,233,255,73,199,195,1,0,
  0,0,77,49,210,248,1,255,72,199,193,64,0,0,0,76,41,217,72,211,232,76,137,217,
  72,211,252,234,76,49,208,72,131,226,1,76,15,69,208,73,131,195,1,73,131,252,
  251,64,15,133,244,1,255,77,49,219,77,49,210,248,1,255,72,199,193,63,0,0,0,
  76,41,217,72,211,232,76,137,217,72,211,252,234,76,49,208,72,131,226,1,76,
  15,69,208,73,131,195,1,73,131,252,251,64,15,133,244,1,255,72,49,210,73,186,
  237,237,77,137,211,73,33,194,15,132,244,247,76,9,218,248,1,73,186,237,237,
  77,137,211,73,33,194,15,132,244,248,76,9,218,248,2,73,186,237,237,77,137,
  211,73,33,194,15,132,244,249,76,9,218,248,3,73,186,237,237,77,137,211,73,
  33,194,15,132,244,250,255,76,9,218,248,4,73,186,237,237,77,137,211,73,33,
  194,15,132,244,251,76,9,218,248,5,73,186,237,237,77,137,211,73,33,194,15,
  132,244,252,76,9,218,248,6,73,186,237,237,77,137,211,73,33,194,15,132,244,
  253,76,9,218,248,7,73,186,237,237,77,137,211,73,33,194,15,132,244,254,255,
  76,9,218,248,8,255,72,49,210,73,186,237,237,73,33,194,73,193,226,56,76,9,
  210,73,186,237,237,73,33,194,73,193,226,40,76,9,210,73,186,237,237,73,33,
  194,73,193,226,24,76,9,210,73,186,237,237,73,33,194,73,193,226,8,76,9,210,
  73,186,237,237,73,33,194,73,193,252,234,8,76,9,210,73,186,237,237,73,33,194,
  73,193,252,234,24,76,9,210,73,186,237,237,73,33,194,73,193,252,234,40,76,
  9,210,73,186,237,237,73,33,194,73,193,252,234,56,76,9,210,255,72,211,192,
  240,35,255,72,211,135,233,255,72,211,200,240,35,255,72,211,143,233,255,72,
  57,193,240,131,255,72,59,143,233,255,72,199,192,237,255,72,57,193,255,15,
  148,209,72,15,182,201,255,72,137,200,240,35,255,72,137,143,233,255,15,156,
  209,255,15,146,209,255,72,57,192,240,35,255,72,57,135,233,255,72,129,252,
  248,240,35,239,255,72,129,191,233,239,255,252,233,244,248,248,1,255,72,131,
  252,248,240,35,1,15,133,244,247,255,72,131,252,248,0,15,132,244,247,72,15,
  189,192,72,252,247,216,72,131,192,63,255,252,233,244,248,248,2,255,72,131,
  252,248,0,15,132,244,247,72,15,188,192,255,72,137,194,72,209,252,234,72,185,
  237,237,72,33,202,72,41,208,72,137,194,72,185,237,237,72,33,202,72,193,232,
  2,72,33,200,72,1,208,72,137,194,72,193,252,234,4,72,1,208,72,185,237,237,
  72,33,200,72,137,194,72,193,252,234,8,72,1,208,72,137,194,72,193,252,234,
  16,72,1,208,72,137,194,72,193,252,234,32,72,1,208,72,131,224,127,255,72,99,
  192,255,137,192,255,185,64,0,0,0,131,226,63,41,209,72,211,224,137,209,255,
  72,211,252,248,255,72,211,232,255,72,193,224,235,255,72,193,252,248,235,255,
  72,193,232,235,255,72,137,194,240,131,185,64,0,0,0,131,226,63,41,209,72,211,
  224,137,209,255,72,199,192,237,252,233,244,12,255,72,3,135,233,15,131,244,
  247,255,248,1,72,59,135,233,15,134,244,248,255,248,2,72,137,135,233,255,72,
  199,135,233,237,252,233,245,255,72,137,135,253,240,131,233,255,72,199,194,
  237,232,244,10,72,131,252,250,0,15,133,244,247,72,141,151,233,255,136,12,
  2,255,102,137,12,2,255,72,137,12,2,255,252,233,244,248,248,1,72,137,208,252,
  233,244,12,248,2,255,72,199,194,237,232,244,11,72,131,252,250,0,15,133,244,
  247,72,137,194,72,129,194,239,15,130,244,247,72,129,252,250,239,255,15,135,
  244,247,255,15,182,12,2,255,15,183,12,2,255,139,12,2,255,72,139,12,2,255,
  72,137,192,240,131,240,35,255,72,139,135,253,240,131,233,255,72,139,135,253,
  240,131,233,72,137,135,253,240,131,233,255,72,184,240,34,237,237,255
};

#line 22 "src/machine/aot/aot.x64.c"

typedef struct {
  dasm_State* d;
  void* labels[lbl__MAX];
  uint32_t npc;
  uint32_t version;
} AotContext;

/*
 * RISC-V has 32 general purpose registers, rax, rcx * and rdx are set aside
 * for x64 level work. PC is handled separately so we can inline jumps.
 * Besides that, we also have other temporary registers for handling AST
 * intermediate nodes.
 *
 */
#define REGISTER_ZERO 0
#define REGISTER_RA 1
#define REGISTER_SP 2
#define REGISTER_GP 3
#define REGISTER_TP 4
#define REGISTER_T0 5
#define REGISTER_T1 6
#define REGISTER_T2 7
#define REGISTER_S0 8
#define REGISTER_S1 9
#define REGISTER_A0 10
#define REGISTER_A1 11
#define REGISTER_A2 12
#define REGISTER_A3 13
#define REGISTER_A4 14
#define REGISTER_A5 15
#define REGISTER_A6 16
#define REGISTER_A7 17
#define REGISTER_S2 18
#define REGISTER_S3 19
#define REGISTER_S4 20
#define REGISTER_S5 21
#define REGISTER_S6 22
#define REGISTER_S7 23
#define REGISTER_S8 24
#define REGISTER_S9 25
#define REGISTER_S10 26
#define REGISTER_S11 27
#define REGISTER_T3 28
#define REGISTER_T4 29
#define REGISTER_T5 30
#define REGISTER_T6 31
#define REGISTER_TEMP1 32
#define REGISTER_TEMP2 33
#define REGISTER_TEMP3 34
#define REGISTER_TEMP4 35
#define REGISTER_TEMP5 36
#define MAXIMUM_REGISTER 36
#define INVALID_REGISTER (MAXIMUM_REGISTER + 1)
#define VALID_REGISTER(r) ((r) <= MAXIMUM_REGISTER)

#define X64_RAX 0
#define X64_RCX 1
#define X64_RDX 2
#define X64_RBX 3
#define X64_RSP 4
#define X64_RBP 5
#define X64_RSI 6
#define X64_RDI 7
#define X64_R8 8
#define X64_R9 9
#define X64_R10 10
#define X64_R11 11
#define X64_R12 12
#define X64_R13 13
#define X64_R14 14
#define X64_R15 15
#define MAXIMUM_X64_REGISTER 15
#define INVALID_X64_REGISTER (MAXIMUM_X64_REGISTER + 1)
#define VALID_X64_REGISTER(r) ((r) <= MAXIMUM_X64_REGISTER)

typedef struct {
  uint64_t registers[32];
  uint64_t pc;
  uint64_t next_pc;
  uint8_t running;
  uint64_t cycles;
  uint64_t max_cycles;
  uint8_t chaos_mode;
  uint32_t chaos_seed;
  uint8_t reset_signal;
  uint8_t isa;
  uint32_t version;
  uint8_t flags[CKB_VM_ASM_RISCV_PAGES];
  uint8_t memory[CKB_VM_ASM_RISCV_MAX_MEMORY];
  uint8_t frames[CKB_VM_ASM_MEMORY_FRAMES];
  /* We won't access traces here */
  uint8_t _traces[CKB_VM_ASM_ASM_CORE_MACHINE_STRUCT_SIZE -
                  CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_TRACES];
} AsmMachine;

extern void inited_memory(uint64_t frame_index, AsmMachine* machine);

#define AOT_TAG_REGISTER 0x1
#define AOT_TAG_IMMEDIATE 0x2
#define AOT_TAG_X64_REGISTER 0x3

typedef uint32_t riscv_register_t;
typedef int32_t x64_register_t;

typedef struct {
  uint32_t tag;
  union {
    riscv_register_t reg;
    uint64_t i;
    x64_register_t x64_reg;
  } value;
} AotValue;

int aot_value_is_riscv_register(AotValue v, riscv_register_t r)
{
  return (v.tag == AOT_TAG_REGISTER) && (v.value.reg == r);
}

x64_register_t riscv_reg_to_x64_reg(riscv_register_t r)
{
  switch (r) {
    case REGISTER_RA:
      return X64_RSI;
    case REGISTER_SP:
      return X64_R8;
    case REGISTER_A0:
      return X64_R9;
    case REGISTER_TEMP1:
      return X64_R10;
    case REGISTER_TEMP2:
      return X64_R11;
    case REGISTER_TEMP3:
      return X64_R12;
    case REGISTER_TEMP4:
      return X64_R13;
    case REGISTER_TEMP5:
      return X64_R14;
    default:
      return INVALID_X64_REGISTER;
  }
}

//|.type machine, AsmMachine, rdi
#define Dt1(_V) (int)(ptrdiff_t)&(((AsmMachine *)0)_V)
#line 166 "src/machine/aot/aot.x64.c"

//|.macro load_imm64, x64_reg, imm64
//| mov64 x64_reg, imm64
//|.endmacro

/* We can leverage sign extension to save bits when handling negative integers */
//|.macro load_imm, x64_reg, imm
//||if ((imm >> (context->version >= 1? 31: 32)) > 0 && ((imm & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
//|   load_imm64 x64_reg, imm
//||} else {
//|   mov x64_reg, imm
//||}
//|.endmacro

/* r_r means both operands here are RISC-V registers */
//|.macro op2_r_r, op, target, source, x64_temp_reg
//||loc1 = riscv_reg_to_x64_reg(target);
//||loc2 = riscv_reg_to_x64_reg(source);
//||if (VALID_X64_REGISTER(loc1) && VALID_X64_REGISTER(loc2)) {
//|  op Rq(loc1), Rq(loc2)
//||} else if (VALID_X64_REGISTER(loc1)) {
//|  op Rq(loc1), machine->registers[source]
//||} else if (VALID_X64_REGISTER(loc2)) {
//|  op machine->registers[target], Rq(loc2)
//||} else {
//|  mov x64_temp_reg, qword machine->registers[source]
//|  op qword machine->registers[target], x64_temp_reg
//||}
//|.endmacro

//|.macro op1_r, op, reg
//||loc1 = riscv_reg_to_x64_reg(reg);
//||if (VALID_X64_REGISTER(loc1)) {
//|  op Rq(loc1)
//||} else {
//|  op qword machine->registers[reg]
//||}
//|.endmacro

/* r_x means that the first operand is RISC-V register, the second is X86 one */
//|.macro op2_r_x, op, target, x64_source
//||loc1 = riscv_reg_to_x64_reg(target);
//||if (VALID_X64_REGISTER(loc1)) {
//|  op Rq(loc1), x64_source
//||} else {
//|  op qword machine->registers[target], x64_source
//||}
//|.endmacro

/*
 * In version 0, imm 0x80000000 will be wrongly treated as 0xffffffff80000000.
 * It is feasible to directly compare imm with 0x7FFFFFFF or 0xFFFFFFFF, but
 * this will trigger a gcc warning when imm is 0 or UINT64_MAX: comparison is
 * always true due to limited range of data type.
 */
//|.macro op2_r_imm, op, target, imm, x64_temp_reg
//||if ((imm >> (context->version >= 1? 31: 32)) > 0 && ((imm & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
//||  loc1 = riscv_reg_to_x64_reg(target);
//|   load_imm64 x64_temp_reg, imm
//||  if (VALID_X64_REGISTER(loc1)) {
//|     op Rq(loc1), x64_temp_reg
//||  } else {
//|     op qword machine->registers[target], x64_temp_reg
//||  }
//||} else {
//||  loc1 = riscv_reg_to_x64_reg(target);
//||  if (VALID_X64_REGISTER(loc1)) {
//|     op Rq(loc1), imm
//||  } else {
//|     op qword machine->registers[target], imm
//||  }
//||}
//|.endmacro

//|.macro op2_x_r, op, x64_target, source
//||loc1 = riscv_reg_to_x64_reg(source);
//||if (VALID_X64_REGISTER(loc1)) {
//|  op x64_target, Rq(loc1)
//||} else {
//|  op x64_target, machine->registers[source]
//||}
//|.endmacro

AotContext* aot_new(uint32_t npc, uint32_t version)
{
  dasm_State** Dst;
  AotContext* context = malloc(sizeof(AotContext));
  context->npc = npc;
  context->version = version;
  dasm_init(&context->d, DASM_MAXSECTION);
  dasm_setupglobal(&context->d, context->labels, lbl__MAX);
  dasm_setup(&context->d, bf_actions);
  dasm_growpc(&context->d, context->npc);
  Dst = &context->d;

  //|.if WIN
    //|.define rArg1, rcx
    //|.define rArg2, rdx
  //|.else
    //|.define rArg1, rdi
    //|.define rArg2, rsi
  //|.endif
  //|.macro prepcall
    //| push rdi
    //| push rsi
    //| push rax
    //| push rcx
    //| push rdx
    //| push r8
    //| push r9
    //| push r10
    //| push rbp
    //| mov rbp, rsp
    //| and rsp, -16
    //|.if WIN
      //| sub rsp, 32
    //|.endif
  //|.endmacro
  //|.macro postcall
    //|.if WIN
      //| add rsp, 32
    //|.endif
    //| mov rsp, rbp
    //| pop rbp
    //| pop r10
    //| pop r9
    //| pop r8
    //| pop rdx
    //| pop rcx
    //| pop rax
    //| pop rsi
    //| pop rdi
  //|.endmacro
  //|.macro call_inited_memory
    //| prepcall
    //| mov rArg2, machine
    //| mov rArg1, rcx
    //| mov64 rax, (uint64_t)inited_memory
    //| call rax
    //| postcall
  //|.endmacro

  /*
   * The function we are generating has the following prototype:
   *
   * uint8_t execute_aot_code(AsmMachine* machine, uint64_t offset);
   *
   * +machine+ here contains the actual data used by the VM, offset specify
   * the location in the x64 assembly to jump to so as to start execution, it
   * should be derived from a label associated with the binary.
   * In System V AMD64 ABI, the first argument is already kept in rdi, so we
   * don't need any tweak on AsmMachine variable, the second argument is kept
   * in rsi, since we would keep RISC-V register RA in rsi, we first copy rsi
   * to rax for latter jumps.
   * As shown in aot_exit, the return result is kept in rax.
   */
  //|.code
  dasm_put(Dst, 0);
#line 323 "src/machine/aot/aot.x64.c"
  //| push r12
  //| push r13
  //| push r14
  //| push r15
  //| push rbx
  //| push rbp
  //|.if WIN
    //| push rdi
    //| push rsi
    //| mov rdi, rcx
    //| mov rax, rdx
  //|.else
    //| mov rax, rsi
  //|.endif
  //| mov rsi, machine->registers[REGISTER_RA]
  //| mov r8, machine->registers[REGISTER_SP]
  //| mov r9, machine->registers[REGISTER_A0]
  //| jmp rax
  dasm_put(Dst, 2, Dt1(->registers[REGISTER_RA]), Dt1(->registers[REGISTER_SP]), Dt1(->registers[REGISTER_A0]));
#line 341 "src/machine/aot/aot.x64.c"
  return context;
}

void aot_finalize(AotContext* context)
{
  dasm_free(&context->d);
  free(context);
}

int aot_link(AotContext* context, size_t *szp)
{
  dasm_State** Dst = &context->d;

  /*
   * Check memory write permissions. Note this pseudo function does not use
   * C's standard calling convention, since the AOT code here has its own
   * register allocations for maximum performance. Required arguments to this
   * pseudo function include:
   *
   * rax: the memory address to check for permissions
   * rdx: length of memory to write
   *
   * The return value is kept in rdx, 0 means success, while non-zero values
   * mean permission check fails.
   *
   * Note the free register rcx might also be modified in this pseudo function.
   */
  //|->check_write:
  //| push rsi
  //| push r8
  //| mov rsi, rdx
  //| mov rcx, rax
  //| shr rcx, CKB_VM_ASM_RISCV_PAGE_SHIFTS
  dasm_put(Dst, 36, CKB_VM_ASM_RISCV_PAGE_SHIFTS);
#line 374 "src/machine/aot/aot.x64.c"
  /*
   * Test if the page stored in rcx is out of bound, and if the page has
   * correct write permissions
   */
  //| cmp rcx, CKB_VM_ASM_RISCV_PAGES
  //| jae >3
  //| lea rdx, machine->flags
  //| movzx edx, byte [rdx+rcx]
  //| and edx, CKB_VM_ASM_MEMORY_FLAG_WXORX_BIT
  //| cmp edx, CKB_VM_ASM_MEMORY_FLAG_WRITABLE
  //| jne >4
  dasm_put(Dst, 53, CKB_VM_ASM_RISCV_PAGES, Dt1(->flags), CKB_VM_ASM_MEMORY_FLAG_WXORX_BIT, CKB_VM_ASM_MEMORY_FLAG_WRITABLE);
#line 385 "src/machine/aot/aot.x64.c"
  /*
   * Set the page as dirty
   */
  //| lea rdx, machine->flags
  //| movzx r8d, byte [rdx+rcx]
  //| or r8d, CKB_VM_ASM_MEMORY_FLAG_DIRTY
  //| mov byte [rdx+rcx], r8b
  dasm_put(Dst, 82, Dt1(->flags), CKB_VM_ASM_MEMORY_FLAG_DIRTY);
#line 392 "src/machine/aot/aot.x64.c"
  /*
   * If the frame not initialized, then initialize it.
   */
  //| shr rcx, CKB_VM_ASM_MEMORY_FRAME_PAGE_SHIFTS
  //| lea rdx, machine->frames
  //| movzx r8d, byte [rdx+rcx]
  //| cmp r8d, 0
  //| jne >1
  //| mov byte [rdx+rcx], 1
  //| call_inited_memory
  //|1:
  dasm_put(Dst, 100, CKB_VM_ASM_MEMORY_FRAME_PAGE_SHIFTS, Dt1(->frames), (unsigned int)((uint64_t)inited_memory), (unsigned int)(((uint64_t)inited_memory)>>32));
#line 403 "src/machine/aot/aot.x64.c"
  /* Check if the write spans to a second memory page */
  //| mov rdx, rax
  //| add rdx, rsi
  //| sub rdx, 1
  //| shr rdx, CKB_VM_ASM_RISCV_PAGE_SHIFTS
  //| add rcx, 1
  //| cmp rcx, rdx
  //| jne >2
  dasm_put(Dst, 189, CKB_VM_ASM_RISCV_PAGE_SHIFTS);
#line 411 "src/machine/aot/aot.x64.c"
  /*
   * Test if the page stored in rcx is out of bound, and if the page has
   * correct write permissions
   */
  //| cmp rcx, CKB_VM_ASM_RISCV_PAGES
  //| jae >3
  //| lea rdx, machine->flags
  //| movzx edx, byte [rdx+rcx]
  //| and edx, CKB_VM_ASM_MEMORY_FLAG_WXORX_BIT
  //| cmp edx, CKB_VM_ASM_MEMORY_FLAG_WRITABLE
  //| jne >4
  dasm_put(Dst, 53, CKB_VM_ASM_RISCV_PAGES, Dt1(->flags), CKB_VM_ASM_MEMORY_FLAG_WXORX_BIT, CKB_VM_ASM_MEMORY_FLAG_WRITABLE);
#line 422 "src/machine/aot/aot.x64.c"
  /*
   * Set the page as dirty
   */
  //| lea rdx, machine->flags
  //| movzx r8d, byte [rdx+rcx]
  //| or r8d, CKB_VM_ASM_MEMORY_FLAG_DIRTY
  //| mov byte [rdx+rcx], r8b
  //| shr rcx, CKB_VM_ASM_MEMORY_FRAME_PAGE_SHIFTS
  //| lea rdx, machine->frames
  //| movzx r8d, byte [rdx+rcx]
  //| cmp r8d, 0
  //| jne >2
  //| mov byte [rdx+rcx], 1
  //| call_inited_memory
  //|2:
  //| mov rdx, 0
  //| pop r8
  //| pop rsi
  //| ret
  //|3:
  //| mov rdx, CKB_VM_ASM_RET_OUT_OF_BOUND
  //| pop r8
  //| pop rsi
  //| ret
  //|4:
  //| mov rdx, CKB_VM_ASM_RET_INVALID_PERMISSION
  //| pop r8
  //| pop rsi
  //| ret
  dasm_put(Dst, 218, Dt1(->flags), CKB_VM_ASM_MEMORY_FLAG_DIRTY, CKB_VM_ASM_MEMORY_FRAME_PAGE_SHIFTS, Dt1(->frames), (unsigned int)((uint64_t)inited_memory), (unsigned int)(((uint64_t)inited_memory)>>32), CKB_VM_ASM_RET_OUT_OF_BOUND, CKB_VM_ASM_RET_INVALID_PERMISSION);
#line 451 "src/machine/aot/aot.x64.c"
  /*
   * Zeroed frame by memory address and length if it's necessary.
   *
   * rax: the memory address to read/write
   * rdx: length of memory to read/write
   */
  //|->check_read:
  //| push rsi
  //| push r8
  //| mov rcx, rax
  //| shr rcx, CKB_VM_ASM_MEMORY_FRAME_SHIFTS
  //| cmp rcx, CKB_VM_ASM_MEMORY_FRAMES
  //| jae >3
  //| lea rsi, machine->frames
  //| movzx r8d, byte [rsi+rcx]
  //| cmp r8d, 0
  //| jne >1
  //| mov byte [rsi+rcx], 1
  //| call_inited_memory
  //|1:
  //| mov rcx, rax
  //| add rcx, rdx
  //| sub rcx, 1
  //| shr rcx, CKB_VM_ASM_MEMORY_FRAME_SHIFTS
  //| cmp rcx, CKB_VM_ASM_MEMORY_FRAMES
  //| jae >3
  //| movzx r8d, byte [rsi+rcx]
  //| cmp r8d, 0
  //| jne >2
  //| mov byte [rsi+rcx], 1
  //| call_inited_memory
  dasm_put(Dst, 355, CKB_VM_ASM_MEMORY_FRAME_SHIFTS, CKB_VM_ASM_MEMORY_FRAMES, Dt1(->frames), (unsigned int)((uint64_t)inited_memory), (unsigned int)(((uint64_t)inited_memory)>>32), CKB_VM_ASM_MEMORY_FRAME_SHIFTS, CKB_VM_ASM_MEMORY_FRAMES, (unsigned int)((uint64_t)inited_memory), (unsigned int)(((uint64_t)inited_memory)>>32));
#line 482 "src/machine/aot/aot.x64.c"
  //| jmp >2
  //|2:
  //| mov rdx, 0
  //| pop r8
  //| pop rsi
  //| ret
  //|3:
  //| mov rdx, CKB_VM_ASM_RET_OUT_OF_BOUND
  //| pop r8
  //| pop rsi
  //| ret
  dasm_put(Dst, 540, CKB_VM_ASM_RET_OUT_OF_BOUND);
#line 493 "src/machine/aot/aot.x64.c"
  /* rax should store the return value here */
  //|->exit:
  //| mov machine->registers[REGISTER_RA], rsi
  //| mov machine->registers[REGISTER_SP], r8
  //| mov machine->registers[REGISTER_A0], r9
  //|.if WIN
    //| pop rsi
    //| pop rdi
  //|.endif
  //| pop rbp
  //| pop rbx
  //| pop r15
  //| pop r14
  //| pop r13
  //| pop r12
  //| ret
  dasm_put(Dst, 591, Dt1(->registers[REGISTER_RA]), Dt1(->registers[REGISTER_SP]), Dt1(->registers[REGISTER_A0]));
#line 509 "src/machine/aot/aot.x64.c"
  return dasm_link(&context->d, szp);
}

int aot_encode(AotContext* context, void *buffer)
{
  return dasm_encode(&context->d, buffer);
}

int aot_getpclabel(AotContext* context, uint32_t label, uint32_t* offset)
{
  int ret;
  if (label >= context->npc) {
    return ERROR_NOT_ENOUGH_LABELS;
  }
  ret = dasm_getpclabel(&context->d, label);
  if (ret < 0) { return ret; }
  *offset = (uint32_t) ret;
  return DASM_S_OK;
}

int aot_label(AotContext* context, uint32_t label)
{
  dasm_State** Dst = &context->d;
  if (label >= context->npc) {
    return ERROR_NOT_ENOUGH_LABELS;
  }
  //|=>label:
  dasm_put(Dst, 619, label);
#line 536 "src/machine/aot/aot.x64.c"
  return DASM_S_OK;
}

static int aot_mov_internal(AotContext* context, riscv_register_t target, AotValue value, x64_register_t x64_temp_reg);
static int aot_mov_pc_internal(AotContext* context, AotValue value);
static int aot_mov_x64(AotContext* context, x64_register_t x64_target, AotValue value);

int aot_mov(AotContext* context, riscv_register_t target, AotValue value)
{
  return aot_mov_internal(context, target, value, X64_RAX);
}

int aot_mov_pc(AotContext* context, AotValue value)
{
  return aot_mov_pc_internal(context, value);
}

int aot_add(AotContext* context, riscv_register_t target, AotValue a, AotValue b)
{
  int ret;
  uint32_t loc1, loc2;
  dasm_State** Dst = &context->d;

  if (aot_value_is_riscv_register(b, target)) {
    ret = aot_mov_x64(context, X64_RCX, b);
    if (ret != DASM_S_OK) { return ret; }
    b.tag = AOT_TAG_X64_REGISTER;
    b.value.x64_reg = X64_RCX;
  }

  ret = aot_mov_internal(context, target, a, X64_RAX);
  if (ret != DASM_S_OK) { return ret; }

  switch (b.tag) {
    case AOT_TAG_REGISTER:
      //| op2_r_r add, target, b.value.reg, rax
      loc1 = riscv_reg_to_x64_reg(target);
      loc2 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REGISTER(loc1) && VALID_X64_REGISTER(loc2)) {
      dasm_put(Dst, 621, (loc2), (loc1));
      } else if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 629, (loc1), Dt1(->registers[b.value.reg]));
      } else if (VALID_X64_REGISTER(loc2)) {
      dasm_put(Dst, 637, (loc2), Dt1(->registers[target]));
      } else {
      dasm_put(Dst, 645, Dt1(->registers[b.value.reg]), Dt1(->registers[target]));
      }
#line 572 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_IMMEDIATE:
      //| op2_r_imm add, target, b.value.i, rax
      if ((b.value.i >> (context->version >= 1? 31: 32)) > 0 && ((b.value.i & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
        loc1 = riscv_reg_to_x64_reg(target);
      dasm_put(Dst, 535, (unsigned int)(b.value.i), (unsigned int)((b.value.i)>>32));
        if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 654, (loc1));
        } else {
      dasm_put(Dst, 649, Dt1(->registers[target]));
        }
      } else {
        loc1 = riscv_reg_to_x64_reg(target);
        if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 660, (loc1), b.value.i);
        } else {
      dasm_put(Dst, 667, Dt1(->registers[target]), b.value.i);
        }
      }
#line 575 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_X64_REGISTER:
      //| op2_r_x add, target, Rq(b.value.x64_reg)
      loc1 = riscv_reg_to_x64_reg(target);
      if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 621, (b.value.x64_reg), (loc1));
      } else {
      dasm_put(Dst, 637, (b.value.x64_reg), Dt1(->registers[target]));
      }
#line 578 "src/machine/aot/aot.x64.c"
      break;
  }

  return DASM_S_OK;
}

int aot_sub(AotContext* context, riscv_register_t target, AotValue a, AotValue b)
{
  int ret;
  uint32_t loc1, loc2;
  dasm_State** Dst = &context->d;

  if (aot_value_is_riscv_register(b, target)) {
    ret = aot_mov_x64(context, X64_RCX, b);
    if (ret != DASM_S_OK) { return ret; }
    b.tag = AOT_TAG_X64_REGISTER;
    b.value.x64_reg = X64_RCX;
  }

  ret = aot_mov_internal(context, target, a, X64_RAX);
  if (ret != DASM_S_OK) { return ret; }

  switch (b.tag) {
    case AOT_TAG_REGISTER:
      //| op2_r_r sub, target, b.value.reg, rax
      loc1 = riscv_reg_to_x64_reg(target);
      loc2 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REGISTER(loc1) && VALID_X64_REGISTER(loc2)) {
      dasm_put(Dst, 673, (loc2), (loc1));
      } else if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 681, (loc1), Dt1(->registers[b.value.reg]));
      } else if (VALID_X64_REGISTER(loc2)) {
      dasm_put(Dst, 689, (loc2), Dt1(->registers[target]));
      } else {
      dasm_put(Dst, 697, Dt1(->registers[b.value.reg]), Dt1(->registers[target]));
      }
#line 603 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_IMMEDIATE:
      //| op2_r_imm sub, target, b.value.i, rax
      if ((b.value.i >> (context->version >= 1? 31: 32)) > 0 && ((b.value.i & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
        loc1 = riscv_reg_to_x64_reg(target);
      dasm_put(Dst, 535, (unsigned int)(b.value.i), (unsigned int)((b.value.i)>>32));
        if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 706, (loc1));
        } else {
      dasm_put(Dst, 701, Dt1(->registers[target]));
        }
      } else {
        loc1 = riscv_reg_to_x64_reg(target);
        if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 712, (loc1), b.value.i);
        } else {
      dasm_put(Dst, 719, Dt1(->registers[target]), b.value.i);
        }
      }
#line 606 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_X64_REGISTER:
      //| op2_r_x sub, target, Rq(b.value.x64_reg)
      loc1 = riscv_reg_to_x64_reg(target);
      if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 673, (b.value.x64_reg), (loc1));
      } else {
      dasm_put(Dst, 689, (b.value.x64_reg), Dt1(->registers[target]));
      }
#line 609 "src/machine/aot/aot.x64.c"
      break;
  }

  return DASM_S_OK;
}

int aot_mul(AotContext* context, riscv_register_t target, AotValue a, AotValue b)
{
  int ret;
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }

  switch (b.tag) {
    case AOT_TAG_REGISTER:
      //| op2_x_r imul, rax, b.value.reg
      loc1 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 725, (loc1));
      } else {
      dasm_put(Dst, 732, Dt1(->registers[b.value.reg]));
      }
#line 627 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_IMMEDIATE:
      //| load_imm rcx, b.value.i
      if ((b.value.i >> (context->version >= 1? 31: 32)) > 0 && ((b.value.i & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
      dasm_put(Dst, 738, (unsigned int)(b.value.i), (unsigned int)((b.value.i)>>32));
      } else {
      dasm_put(Dst, 743, b.value.i);
      }
#line 630 "src/machine/aot/aot.x64.c"
      //| imul rax, rcx
      dasm_put(Dst, 748);
#line 631 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_X64_REGISTER:
      //| imul rax, Rq(b.value.x64_reg)
      dasm_put(Dst, 725, (b.value.x64_reg));
#line 634 "src/machine/aot/aot.x64.c"
      break;
  }

  //| op2_r_x mov, target, rax
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 753, (loc1));
  } else {
  dasm_put(Dst, 759, Dt1(->registers[target]));
  }
#line 638 "src/machine/aot/aot.x64.c"

  return DASM_S_OK;
}

int aot_mulh(AotContext* context, riscv_register_t target, AotValue a, AotValue b, int is_signed)
{
  int ret;
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }

  switch (b.tag) {
    case AOT_TAG_REGISTER:
      if (is_signed) {
        //| op1_r imul, b.value.reg
        loc1 = riscv_reg_to_x64_reg(b.value.reg);
        if (VALID_X64_REGISTER(loc1)) {
        dasm_put(Dst, 764, (loc1));
        } else {
        dasm_put(Dst, 771, Dt1(->registers[b.value.reg]));
        }
#line 655 "src/machine/aot/aot.x64.c"
      } else {
        //| op1_r mul, b.value.reg
        loc1 = riscv_reg_to_x64_reg(b.value.reg);
        if (VALID_X64_REGISTER(loc1)) {
        dasm_put(Dst, 777, (loc1));
        } else {
        dasm_put(Dst, 784, Dt1(->registers[b.value.reg]));
        }
#line 657 "src/machine/aot/aot.x64.c"
      }
      break;
    case AOT_TAG_IMMEDIATE:
      //| load_imm rcx, b.value.i
      if ((b.value.i >> (context->version >= 1? 31: 32)) > 0 && ((b.value.i & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
      dasm_put(Dst, 738, (unsigned int)(b.value.i), (unsigned int)((b.value.i)>>32));
      } else {
      dasm_put(Dst, 743, b.value.i);
      }
#line 661 "src/machine/aot/aot.x64.c"
      if (is_signed) {
        //| imul rcx
        dasm_put(Dst, 790);
#line 663 "src/machine/aot/aot.x64.c"
      } else {
        //| mul rcx
        dasm_put(Dst, 796);
#line 665 "src/machine/aot/aot.x64.c"
      }
      break;
    case AOT_TAG_X64_REGISTER:
      if (is_signed) {
        //| imul Rq(b.value.x64_reg)
        dasm_put(Dst, 764, (b.value.x64_reg));
#line 670 "src/machine/aot/aot.x64.c"
      } else {
        //| mul Rq(b.value.x64_reg)
        dasm_put(Dst, 777, (b.value.x64_reg));
#line 672 "src/machine/aot/aot.x64.c"
      }
      break;
  }
  //| op2_r_x mov, target, rdx
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 801, (loc1));
  } else {
  dasm_put(Dst, 807, Dt1(->registers[target]));
  }
#line 676 "src/machine/aot/aot.x64.c"

  return DASM_S_OK;
}

/* Inspired from https://github.com/rv8-io/rv8/blob/834259098a5c182874aac97d82a164d144244e1a/src/jit/jit-emitter-rv64.h#L931 */
int aot_mulhsu(AotContext* context, riscv_register_t target, AotValue a, AotValue b)
{
  int ret;
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }

  //| test rax, rax
  //| jns >1
  dasm_put(Dst, 812);
#line 692 "src/machine/aot/aot.x64.c"
  /* calculate res = mulhu(-a, b), res is stored in rdx after this. */
  //| neg rax
  dasm_put(Dst, 820);
#line 694 "src/machine/aot/aot.x64.c"
  switch (b.tag) {
    case AOT_TAG_REGISTER:
      //| op1_r mul, b.value.reg
      loc1 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 777, (loc1));
      } else {
      dasm_put(Dst, 784, Dt1(->registers[b.value.reg]));
      }
#line 697 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_IMMEDIATE:
      //| load_imm rcx, b.value.i
      if ((b.value.i >> (context->version >= 1? 31: 32)) > 0 && ((b.value.i & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
      dasm_put(Dst, 738, (unsigned int)(b.value.i), (unsigned int)((b.value.i)>>32));
      } else {
      dasm_put(Dst, 743, b.value.i);
      }
#line 700 "src/machine/aot/aot.x64.c"
      //| mul rcx
      dasm_put(Dst, 796);
#line 701 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_X64_REGISTER:
      //| mul Rq(b.value.x64_reg)
      dasm_put(Dst, 777, (b.value.x64_reg));
#line 704 "src/machine/aot/aot.x64.c"
      break;
  }
  /* calculate ~res and store it in rcx */
  //| xor rdx, -1
  //| mov rcx, rdx
  dasm_put(Dst, 825);
#line 709 "src/machine/aot/aot.x64.c"
  /*
   * calculate (a * b), then test (a * b == 0) and convert that to 1 or 0,
   * result is stored in rax after this.
   */
  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }
  switch (b.tag) {
    case AOT_TAG_REGISTER:
      //| op2_x_r imul, rax, b.value.reg
      loc1 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 725, (loc1));
      } else {
      dasm_put(Dst, 732, Dt1(->registers[b.value.reg]));
      }
#line 718 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_IMMEDIATE:
      //| load_imm rdx, b.value.i
      if ((b.value.i >> (context->version >= 1? 31: 32)) > 0 && ((b.value.i & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
      dasm_put(Dst, 835, (unsigned int)(b.value.i), (unsigned int)((b.value.i)>>32));
      } else {
      dasm_put(Dst, 840, b.value.i);
      }
#line 721 "src/machine/aot/aot.x64.c"
      //| imul rax, rdx
      dasm_put(Dst, 845);
#line 722 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_X64_REGISTER:
      //| imul rax, Rq(b.value.x64_reg)
      dasm_put(Dst, 725, (b.value.x64_reg));
#line 725 "src/machine/aot/aot.x64.c"
      break;
  }
  //| test rax, rax
  //| setz al
  //| movzx rax, al
  dasm_put(Dst, 850);
#line 730 "src/machine/aot/aot.x64.c"
  /* calculate ~res + (a * b == 0) */
  //| add rax, rcx
  //| jmp >2
  dasm_put(Dst, 861);
#line 733 "src/machine/aot/aot.x64.c"
  /* just mulhu here */
  //|1:
  dasm_put(Dst, 186);
#line 735 "src/machine/aot/aot.x64.c"
  switch (b.tag) {
    case AOT_TAG_REGISTER:
      //| op1_r mul, b.value.reg
      loc1 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 777, (loc1));
      } else {
      dasm_put(Dst, 784, Dt1(->registers[b.value.reg]));
      }
#line 738 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_IMMEDIATE:
      //| load_imm rcx, b.value.i
      if ((b.value.i >> (context->version >= 1? 31: 32)) > 0 && ((b.value.i & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
      dasm_put(Dst, 738, (unsigned int)(b.value.i), (unsigned int)((b.value.i)>>32));
      } else {
      dasm_put(Dst, 743, b.value.i);
      }
#line 741 "src/machine/aot/aot.x64.c"
      //| mul rcx
      dasm_put(Dst, 796);
#line 742 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_X64_REGISTER:
      //| mul Rq(b.value.x64_reg)
      dasm_put(Dst, 777, (b.value.x64_reg));
#line 745 "src/machine/aot/aot.x64.c"
      break;
  }
  //| mov rax, rdx
  //|2:
  //| op2_r_x mov, target, rax
  dasm_put(Dst, 869);
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 753, (loc1));
  } else {
  dasm_put(Dst, 759, Dt1(->registers[target]));
  }
#line 750 "src/machine/aot/aot.x64.c"

  return DASM_S_OK;
}

int aot_div(AotContext* context, riscv_register_t target, AotValue a, AotValue b, int is_signed)
{
  int ret;
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  if (is_signed) {
    //| mov64 rax, INT64_MIN
    dasm_put(Dst, 535, (unsigned int)(INT64_MIN), (unsigned int)((INT64_MIN)>>32));
#line 762 "src/machine/aot/aot.x64.c"
    ret = aot_mov_x64(context, X64_RCX, a);
    if (ret != DASM_S_OK) { return ret; }
    //| cmp rax, rcx
    //| jne >1
    //| mov rax, -1
    dasm_put(Dst, 875);
#line 767 "src/machine/aot/aot.x64.c"
    ret = aot_mov_x64(context, X64_RCX, b);
    if (ret != DASM_S_OK) { return ret; }
    //| cmp rax, rcx
    //| jne >1
    dasm_put(Dst, 894);
#line 771 "src/machine/aot/aot.x64.c"
    ret = aot_mov_internal(context, target, a, X64_RAX);
    if (ret != DASM_S_OK) { return ret; }
    //| jmp >3
    dasm_put(Dst, 902);
#line 774 "src/machine/aot/aot.x64.c"
  }
  //|1:
  //| mov rax, 0
  dasm_put(Dst, 907);
#line 777 "src/machine/aot/aot.x64.c"
  ret = aot_mov_x64(context, X64_RCX, b);
  if (ret != DASM_S_OK) { return ret; }
  //| cmp rax, rcx
  //| jne >2
  //| op2_r_imm mov, target, (uint64_t)UINT64_MAX, rax
  dasm_put(Dst, 917);
  if (((uint64_t)UINT64_MAX >> (context->version >= 1? 31: 32)) > 0 && (((uint64_t)UINT64_MAX & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
    loc1 = riscv_reg_to_x64_reg(target);
  dasm_put(Dst, 535, (unsigned int)((uint64_t)UINT64_MAX), (unsigned int)(((uint64_t)UINT64_MAX)>>32));
    if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 753, (loc1));
    } else {
  dasm_put(Dst, 759, Dt1(->registers[target]));
    }
  } else {
    loc1 = riscv_reg_to_x64_reg(target);
    if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 925, (loc1), (uint64_t)UINT64_MAX);
    } else {
  dasm_put(Dst, 932, Dt1(->registers[target]), (uint64_t)UINT64_MAX);
    }
  }
#line 782 "src/machine/aot/aot.x64.c"
  //| jmp >3
  //|2:
  dasm_put(Dst, 938);
#line 784 "src/machine/aot/aot.x64.c"
  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }
  switch (b.tag) {
    case AOT_TAG_REGISTER:
      if (is_signed) {
        //| cqo
        //| op1_r idiv, b.value.reg
        dasm_put(Dst, 945);
        loc1 = riscv_reg_to_x64_reg(b.value.reg);
        if (VALID_X64_REGISTER(loc1)) {
        dasm_put(Dst, 948, (loc1));
        } else {
        dasm_put(Dst, 956, Dt1(->registers[b.value.reg]));
        }
#line 791 "src/machine/aot/aot.x64.c"
      } else {
        //| xor rdx, rdx
        //| op1_r div, b.value.reg
        dasm_put(Dst, 962);
        loc1 = riscv_reg_to_x64_reg(b.value.reg);
        if (VALID_X64_REGISTER(loc1)) {
        dasm_put(Dst, 966, (loc1));
        } else {
        dasm_put(Dst, 974, Dt1(->registers[b.value.reg]));
        }
#line 794 "src/machine/aot/aot.x64.c"
      }
      break;
    case AOT_TAG_IMMEDIATE:
      //| load_imm, rcx, b.value.i
      if ((b.value.i >> (context->version >= 1? 31: 32)) > 0 && ((b.value.i & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
      dasm_put(Dst, 738, (unsigned int)(b.value.i), (unsigned int)((b.value.i)>>32));
      } else {
      dasm_put(Dst, 743, b.value.i);
      }
#line 798 "src/machine/aot/aot.x64.c"
      if (is_signed) {
        //| cqo
        //| idiv rcx
        dasm_put(Dst, 980);
#line 801 "src/machine/aot/aot.x64.c"
      } else {
        //| xor rdx, rdx
        //| div rcx
        dasm_put(Dst, 988);
#line 804 "src/machine/aot/aot.x64.c"
      }
      break;
    case AOT_TAG_X64_REGISTER:
      if (is_signed) {
        //| cqo
        //| idiv Rq(b.value.x64_reg)
        dasm_put(Dst, 997, (b.value.x64_reg));
#line 810 "src/machine/aot/aot.x64.c"
      } else {
        //| xor rdx, rdx
        //| div Rq(b.value.x64_reg)
        dasm_put(Dst, 1007, (b.value.x64_reg));
#line 813 "src/machine/aot/aot.x64.c"
      }
      break;
  }
  //| op2_r_x mov, target, rax
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 753, (loc1));
  } else {
  dasm_put(Dst, 759, Dt1(->registers[target]));
  }
#line 817 "src/machine/aot/aot.x64.c"
  //|3:
  dasm_put(Dst, 1018);
#line 818 "src/machine/aot/aot.x64.c"

  return DASM_S_OK;
}

int aot_rem(AotContext* context, riscv_register_t target, AotValue a, AotValue b, int is_signed)
{
  int ret;
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  if (is_signed) {
    //| mov64 rax, INT64_MIN
    dasm_put(Dst, 535, (unsigned int)(INT64_MIN), (unsigned int)((INT64_MIN)>>32));
#line 830 "src/machine/aot/aot.x64.c"
    ret = aot_mov_x64(context, X64_RCX, a);
    if (ret != DASM_S_OK) { return ret; }
    //| cmp rax, rcx
    //| jne >1
    //| mov rax, -1
    dasm_put(Dst, 875);
#line 835 "src/machine/aot/aot.x64.c"
    ret = aot_mov_x64(context, X64_RCX, b);
    if (ret != DASM_S_OK) { return ret; }
    //| cmp rax, rcx
    //| jne >1
    //| op2_r_imm mov, target, (uint64_t)0, rax
    dasm_put(Dst, 894);
    if (((uint64_t)0 >> (context->version >= 1? 31: 32)) > 0 && (((uint64_t)0 & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
      loc1 = riscv_reg_to_x64_reg(target);
    dasm_put(Dst, 535, (unsigned int)((uint64_t)0), (unsigned int)(((uint64_t)0)>>32));
      if (VALID_X64_REGISTER(loc1)) {
    dasm_put(Dst, 753, (loc1));
      } else {
    dasm_put(Dst, 759, Dt1(->registers[target]));
      }
    } else {
      loc1 = riscv_reg_to_x64_reg(target);
      if (VALID_X64_REGISTER(loc1)) {
    dasm_put(Dst, 925, (loc1), (uint64_t)0);
      } else {
    dasm_put(Dst, 932, Dt1(->registers[target]), (uint64_t)0);
      }
    }
#line 840 "src/machine/aot/aot.x64.c"
    //| jmp >3
    dasm_put(Dst, 902);
#line 841 "src/machine/aot/aot.x64.c"
  }
  //|1:
  //| mov rax, 0
  dasm_put(Dst, 907);
#line 844 "src/machine/aot/aot.x64.c"
  ret = aot_mov_x64(context, X64_RCX, b);
  if (ret != DASM_S_OK) { return ret; }
  //| cmp rax, rcx
  //| jne >2
  dasm_put(Dst, 917);
#line 848 "src/machine/aot/aot.x64.c"
  ret = aot_mov_internal(context, target, a, X64_RAX);
  if (ret != DASM_S_OK) { return ret; }
  //| jmp >3
  //|2:
  dasm_put(Dst, 938);
#line 852 "src/machine/aot/aot.x64.c"
  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }
  switch (b.tag) {
    case AOT_TAG_REGISTER:
      if (is_signed) {
        //| cqo
        //| op1_r idiv, b.value.reg
        dasm_put(Dst, 945);
        loc1 = riscv_reg_to_x64_reg(b.value.reg);
        if (VALID_X64_REGISTER(loc1)) {
        dasm_put(Dst, 948, (loc1));
        } else {
        dasm_put(Dst, 956, Dt1(->registers[b.value.reg]));
        }
#line 859 "src/machine/aot/aot.x64.c"
      } else {
        //| xor rdx, rdx
        //| op1_r div, b.value.reg
        dasm_put(Dst, 962);
        loc1 = riscv_reg_to_x64_reg(b.value.reg);
        if (VALID_X64_REGISTER(loc1)) {
        dasm_put(Dst, 966, (loc1));
        } else {
        dasm_put(Dst, 974, Dt1(->registers[b.value.reg]));
        }
#line 862 "src/machine/aot/aot.x64.c"
      }
      break;
    case AOT_TAG_IMMEDIATE:
      //| load_imm, rcx, b.value.i
      if ((b.value.i >> (context->version >= 1? 31: 32)) > 0 && ((b.value.i & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
      dasm_put(Dst, 738, (unsigned int)(b.value.i), (unsigned int)((b.value.i)>>32));
      } else {
      dasm_put(Dst, 743, b.value.i);
      }
#line 866 "src/machine/aot/aot.x64.c"
      if (is_signed) {
        //| cqo
        //| idiv rcx
        dasm_put(Dst, 980);
#line 869 "src/machine/aot/aot.x64.c"
      } else {
        //| xor rdx, rdx
        //| div rcx
        dasm_put(Dst, 988);
#line 872 "src/machine/aot/aot.x64.c"
      }
      break;
    case AOT_TAG_X64_REGISTER:
      if (is_signed) {
        //| cqo
        //| idiv Rq(b.value.x64_reg)
        dasm_put(Dst, 997, (b.value.x64_reg));
#line 878 "src/machine/aot/aot.x64.c"
      } else {
        //| xor rdx, rdx
        //| div Rq(b.value.x64_reg)
        dasm_put(Dst, 1007, (b.value.x64_reg));
#line 881 "src/machine/aot/aot.x64.c"
      }
      break;
  }
  //| op2_r_x mov, target, rdx
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 801, (loc1));
  } else {
  dasm_put(Dst, 807, Dt1(->registers[target]));
  }
#line 885 "src/machine/aot/aot.x64.c"
  //|3:
  dasm_put(Dst, 1018);
#line 886 "src/machine/aot/aot.x64.c"

  return DASM_S_OK;
}

int aot_and(AotContext* context, riscv_register_t target, AotValue a, AotValue b)
{
  int ret;
  uint32_t loc1, loc2;
  dasm_State** Dst = &context->d;

  if (aot_value_is_riscv_register(b, target)) {
    ret = aot_mov_x64(context, X64_RCX, b);
    if (ret != DASM_S_OK) { return ret; }
    b.tag = AOT_TAG_X64_REGISTER;
    b.value.x64_reg = X64_RCX;
  }

  ret = aot_mov_internal(context, target, a, X64_RAX);
  if (ret != DASM_S_OK) { return ret; }

  switch (b.tag) {
    case AOT_TAG_REGISTER:
      //| op2_r_r and, target, b.value.reg, rax
      loc1 = riscv_reg_to_x64_reg(target);
      loc2 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REGISTER(loc1) && VALID_X64_REGISTER(loc2)) {
      dasm_put(Dst, 1021, (loc2), (loc1));
      } else if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 1029, (loc1), Dt1(->registers[b.value.reg]));
      } else if (VALID_X64_REGISTER(loc2)) {
      dasm_put(Dst, 1037, (loc2), Dt1(->registers[target]));
      } else {
      dasm_put(Dst, 1045, Dt1(->registers[b.value.reg]), Dt1(->registers[target]));
      }
#line 909 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_IMMEDIATE:
      //| op2_r_imm and, target, b.value.i, rax
      if ((b.value.i >> (context->version >= 1? 31: 32)) > 0 && ((b.value.i & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
        loc1 = riscv_reg_to_x64_reg(target);
      dasm_put(Dst, 535, (unsigned int)(b.value.i), (unsigned int)((b.value.i)>>32));
        if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 1054, (loc1));
        } else {
      dasm_put(Dst, 1049, Dt1(->registers[target]));
        }
      } else {
        loc1 = riscv_reg_to_x64_reg(target);
        if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 1060, (loc1), b.value.i);
        } else {
      dasm_put(Dst, 1067, Dt1(->registers[target]), b.value.i);
        }
      }
#line 912 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_X64_REGISTER:
      //| op2_r_x and, target, Rq(b.value.x64_reg)
      loc1 = riscv_reg_to_x64_reg(target);
      if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 1021, (b.value.x64_reg), (loc1));
      } else {
      dasm_put(Dst, 1037, (b.value.x64_reg), Dt1(->registers[target]));
      }
#line 915 "src/machine/aot/aot.x64.c"
      break;
  }

  return DASM_S_OK;
}

int aot_or(AotContext* context, riscv_register_t target, AotValue a, AotValue b)
{
  int ret;
  uint32_t loc1, loc2;
  dasm_State** Dst = &context->d;

  if (aot_value_is_riscv_register(b, target)) {
    ret = aot_mov_x64(context, X64_RCX, b);
    if (ret != DASM_S_OK) { return ret; }
    b.tag = AOT_TAG_X64_REGISTER;
    b.value.x64_reg = X64_RCX;
  }

  ret = aot_mov_internal(context, target, a, X64_RAX);
  if (ret != DASM_S_OK) { return ret; }

  switch (b.tag) {
    case AOT_TAG_REGISTER:
      //| op2_r_r or, target, b.value.reg, rax
      loc1 = riscv_reg_to_x64_reg(target);
      loc2 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REGISTER(loc1) && VALID_X64_REGISTER(loc2)) {
      dasm_put(Dst, 1073, (loc2), (loc1));
      } else if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 1081, (loc1), Dt1(->registers[b.value.reg]));
      } else if (VALID_X64_REGISTER(loc2)) {
      dasm_put(Dst, 1089, (loc2), Dt1(->registers[target]));
      } else {
      dasm_put(Dst, 1097, Dt1(->registers[b.value.reg]), Dt1(->registers[target]));
      }
#line 940 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_IMMEDIATE:
      //| op2_r_imm or, target, b.value.i, rax
      if ((b.value.i >> (context->version >= 1? 31: 32)) > 0 && ((b.value.i & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
        loc1 = riscv_reg_to_x64_reg(target);
      dasm_put(Dst, 535, (unsigned int)(b.value.i), (unsigned int)((b.value.i)>>32));
        if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 1106, (loc1));
        } else {
      dasm_put(Dst, 1101, Dt1(->registers[target]));
        }
      } else {
        loc1 = riscv_reg_to_x64_reg(target);
        if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 1112, (loc1), b.value.i);
        } else {
      dasm_put(Dst, 1119, Dt1(->registers[target]), b.value.i);
        }
      }
#line 943 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_X64_REGISTER:
      //| op2_r_x or, target, Rq(b.value.x64_reg)
      loc1 = riscv_reg_to_x64_reg(target);
      if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 1073, (b.value.x64_reg), (loc1));
      } else {
      dasm_put(Dst, 1089, (b.value.x64_reg), Dt1(->registers[target]));
      }
#line 946 "src/machine/aot/aot.x64.c"
      break;
  }

  return DASM_S_OK;
}

int aot_not(AotContext* context, riscv_register_t target, AotValue a, int logical)
{
  int ret;
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  ret = aot_mov_internal(context, target, a, X64_RAX);
  if (ret != DASM_S_OK) { return ret; }

  //| op1_r not, target
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 1125, (loc1));
  } else {
  dasm_put(Dst, 1132, Dt1(->registers[target]));
  }
#line 962 "src/machine/aot/aot.x64.c"
  if (logical) {
    //| op2_r_imm and, target, (uint64_t)1, rax
    if (((uint64_t)1 >> (context->version >= 1? 31: 32)) > 0 && (((uint64_t)1 & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
      loc1 = riscv_reg_to_x64_reg(target);
    dasm_put(Dst, 535, (unsigned int)((uint64_t)1), (unsigned int)(((uint64_t)1)>>32));
      if (VALID_X64_REGISTER(loc1)) {
    dasm_put(Dst, 1054, (loc1));
      } else {
    dasm_put(Dst, 1049, Dt1(->registers[target]));
      }
    } else {
      loc1 = riscv_reg_to_x64_reg(target);
      if (VALID_X64_REGISTER(loc1)) {
    dasm_put(Dst, 1060, (loc1), (uint64_t)1);
      } else {
    dasm_put(Dst, 1067, Dt1(->registers[target]), (uint64_t)1);
      }
    }
#line 964 "src/machine/aot/aot.x64.c"
  }

  return DASM_S_OK;
}

int aot_xor(AotContext* context, riscv_register_t target, AotValue a, AotValue b)
{
  int ret;
  uint32_t loc1, loc2;
  dasm_State** Dst = &context->d;

  if (aot_value_is_riscv_register(b, target)) {
    ret = aot_mov_x64(context, X64_RCX, b);
    if (ret != DASM_S_OK) { return ret; }
    b.tag = AOT_TAG_X64_REGISTER;
    b.value.x64_reg = X64_RCX;
  }

  ret = aot_mov_internal(context, target, a, X64_RAX);
  if (ret != DASM_S_OK) { return ret; }

  switch (b.tag) {
    case AOT_TAG_REGISTER:
      //| op2_r_r xor, target, b.value.reg, rax
      loc1 = riscv_reg_to_x64_reg(target);
      loc2 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REGISTER(loc1) && VALID_X64_REGISTER(loc2)) {
      dasm_put(Dst, 1138, (loc2), (loc1));
      } else if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 1146, (loc1), Dt1(->registers[b.value.reg]));
      } else if (VALID_X64_REGISTER(loc2)) {
      dasm_put(Dst, 1154, (loc2), Dt1(->registers[target]));
      } else {
      dasm_put(Dst, 1162, Dt1(->registers[b.value.reg]), Dt1(->registers[target]));
      }
#line 988 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_IMMEDIATE:
      //| op2_r_imm xor, target, b.value.i, rax
      if ((b.value.i >> (context->version >= 1? 31: 32)) > 0 && ((b.value.i & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
        loc1 = riscv_reg_to_x64_reg(target);
      dasm_put(Dst, 535, (unsigned int)(b.value.i), (unsigned int)((b.value.i)>>32));
        if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 1171, (loc1));
        } else {
      dasm_put(Dst, 1166, Dt1(->registers[target]));
        }
      } else {
        loc1 = riscv_reg_to_x64_reg(target);
        if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 1177, (loc1), b.value.i);
        } else {
      dasm_put(Dst, 1185, Dt1(->registers[target]), b.value.i);
        }
      }
#line 991 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_X64_REGISTER:
      //| op2_r_x xor, target, Rq(b.value.x64_reg)
      loc1 = riscv_reg_to_x64_reg(target);
      if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 1138, (b.value.x64_reg), (loc1));
      } else {
      dasm_put(Dst, 1154, (b.value.x64_reg), Dt1(->registers[target]));
      }
#line 994 "src/machine/aot/aot.x64.c"
      break;
  }

  return DASM_S_OK;
}

int aot_shl(AotContext* context, riscv_register_t target, AotValue a, AotValue b)
{
  int ret;
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  ret = aot_mov_internal(context, target, a, X64_RAX);
  if (ret != DASM_S_OK) { return ret; }

  switch (b.tag) {
    case AOT_TAG_REGISTER:
      //| op2_x_r mov, rcx, b.value.reg
      loc1 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 1191, (loc1));
      } else {
      dasm_put(Dst, 1197, Dt1(->registers[b.value.reg]));
      }
#line 1012 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_IMMEDIATE:
      /*
       * shift operations only use cl as operand, there won't be any
       * overflowing issues.
       */
      //| mov ecx, b.value.i
      dasm_put(Dst, 1202, b.value.i);
#line 1019 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_X64_REGISTER:
      //| mov rcx, Rq(b.value.x64_reg)
      dasm_put(Dst, 1191, (b.value.x64_reg));
#line 1022 "src/machine/aot/aot.x64.c"
      break;
  }

  //| op2_r_x shl, target, cl
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 1205, (loc1));
  } else {
  dasm_put(Dst, 1211, Dt1(->registers[target]));
  }
#line 1026 "src/machine/aot/aot.x64.c"

  return DASM_S_OK;
}

int aot_shr(AotContext* context, riscv_register_t target, AotValue a, AotValue b, int is_signed)
{
  int ret;
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  ret = aot_mov_internal(context, target, a, X64_RAX);
  if (ret != DASM_S_OK) { return ret; }

  switch (b.tag) {
    case AOT_TAG_REGISTER:
      //| op2_x_r mov, rcx, b.value.reg
      loc1 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 1191, (loc1));
      } else {
      dasm_put(Dst, 1197, Dt1(->registers[b.value.reg]));
      }
#line 1042 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_IMMEDIATE:
      /*
       * shift operations only use cl as operand, there won't be any
       * overflowing issues.
       */
      //| mov ecx, b.value.i
      dasm_put(Dst, 1202, b.value.i);
#line 1049 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_X64_REGISTER:
      //| mov rcx, Rq(b.value.x64_reg)
      dasm_put(Dst, 1191, (b.value.x64_reg));
#line 1052 "src/machine/aot/aot.x64.c"
      break;
  }

  if (is_signed) {
    //| op2_r_x sar, target, cl
    loc1 = riscv_reg_to_x64_reg(target);
    if (VALID_X64_REGISTER(loc1)) {
    dasm_put(Dst, 1216, (loc1));
    } else {
    dasm_put(Dst, 1223, Dt1(->registers[target]));
    }
#line 1057 "src/machine/aot/aot.x64.c"
  } else {
    //| op2_r_x shr, target, cl
    loc1 = riscv_reg_to_x64_reg(target);
    if (VALID_X64_REGISTER(loc1)) {
    dasm_put(Dst, 1228, (loc1));
    } else {
    dasm_put(Dst, 1234, Dt1(->registers[target]));
    }
#line 1059 "src/machine/aot/aot.x64.c"
  }

  return DASM_S_OK;
}

int aot_clmul(AotContext* context, riscv_register_t target, AotValue a, AotValue b) {
  int ret;
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  //| xor ecx, ecx
  //| xor r10, r10
  //|1:
  dasm_put(Dst, 1239);
#line 1072 "src/machine/aot/aot.x64.c"
  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }
  ret = aot_mov_x64(context, X64_RDX, b);
  if (ret != DASM_S_OK) { return ret; }
  //| shl rax, cl
  //| shr rdx, cl
  //| xor rax, r10
  //| and rdx, 1
  //| cmovne r10, rax
  //| add rcx, 1
  //| cmp rcx, 64
  //| jne <1
  //| op2_r_x mov, target, r10
  dasm_put(Dst, 1247);
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 1279, (loc1));
  } else {
  dasm_put(Dst, 1285, Dt1(->registers[target]));
  }
#line 1085 "src/machine/aot/aot.x64.c"

  return DASM_S_OK;
}

int aot_clmulh(AotContext* context, riscv_register_t target, AotValue a, AotValue b) {
  int ret;
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  //| mov r11, 1
  //| xor r10, r10
  //|1:
  dasm_put(Dst, 1290);
#line 1097 "src/machine/aot/aot.x64.c"
  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }
  ret = aot_mov_x64(context, X64_RDX, b);
  if (ret != DASM_S_OK) { return ret; }
  //| mov rcx, 64
  //| sub rcx, r11
  //| shr rax, cl
  //| mov rcx, r11
  //| shr rdx, cl
  //| xor rax, r10
  //| and rdx, 1
  //| cmovne r10, rax
  //| add r11, 1
  //| cmp r11, 64
  //| jne <1
  //| op2_r_x mov, target, r10
  dasm_put(Dst, 1303);
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 1279, (loc1));
  } else {
  dasm_put(Dst, 1285, Dt1(->registers[target]));
  }
#line 1113 "src/machine/aot/aot.x64.c"

  return DASM_S_OK;
}

int aot_clmulr(AotContext* context, riscv_register_t target, AotValue a, AotValue b)
{
  int ret;
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  //| xor r11, r11
  //| xor r10, r10
  //|1:
  dasm_put(Dst, 1348);
#line 1126 "src/machine/aot/aot.x64.c"
  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }
  ret = aot_mov_x64(context, X64_RDX, b);
  if (ret != DASM_S_OK) { return ret; }
  //| mov rcx, 63
  //| sub rcx, r11
  //| shr rax, cl
  //| mov rcx, r11
  //| shr rdx, cl
  //| xor rax, r10
  //| and rdx, 1
  //| cmovne r10, rax
  //| add r11, 1
  //| cmp r11, 64
  //| jne <1
  //| op2_r_x mov, target, r10
  dasm_put(Dst, 1357);
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 1279, (loc1));
  } else {
  dasm_put(Dst, 1285, Dt1(->registers[target]));
  }
#line 1142 "src/machine/aot/aot.x64.c"

  return DASM_S_OK;
}

int aot_orcb(AotContext* context, riscv_register_t target, AotValue a)
{
  uint32_t loc1;
  int ret;
  dasm_State** Dst = &context->d;

  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }

  //| xor rdx, rdx
  //| mov64 r10, (uint64_t)0x00000000000000ff
  //| mov r11, r10
  //| and r10, rax
  //| je >1
  //| or rdx, r11
  //|1:
  //| mov64 r10, (uint64_t)0x000000000000ff00
  //| mov r11, r10
  //| and r10, rax
  //| je >2
  //| or rdx, r11
  //|2:
  //| mov64 r10, (uint64_t)0x0000000000ff0000
  //| mov r11, r10
  //| and r10, rax
  //| je >3
  //| or rdx, r11
  //|3:
  //| mov64 r10, (uint64_t)0x00000000ff000000
  //| mov r11, r10
  //| and r10, rax
  //| je >4
  //| or rdx, r11
  dasm_put(Dst, 1402, (unsigned int)((uint64_t)0x00000000000000ff), (unsigned int)(((uint64_t)0x00000000000000ff)>>32), (unsigned int)((uint64_t)0x000000000000ff00), (unsigned int)(((uint64_t)0x000000000000ff00)>>32), (unsigned int)((uint64_t)0x0000000000ff0000), (unsigned int)(((uint64_t)0x0000000000ff0000)>>32), (unsigned int)((uint64_t)0x00000000ff000000), (unsigned int)(((uint64_t)0x00000000ff000000)>>32));
#line 1179 "src/machine/aot/aot.x64.c"
  //|4:
  //| mov64 r10, (uint64_t)0x000000ff00000000
  //| mov r11, r10
  //| and r10, rax
  //| je >5
  //| or rdx, r11
  //|5:
  //| mov64 r10, (uint64_t)0x0000ff0000000000
  //| mov r11, r10
  //| and r10, rax
  //| je >6
  //| or rdx, r11
  //|6:
  //| mov64 r10, (uint64_t)0x00ff000000000000
  //| mov r11, r10
  //| and r10, rax
  //| je >7
  //| or rdx, r11
  //|7:
  //| mov64 r10, (uint64_t)0xff00000000000000
  //| mov r11, r10
  //| and r10, rax
  //| je >8
  //| or rdx, r11
  dasm_put(Dst, 1477, (unsigned int)((uint64_t)0x000000ff00000000), (unsigned int)(((uint64_t)0x000000ff00000000)>>32), (unsigned int)((uint64_t)0x0000ff0000000000), (unsigned int)(((uint64_t)0x0000ff0000000000)>>32), (unsigned int)((uint64_t)0x00ff000000000000), (unsigned int)(((uint64_t)0x00ff000000000000)>>32), (unsigned int)((uint64_t)0xff00000000000000), (unsigned int)(((uint64_t)0xff00000000000000)>>32));
#line 1203 "src/machine/aot/aot.x64.c"
  //|8:
  //| op2_r_x mov, target, rdx
  dasm_put(Dst, 1554);
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 801, (loc1));
  } else {
  dasm_put(Dst, 807, Dt1(->registers[target]));
  }
#line 1205 "src/machine/aot/aot.x64.c"

  return DASM_S_OK;
}

int aot_rev8(AotContext* context, riscv_register_t target, AotValue a)
{
  uint32_t loc1;
  int ret;
  dasm_State** Dst = &context->d;

  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }

  //| xor rdx, rdx
  //| mov64 r10, (uint64_t)0x00000000000000ff
  //| and r10, rax
  //| shl r10, 56
  //| or rdx, r10
  //| mov64 r10, (uint64_t)0x000000000000ff00
  //| and r10, rax
  //| shl r10, 40
  //| or rdx, r10
  //| mov64 r10, (uint64_t)0x0000000000ff0000
  //| and r10, rax
  //| shl r10, 24
  //| or rdx, r10
  //| mov64 r10, (uint64_t)0x00000000ff000000
  //| and r10, rax
  //| shl r10, 8
  //| or rdx, r10
  //| mov64 r10, (uint64_t)0x000000ff00000000
  //| and r10, rax
  //| shr r10, 8
  //| or rdx, r10
  //| mov64 r10, (uint64_t)0x0000ff0000000000
  //| and r10, rax
  //| shr r10, 24
  //| or rdx, r10
  //| mov64 r10, (uint64_t)0x00ff000000000000
  //| and r10, rax
  //| shr r10, 40
  //| or rdx, r10
  //| mov64 r10, (uint64_t)0xff00000000000000
  //| and r10, rax
  //| shr r10, 56
  //| or rdx, r10
  //| op2_r_x mov, target, rdx
  dasm_put(Dst, 1560, (unsigned int)((uint64_t)0x00000000000000ff), (unsigned int)(((uint64_t)0x00000000000000ff)>>32), (unsigned int)((uint64_t)0x000000000000ff00), (unsigned int)(((uint64_t)0x000000000000ff00)>>32), (unsigned int)((uint64_t)0x0000000000ff0000), (unsigned int)(((uint64_t)0x0000000000ff0000)>>32), (unsigned int)((uint64_t)0x00000000ff000000), (unsigned int)(((uint64_t)0x00000000ff000000)>>32), (unsigned int)((uint64_t)0x000000ff00000000), (unsigned int)(((uint64_t)0x000000ff00000000)>>32), (unsigned int)((uint64_t)0x0000ff0000000000), (unsigned int)(((uint64_t)0x0000ff0000000000)>>32), (unsigned int)((uint64_t)0x00ff000000000000), (unsigned int)(((uint64_t)0x00ff000000000000)>>32), (unsigned int)((uint64_t)0xff00000000000000), (unsigned int)(((uint64_t)0xff00000000000000)>>32));
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 801, (loc1));
  } else {
  dasm_put(Dst, 807, Dt1(->registers[target]));
  }
#line 1252 "src/machine/aot/aot.x64.c"

  return DASM_S_OK;
}

int aot_rol(AotContext* context, riscv_register_t target, AotValue a, AotValue b)
{
  int ret;
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  ret = aot_mov_internal(context, target, a, X64_RAX);
  if (ret != DASM_S_OK) { return ret; }

  switch (b.tag) {
    case AOT_TAG_REGISTER:
      //| op2_x_r mov, rcx, b.value.reg
      loc1 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 1191, (loc1));
      } else {
      dasm_put(Dst, 1197, Dt1(->registers[b.value.reg]));
      }
#line 1268 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_IMMEDIATE:
      /*
       * shift operations only use cl as operand, there won't be any
       * overflowing issues.
       */
      //| mov ecx, b.value.i
      dasm_put(Dst, 1202, b.value.i);
#line 1275 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_X64_REGISTER:
      //| mov rcx, Rq(b.value.x64_reg)
      dasm_put(Dst, 1191, (b.value.x64_reg));
#line 1278 "src/machine/aot/aot.x64.c"
      break;
  }

  //| op2_r_x rol, target, cl
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 1680, (loc1));
  } else {
  dasm_put(Dst, 1686, Dt1(->registers[target]));
  }
#line 1282 "src/machine/aot/aot.x64.c"

  return DASM_S_OK;
}

int aot_ror(AotContext* context, riscv_register_t target, AotValue a, AotValue b)
{
  int ret;
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  ret = aot_mov_internal(context, target, a, X64_RAX);
  if (ret != DASM_S_OK) { return ret; }

  switch (b.tag) {
    case AOT_TAG_REGISTER:
      //| op2_x_r mov, rcx, b.value.reg
      loc1 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 1191, (loc1));
      } else {
      dasm_put(Dst, 1197, Dt1(->registers[b.value.reg]));
      }
#line 1298 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_IMMEDIATE:
      /*
       * shift operations only use cl as operand, there won't be any
       * overflowing issues.
       */
      //| mov ecx, b.value.i
      dasm_put(Dst, 1202, b.value.i);
#line 1305 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_X64_REGISTER:
      //| mov rcx, Rq(b.value.x64_reg)
      dasm_put(Dst, 1191, (b.value.x64_reg));
#line 1308 "src/machine/aot/aot.x64.c"
      break;
  }

  //| op2_r_x ror, target, cl
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 1691, (loc1));
  } else {
  dasm_put(Dst, 1697, Dt1(->registers[target]));
  }
#line 1312 "src/machine/aot/aot.x64.c"

  return DASM_S_OK;
}

int aot_eq(AotContext* context, riscv_register_t target, AotValue a, AotValue b)
{
  uint32_t loc1;
  int ret;
  dasm_State** Dst = &context->d;

  ret = aot_mov_x64(context, X64_RCX, a);
  if (ret != DASM_S_OK) { return ret; }

  switch (b.tag) {
    case AOT_TAG_REGISTER:
      //| op2_x_r cmp, rcx, b.value.reg
      loc1 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 1702, (loc1));
      } else {
      dasm_put(Dst, 1708, Dt1(->registers[b.value.reg]));
      }
#line 1328 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_IMMEDIATE:
      //| load_imm rax, b.value.i
      if ((b.value.i >> (context->version >= 1? 31: 32)) > 0 && ((b.value.i & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
      dasm_put(Dst, 535, (unsigned int)(b.value.i), (unsigned int)((b.value.i)>>32));
      } else {
      dasm_put(Dst, 1713, b.value.i);
      }
#line 1331 "src/machine/aot/aot.x64.c"
      //| cmp rcx, rax
      dasm_put(Dst, 1718);
#line 1332 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_X64_REGISTER:
      //| cmp rcx, Rq(b.value.x64_reg)
      dasm_put(Dst, 1702, (b.value.x64_reg));
#line 1335 "src/machine/aot/aot.x64.c"
      break;
  }

  //| sete cl
  //| movzx rcx, cl
  //| op2_r_x mov, target, rcx
  dasm_put(Dst, 1722);
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 1730, (loc1));
  } else {
  dasm_put(Dst, 1736, Dt1(->registers[target]));
  }
#line 1341 "src/machine/aot/aot.x64.c"

  return DASM_S_OK;
}

int aot_lt(AotContext* context, riscv_register_t target, AotValue a, AotValue b, int is_signed)
{
  uint32_t loc1;
  int ret;
  dasm_State** Dst = &context->d;

  ret = aot_mov_x64(context, X64_RCX, a);
  if (ret != DASM_S_OK) { return ret; }

  switch (b.tag) {
    case AOT_TAG_REGISTER:
      //| op2_x_r cmp, rcx, b.value.reg
      loc1 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 1702, (loc1));
      } else {
      dasm_put(Dst, 1708, Dt1(->registers[b.value.reg]));
      }
#line 1357 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_IMMEDIATE:
      //| load_imm rax, b.value.i
      if ((b.value.i >> (context->version >= 1? 31: 32)) > 0 && ((b.value.i & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
      dasm_put(Dst, 535, (unsigned int)(b.value.i), (unsigned int)((b.value.i)>>32));
      } else {
      dasm_put(Dst, 1713, b.value.i);
      }
#line 1360 "src/machine/aot/aot.x64.c"
      //| cmp rcx, rax
      dasm_put(Dst, 1718);
#line 1361 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_X64_REGISTER:
      //| cmp rcx, Rq(b.value.x64_reg)
      dasm_put(Dst, 1702, (b.value.x64_reg));
#line 1364 "src/machine/aot/aot.x64.c"
      break;
  }

  if (is_signed) {
    //| setl cl
    dasm_put(Dst, 1741);
#line 1369 "src/machine/aot/aot.x64.c"
  } else {
    //| setb cl
    dasm_put(Dst, 1745);
#line 1371 "src/machine/aot/aot.x64.c"
  }
  //| movzx rcx, cl
  //| op2_r_x mov, target, rcx
  dasm_put(Dst, 1725);
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 1730, (loc1));
  } else {
  dasm_put(Dst, 1736, Dt1(->registers[target]));
  }
#line 1374 "src/machine/aot/aot.x64.c"

  return DASM_S_OK;
}

int aot_cond(AotContext* context, riscv_register_t target, AotValue condition, AotValue true_value, AotValue false_value) {
  uint32_t loc1;
  int ret;
  dasm_State** Dst = &context->d;

  switch (condition.tag) {
    case AOT_TAG_REGISTER:
      //| op2_r_imm cmp, condition.value.reg, (uint64_t)1, rax
      if (((uint64_t)1 >> (context->version >= 1? 31: 32)) > 0 && (((uint64_t)1 & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
        loc1 = riscv_reg_to_x64_reg(condition.value.reg);
      dasm_put(Dst, 535, (unsigned int)((uint64_t)1), (unsigned int)(((uint64_t)1)>>32));
        if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 1749, (loc1));
        } else {
      dasm_put(Dst, 1755, Dt1(->registers[condition.value.reg]));
        }
      } else {
        loc1 = riscv_reg_to_x64_reg(condition.value.reg);
        if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 1760, (loc1), (uint64_t)1);
        } else {
      dasm_put(Dst, 1768, Dt1(->registers[condition.value.reg]), (uint64_t)1);
        }
      }
#line 1386 "src/machine/aot/aot.x64.c"
      //| jne >1
      dasm_put(Dst, 897);
#line 1387 "src/machine/aot/aot.x64.c"
      ret = aot_mov_internal(context, target, true_value, X64_RAX);
      if (ret != DASM_S_OK) { return ret; }
      //| jmp >2
      //|1:
      dasm_put(Dst, 1774);
#line 1391 "src/machine/aot/aot.x64.c"
      ret = aot_mov_internal(context, target, false_value, X64_RAX);
      if (ret != DASM_S_OK) { return ret; }
      //|2:
      dasm_put(Dst, 872);
#line 1394 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_IMMEDIATE:
      ret = aot_mov_internal(context, target, (condition.value.i == 1) ? true_value : false_value, X64_RAX);
      if (ret != DASM_S_OK) { return ret; }
      break;
    case AOT_TAG_X64_REGISTER:
      //| cmp Rq(condition.value.x64_reg), 1
      //| jne >1
      dasm_put(Dst, 1781, (condition.value.x64_reg));
#line 1402 "src/machine/aot/aot.x64.c"
      ret = aot_mov_internal(context, target, true_value, X64_RAX);
      if (ret != DASM_S_OK) { return ret; }
      //| jmp >2
      //|1:
      dasm_put(Dst, 1774);
#line 1406 "src/machine/aot/aot.x64.c"
      ret = aot_mov_internal(context, target, false_value, X64_RAX);
      if (ret != DASM_S_OK) { return ret; }
      //|2:
      dasm_put(Dst, 872);
#line 1409 "src/machine/aot/aot.x64.c"
      break;
  }

  return DASM_S_OK;
}

int aot_clz(AotContext* context, riscv_register_t target, AotValue a)
{
  uint32_t loc1;
  int ret;
  dasm_State** Dst = &context->d;

  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }

  //| cmp rax, 0
  //| je >1
  //| bsr rax, rax
  //| neg rax
  //| add rax, 63
  //| op2_r_x mov, target, rax
  dasm_put(Dst, 1793);
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 753, (loc1));
  } else {
  dasm_put(Dst, 759, Dt1(->registers[target]));
  }
#line 1430 "src/machine/aot/aot.x64.c"
  //| jmp >2
  //|1:
  //| op2_r_imm mov, target, (uint64_t)64, rax
  dasm_put(Dst, 1774);
  if (((uint64_t)64 >> (context->version >= 1? 31: 32)) > 0 && (((uint64_t)64 & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
    loc1 = riscv_reg_to_x64_reg(target);
  dasm_put(Dst, 535, (unsigned int)((uint64_t)64), (unsigned int)(((uint64_t)64)>>32));
    if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 753, (loc1));
    } else {
  dasm_put(Dst, 759, Dt1(->registers[target]));
    }
  } else {
    loc1 = riscv_reg_to_x64_reg(target);
    if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 925, (loc1), (uint64_t)64);
    } else {
  dasm_put(Dst, 932, Dt1(->registers[target]), (uint64_t)64);
    }
  }
#line 1433 "src/machine/aot/aot.x64.c"
  //| jmp >2
  //|2:
  dasm_put(Dst, 1815);
#line 1435 "src/machine/aot/aot.x64.c"

  return DASM_S_OK;
}

int aot_ctz(AotContext* context, riscv_register_t target, AotValue a)
{
  uint32_t loc1;
  int ret;
  dasm_State** Dst = &context->d;

  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }

  //| cmp rax, 0
  //| je >1
  //| bsf rax, rax
  //| op2_r_x mov, target, rax
  dasm_put(Dst, 1822);
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 753, (loc1));
  } else {
  dasm_put(Dst, 759, Dt1(->registers[target]));
  }
#line 1452 "src/machine/aot/aot.x64.c"
  //| jmp >2
  //|1:
  //| op2_r_imm mov, target, (uint64_t)64, rax
  dasm_put(Dst, 1774);
  if (((uint64_t)64 >> (context->version >= 1? 31: 32)) > 0 && (((uint64_t)64 & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
    loc1 = riscv_reg_to_x64_reg(target);
  dasm_put(Dst, 535, (unsigned int)((uint64_t)64), (unsigned int)(((uint64_t)64)>>32));
    if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 753, (loc1));
    } else {
  dasm_put(Dst, 759, Dt1(->registers[target]));
    }
  } else {
    loc1 = riscv_reg_to_x64_reg(target);
    if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 925, (loc1), (uint64_t)64);
    } else {
  dasm_put(Dst, 932, Dt1(->registers[target]), (uint64_t)64);
    }
  }
#line 1455 "src/machine/aot/aot.x64.c"
  //| jmp >2
  //|2:
  dasm_put(Dst, 1815);
#line 1457 "src/machine/aot/aot.x64.c"

  return DASM_S_OK;
}

int aot_cpop(AotContext* context, riscv_register_t target, AotValue a)
{
  uint32_t loc1;
  int ret;
  dasm_State** Dst = &context->d;

  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }

  //| mov rdx, rax
  //| shr rdx, 1
  //| mov64 rcx, 0x5555555555555555
  //| and rdx, rcx
  //| sub rax, rdx
  //| mov rdx, rax
  //| mov64 rcx, 0x3333333333333333
  //| and rdx, rcx
  //| shr rax, 2
  //| and rax, rcx
  //| add rax, rdx
  //| mov rdx, rax
  //| shr rdx, 4
  //| add rax, rdx
  //| mov64 rcx, 0x0F0F0F0F0F0F0F0F
  //| and rax, rcx
  //| mov rdx, rax
  //| shr rdx, 8
  //| add rax, rdx
  //| mov rdx, rax
  //| shr rdx, 16
  //| add rax, rdx
  //| mov rdx, rax
  //| shr rdx, 32
  //| add rax, rdx
  //| and rax, 0x7F
  //| op2_r_x mov, target, rax
  dasm_put(Dst, 1836, (unsigned int)(0x5555555555555555), (unsigned int)((0x5555555555555555)>>32), (unsigned int)(0x3333333333333333), (unsigned int)((0x3333333333333333)>>32), (unsigned int)(0x0F0F0F0F0F0F0F0F), (unsigned int)((0x0F0F0F0F0F0F0F0F)>>32));
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 753, (loc1));
  } else {
  dasm_put(Dst, 759, Dt1(->registers[target]));
  }
#line 1497 "src/machine/aot/aot.x64.c"

  return DASM_S_OK;
}

int aot_extend(AotContext* context, riscv_register_t target, AotValue src, AotValue bits, int is_signed)
{
  uint32_t loc1;
  int ret;
  dasm_State** Dst = &context->d;

  if (bits.tag == AOT_TAG_IMMEDIATE && bits.value.i == 32) {
    /* Shortcut */
    ret = aot_mov_x64(context, X64_RAX, src);
    if (ret != DASM_S_OK) { return ret; }

    if (is_signed) {
      //| movsxd rax, eax
      dasm_put(Dst, 1929);
#line 1514 "src/machine/aot/aot.x64.c"
    } else {
      //| mov eax, eax
      dasm_put(Dst, 1933);
#line 1516 "src/machine/aot/aot.x64.c"
    }
    //| op2_r_x mov, target, rax
    loc1 = riscv_reg_to_x64_reg(target);
    if (VALID_X64_REGISTER(loc1)) {
    dasm_put(Dst, 753, (loc1));
    } else {
    dasm_put(Dst, 759, Dt1(->registers[target]));
    }
#line 1518 "src/machine/aot/aot.x64.c"
    return DASM_S_OK;
  }

  /*
   * In the general path, we do sign_extend by shifting left (64 - bits) bits,
   * then shifting right arithmetically (64 - bits) bits again.
   */
  ret = aot_mov_x64(context, X64_RAX, src);
  if (ret != DASM_S_OK) { return ret; }

  switch (bits.tag) {
    case AOT_TAG_REGISTER:
      ret = aot_mov_x64(context, X64_RDX, bits);
      if (ret != DASM_S_OK) { return ret; }

      //| mov ecx, 64
      //| and edx, 0x3F
      //| sub ecx, edx
      //| shl rax, cl
      //| mov ecx, edx
      dasm_put(Dst, 1936);
#line 1538 "src/machine/aot/aot.x64.c"
      if (is_signed) {
        //| sar rax, cl
        dasm_put(Dst, 1952);
#line 1540 "src/machine/aot/aot.x64.c"
      } else {
        //| shr rax, cl
        dasm_put(Dst, 1957);
#line 1542 "src/machine/aot/aot.x64.c"
      }
      break;
    case AOT_TAG_IMMEDIATE:
      if (bits.value.i < 64) {
        //| shl rax, (64 - bits.value.i)
        dasm_put(Dst, 1961, (64 - bits.value.i));
#line 1547 "src/machine/aot/aot.x64.c"
        if (is_signed) {
          //| sar rax, (64 - bits.value.i)
          dasm_put(Dst, 1966, (64 - bits.value.i));
#line 1549 "src/machine/aot/aot.x64.c"
        } else {
          //| shr rax, (64 - bits.value.i)
          dasm_put(Dst, 1972, (64 - bits.value.i));
#line 1551 "src/machine/aot/aot.x64.c"
        }
      }
      break;
    case AOT_TAG_X64_REGISTER:
      //| mov rdx, Rq(bits.value.x64_reg)
      //| mov ecx, 64
      //| and edx, 0x3F
      //| sub ecx, edx
      //| shl rax, cl
      //| mov ecx, edx
      dasm_put(Dst, 1977, (bits.value.x64_reg));
#line 1561 "src/machine/aot/aot.x64.c"
      if (is_signed) {
        //| sar rax, cl
        dasm_put(Dst, 1952);
#line 1563 "src/machine/aot/aot.x64.c"
      } else {
        //| shr rax, cl
        dasm_put(Dst, 1957);
#line 1565 "src/machine/aot/aot.x64.c"
      }
      break;
  }

  //| op2_r_x mov, target, rax
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 753, (loc1));
  } else {
  dasm_put(Dst, 759, Dt1(->registers[target]));
  }
#line 1570 "src/machine/aot/aot.x64.c"

  return DASM_S_OK;
}

int aot_exit(AotContext* context, int code)
{
  dasm_State** Dst = &context->d;
  //| mov rax, code
  //| jmp ->exit
  dasm_put(Dst, 1998, code);
#line 1579 "src/machine/aot/aot.x64.c"
  return DASM_S_OK;
}

int aot_add_cycles(AotContext* context, uint64_t cycles)
{
  int ret;
  dasm_State** Dst = &context->d;
  if (cycles == 0) {
    return DASM_S_OK;
  }
  //| load_imm rax, cycles
  if ((cycles >> (context->version >= 1? 31: 32)) > 0 && ((cycles & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
  dasm_put(Dst, 535, (unsigned int)(cycles), (unsigned int)((cycles)>>32));
  } else {
  dasm_put(Dst, 1713, cycles);
  }
#line 1590 "src/machine/aot/aot.x64.c"
  //| add rax, machine->cycles
  //| jnc >1
  dasm_put(Dst, 2007, Dt1(->cycles));
#line 1592 "src/machine/aot/aot.x64.c"
  ret = aot_exit(context, CKB_VM_ASM_RET_CYCLES_OVERFLOW);
  if (ret != DASM_S_OK) { return ret; }
  //|1:
  //| cmp rax, machine->max_cycles
  //| jna >2
  dasm_put(Dst, 2016, Dt1(->max_cycles));
#line 1597 "src/machine/aot/aot.x64.c"
  ret = aot_exit(context, CKB_VM_ASM_RET_MAX_CYCLES_EXCEEDED);
  if (ret != DASM_S_OK) { return ret; }
  //|2:
  //| mov machine->cycles, rax
  dasm_put(Dst, 2027, Dt1(->cycles));
#line 1601 "src/machine/aot/aot.x64.c"
  return DASM_S_OK;
}

int aot_ecall(AotContext* context)
{
  return aot_exit(context, CKB_VM_ASM_RET_ECALL);
}

int aot_ebreak(AotContext* context)
{
  return aot_exit(context, CKB_VM_ASM_RET_EBREAK);
}

int aot_slowpath(AotContext* context)
{
  return aot_exit(context, CKB_VM_ASM_RET_SLOWPATH);
}

int aot_mov_pc_internal(AotContext* context, AotValue value)
{
  int ret;
  dasm_State** Dst = &context->d;

  switch (value.tag) {
    case AOT_TAG_REGISTER:
      /*
       * At encoding time we cannot tell what address to jump to here,
       * so all we can do here is to update the correct PC register,
       * exit from current function call and defer to the machine to
       * handle this.
       */
      ret = aot_mov_x64(context, X64_RCX, value);
      if (ret != DASM_S_OK) { return ret; }
      //| mov machine->pc, rcx
      dasm_put(Dst, 1736, Dt1(->pc));
#line 1635 "src/machine/aot/aot.x64.c"
      ret = aot_exit(context, CKB_VM_ASM_RET_DYNAMIC_JUMP);
      if (ret != DASM_S_OK) { return ret; }
      break;
    case AOT_TAG_IMMEDIATE:
      /*
       * It's very unlikely we will expand CKB-VM to have more than 4GB memory,
       * hence we are leveraging this fact to encode dynasm dynamic label into
       * the upper 32-bit part of immediate. This way we can both update correct
       * PC value, and do quick jumps.
       * Also, since the maximum number of labels supported now is 65535, we
       * actually only need 16 bits of the upper 32-bit part, hence we are using
       * the highest byte to store flags for different kinds of labels.
       */
      switch ((uint8_t) (value.value.i >> 56)) {
        case 0x80:
          /*
           * This means just writing the result without actually jump
           */
          //| load_imm rcx, (value.value.i & 0xFFFFFFFFFFFFFF)
          if (((value.value.i & 0xFFFFFFFFFFFFFF) >> (context->version >= 1? 31: 32)) > 0 && (((value.value.i & 0xFFFFFFFFFFFFFF) & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
          dasm_put(Dst, 738, (unsigned int)((value.value.i & 0xFFFFFFFFFFFFFF)), (unsigned int)(((value.value.i & 0xFFFFFFFFFFFFFF))>>32));
          } else {
          dasm_put(Dst, 743, (value.value.i & 0xFFFFFFFFFFFFFF));
          }
#line 1654 "src/machine/aot/aot.x64.c"
          //| mov qword machine->pc, rcx
          dasm_put(Dst, 1736, Dt1(->pc));
#line 1655 "src/machine/aot/aot.x64.c"
          break;
        case 0x40:
          //| mov qword machine->pc, ((uint32_t)(value.value.i & 0x7FFFFFFF))
          //| jmp =>((value.value.i >> 32) ^ 0x40000000)
          dasm_put(Dst, 2034, Dt1(->pc), ((uint32_t)(value.value.i & 0x7FFFFFFF)), ((value.value.i >> 32) ^ 0x40000000));
#line 1659 "src/machine/aot/aot.x64.c"
          break;
        case 0x0:
          //| load_imm rcx, value.value.i
          if ((value.value.i >> (context->version >= 1? 31: 32)) > 0 && ((value.value.i & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
          dasm_put(Dst, 738, (unsigned int)(value.value.i), (unsigned int)((value.value.i)>>32));
          } else {
          dasm_put(Dst, 743, value.value.i);
          }
#line 1662 "src/machine/aot/aot.x64.c"
          //| mov machine->pc, rcx
          dasm_put(Dst, 1736, Dt1(->pc));
#line 1663 "src/machine/aot/aot.x64.c"
          ret = aot_exit(context, CKB_VM_ASM_RET_DYNAMIC_JUMP);
          if (ret != DASM_S_OK) { return ret; }
          break;
        default:
          return ERROR_INVALID_VALUE;
      }
      break;
    case AOT_TAG_X64_REGISTER:
      //| mov machine->pc, Rq(value.value.x64_reg)
      dasm_put(Dst, 2043, (value.value.x64_reg), Dt1(->pc));
#line 1672 "src/machine/aot/aot.x64.c"
      ret = aot_exit(context, CKB_VM_ASM_RET_DYNAMIC_JUMP);
      if (ret != DASM_S_OK) { return ret; }
      break;
  }

  return DASM_S_OK;
}

int aot_cond_pc(AotContext* context, AotValue condition, AotValue true_value, AotValue false_value)
{
  uint32_t loc1;
  int ret;
  dasm_State** Dst = &context->d;

  switch (condition.tag) {
    case AOT_TAG_REGISTER:
      //| op2_r_imm cmp, condition.value.reg, (uint64_t)1, rax
      if (((uint64_t)1 >> (context->version >= 1? 31: 32)) > 0 && (((uint64_t)1 & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
        loc1 = riscv_reg_to_x64_reg(condition.value.reg);
      dasm_put(Dst, 535, (unsigned int)((uint64_t)1), (unsigned int)(((uint64_t)1)>>32));
        if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 1749, (loc1));
        } else {
      dasm_put(Dst, 1755, Dt1(->registers[condition.value.reg]));
        }
      } else {
        loc1 = riscv_reg_to_x64_reg(condition.value.reg);
        if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 1760, (loc1), (uint64_t)1);
        } else {
      dasm_put(Dst, 1768, Dt1(->registers[condition.value.reg]), (uint64_t)1);
        }
      }
#line 1689 "src/machine/aot/aot.x64.c"
      //| jne >1
      dasm_put(Dst, 897);
#line 1690 "src/machine/aot/aot.x64.c"
      ret = aot_mov_pc_internal(context, true_value);
      if (ret != DASM_S_OK) { return ret; }
      //|1:
      dasm_put(Dst, 186);
#line 1693 "src/machine/aot/aot.x64.c"
      ret = aot_mov_pc_internal(context, false_value);
      if (ret != DASM_S_OK) { return ret; }
      break;
    case AOT_TAG_IMMEDIATE:
      ret = aot_mov_pc_internal(context, (condition.value.i == 1) ? true_value : false_value);
      if (ret != DASM_S_OK) { return ret; }
      break;
    case AOT_TAG_X64_REGISTER:
      //| cmp Rq(condition.value.x64_reg), 1
      //| jne >1
      dasm_put(Dst, 1781, (condition.value.x64_reg));
#line 1703 "src/machine/aot/aot.x64.c"
      ret = aot_mov_pc_internal(context, true_value);
      if (ret != DASM_S_OK) { return ret; }
      //|1:
      dasm_put(Dst, 186);
#line 1706 "src/machine/aot/aot.x64.c"
      ret = aot_mov_pc_internal(context, false_value);
      if (ret != DASM_S_OK) { return ret; }
      break;
  }

  return DASM_S_OK;
}

int aot_memory_write(AotContext* context, AotValue address, AotValue v, uint32_t size)
{
  int ret;
  dasm_State** Dst = &context->d;

  ret = aot_mov_x64(context, X64_RAX, address);
  if (ret != DASM_S_OK) { return ret; }

  //| mov rdx, size
  //| call ->check_write
  //| cmp rdx, 0
  //| jne >1
  //| lea rdx, machine->memory
  dasm_put(Dst, 2051, size, Dt1(->memory));
#line 1727 "src/machine/aot/aot.x64.c"
  ret = aot_mov_x64(context, X64_RCX, v);
  if (ret != DASM_S_OK) { return ret; }
  switch (size) {
    case 1:
      //| mov byte [rdx+rax], cl
      dasm_put(Dst, 2072);
#line 1732 "src/machine/aot/aot.x64.c"
      break;
    case 2:
      //| mov word [rdx+rax], cx
      dasm_put(Dst, 2076);
#line 1735 "src/machine/aot/aot.x64.c"
      break;
    case 4:
      //| mov dword [rdx+rax], ecx
      dasm_put(Dst, 2077);
#line 1738 "src/machine/aot/aot.x64.c"
      break;
    case 8:
      //| mov qword [rdx+rax], rcx
      dasm_put(Dst, 2081);
#line 1741 "src/machine/aot/aot.x64.c"
      break;
    default:
      return ERROR_INVALID_MEMORY_SIZE;
  }
  //| jmp >2
  //|1:
  //| mov rax, rdx
  //| jmp ->exit
  //|2:
  dasm_put(Dst, 2086);
#line 1750 "src/machine/aot/aot.x64.c"

  return DASM_S_OK;
}

int aot_memory_read(AotContext* context, uint32_t target, AotValue address, uint32_t size)
{
  uint32_t loc1;
  int ret;
  dasm_State** Dst = &context->d;

  ret = aot_mov_x64(context, X64_RAX, address);
  if (ret != DASM_S_OK) { return ret; }

  //| mov rdx, size
  //| call ->check_read
  //| cmp rdx, 0
  //| jne >1
  //| mov rdx, rax
  //| add rdx, size
  //| jc >1
  //| cmp rdx, CKB_VM_ASM_RISCV_MAX_MEMORY
  dasm_put(Dst, 2102, size, size, CKB_VM_ASM_RISCV_MAX_MEMORY);
#line 1771 "src/machine/aot/aot.x64.c"
  if (context->version >= 1) {
    //| ja >1
    dasm_put(Dst, 2135);
#line 1773 "src/machine/aot/aot.x64.c"
  } else {
    //| jae >1
    dasm_put(Dst, 2011);
#line 1775 "src/machine/aot/aot.x64.c"
  }
  //| lea rdx, machine->memory
  dasm_put(Dst, 2067, Dt1(->memory));
#line 1777 "src/machine/aot/aot.x64.c"
  switch (size) {
    case 1:
      //| movzx ecx, byte [rdx+rax]
      dasm_put(Dst, 2140);
#line 1780 "src/machine/aot/aot.x64.c"
      break;
    case 2:
      //| movzx ecx, word [rdx+rax]
      dasm_put(Dst, 2145);
#line 1783 "src/machine/aot/aot.x64.c"
      break;
    case 4:
      //| mov ecx, dword [rdx+rax]
      dasm_put(Dst, 2150);
#line 1786 "src/machine/aot/aot.x64.c"
      break;
    case 8:
      //| mov rcx, qword [rdx+rax]
      dasm_put(Dst, 2154);
#line 1789 "src/machine/aot/aot.x64.c"
      break;
    default:
      return ERROR_INVALID_MEMORY_SIZE;
  }
  //| op2_r_x mov, target, rcx
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REGISTER(loc1)) {
  dasm_put(Dst, 1730, (loc1));
  } else {
  dasm_put(Dst, 1736, Dt1(->registers[target]));
  }
#line 1794 "src/machine/aot/aot.x64.c"
  //| jmp >2
  //| 1:
  dasm_put(Dst, 1774);
#line 1796 "src/machine/aot/aot.x64.c"
  ret = aot_exit(context, CKB_VM_ASM_RET_OUT_OF_BOUND);
  if (ret != DASM_S_OK) { return ret; }
  //| 2:
  dasm_put(Dst, 872);
#line 1799 "src/machine/aot/aot.x64.c"

  return DASM_S_OK;
}

static int aot_mov_internal(AotContext* context, riscv_register_t target, AotValue value, x64_register_t x64_temp_reg)
{
  uint32_t loc1, loc2;
  dasm_State** Dst = &context->d;

  switch (value.tag) {
    case AOT_TAG_REGISTER:
      if (target == value.value.reg) { return DASM_S_OK; }
      //| op2_r_r mov, target, value.value.reg, Rq(x64_temp_reg)
      loc1 = riscv_reg_to_x64_reg(target);
      loc2 = riscv_reg_to_x64_reg(value.value.reg);
      if (VALID_X64_REGISTER(loc1) && VALID_X64_REGISTER(loc2)) {
      dasm_put(Dst, 2159, (loc2), (loc1));
      } else if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 2167, (loc1), Dt1(->registers[value.value.reg]));
      } else if (VALID_X64_REGISTER(loc2)) {
      dasm_put(Dst, 2043, (loc2), Dt1(->registers[target]));
      } else {
      dasm_put(Dst, 2175, (x64_temp_reg), Dt1(->registers[value.value.reg]), (x64_temp_reg), Dt1(->registers[target]));
      }
#line 1812 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_IMMEDIATE:
      //| op2_r_imm mov, target, value.value.i, Rq(x64_temp_reg)
      if ((value.value.i >> (context->version >= 1? 31: 32)) > 0 && ((value.value.i & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
        loc1 = riscv_reg_to_x64_reg(target);
      dasm_put(Dst, 2190, (x64_temp_reg), (unsigned int)(value.value.i), (unsigned int)((value.value.i)>>32));
        if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 2159, (x64_temp_reg), (loc1));
        } else {
      dasm_put(Dst, 2043, (x64_temp_reg), Dt1(->registers[target]));
        }
      } else {
        loc1 = riscv_reg_to_x64_reg(target);
        if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 925, (loc1), value.value.i);
        } else {
      dasm_put(Dst, 932, Dt1(->registers[target]), value.value.i);
        }
      }
#line 1815 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_X64_REGISTER:
      //| op2_r_x mov, target, Rq(value.value.x64_reg)
      loc1 = riscv_reg_to_x64_reg(target);
      if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 2159, (value.value.x64_reg), (loc1));
      } else {
      dasm_put(Dst, 2043, (value.value.x64_reg), Dt1(->registers[target]));
      }
#line 1818 "src/machine/aot/aot.x64.c"
      break;
  }

  return DASM_S_OK;
}

static int aot_mov_x64(AotContext* context, x64_register_t x64_target, AotValue value)
{
  uint32_t loc1;
  dasm_State** Dst = &context->d;
  switch (value.tag) {
    case AOT_TAG_REGISTER:
      //| op2_x_r mov, Rq(x64_target), value.value.reg
      loc1 = riscv_reg_to_x64_reg(value.value.reg);
      if (VALID_X64_REGISTER(loc1)) {
      dasm_put(Dst, 2159, (loc1), (x64_target));
      } else {
      dasm_put(Dst, 2167, (x64_target), Dt1(->registers[value.value.reg]));
      }
#line 1831 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_IMMEDIATE:
      //| load_imm Rq(x64_target), value.value.i
      if ((value.value.i >> (context->version >= 1? 31: 32)) > 0 && ((value.value.i & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
      dasm_put(Dst, 2190, (x64_target), (unsigned int)(value.value.i), (unsigned int)((value.value.i)>>32));
      } else {
      dasm_put(Dst, 925, (x64_target), value.value.i);
      }
#line 1834 "src/machine/aot/aot.x64.c"
      break;
    case AOT_TAG_X64_REGISTER:
      if (x64_target == value.value.x64_reg) { return DASM_S_OK; }
      //| mov Rq(x64_target), Rq(value.value.x64_reg)
      dasm_put(Dst, 2159, (value.value.x64_reg), (x64_target));
#line 1838 "src/machine/aot/aot.x64.c"
      break;
  }
  return DASM_S_OK;
}
