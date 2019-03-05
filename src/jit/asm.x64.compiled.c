/*
** This file has been pre-processed with DynASM.
** http://luajit.org/dynasm.html
** DynASM version 1.4.0, DynASM x64 version 1.4.0
** DO NOT EDIT! The original file is in "src/jit/asm.x64.c".
*/

#line 1 "src/jit/asm.x64.c"
#include <stdint.h>
#include <stdio.h>

#include "dasm_proto.h"
#include "dasm_x86.h"

#define ERROR_INVALID_SIZE 0xFFFFFF00
#define ERROR_INVALID_ARGUMENT 0xFFFFFF01

extern int ckb_vm_jit_ffi_load8(void*, uint64_t, uint64_t*);
extern int ckb_vm_jit_ffi_load16(void*, uint64_t, uint64_t*);
extern int ckb_vm_jit_ffi_load32(void*, uint64_t, uint64_t*);
extern int ckb_vm_jit_ffi_load64(void*, uint64_t, uint64_t*);
extern int ckb_vm_jit_ffi_store8(void*, uint64_t, uint64_t);
extern int ckb_vm_jit_ffi_store16(void*, uint64_t, uint64_t);
extern int ckb_vm_jit_ffi_store32(void*, uint64_t, uint64_t);
extern int ckb_vm_jit_ffi_store64(void*, uint64_t, uint64_t);

//|.arch x64
#if DASM_VERSION != 10400
#error "Version mismatch between DynASM and included encoding engine"
#endif
#line 20 "src/jit/asm.x64.c"
//|.section code
#define DASM_SECTION_CODE	0
#define DASM_MAXSECTION		1
#line 21 "src/jit/asm.x64.c"
//|.globals lbl_
enum {
  lbl__MAX
};
#line 22 "src/jit/asm.x64.c"
//|.actionlist bf_actions
static const unsigned char bf_actions[945] = {
  254,0,65,84,65,85,65,86,65,87,83,85,72,137,252,253,255,72,139,149,233,72,
  139,157,233,72,139,181,233,72,139,189,233,76,139,133,233,76,139,141,233,76,
  139,149,233,76,139,157,233,76,139,165,233,76,139,173,233,76,139,181,233,76,
  139,189,233,255,72,137,149,233,72,137,157,233,72,137,181,233,72,137,189,233,
  76,137,133,233,76,137,141,233,76,137,149,233,76,137,157,233,76,137,165,233,
  76,137,173,233,76,137,181,233,76,137,189,233,255,93,91,65,95,65,94,65,93,
  65,92,195,255,72,137,192,240,131,240,35,255,72,139,133,253,240,131,233,255,
  72,137,133,253,240,131,233,255,80,255,72,139,133,233,72,137,133,233,255,88,
  255,64,80,240,42,255,72,199,192,240,35,237,72,193,224,240,35,32,72,129,200,
  240,35,239,255,64,88,240,42,255,72,199,192,240,35,237,255,72,199,133,233,
  237,255,72,1,192,240,131,240,35,255,72,3,133,253,240,131,233,255,72,1,133,
  253,240,131,233,255,72,139,133,233,72,1,133,233,255,72,129,192,240,35,239,
  255,72,129,133,233,239,255,72,41,192,240,131,240,35,255,72,43,133,253,240,
  131,233,255,72,41,133,253,240,131,233,255,72,139,133,233,72,41,133,233,255,
  72,129,232,240,35,239,255,72,129,173,233,239,255,72,15,175,192,240,36,255,
  72,15,175,133,233,255,72,199,193,237,72,193,225,32,72,129,201,239,72,15,175,
  193,255,72,137,192,240,35,255,72,137,209,255,72,252,247,232,240,35,255,72,
  252,247,173,233,255,72,252,247,224,240,35,255,72,252,247,165,233,255,81,255,
  72,199,193,237,72,193,225,32,72,129,201,239,255,72,199,193,237,255,72,252,
  247,252,233,255,72,252,247,225,255,89,255,72,137,208,72,137,202,255,72,133,
  192,15,137,244,247,255,72,252,247,216,82,255,72,131,252,242,252,255,72,137,
  209,90,255,72,15,175,193,89,255,72,133,192,15,148,208,72,15,182,192,255,72,
  1,200,252,233,244,248,255,248,1,82,255,72,137,208,90,248,2,255,72,184,237,
  237,255,72,57,200,15,133,244,247,72,199,192,252,255,252,255,252,255,252,255,
  255,72,57,200,15,133,244,247,255,252,233,244,249,255,248,1,72,199,192,0,0,
  0,0,255,72,57,200,15,133,244,248,255,252,233,244,249,248,2,255,72,153,255,
  72,252,247,252,248,240,35,255,72,252,247,189,233,255,72,49,210,255,72,252,
  247,252,240,240,35,255,72,252,247,181,233,255,72,153,72,252,247,252,249,255,
  72,49,210,72,252,247,252,241,255,248,3,255,72,33,192,240,131,240,35,255,72,
  35,133,253,240,131,233,255,72,33,133,253,240,131,233,255,72,139,133,233,72,
  33,133,233,255,72,129,224,240,35,239,255,72,129,165,233,239,255,72,9,192,
  240,131,240,35,255,72,11,133,253,240,131,233,255,72,9,133,253,240,131,233,
  255,72,139,133,233,72,9,133,233,255,72,129,141,233,239,255,72,252,247,208,
  240,35,255,72,252,247,149,233,255,72,49,192,240,131,240,35,255,72,51,133,
  253,240,131,233,255,72,49,133,253,240,131,233,255,72,139,133,233,72,49,133,
  233,255,72,129,252,240,240,35,239,255,72,129,181,233,239,255,72,137,193,240,
  131,255,72,139,141,233,255,72,211,224,240,35,255,72,211,165,233,255,72,211,
  252,248,240,35,255,72,211,189,233,255,72,211,232,240,35,255,72,211,173,233,
  255,72,57,193,240,131,255,72,59,141,233,255,72,199,192,237,72,193,224,32,
  72,129,200,239,255,72,199,192,237,255,72,57,193,255,15,148,209,72,15,182,
  201,255,72,137,200,240,35,255,72,137,141,233,255,15,156,209,255,15,146,209,
  255,72,57,192,240,131,240,35,255,72,57,133,253,240,131,233,255,72,129,252,
  248,240,35,239,255,72,129,189,233,239,255,252,233,244,248,248,1,255,72,139,
  189,233,255,72,139,181,233,255,72,199,198,237,72,193,230,32,72,129,206,239,
  255,72,199,198,237,255,72,139,149,233,255,72,199,194,237,72,193,226,32,72,
  129,202,239,255,72,199,194,237,255,84,252,255,52,36,72,131,228,252,240,255,
  252,255,208,72,139,100,36,8,255,72,141,149,233,255
};

#line 23 "src/jit/asm.x64.c"

typedef struct {
  dasm_State* d;
  void* labels[lbl__MAX];
  uint32_t npc;
  uint32_t nextpc;
  uint32_t x64_reg_flags;
} AsmContext;

/*
 * RISC-V has 32 general purpose registers, we take advantage of this
 * fact and encode 32 as PC registers for unified processing. After that,
 * 2 temporary registers are created to store intermediate value. On x64
 * they are mapped to rax and rcx. Other platforms might use different
 * registers and in the worst case, they might be memory location stored
 * in context struct.
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
#define REGISTER_PC 32
#define REGISTER_TEMP_RAX 33
#define REGISTER_TEMP_RCX 34
#define MAXIMUM_REGISTER 34
#define INVALID_REGISTER (MAXIMUM_REGISTER + 1)
#define VALID_REG(r) ((r) <= MAXIMUM_REGISTER)

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
#define VALID_X64_REG(r) ((r) <= MAXIMUM_X64_REGISTER)

#define X64_REG_FLAG(reg) (1 << (reg))
#define MARK_X64_REG_USED(context, r) ((context)->x64_reg_flags |= X64_REG_FLAG(r))
#define UNMARK_X64_REG_USED(context, r) ((context)->x64_reg_flags &= (~X64_REG_FLAG(r)))
#define TEST_X64_REG_USED(context, r) (((context)->x64_reg_flags & X64_REG_FLAG(r)) != 0)

typedef struct {
  uint64_t registers[33];
  void* m;
} AsmMachine;

#define ASM_TAG_REGISTER 0x1
#define ASM_TAG_IMMEDIATE 0x2

typedef struct {
  uint32_t tag;
  union {
    uint32_t reg;
    uint64_t i;
  } value;
} AsmValue;

#define ASM_VALUE_IS_REGISTER_LOCATION(v, r) \
  ((v).tag == ASM_TAG_REGISTER && \
   (v).value.reg == (r))

int32_t riscv_reg_to_x64_reg(uint32_t reg)
{
  switch (reg) {
    case REGISTER_RA:
      return X64_RDX;
    case REGISTER_SP:
      return X64_RBX;
    case REGISTER_T0:
      return X64_RSI;
    case REGISTER_T1:
      return X64_RDI;
    case REGISTER_A0:
      return X64_R8;
    case REGISTER_A1:
      return X64_R9;
    case REGISTER_A2:
      return X64_R10;
    case REGISTER_A3:
      return X64_R11;
    case REGISTER_A4:
      return X64_R12;
    case REGISTER_A5:
      return X64_R13;
    case REGISTER_A6:
      return X64_R14;
    case REGISTER_A7:
      return X64_R15;
    case REGISTER_TEMP_RAX:
      return X64_RAX;
    case REGISTER_TEMP_RCX:
      return X64_RCX;
    default:
      return INVALID_X64_REGISTER;
  }
}

//|.type machine, AsmMachine, rbp
#define Dt1(_V) (int)(ptrdiff_t)&(((AsmMachine *)0)_V)
#line 161 "src/jit/asm.x64.c"

//|.macro load_imm64, x64_reg, imm64
//| mov x64_reg, imm64 >> 32
//| shl x64_reg, 32
//| or x64_reg, imm64 & 0xFFFFFFFF
//|.endmacro

//|.macro load_imm, x64_reg, imm
//||if (imm > 0xFFFFFFFF) {
//|   load_imm64 x64_reg, imm
//||} else {
//|   mov x64_reg, imm
//||}
//|.endmacro

/* r_r means both operands here are RISC-V registers */
//|.macro op2_r_r, op, target, source
//||loc1 = riscv_reg_to_x64_reg(target);
//||loc2 = riscv_reg_to_x64_reg(source);
//||if (VALID_X64_REG(loc1) && VALID_X64_REG(loc2)) {
//|  op Rq(loc1), Rq(loc2)
//||} else if (VALID_X64_REG(loc1)) {
//|  op Rq(loc1), machine->registers[source]
//||} else if (VALID_X64_REG(loc2)) {
//|  op machine->registers[target], Rq(loc2)
//||} else {
/* If target is RAX, we won't be in this branch */
//|| if (TEST_X64_REG_USED(context, X64_RAX)) {
//|    push rax
//|| }
//|  mov rax, qword machine->registers[source]
//|  op qword machine->registers[target], rax
//|| if (TEST_X64_REG_USED(context, X64_RAX)) {
//|    pop rax
//|| }
//||}
//|.endmacro

//|.macro op1_r, op, reg
//||loc1 = riscv_reg_to_x64_reg(reg);
//||if (VALID_X64_REG(loc1)) {
//|  op Rq(loc1)
//||} else {
//|  op qword machine->registers[reg]
//||}
//|.endmacro

/* r_x means that the first operand is RISC-V register, the second is X86 one */
//|.macro op2_r_x, op, target, x64_source
//||loc1 = riscv_reg_to_x64_reg(target);
//||if (VALID_X64_REG(loc1)) {
//|  op Rq(loc1), x64_source
//||} else {
//|  op qword machine->registers[target], x64_source
//||}
//|.endmacro

//|.macro op2_r_imm, op, target, imm
//||if (imm > 0xFFFFFFFF) {
//||  loc1 = riscv_reg_to_x64_reg(target);
//||  loc2 = (loc1 == X64_RAX) ? (X64_RCX) : (X64_RAX);
//||  if (TEST_X64_REG_USED(context, loc2)) {
//|     push Rq(loc2)
//||  }
//|   load_imm64 Rq(loc2), imm
//||  if (VALID_X64_REG(loc1)) {
//|     op Rq(loc1), Rq(loc2)
//||  } else {
//|     op qword machine->registers[target], Rq(loc2)
//||  }
//||  if (TEST_X64_REG_USED(context, loc2)) {
//|     pop Rq(loc2)
//||  }
//||} else {
//||  loc1 = riscv_reg_to_x64_reg(target);
//||  if (VALID_X64_REG(loc1)) {
//|     op Rq(loc1), imm
//||  } else {
//|     op qword machine->registers[target], imm
//||  }
//||}
//|.endmacro

//|.macro op2_x_r, op, x64_target, source
//||loc1 = riscv_reg_to_x64_reg(source);
//||if (VALID_X64_REG(loc1)) {
//|  op x64_target, Rq(loc1)
//||} else {
//|  op x64_target, machine->registers[source]
//||}
//|.endmacro

static int asm_lock_temp(AsmContext* context, uint32_t temp_reg, AsmValue v, int *used);
static int asm_release_temp(AsmContext* context, uint32_t temp_reg, int used);
static int asm_lock_x64_reg(AsmContext* context, int32_t x64_reg, int* used);
static int asm_release_x64_reg(AsmContext* context, int32_t x64_reg, int used);
static int asm_mov_to_x64_reg(AsmContext* context, int32_t x64_reg, AsmValue value);

AsmContext* asm_new()
{
  AsmContext* context = malloc(sizeof(AsmContext));
  dasm_init(&context->d, DASM_MAXSECTION);
  dasm_setupglobal(&context->d, context->labels, lbl__MAX);
  context->npc = 8;
  context->nextpc = 0;
  context->x64_reg_flags = 0;
  return context;
}

void asm_finalize(AsmContext* context)
{
  dasm_free(&context->d);
  free(context);
}

int asm_setup(AsmContext* context)
{
  dasm_State** Dst = &context->d;
  dasm_setup(&context->d, bf_actions);
  dasm_growpc(&context->d, context->npc);
  //|.code
  dasm_put(Dst, 0);
#line 282 "src/jit/asm.x64.c"
  //| push r12
  //| push r13
  //| push r14
  //| push r15
  //| push rbx
  //| push rbp
  //| mov rbp, rdi
  dasm_put(Dst, 2);
#line 289 "src/jit/asm.x64.c"
  return DASM_S_OK;
}

int asm_emit_prologue(AsmContext* context)
{
  dasm_State** Dst = &context->d;
  //| mov rdx, machine->registers[REGISTER_RA]
  //| mov rbx, machine->registers[REGISTER_SP]
  //| mov rsi, machine->registers[REGISTER_T0]
  //| mov rdi, machine->registers[REGISTER_T1]
  //| mov r8, machine->registers[REGISTER_A0]
  //| mov r9, machine->registers[REGISTER_A1]
  //| mov r10, machine->registers[REGISTER_A2]
  //| mov r11, machine->registers[REGISTER_A3]
  //| mov r12, machine->registers[REGISTER_A4]
  //| mov r13, machine->registers[REGISTER_A5]
  //| mov r14, machine->registers[REGISTER_A6]
  //| mov r15, machine->registers[REGISTER_A7]
  dasm_put(Dst, 17, Dt1(->registers[REGISTER_RA]), Dt1(->registers[REGISTER_SP]), Dt1(->registers[REGISTER_T0]), Dt1(->registers[REGISTER_T1]), Dt1(->registers[REGISTER_A0]), Dt1(->registers[REGISTER_A1]), Dt1(->registers[REGISTER_A2]), Dt1(->registers[REGISTER_A3]), Dt1(->registers[REGISTER_A4]), Dt1(->registers[REGISTER_A5]), Dt1(->registers[REGISTER_A6]), Dt1(->registers[REGISTER_A7]));
#line 307 "src/jit/asm.x64.c"
  return DASM_S_OK;
}

int asm_emit_epilogue(AsmContext* context)
{
  dasm_State** Dst = &context->d;
  //| mov machine->registers[REGISTER_RA], rdx
  //| mov machine->registers[REGISTER_SP], rbx
  //| mov machine->registers[REGISTER_T0], rsi
  //| mov machine->registers[REGISTER_T1], rdi
  //| mov machine->registers[REGISTER_A0], r8
  //| mov machine->registers[REGISTER_A1], r9
  //| mov machine->registers[REGISTER_A2], r10
  //| mov machine->registers[REGISTER_A3], r11
  //| mov machine->registers[REGISTER_A4], r12
  //| mov machine->registers[REGISTER_A5], r13
  //| mov machine->registers[REGISTER_A6], r14
  //| mov machine->registers[REGISTER_A7], r15
  dasm_put(Dst, 66, Dt1(->registers[REGISTER_RA]), Dt1(->registers[REGISTER_SP]), Dt1(->registers[REGISTER_T0]), Dt1(->registers[REGISTER_T1]), Dt1(->registers[REGISTER_A0]), Dt1(->registers[REGISTER_A1]), Dt1(->registers[REGISTER_A2]), Dt1(->registers[REGISTER_A3]), Dt1(->registers[REGISTER_A4]), Dt1(->registers[REGISTER_A5]), Dt1(->registers[REGISTER_A6]), Dt1(->registers[REGISTER_A7]));
#line 325 "src/jit/asm.x64.c"
  return DASM_S_OK;
}

int asm_link(AsmContext* context, size_t *szp)
{
  dasm_State** Dst = &context->d;
  //| pop rbp
  //| pop rbx
  //| pop r15
  //| pop r14
  //| pop r13
  //| pop r12
  //| ret
  dasm_put(Dst, 115);
#line 338 "src/jit/asm.x64.c"
  return dasm_link(&context->d, szp);
}

int asm_encode(AsmContext* context, void *buffer)
{
  return dasm_encode(&context->d, buffer);
}

int asm_mov(AsmContext* context, uint32_t target, AsmValue value)
{
  uint32_t loc1, loc2;
  dasm_State** Dst = &context->d;
  switch (value.tag) {
    case ASM_TAG_REGISTER:
      if (target == value.value.reg) { return DASM_S_OK; }
      //| op2_r_r mov, target, value.value.reg
      loc1 = riscv_reg_to_x64_reg(target);
      loc2 = riscv_reg_to_x64_reg(value.value.reg);
      if (VALID_X64_REG(loc1) && VALID_X64_REG(loc2)) {
      dasm_put(Dst, 127, (loc2), (loc1));
      } else if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 135, (loc1), Dt1(->registers[value.value.reg]));
      } else if (VALID_X64_REG(loc2)) {
      dasm_put(Dst, 143, (loc2), Dt1(->registers[target]));
      } else {
       if (TEST_X64_REG_USED(context, X64_RAX)) {
      dasm_put(Dst, 151);
       }
      dasm_put(Dst, 153, Dt1(->registers[value.value.reg]), Dt1(->registers[target]));
       if (TEST_X64_REG_USED(context, X64_RAX)) {
      dasm_put(Dst, 162);
       }
      }
#line 354 "src/jit/asm.x64.c"
      break;
    case ASM_TAG_IMMEDIATE:
      //| op2_r_imm mov, target, value.value.i
      if (value.value.i > 0xFFFFFFFF) {
        loc1 = riscv_reg_to_x64_reg(target);
        loc2 = (loc1 == X64_RAX) ? (X64_RCX) : (X64_RAX);
        if (TEST_X64_REG_USED(context, loc2)) {
      dasm_put(Dst, 164, (loc2));
        }
      dasm_put(Dst, 169, (loc2), value.value.i >> 32, (loc2), (loc2), value.value.i & 0xFFFFFFFF);
        if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 127, (loc2), (loc1));
        } else {
      dasm_put(Dst, 143, (loc2), Dt1(->registers[target]));
        }
        if (TEST_X64_REG_USED(context, loc2)) {
      dasm_put(Dst, 188, (loc2));
        }
      } else {
        loc1 = riscv_reg_to_x64_reg(target);
        if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 193, (loc1), value.value.i);
        } else {
      dasm_put(Dst, 200, Dt1(->registers[target]), value.value.i);
        }
      }
#line 357 "src/jit/asm.x64.c"
      break;
  }
  return DASM_S_OK;
}

int asm_add(AsmContext* context, uint32_t target, AsmValue a, AsmValue b)
{
  int ret, used, moved = 0;
  uint32_t loc1, loc2;
  dasm_State** Dst = &context->d;

  if (ASM_VALUE_IS_REGISTER_LOCATION(b, target)) {
    moved = 1;
    ret = asm_lock_temp(context, REGISTER_TEMP_RCX, b, &used);
    if (ret != DASM_S_OK) { return ret; }

    b.value.reg = REGISTER_TEMP_RCX;
  }

  ret = asm_mov(context, target, a);
  if (ret != DASM_S_OK) { return ret; }

  switch (b.tag) {
    case ASM_TAG_REGISTER:
      //| op2_r_r add, target, b.value.reg
      loc1 = riscv_reg_to_x64_reg(target);
      loc2 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REG(loc1) && VALID_X64_REG(loc2)) {
      dasm_put(Dst, 206, (loc2), (loc1));
      } else if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 214, (loc1), Dt1(->registers[b.value.reg]));
      } else if (VALID_X64_REG(loc2)) {
      dasm_put(Dst, 222, (loc2), Dt1(->registers[target]));
      } else {
       if (TEST_X64_REG_USED(context, X64_RAX)) {
      dasm_put(Dst, 151);
       }
      dasm_put(Dst, 230, Dt1(->registers[b.value.reg]), Dt1(->registers[target]));
       if (TEST_X64_REG_USED(context, X64_RAX)) {
      dasm_put(Dst, 162);
       }
      }
#line 382 "src/jit/asm.x64.c"
      break;
    case ASM_TAG_IMMEDIATE:
      //| op2_r_imm add, target, b.value.i
      if (b.value.i > 0xFFFFFFFF) {
        loc1 = riscv_reg_to_x64_reg(target);
        loc2 = (loc1 == X64_RAX) ? (X64_RCX) : (X64_RAX);
        if (TEST_X64_REG_USED(context, loc2)) {
      dasm_put(Dst, 164, (loc2));
        }
      dasm_put(Dst, 169, (loc2), b.value.i >> 32, (loc2), (loc2), b.value.i & 0xFFFFFFFF);
        if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 206, (loc2), (loc1));
        } else {
      dasm_put(Dst, 222, (loc2), Dt1(->registers[target]));
        }
        if (TEST_X64_REG_USED(context, loc2)) {
      dasm_put(Dst, 188, (loc2));
        }
      } else {
        loc1 = riscv_reg_to_x64_reg(target);
        if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 239, (loc1), b.value.i);
        } else {
      dasm_put(Dst, 246, Dt1(->registers[target]), b.value.i);
        }
      }
#line 385 "src/jit/asm.x64.c"
      break;
  }

  if (moved) {
    ret = asm_release_temp(context, REGISTER_TEMP_RCX, used);
    if (ret != DASM_S_OK) { return ret; }
  }

  return DASM_S_OK;
}

int asm_sub(AsmContext* context, uint32_t target, AsmValue a, AsmValue b)
{
  int ret, used, moved = 0;
  uint32_t loc1, loc2;
  dasm_State** Dst = &context->d;

  if (ASM_VALUE_IS_REGISTER_LOCATION(b, target)) {
    moved = 1;
    ret = asm_lock_temp(context, REGISTER_TEMP_RCX, b, &used);
    if (ret != DASM_S_OK) { return ret; }

    b.value.reg = REGISTER_TEMP_RCX;
  }

  ret = asm_mov(context, target, a);
  if (ret != DASM_S_OK) { return ret; }

  switch (b.tag) {
    case ASM_TAG_REGISTER:
      //| op2_r_r sub, target, b.value.reg
      loc1 = riscv_reg_to_x64_reg(target);
      loc2 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REG(loc1) && VALID_X64_REG(loc2)) {
      dasm_put(Dst, 252, (loc2), (loc1));
      } else if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 260, (loc1), Dt1(->registers[b.value.reg]));
      } else if (VALID_X64_REG(loc2)) {
      dasm_put(Dst, 268, (loc2), Dt1(->registers[target]));
      } else {
       if (TEST_X64_REG_USED(context, X64_RAX)) {
      dasm_put(Dst, 151);
       }
      dasm_put(Dst, 276, Dt1(->registers[b.value.reg]), Dt1(->registers[target]));
       if (TEST_X64_REG_USED(context, X64_RAX)) {
      dasm_put(Dst, 162);
       }
      }
#line 416 "src/jit/asm.x64.c"
      break;
    case ASM_TAG_IMMEDIATE:
      //| op2_r_imm sub, target, b.value.i
      if (b.value.i > 0xFFFFFFFF) {
        loc1 = riscv_reg_to_x64_reg(target);
        loc2 = (loc1 == X64_RAX) ? (X64_RCX) : (X64_RAX);
        if (TEST_X64_REG_USED(context, loc2)) {
      dasm_put(Dst, 164, (loc2));
        }
      dasm_put(Dst, 169, (loc2), b.value.i >> 32, (loc2), (loc2), b.value.i & 0xFFFFFFFF);
        if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 252, (loc2), (loc1));
        } else {
      dasm_put(Dst, 268, (loc2), Dt1(->registers[target]));
        }
        if (TEST_X64_REG_USED(context, loc2)) {
      dasm_put(Dst, 188, (loc2));
        }
      } else {
        loc1 = riscv_reg_to_x64_reg(target);
        if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 285, (loc1), b.value.i);
        } else {
      dasm_put(Dst, 292, Dt1(->registers[target]), b.value.i);
        }
      }
#line 419 "src/jit/asm.x64.c"
      break;
  }

  if (moved) {
    ret = asm_release_temp(context, REGISTER_TEMP_RCX, used);
    if (ret != DASM_S_OK) { return ret; }
  }

  return DASM_S_OK;
}

int asm_mul(AsmContext* context, uint32_t target, AsmValue a, AsmValue b)
{
  int ret, rax_used, rcx_used;
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  ret = asm_lock_x64_reg(context, X64_RAX, &rax_used);
  if (ret != DASM_S_OK) { return ret; }
  ret = asm_mov_to_x64_reg(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }

  switch (b.tag) {
    case ASM_TAG_REGISTER:
      //| op2_x_r imul, rax, b.value.reg
      loc1 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 298, (loc1));
      } else {
      dasm_put(Dst, 305, Dt1(->registers[b.value.reg]));
      }
#line 444 "src/jit/asm.x64.c"
      break;
    case ASM_TAG_IMMEDIATE:
      ret = asm_lock_x64_reg(context, X64_RCX, &rcx_used);
      if (ret != DASM_S_OK) { return ret; }
      ret = asm_mov_to_x64_reg(context, X64_RCX, a);
      if (ret != DASM_S_OK) { return ret; }
      //| load_imm64 rcx, b.value.i
      //| imul rax, rcx
      dasm_put(Dst, 311, b.value.i >> 32, b.value.i & 0xFFFFFFFF);
#line 452 "src/jit/asm.x64.c"
      ret = asm_release_x64_reg(context, X64_RCX, rcx_used);
      if (ret != DASM_S_OK) { return ret; }
      break;
  }

  //| op2_r_x mov, target, rax
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REG(loc1)) {
  dasm_put(Dst, 328, (loc1));
  } else {
  dasm_put(Dst, 157, Dt1(->registers[target]));
  }
#line 458 "src/jit/asm.x64.c"

  ret = asm_release_x64_reg(context, X64_RAX, rax_used);
  if (ret != DASM_S_OK) { return ret; }

  return DASM_S_OK;
}

int asm_mulh(AsmContext* context, uint32_t target, AsmValue a, AsmValue b, int is_signed)
{
  int ret, rax_used, rcx_used;
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  ret = asm_lock_x64_reg(context, X64_RAX, &rax_used);
  if (ret != DASM_S_OK) { return ret; }
  ret = asm_lock_x64_reg(context, X64_RCX, &rcx_used);
  if (ret != DASM_S_OK) { return ret; }

  ret = asm_mov_to_x64_reg(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }
  //| mov rcx, rdx
  dasm_put(Dst, 334);
#line 479 "src/jit/asm.x64.c"
  switch (b.tag) {
    case ASM_TAG_REGISTER:
      if (is_signed) {
        //| op1_r imul, b.value.reg
        loc1 = riscv_reg_to_x64_reg(b.value.reg);
        if (VALID_X64_REG(loc1)) {
        dasm_put(Dst, 338, (loc1));
        } else {
        dasm_put(Dst, 345, Dt1(->registers[b.value.reg]));
        }
#line 483 "src/jit/asm.x64.c"
      } else {
        //| op1_r mul, b.value.reg
        loc1 = riscv_reg_to_x64_reg(b.value.reg);
        if (VALID_X64_REG(loc1)) {
        dasm_put(Dst, 351, (loc1));
        } else {
        dasm_put(Dst, 358, Dt1(->registers[b.value.reg]));
        }
#line 485 "src/jit/asm.x64.c"
      }
      break;
    case ASM_TAG_IMMEDIATE:
      //| push rcx
      //| load_imm rcx, b.value.i
      dasm_put(Dst, 364);
      if (b.value.i > 0xFFFFFFFF) {
      dasm_put(Dst, 366, b.value.i >> 32, b.value.i & 0xFFFFFFFF);
      } else {
      dasm_put(Dst, 379, b.value.i);
      }
#line 490 "src/jit/asm.x64.c"
      if (is_signed) {
        //| imul rcx
        dasm_put(Dst, 384);
#line 492 "src/jit/asm.x64.c"
      } else {
        //| mul rcx
        dasm_put(Dst, 390);
#line 494 "src/jit/asm.x64.c"
      }
      //| pop rcx
      dasm_put(Dst, 395);
#line 496 "src/jit/asm.x64.c"
      break;
  }
  //| mov rax, rdx
  //| mov rdx, rcx
  //| op2_r_x mov, target, rax
  dasm_put(Dst, 397);
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REG(loc1)) {
  dasm_put(Dst, 328, (loc1));
  } else {
  dasm_put(Dst, 157, Dt1(->registers[target]));
  }
#line 501 "src/jit/asm.x64.c"

  ret = asm_release_x64_reg(context, X64_RCX, rcx_used);
  if (ret != DASM_S_OK) { return ret; }
  ret = asm_release_x64_reg(context, X64_RAX, rax_used);
  if (ret != DASM_S_OK) { return ret; }

  return DASM_S_OK;
}

/* Inspired from https://github.com/rv8-io/rv8/blob/834259098a5c182874aac97d82a164d144244e1a/src/jit/jit-emitter-rv64.h#L931 */
int asm_mulhsu(AsmContext* context, uint32_t target, AsmValue a, AsmValue b)
{
  int ret, rax_used, rcx_used;
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  ret = asm_lock_x64_reg(context, X64_RAX, &rax_used);
  if (ret != DASM_S_OK) { return ret; }
  ret = asm_lock_x64_reg(context, X64_RCX, &rcx_used);
  if (ret != DASM_S_OK) { return ret; }

  ret = asm_mov_to_x64_reg(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }

  //| test rax, rax
  //| jns >1
  dasm_put(Dst, 404);
#line 527 "src/jit/asm.x64.c"
  /* calculate res = mulhu(-a, b), res is stored in rdx after this. */
  //| neg rax
  //| push rdx
  dasm_put(Dst, 412);
#line 530 "src/jit/asm.x64.c"
  switch (b.tag) {
    case ASM_TAG_REGISTER:
      //| op1_r mul, b.value.reg
      loc1 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 351, (loc1));
      } else {
      dasm_put(Dst, 358, Dt1(->registers[b.value.reg]));
      }
#line 533 "src/jit/asm.x64.c"
      break;
    case ASM_TAG_IMMEDIATE:
      //| load_imm rcx, b.value.i
      if (b.value.i > 0xFFFFFFFF) {
      dasm_put(Dst, 366, b.value.i >> 32, b.value.i & 0xFFFFFFFF);
      } else {
      dasm_put(Dst, 379, b.value.i);
      }
#line 536 "src/jit/asm.x64.c"
      //| mul rcx
      dasm_put(Dst, 390);
#line 537 "src/jit/asm.x64.c"
      break;
  }
  /* calculate ~res and store it in rcx */
  //| xor rdx, -1
  //| mov rcx, rdx
  //| pop rdx
  dasm_put(Dst, 418);
#line 543 "src/jit/asm.x64.c"
  /*
   * calculate (a * b), then test (a * b == 0) and convert that to 1 or 0,
   * result is stored in rax after this.
   */
  ret = asm_mov_to_x64_reg(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }
  switch (b.tag) {
    case ASM_TAG_REGISTER:
      //| op2_x_r imul, rax, b.value.reg
      loc1 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 298, (loc1));
      } else {
      dasm_put(Dst, 305, Dt1(->registers[b.value.reg]));
      }
#line 552 "src/jit/asm.x64.c"
      break;
    case ASM_TAG_IMMEDIATE:
      //| push rcx
      //| load_imm rcx, b.value.i
      dasm_put(Dst, 364);
      if (b.value.i > 0xFFFFFFFF) {
      dasm_put(Dst, 366, b.value.i >> 32, b.value.i & 0xFFFFFFFF);
      } else {
      dasm_put(Dst, 379, b.value.i);
      }
#line 556 "src/jit/asm.x64.c"
      //| imul rax, rcx
      //| pop rcx
      dasm_put(Dst, 429);
#line 558 "src/jit/asm.x64.c"
      break;
  }
  //| test rax, rax
  //| setz al
  //| movzx rax, al
  dasm_put(Dst, 435);
#line 563 "src/jit/asm.x64.c"
  /* calculate ~res + (a * b == 0) */
  //| add rax, rcx
  //| jmp >2
  dasm_put(Dst, 446);
#line 566 "src/jit/asm.x64.c"
  /* just mulhu here */
  //|1:
  //| push rdx
  dasm_put(Dst, 454);
#line 569 "src/jit/asm.x64.c"
  switch (b.tag) {
    case ASM_TAG_REGISTER:
      //| op1_r mul, b.value.reg
      loc1 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 351, (loc1));
      } else {
      dasm_put(Dst, 358, Dt1(->registers[b.value.reg]));
      }
#line 572 "src/jit/asm.x64.c"
      break;
    case ASM_TAG_IMMEDIATE:
      //| load_imm rcx, b.value.i
      if (b.value.i > 0xFFFFFFFF) {
      dasm_put(Dst, 366, b.value.i >> 32, b.value.i & 0xFFFFFFFF);
      } else {
      dasm_put(Dst, 379, b.value.i);
      }
#line 575 "src/jit/asm.x64.c"
      //| mul rcx
      dasm_put(Dst, 390);
#line 576 "src/jit/asm.x64.c"
      break;
  }
  /* calculate ~res and store it in rcx */
  //| mov rax, rdx
  //| pop rdx
  //|2:
  //| op2_r_x mov, target, rax
  dasm_put(Dst, 458);
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REG(loc1)) {
  dasm_put(Dst, 328, (loc1));
  } else {
  dasm_put(Dst, 157, Dt1(->registers[target]));
  }
#line 583 "src/jit/asm.x64.c"

  ret = asm_release_x64_reg(context, X64_RCX, rcx_used);
  if (ret != DASM_S_OK) { return ret; }
  ret = asm_release_x64_reg(context, X64_RAX, rax_used);
  if (ret != DASM_S_OK) { return ret; }

  return DASM_S_OK;
}

int asm_div(AsmContext* context, uint32_t target, AsmValue a, AsmValue b, int is_signed)
{
  int ret, rax_used, rcx_used;
  uint32_t loc1;
  dasm_State** Dst = &context->d;
  AsmValue t;

  ret = asm_lock_x64_reg(context, X64_RAX, &rax_used);
  if (ret != DASM_S_OK) { return ret; }
  ret = asm_lock_x64_reg(context, X64_RCX, &rcx_used);
  if (ret != DASM_S_OK) { return ret; }

  if (is_signed) {
    //| mov64 rax, INT64_MIN
    dasm_put(Dst, 465, (unsigned int)(INT64_MIN), (unsigned int)((INT64_MIN)>>32));
#line 606 "src/jit/asm.x64.c"
    ret = asm_mov_to_x64_reg(context, X64_RCX, a);
    if (ret != DASM_S_OK) { return ret; }
    //| cmp rax, rcx
    //| jne >1
    //| mov rax, -1
    dasm_put(Dst, 470);
#line 611 "src/jit/asm.x64.c"
    ret = asm_mov_to_x64_reg(context, X64_RCX, b);
    if (ret != DASM_S_OK) { return ret; }
    //| cmp rax, rcx
    //| jne >1
    dasm_put(Dst, 489);
#line 615 "src/jit/asm.x64.c"
    ret = asm_mov(context, target, a);
    if (ret != DASM_S_OK) { return ret; }
    //| jmp >3
    dasm_put(Dst, 497);
#line 618 "src/jit/asm.x64.c"
  }
  //|1:
  //| mov rax, 0
  dasm_put(Dst, 502);
#line 621 "src/jit/asm.x64.c"
  ret = asm_mov_to_x64_reg(context, X64_RCX, b);
  if (ret != DASM_S_OK) { return ret; }
  //| cmp rax, rcx
  //| jne >2
  dasm_put(Dst, 512);
#line 625 "src/jit/asm.x64.c"
  t.tag = ASM_TAG_IMMEDIATE;
  t.value.i = UINT64_MAX;
  ret = asm_mov(context, target, t);
  if (ret != DASM_S_OK) { return ret; }
  //| jmp >3
  //|2:
  dasm_put(Dst, 520);
#line 631 "src/jit/asm.x64.c"
  ret = asm_mov_to_x64_reg(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }
  //| mov rcx, rdx
  dasm_put(Dst, 334);
#line 634 "src/jit/asm.x64.c"
  switch (b.tag) {
    case ASM_TAG_REGISTER:
      if (is_signed) {
        //| cqo
        //| op1_r idiv, b.value.reg
        dasm_put(Dst, 527);
        loc1 = riscv_reg_to_x64_reg(b.value.reg);
        if (VALID_X64_REG(loc1)) {
        dasm_put(Dst, 530, (loc1));
        } else {
        dasm_put(Dst, 538, Dt1(->registers[b.value.reg]));
        }
#line 639 "src/jit/asm.x64.c"
      } else {
        //| xor rdx, rdx
        //| op1_r div, b.value.reg
        dasm_put(Dst, 544);
        loc1 = riscv_reg_to_x64_reg(b.value.reg);
        if (VALID_X64_REG(loc1)) {
        dasm_put(Dst, 548, (loc1));
        } else {
        dasm_put(Dst, 556, Dt1(->registers[b.value.reg]));
        }
#line 642 "src/jit/asm.x64.c"
      }
      break;
    case ASM_TAG_IMMEDIATE:
      //| push rcx
      //| load_imm, rcx, b.value.i
      dasm_put(Dst, 364);
      if (b.value.i > 0xFFFFFFFF) {
      dasm_put(Dst, 366, b.value.i >> 32, b.value.i & 0xFFFFFFFF);
      } else {
      dasm_put(Dst, 379, b.value.i);
      }
#line 647 "src/jit/asm.x64.c"
      if (is_signed) {
        //| cqo
        //| idiv rcx
        dasm_put(Dst, 562);
#line 650 "src/jit/asm.x64.c"
      } else {
        //| xor rdx, rdx
        //| div rcx
        dasm_put(Dst, 570);
#line 653 "src/jit/asm.x64.c"
      }
      //| pop rcx
      dasm_put(Dst, 395);
#line 655 "src/jit/asm.x64.c"
      break;
  }
  //| mov rdx, rcx
  //| op2_r_x mov, target, rax
  dasm_put(Dst, 400);
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REG(loc1)) {
  dasm_put(Dst, 328, (loc1));
  } else {
  dasm_put(Dst, 157, Dt1(->registers[target]));
  }
#line 659 "src/jit/asm.x64.c"
  //|3:
  dasm_put(Dst, 579);
#line 660 "src/jit/asm.x64.c"

  ret = asm_release_x64_reg(context, X64_RCX, rcx_used);
  if (ret != DASM_S_OK) { return ret; }
  ret = asm_release_x64_reg(context, X64_RAX, rax_used);
  if (ret != DASM_S_OK) { return ret; }

  return DASM_S_OK;
}

int asm_rem(AsmContext* context, uint32_t target, AsmValue a, AsmValue b, int is_signed)
{
  int ret, rax_used, rcx_used;
  uint32_t loc1;
  dasm_State** Dst = &context->d;
  AsmValue t;

  ret = asm_lock_x64_reg(context, X64_RAX, &rax_used);
  if (ret != DASM_S_OK) { return ret; }
  ret = asm_lock_x64_reg(context, X64_RCX, &rcx_used);
  if (ret != DASM_S_OK) { return ret; }

  if (is_signed) {
    //| mov64 rax, INT64_MIN
    dasm_put(Dst, 465, (unsigned int)(INT64_MIN), (unsigned int)((INT64_MIN)>>32));
#line 683 "src/jit/asm.x64.c"
    ret = asm_mov_to_x64_reg(context, X64_RCX, a);
    if (ret != DASM_S_OK) { return ret; }
    //| cmp rax, rcx
    //| jne >1
    //| mov rax, -1
    dasm_put(Dst, 470);
#line 688 "src/jit/asm.x64.c"
    ret = asm_mov_to_x64_reg(context, X64_RCX, b);
    if (ret != DASM_S_OK) { return ret; }
    //| cmp rax, rcx
    //| jne >1
    dasm_put(Dst, 489);
#line 692 "src/jit/asm.x64.c"
    t.tag = ASM_TAG_IMMEDIATE;
    t.value.i = 0;
    ret = asm_mov(context, target, t);
    if (ret != DASM_S_OK) { return ret; }
    //| jmp >3
    dasm_put(Dst, 497);
#line 697 "src/jit/asm.x64.c"
  }
  //|1:
  //| mov rax, 0
  dasm_put(Dst, 502);
#line 700 "src/jit/asm.x64.c"
  ret = asm_mov_to_x64_reg(context, X64_RCX, b);
  if (ret != DASM_S_OK) { return ret; }
  //| cmp rax, rcx
  //| jne >2
  dasm_put(Dst, 512);
#line 704 "src/jit/asm.x64.c"
  ret = asm_mov(context, target, a);
  if (ret != DASM_S_OK) { return ret; }
  //| jmp >3
  //|2:
  dasm_put(Dst, 520);
#line 708 "src/jit/asm.x64.c"
  ret = asm_mov_to_x64_reg(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }
  //| mov rcx, rdx
  dasm_put(Dst, 334);
#line 711 "src/jit/asm.x64.c"
  switch (b.tag) {
    case ASM_TAG_REGISTER:
      if (is_signed) {
        //| cqo
        //| op1_r idiv, b.value.reg
        dasm_put(Dst, 527);
        loc1 = riscv_reg_to_x64_reg(b.value.reg);
        if (VALID_X64_REG(loc1)) {
        dasm_put(Dst, 530, (loc1));
        } else {
        dasm_put(Dst, 538, Dt1(->registers[b.value.reg]));
        }
#line 716 "src/jit/asm.x64.c"
      } else {
        //| xor rdx, rdx
        //| op1_r div, b.value.reg
        dasm_put(Dst, 544);
        loc1 = riscv_reg_to_x64_reg(b.value.reg);
        if (VALID_X64_REG(loc1)) {
        dasm_put(Dst, 548, (loc1));
        } else {
        dasm_put(Dst, 556, Dt1(->registers[b.value.reg]));
        }
#line 719 "src/jit/asm.x64.c"
      }
      break;
    case ASM_TAG_IMMEDIATE:
      //| push rcx
      //| load_imm, rcx, b.value.i
      dasm_put(Dst, 364);
      if (b.value.i > 0xFFFFFFFF) {
      dasm_put(Dst, 366, b.value.i >> 32, b.value.i & 0xFFFFFFFF);
      } else {
      dasm_put(Dst, 379, b.value.i);
      }
#line 724 "src/jit/asm.x64.c"
      if (is_signed) {
        //| cqo
        //| idiv rcx
        dasm_put(Dst, 562);
#line 727 "src/jit/asm.x64.c"
      } else {
        //| xor rdx, rdx
        //| div rcx
        dasm_put(Dst, 570);
#line 730 "src/jit/asm.x64.c"
      }
      //| pop rcx
      dasm_put(Dst, 395);
#line 732 "src/jit/asm.x64.c"
      break;
  }
  //| mov rax, rdx
  //| mov rdx, rcx
  //| op2_r_x mov, target, rax
  dasm_put(Dst, 397);
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REG(loc1)) {
  dasm_put(Dst, 328, (loc1));
  } else {
  dasm_put(Dst, 157, Dt1(->registers[target]));
  }
#line 737 "src/jit/asm.x64.c"
  //|3:
  dasm_put(Dst, 579);
#line 738 "src/jit/asm.x64.c"

  ret = asm_release_x64_reg(context, X64_RCX, rcx_used);
  if (ret != DASM_S_OK) { return ret; }
  ret = asm_release_x64_reg(context, X64_RAX, rax_used);
  if (ret != DASM_S_OK) { return ret; }

  return DASM_S_OK;
}

int asm_and(AsmContext* context, uint32_t target, AsmValue a, AsmValue b)
{
  int ret, used, moved = 0;
  uint32_t loc1, loc2;
  dasm_State** Dst = &context->d;

  if (ASM_VALUE_IS_REGISTER_LOCATION(b, target)) {
    moved = 1;
    ret = asm_lock_temp(context, REGISTER_TEMP_RCX, b, &used);
    if (ret != DASM_S_OK) { return ret; }

    b.value.reg = REGISTER_TEMP_RCX;
  }

  ret = asm_mov(context, target, a);
  if (ret != DASM_S_OK) { return ret; }

  switch (b.tag) {
    case ASM_TAG_REGISTER:
      //| op2_r_r and, target, b.value.reg
      loc1 = riscv_reg_to_x64_reg(target);
      loc2 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REG(loc1) && VALID_X64_REG(loc2)) {
      dasm_put(Dst, 582, (loc2), (loc1));
      } else if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 590, (loc1), Dt1(->registers[b.value.reg]));
      } else if (VALID_X64_REG(loc2)) {
      dasm_put(Dst, 598, (loc2), Dt1(->registers[target]));
      } else {
       if (TEST_X64_REG_USED(context, X64_RAX)) {
      dasm_put(Dst, 151);
       }
      dasm_put(Dst, 606, Dt1(->registers[b.value.reg]), Dt1(->registers[target]));
       if (TEST_X64_REG_USED(context, X64_RAX)) {
      dasm_put(Dst, 162);
       }
      }
#line 767 "src/jit/asm.x64.c"
      break;
    case ASM_TAG_IMMEDIATE:
      //| op2_r_imm and, target, b.value.i
      if (b.value.i > 0xFFFFFFFF) {
        loc1 = riscv_reg_to_x64_reg(target);
        loc2 = (loc1 == X64_RAX) ? (X64_RCX) : (X64_RAX);
        if (TEST_X64_REG_USED(context, loc2)) {
      dasm_put(Dst, 164, (loc2));
        }
      dasm_put(Dst, 169, (loc2), b.value.i >> 32, (loc2), (loc2), b.value.i & 0xFFFFFFFF);
        if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 582, (loc2), (loc1));
        } else {
      dasm_put(Dst, 598, (loc2), Dt1(->registers[target]));
        }
        if (TEST_X64_REG_USED(context, loc2)) {
      dasm_put(Dst, 188, (loc2));
        }
      } else {
        loc1 = riscv_reg_to_x64_reg(target);
        if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 615, (loc1), b.value.i);
        } else {
      dasm_put(Dst, 622, Dt1(->registers[target]), b.value.i);
        }
      }
#line 770 "src/jit/asm.x64.c"
      break;
  }

  if (moved) {
    ret = asm_release_temp(context, REGISTER_TEMP_RCX, used);
    if (ret != DASM_S_OK) { return ret; }
  }

  return DASM_S_OK;
}

int asm_or(AsmContext* context, uint32_t target, AsmValue a, AsmValue b)
{
  int ret, used, moved = 0;
  uint32_t loc1, loc2;
  dasm_State** Dst = &context->d;

  if (ASM_VALUE_IS_REGISTER_LOCATION(b, target)) {
    moved = 1;
    ret = asm_lock_temp(context, REGISTER_TEMP_RCX, b, &used);
    if (ret != DASM_S_OK) { return ret; }

    b.value.reg = REGISTER_TEMP_RCX;
  }

  ret = asm_mov(context, target, a);
  if (ret != DASM_S_OK) { return ret; }

  switch (b.tag) {
    case ASM_TAG_REGISTER:
      //| op2_r_r or, target, b.value.reg
      loc1 = riscv_reg_to_x64_reg(target);
      loc2 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REG(loc1) && VALID_X64_REG(loc2)) {
      dasm_put(Dst, 628, (loc2), (loc1));
      } else if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 636, (loc1), Dt1(->registers[b.value.reg]));
      } else if (VALID_X64_REG(loc2)) {
      dasm_put(Dst, 644, (loc2), Dt1(->registers[target]));
      } else {
       if (TEST_X64_REG_USED(context, X64_RAX)) {
      dasm_put(Dst, 151);
       }
      dasm_put(Dst, 652, Dt1(->registers[b.value.reg]), Dt1(->registers[target]));
       if (TEST_X64_REG_USED(context, X64_RAX)) {
      dasm_put(Dst, 162);
       }
      }
#line 801 "src/jit/asm.x64.c"
      break;
    case ASM_TAG_IMMEDIATE:
      //| op2_r_imm or, target, b.value.i
      if (b.value.i > 0xFFFFFFFF) {
        loc1 = riscv_reg_to_x64_reg(target);
        loc2 = (loc1 == X64_RAX) ? (X64_RCX) : (X64_RAX);
        if (TEST_X64_REG_USED(context, loc2)) {
      dasm_put(Dst, 164, (loc2));
        }
      dasm_put(Dst, 169, (loc2), b.value.i >> 32, (loc2), (loc2), b.value.i & 0xFFFFFFFF);
        if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 628, (loc2), (loc1));
        } else {
      dasm_put(Dst, 644, (loc2), Dt1(->registers[target]));
        }
        if (TEST_X64_REG_USED(context, loc2)) {
      dasm_put(Dst, 188, (loc2));
        }
      } else {
        loc1 = riscv_reg_to_x64_reg(target);
        if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 181, (loc1), b.value.i);
        } else {
      dasm_put(Dst, 661, Dt1(->registers[target]), b.value.i);
        }
      }
#line 804 "src/jit/asm.x64.c"
      break;
  }

  if (moved) {
    ret = asm_release_temp(context, REGISTER_TEMP_RCX, used);
    if (ret != DASM_S_OK) { return ret; }
  }

  return DASM_S_OK;
}

int asm_not(AsmContext* context, uint32_t target, AsmValue a, int logical)
{
  int ret;
  uint32_t loc1, loc2;
  dasm_State** Dst = &context->d;

  ret = asm_mov(context, target, a);
  if (ret != DASM_S_OK) { return ret; }

  //| op1_r not, target
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REG(loc1)) {
  dasm_put(Dst, 667, (loc1));
  } else {
  dasm_put(Dst, 674, Dt1(->registers[target]));
  }
#line 825 "src/jit/asm.x64.c"
  if (logical) {
    //| op2_r_imm and, target, (uint64_t)1
    if ((uint64_t)1 > 0xFFFFFFFF) {
      loc1 = riscv_reg_to_x64_reg(target);
      loc2 = (loc1 == X64_RAX) ? (X64_RCX) : (X64_RAX);
      if (TEST_X64_REG_USED(context, loc2)) {
    dasm_put(Dst, 164, (loc2));
      }
    dasm_put(Dst, 169, (loc2), (uint64_t)1 >> 32, (loc2), (loc2), (uint64_t)1 & 0xFFFFFFFF);
      if (VALID_X64_REG(loc1)) {
    dasm_put(Dst, 582, (loc2), (loc1));
      } else {
    dasm_put(Dst, 598, (loc2), Dt1(->registers[target]));
      }
      if (TEST_X64_REG_USED(context, loc2)) {
    dasm_put(Dst, 188, (loc2));
      }
    } else {
      loc1 = riscv_reg_to_x64_reg(target);
      if (VALID_X64_REG(loc1)) {
    dasm_put(Dst, 615, (loc1), (uint64_t)1);
      } else {
    dasm_put(Dst, 622, Dt1(->registers[target]), (uint64_t)1);
      }
    }
#line 827 "src/jit/asm.x64.c"
  }

  return DASM_S_OK;
}

int asm_xor(AsmContext* context, uint32_t target, AsmValue a, AsmValue b)
{
  int ret, used, moved = 0;
  uint32_t loc1, loc2;
  dasm_State** Dst = &context->d;

  if (ASM_VALUE_IS_REGISTER_LOCATION(b, target)) {
    moved = 1;
    ret = asm_lock_temp(context, REGISTER_TEMP_RCX, b, &used);
    if (ret != DASM_S_OK) { return ret; }

    b.value.reg = REGISTER_TEMP_RCX;
  }

  ret = asm_mov(context, target, a);
  if (ret != DASM_S_OK) { return ret; }

  switch (b.tag) {
    case ASM_TAG_REGISTER:
      //| op2_r_r xor, target, b.value.reg
      loc1 = riscv_reg_to_x64_reg(target);
      loc2 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REG(loc1) && VALID_X64_REG(loc2)) {
      dasm_put(Dst, 680, (loc2), (loc1));
      } else if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 688, (loc1), Dt1(->registers[b.value.reg]));
      } else if (VALID_X64_REG(loc2)) {
      dasm_put(Dst, 696, (loc2), Dt1(->registers[target]));
      } else {
       if (TEST_X64_REG_USED(context, X64_RAX)) {
      dasm_put(Dst, 151);
       }
      dasm_put(Dst, 704, Dt1(->registers[b.value.reg]), Dt1(->registers[target]));
       if (TEST_X64_REG_USED(context, X64_RAX)) {
      dasm_put(Dst, 162);
       }
      }
#line 852 "src/jit/asm.x64.c"
      break;
    case ASM_TAG_IMMEDIATE:
      //| op2_r_imm xor, target, b.value.i
      if (b.value.i > 0xFFFFFFFF) {
        loc1 = riscv_reg_to_x64_reg(target);
        loc2 = (loc1 == X64_RAX) ? (X64_RCX) : (X64_RAX);
        if (TEST_X64_REG_USED(context, loc2)) {
      dasm_put(Dst, 164, (loc2));
        }
      dasm_put(Dst, 169, (loc2), b.value.i >> 32, (loc2), (loc2), b.value.i & 0xFFFFFFFF);
        if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 680, (loc2), (loc1));
        } else {
      dasm_put(Dst, 696, (loc2), Dt1(->registers[target]));
        }
        if (TEST_X64_REG_USED(context, loc2)) {
      dasm_put(Dst, 188, (loc2));
        }
      } else {
        loc1 = riscv_reg_to_x64_reg(target);
        if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 713, (loc1), b.value.i);
        } else {
      dasm_put(Dst, 721, Dt1(->registers[target]), b.value.i);
        }
      }
#line 855 "src/jit/asm.x64.c"
      break;
  }

  if (moved) {
    ret = asm_release_temp(context, REGISTER_TEMP_RCX, used);
    if (ret != DASM_S_OK) { return ret; }
  }

  return DASM_S_OK;
}

int asm_shl(AsmContext* context, uint32_t target, AsmValue a, AsmValue b)
{
  int ret, inner_used;
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  ret = asm_lock_x64_reg(context, X64_RCX, &inner_used);
  if (ret != DASM_S_OK) { return ret; }

  switch (b.tag) {
    case ASM_TAG_REGISTER:
      //| op2_x_r mov, rcx, b.value.reg
      loc1 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 727, (loc1));
      } else {
      dasm_put(Dst, 733, Dt1(->registers[b.value.reg]));
      }
#line 878 "src/jit/asm.x64.c"
      break;
    case ASM_TAG_IMMEDIATE:
      /*
       * shift operations only use cl as operand, there won't be any
       * overflowing issues.
       */
      //| mov rcx, b.value.i
      dasm_put(Dst, 379, b.value.i);
#line 885 "src/jit/asm.x64.c"
      break;
  }
  ret = asm_mov(context, target, a);
  if (ret != DASM_S_OK) { return ret; }

  //| op2_r_x shl, target, cl
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REG(loc1)) {
  dasm_put(Dst, 738, (loc1));
  } else {
  dasm_put(Dst, 744, Dt1(->registers[target]));
  }
#line 891 "src/jit/asm.x64.c"

  ret = asm_release_x64_reg(context, X64_RCX, inner_used);
  if (ret != DASM_S_OK) { return ret; }

  return DASM_S_OK;
}

int asm_shr(AsmContext* context, uint32_t target, AsmValue a, AsmValue b, int is_signed)
{
  int ret, inner_used;
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  ret = asm_lock_x64_reg(context, X64_RCX, &inner_used);
  if (ret != DASM_S_OK) { return ret; }

  switch (b.tag) {
    case ASM_TAG_REGISTER:
      //| op2_x_r mov, rcx, b.value.reg
      loc1 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 727, (loc1));
      } else {
      dasm_put(Dst, 733, Dt1(->registers[b.value.reg]));
      }
#line 910 "src/jit/asm.x64.c"
      break;
    case ASM_TAG_IMMEDIATE:
      /*
       * shift operations only use cl as operand, there won't be any
       * overflowing issues.
       */
      //| mov rcx, b.value.i
      dasm_put(Dst, 379, b.value.i);
#line 917 "src/jit/asm.x64.c"
      break;
  }
  ret = asm_mov(context, target, a);
  if (ret != DASM_S_OK) { return ret; }

  if (is_signed) {
    //| op2_r_x sar, target, cl
    loc1 = riscv_reg_to_x64_reg(target);
    if (VALID_X64_REG(loc1)) {
    dasm_put(Dst, 749, (loc1));
    } else {
    dasm_put(Dst, 756, Dt1(->registers[target]));
    }
#line 924 "src/jit/asm.x64.c"
  } else {
    //| op2_r_x shr, target, cl
    loc1 = riscv_reg_to_x64_reg(target);
    if (VALID_X64_REG(loc1)) {
    dasm_put(Dst, 761, (loc1));
    } else {
    dasm_put(Dst, 767, Dt1(->registers[target]));
    }
#line 926 "src/jit/asm.x64.c"
  }

  ret = asm_release_x64_reg(context, X64_RCX, inner_used);
  if (ret != DASM_S_OK) { return ret; }

  return DASM_S_OK;
}

int asm_eq(AsmContext* context, uint32_t target, AsmValue a, AsmValue b)
{
  uint32_t loc1;
  int ret, used, rax_used;
  dasm_State** Dst = &context->d;

  ret = asm_lock_x64_reg(context, X64_RCX, &used);
  if (ret != DASM_S_OK) { return ret; }
  ret = asm_mov_to_x64_reg(context, X64_RCX, a);
  if (ret != DASM_S_OK) { return ret; }

  switch (b.tag) {
    case ASM_TAG_REGISTER:
      //| op2_x_r cmp, rcx, b.value.reg
      loc1 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 772, (loc1));
      } else {
      dasm_put(Dst, 778, Dt1(->registers[b.value.reg]));
      }
#line 948 "src/jit/asm.x64.c"
      break;
    case ASM_TAG_IMMEDIATE:
      ret = asm_lock_x64_reg(context, X64_RAX, &rax_used);
      if (ret != DASM_S_OK) { return ret; }
      //| load_imm rax, b.value.i
      if (b.value.i > 0xFFFFFFFF) {
      dasm_put(Dst, 783, b.value.i >> 32, b.value.i & 0xFFFFFFFF);
      } else {
      dasm_put(Dst, 796, b.value.i);
      }
#line 953 "src/jit/asm.x64.c"
      //| cmp rcx, rax
      dasm_put(Dst, 801);
#line 954 "src/jit/asm.x64.c"
      ret = asm_release_x64_reg(context, X64_RAX, rax_used);
      if (ret != DASM_S_OK) { return ret; }
      break;
  }

  //| sete cl
  //| movzx rcx, cl
  //| op2_r_x mov, target, rcx
  dasm_put(Dst, 805);
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REG(loc1)) {
  dasm_put(Dst, 813, (loc1));
  } else {
  dasm_put(Dst, 819, Dt1(->registers[target]));
  }
#line 962 "src/jit/asm.x64.c"

  ret = asm_release_x64_reg(context, X64_RCX, used);
  if (ret != DASM_S_OK) { return ret; }

  return DASM_S_OK;
}

int asm_lt(AsmContext* context, uint32_t target, AsmValue a, AsmValue b, int is_signed)
{
  uint32_t loc1;
  int ret, used, rax_used;
  dasm_State** Dst = &context->d;

  ret = asm_lock_x64_reg(context, X64_RCX, &used);
  if (ret != DASM_S_OK) { return ret; }
  ret = asm_mov_to_x64_reg(context, X64_RCX, a);
  if (ret != DASM_S_OK) { return ret; }

  switch (b.tag) {
    case ASM_TAG_REGISTER:
      //| op2_x_r cmp, rcx, b.value.reg
      loc1 = riscv_reg_to_x64_reg(b.value.reg);
      if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 772, (loc1));
      } else {
      dasm_put(Dst, 778, Dt1(->registers[b.value.reg]));
      }
#line 983 "src/jit/asm.x64.c"
      break;
    case ASM_TAG_IMMEDIATE:
      ret = asm_lock_x64_reg(context, X64_RAX, &rax_used);
      if (ret != DASM_S_OK) { return ret; }
      //| load_imm rax, b.value.i
      if (b.value.i > 0xFFFFFFFF) {
      dasm_put(Dst, 783, b.value.i >> 32, b.value.i & 0xFFFFFFFF);
      } else {
      dasm_put(Dst, 796, b.value.i);
      }
#line 988 "src/jit/asm.x64.c"
      //| cmp rcx, rax
      dasm_put(Dst, 801);
#line 989 "src/jit/asm.x64.c"
      ret = asm_release_x64_reg(context, X64_RAX, rax_used);
      if (ret != DASM_S_OK) { return ret; }
      break;
  }

  if (is_signed) {
    //| setl cl
    dasm_put(Dst, 824);
#line 996 "src/jit/asm.x64.c"
  } else {
    //| setb cl
    dasm_put(Dst, 828);
#line 998 "src/jit/asm.x64.c"
  }
  //| movzx rcx, cl
  //| op2_r_x mov, target, rcx
  dasm_put(Dst, 808);
  loc1 = riscv_reg_to_x64_reg(target);
  if (VALID_X64_REG(loc1)) {
  dasm_put(Dst, 813, (loc1));
  } else {
  dasm_put(Dst, 819, Dt1(->registers[target]));
  }
#line 1001 "src/jit/asm.x64.c"

  ret = asm_release_x64_reg(context, X64_RCX, used);
  if (ret != DASM_S_OK) { return ret; }

  return DASM_S_OK;
}

int asm_cond(AsmContext* context, uint32_t target, AsmValue condition, AsmValue true_value, AsmValue false_value) {
  uint32_t loc1, loc2;
  int ret;
  dasm_State** Dst = &context->d;

  switch (condition.tag) {
    case ASM_TAG_REGISTER:
      //| op2_r_imm cmp, condition.value.reg, (uint64_t)1
      if ((uint64_t)1 > 0xFFFFFFFF) {
        loc1 = riscv_reg_to_x64_reg(condition.value.reg);
        loc2 = (loc1 == X64_RAX) ? (X64_RCX) : (X64_RAX);
        if (TEST_X64_REG_USED(context, loc2)) {
      dasm_put(Dst, 164, (loc2));
        }
      dasm_put(Dst, 169, (loc2), (uint64_t)1 >> 32, (loc2), (loc2), (uint64_t)1 & 0xFFFFFFFF);
        if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 832, (loc2), (loc1));
        } else {
      dasm_put(Dst, 840, (loc2), Dt1(->registers[condition.value.reg]));
        }
        if (TEST_X64_REG_USED(context, loc2)) {
      dasm_put(Dst, 188, (loc2));
        }
      } else {
        loc1 = riscv_reg_to_x64_reg(condition.value.reg);
        if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 848, (loc1), (uint64_t)1);
        } else {
      dasm_put(Dst, 856, Dt1(->registers[condition.value.reg]), (uint64_t)1);
        }
      }
#line 1016 "src/jit/asm.x64.c"
      //| jne >1
      dasm_put(Dst, 492);
#line 1017 "src/jit/asm.x64.c"
      ret = asm_mov(context, target, true_value);
      if (ret != DASM_S_OK) { return ret; }
      //| jmp >2
      //|1:
      dasm_put(Dst, 862);
#line 1021 "src/jit/asm.x64.c"
      ret = asm_mov(context, target, false_value);
      if (ret != DASM_S_OK) { return ret; }
      //|2:
      dasm_put(Dst, 462);
#line 1024 "src/jit/asm.x64.c"
      break;
    case ASM_TAG_IMMEDIATE:
      ret = asm_mov(context, target, (condition.value.i == 1) ? true_value : false_value);
      if (ret != DASM_S_OK) { return ret; }
      break;
  }

  return DASM_S_OK;
}

int asm_extend(AsmContext* context, uint32_t target, AsmValue src, AsmValue bits, int is_signed)
{
  /* TODO: add fast path for certain bit values such as 32 */
  int ret, used;
  AsmValue v;

  /*
   * In the general path, we do sign_extend by shifting left (64 - bits) bits,
   * then shifting right arithmetically (64 - bits) bits again.
   */
  v.tag = ASM_TAG_IMMEDIATE;
  v.value.i = 64;

  ret = asm_lock_temp(context, REGISTER_TEMP_RAX, v, &used);
  if (ret != DASM_S_OK) { return ret; }

  v.tag = ASM_TAG_REGISTER;
  v.value.reg = REGISTER_TEMP_RAX;

  ret = asm_sub(context, REGISTER_TEMP_RAX, v, bits);
  if (ret != DASM_S_OK) { return ret; }

  ret = asm_shl(context, target, src, v);
  if (ret != DASM_S_OK) { return ret; }

  src.tag = ASM_TAG_REGISTER;
  src.value.reg = target;
  ret = asm_shr(context, target, src, v, is_signed);
  if (ret != DASM_S_OK) { return ret; }

  return asm_release_temp(context, REGISTER_TEMP_RAX, used);
}

int asm_push(AsmContext* context, uint32_t reg)
{
  dasm_State** Dst = &context->d;
  //| push Rq(riscv_reg_to_x64_reg(reg))
  dasm_put(Dst, 164, (riscv_reg_to_x64_reg(reg)));
#line 1071 "src/jit/asm.x64.c"
  return DASM_S_OK;
}

int asm_pop(AsmContext* context, uint32_t target)
{
  dasm_State** Dst = &context->d;
  //| pop Rq(riscv_reg_to_x64_reg(target))
  dasm_put(Dst, 188, (riscv_reg_to_x64_reg(target)));
#line 1078 "src/jit/asm.x64.c"
  return DASM_S_OK;
}

int asm_memory_write(AsmContext* context, AsmValue address, AsmValue v, uint32_t size)
{
  int ret, used;
  dasm_State** Dst = &context->d;
  ret = asm_emit_epilogue(context);
  if (ret != DASM_S_OK) { return ret; }
  //| mov rdi, machine->m
  dasm_put(Dst, 869, Dt1(->m));
#line 1088 "src/jit/asm.x64.c"
  switch (address.tag) {
    case ASM_TAG_REGISTER:
      /* After epilogue, all RISC-V registers live in memory now */
      //| mov rsi, machine->registers[address.value.reg]
      dasm_put(Dst, 874, Dt1(->registers[address.value.reg]));
#line 1092 "src/jit/asm.x64.c"
      break;
    case ASM_TAG_IMMEDIATE:
      //| load_imm, rsi, address.value.i
      if (address.value.i > 0xFFFFFFFF) {
      dasm_put(Dst, 879, address.value.i >> 32, address.value.i & 0xFFFFFFFF);
      } else {
      dasm_put(Dst, 892, address.value.i);
      }
#line 1095 "src/jit/asm.x64.c"
      break;
  }
  switch (v.tag) {
    case ASM_TAG_REGISTER:
      /* After epilogue, all RISC-V registers live in memory now */
      //| mov rdx, machine->registers[v.value.reg]
      dasm_put(Dst, 897, Dt1(->registers[v.value.reg]));
#line 1101 "src/jit/asm.x64.c"
      break;
    case ASM_TAG_IMMEDIATE:
      //| load_imm, rdx, v.value.i
      if (v.value.i > 0xFFFFFFFF) {
      dasm_put(Dst, 902, v.value.i >> 32, v.value.i & 0xFFFFFFFF);
      } else {
      dasm_put(Dst, 915, v.value.i);
      }
#line 1104 "src/jit/asm.x64.c"
      break;
  }
  ret = asm_lock_x64_reg(context, X64_RAX, &used);
  if (ret != DASM_S_OK) { return ret; }
  /* Align rsp on a 16 byte boundary first, inspired from https://stackoverflow.com/a/9600102 */
  //| push rsp
  //| push qword [rsp]
  //| and rsp, -0x10
  dasm_put(Dst, 920);
#line 1112 "src/jit/asm.x64.c"
  switch (size) {
    case 1:
      //| mov64 rax, (ptrdiff_t)ckb_vm_jit_ffi_store8
      dasm_put(Dst, 465, (unsigned int)((ptrdiff_t)ckb_vm_jit_ffi_store8), (unsigned int)(((ptrdiff_t)ckb_vm_jit_ffi_store8)>>32));
#line 1115 "src/jit/asm.x64.c"
      break;
    case 2:
      //| mov64 rax, (ptrdiff_t)ckb_vm_jit_ffi_store16
      dasm_put(Dst, 465, (unsigned int)((ptrdiff_t)ckb_vm_jit_ffi_store16), (unsigned int)(((ptrdiff_t)ckb_vm_jit_ffi_store16)>>32));
#line 1118 "src/jit/asm.x64.c"
      break;
    case 4:
      //| mov64 rax, (ptrdiff_t)ckb_vm_jit_ffi_store32
      dasm_put(Dst, 465, (unsigned int)((ptrdiff_t)ckb_vm_jit_ffi_store32), (unsigned int)(((ptrdiff_t)ckb_vm_jit_ffi_store32)>>32));
#line 1121 "src/jit/asm.x64.c"
      break;
    case 8:
      //| mov64 rax, (ptrdiff_t)ckb_vm_jit_ffi_store64
      dasm_put(Dst, 465, (unsigned int)((ptrdiff_t)ckb_vm_jit_ffi_store64), (unsigned int)(((ptrdiff_t)ckb_vm_jit_ffi_store64)>>32));
#line 1124 "src/jit/asm.x64.c"
      break;
    default:
      return ERROR_INVALID_SIZE;
  }
  //| call rax
  //| mov rsp, qword [rsp+8]
  dasm_put(Dst, 931);
#line 1130 "src/jit/asm.x64.c"
  ret = asm_release_x64_reg(context, X64_RAX, used);
  if (ret != DASM_S_OK) { return ret; }
  return asm_emit_prologue(context);
}

int asm_memory_read(AsmContext* context, uint32_t target, AsmValue address, uint32_t size)
{
  int ret, used;
  dasm_State** Dst = &context->d;
  ret = asm_emit_epilogue(context);
  if (ret != DASM_S_OK) { return ret; }
  //| mov rdi, machine->m
  dasm_put(Dst, 869, Dt1(->m));
#line 1142 "src/jit/asm.x64.c"
  switch (address.tag) {
    case ASM_TAG_REGISTER:
      /* After epilogue, all RISC-V registers live in memory now */
      //| mov rsi, machine->registers[address.value.reg]
      dasm_put(Dst, 874, Dt1(->registers[address.value.reg]));
#line 1146 "src/jit/asm.x64.c"
      break;
    case ASM_TAG_IMMEDIATE:
      //| load_imm, rsi, address.value.i
      if (address.value.i > 0xFFFFFFFF) {
      dasm_put(Dst, 879, address.value.i >> 32, address.value.i & 0xFFFFFFFF);
      } else {
      dasm_put(Dst, 892, address.value.i);
      }
#line 1149 "src/jit/asm.x64.c"
      break;
  }
  //| lea rdx, machine->registers[target]
  dasm_put(Dst, 940, Dt1(->registers[target]));
#line 1152 "src/jit/asm.x64.c"
  ret = asm_lock_x64_reg(context, X64_RAX, &used);
  if (ret != DASM_S_OK) { return ret; }
  /* Align rsp on a 16 byte boundary first, inspired from https://stackoverflow.com/a/9600102 */
  //| push rsp
  //| push qword [rsp]
  //| and rsp, -0x10
  dasm_put(Dst, 920);
#line 1158 "src/jit/asm.x64.c"
  switch (size) {
    case 1:
      //| mov64 rax, (ptrdiff_t)ckb_vm_jit_ffi_load8
      dasm_put(Dst, 465, (unsigned int)((ptrdiff_t)ckb_vm_jit_ffi_load8), (unsigned int)(((ptrdiff_t)ckb_vm_jit_ffi_load8)>>32));
#line 1161 "src/jit/asm.x64.c"
      break;
    case 2:
      //| mov64 rax, (ptrdiff_t)ckb_vm_jit_ffi_load16
      dasm_put(Dst, 465, (unsigned int)((ptrdiff_t)ckb_vm_jit_ffi_load16), (unsigned int)(((ptrdiff_t)ckb_vm_jit_ffi_load16)>>32));
#line 1164 "src/jit/asm.x64.c"
      break;
    case 4:
      //| mov64 rax, (ptrdiff_t)ckb_vm_jit_ffi_load32
      dasm_put(Dst, 465, (unsigned int)((ptrdiff_t)ckb_vm_jit_ffi_load32), (unsigned int)(((ptrdiff_t)ckb_vm_jit_ffi_load32)>>32));
#line 1167 "src/jit/asm.x64.c"
      break;
    case 8:
      //| mov64 rax, (ptrdiff_t)ckb_vm_jit_ffi_load64
      dasm_put(Dst, 465, (unsigned int)((ptrdiff_t)ckb_vm_jit_ffi_load64), (unsigned int)(((ptrdiff_t)ckb_vm_jit_ffi_load64)>>32));
#line 1170 "src/jit/asm.x64.c"
      break;
    default:
      return ERROR_INVALID_SIZE;
  }
  //| call rax
  //| mov rsp, qword [rsp+8]
  dasm_put(Dst, 931);
#line 1176 "src/jit/asm.x64.c"
  ret = asm_release_x64_reg(context, X64_RAX, used);
  if (ret != DASM_S_OK) { return ret; }
  return asm_emit_prologue(context);
}


static int asm_lock_temp(AsmContext* context, uint32_t temp_reg, AsmValue v, int *used)
{
  dasm_State** Dst = &context->d;
  uint32_t loc1;

  loc1 = riscv_reg_to_x64_reg(temp_reg);
  if (!VALID_X64_REG(loc1)) {
    return ERROR_INVALID_ARGUMENT;
  }

  *used = TEST_X64_REG_USED(context, loc1);
  if (*used) {
    //| push Rq(loc1)
    dasm_put(Dst, 164, (loc1));
#line 1195 "src/jit/asm.x64.c"
  } else {
    MARK_X64_REG_USED(context, loc1);
  }
  return asm_mov(context, temp_reg, v);
}

static int asm_release_temp(AsmContext* context, uint32_t temp_reg, int used)
{
  dasm_State** Dst = &context->d;
  uint32_t loc1;

  loc1 = riscv_reg_to_x64_reg(temp_reg);
  if (!VALID_X64_REG(loc1)) {
    return ERROR_INVALID_ARGUMENT;
  }

  if (used) {
    //| pop Rq(loc1)
    dasm_put(Dst, 188, (loc1));
#line 1213 "src/jit/asm.x64.c"
  } else {
    UNMARK_X64_REG_USED(context, loc1);
  }
  return DASM_S_OK;
}

static int asm_lock_x64_reg(AsmContext* context, int32_t x64_reg, int* used)
{
  dasm_State** Dst = &context->d;

  *used = TEST_X64_REG_USED(context, x64_reg);
  if (*used) {
    //| push Rq(x64_reg)
    dasm_put(Dst, 164, (x64_reg));
#line 1226 "src/jit/asm.x64.c"
  } else {
    MARK_X64_REG_USED(context, x64_reg);
  }

  return DASM_S_OK;
}

static int asm_release_x64_reg(AsmContext* context, int32_t x64_reg, int used)
{
  dasm_State** Dst = &context->d;

  if (used) {
    //| pop Rq(x64_reg)
    dasm_put(Dst, 188, (x64_reg));
#line 1239 "src/jit/asm.x64.c"
   } else {
    UNMARK_X64_REG_USED(context, x64_reg);
  }

  return DASM_S_OK;
}

static int asm_mov_to_x64_reg(AsmContext* context, int32_t x64_reg, AsmValue value)
{
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  switch (value.tag) {
    case ASM_TAG_REGISTER:
      //| op2_x_r mov, Rq(x64_reg), value.value.reg
      loc1 = riscv_reg_to_x64_reg(value.value.reg);
      if (VALID_X64_REG(loc1)) {
      dasm_put(Dst, 127, (loc1), (x64_reg));
      } else {
      dasm_put(Dst, 135, (x64_reg), Dt1(->registers[value.value.reg]));
      }
#line 1254 "src/jit/asm.x64.c"
      break;
    case ASM_TAG_IMMEDIATE:
      //| load_imm, Rq(x64_reg), value.value.i
      if (value.value.i > 0xFFFFFFFF) {
      dasm_put(Dst, 169, (x64_reg), value.value.i >> 32, (x64_reg), (x64_reg), value.value.i & 0xFFFFFFFF);
      } else {
      dasm_put(Dst, 193, (x64_reg), value.value.i);
      }
#line 1257 "src/jit/asm.x64.c"
      break;
  }

  return DASM_S_OK;
}
