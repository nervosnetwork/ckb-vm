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

|.arch x64
|.section code
|.globals lbl_
|.actionlist bf_actions

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

|.type machine, AsmMachine, rbp

|.macro load_imm64, x64_reg, imm64
| mov x64_reg, imm64 >> 32
| shl x64_reg, 32
| or x64_reg, imm64 & 0xFFFFFFFF
|.endmacro

|.macro load_imm, x64_reg, imm
||if (imm > 0xFFFFFFFF) {
|   load_imm64 x64_reg, imm
||} else {
|   mov x64_reg, imm
||}
|.endmacro

/* r_r means both operands here are RISC-V registers */
|.macro op2_r_r, op, target, source
||loc1 = riscv_reg_to_x64_reg(target);
||loc2 = riscv_reg_to_x64_reg(source);
||if (VALID_X64_REG(loc1) && VALID_X64_REG(loc2)) {
|  op Rq(loc1), Rq(loc2)
||} else if (VALID_X64_REG(loc1)) {
|  op Rq(loc1), machine->registers[source]
||} else if (VALID_X64_REG(loc2)) {
|  op machine->registers[target], Rq(loc2)
||} else {
/* If target is RAX, we won't be in this branch */
|| if (TEST_X64_REG_USED(context, X64_RAX)) {
|    push rax
|| }
|  mov rax, qword machine->registers[source]
|  op qword machine->registers[target], rax
|| if (TEST_X64_REG_USED(context, X64_RAX)) {
|    pop rax
|| }
||}
|.endmacro

|.macro op1_r, op, reg
||loc1 = riscv_reg_to_x64_reg(reg);
||if (VALID_X64_REG(loc1)) {
|  op Rq(loc1)
||} else {
|  op qword machine->registers[reg]
||}
|.endmacro

/* r_x means that the first operand is RISC-V register, the second is X86 one */
|.macro op2_r_x, op, target, x64_source
||loc1 = riscv_reg_to_x64_reg(target);
||if (VALID_X64_REG(loc1)) {
|  op Rq(loc1), x64_source
||} else {
|  op qword machine->registers[target], x64_source
||}
|.endmacro

|.macro op2_r_imm, op, target, imm
||if (imm > 0xFFFFFFFF) {
||  loc1 = riscv_reg_to_x64_reg(target);
||  loc2 = (loc1 == X64_RAX) ? (X64_RCX) : (X64_RAX);
||  if (TEST_X64_REG_USED(context, loc2)) {
|     push Rq(loc2)
||  }
|   load_imm64 Rq(loc2), imm
||  if (VALID_X64_REG(loc1)) {
|     op Rq(loc1), Rq(loc2)
||  } else {
|     op qword machine->registers[target], Rq(loc2)
||  }
||  if (TEST_X64_REG_USED(context, loc2)) {
|     pop Rq(loc2)
||  }
||} else {
||  loc1 = riscv_reg_to_x64_reg(target);
||  if (VALID_X64_REG(loc1)) {
|     op Rq(loc1), imm
||  } else {
|     op qword machine->registers[target], imm
||  }
||}
|.endmacro

|.macro op2_x_r, op, x64_target, source
||loc1 = riscv_reg_to_x64_reg(source);
||if (VALID_X64_REG(loc1)) {
|  op x64_target, Rq(loc1)
||} else {
|  op x64_target, machine->registers[source]
||}
|.endmacro

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
  |.code
  | push r12
  | push r13
  | push r14
  | push r15
  | push rbx
  | push rbp
  | mov rbp, rdi
  return DASM_S_OK;
}

int asm_emit_prologue(AsmContext* context)
{
  dasm_State** Dst = &context->d;
  | mov rdx, machine->registers[REGISTER_RA]
  | mov rbx, machine->registers[REGISTER_SP]
  | mov rsi, machine->registers[REGISTER_T0]
  | mov rdi, machine->registers[REGISTER_T1]
  | mov r8, machine->registers[REGISTER_A0]
  | mov r9, machine->registers[REGISTER_A1]
  | mov r10, machine->registers[REGISTER_A2]
  | mov r11, machine->registers[REGISTER_A3]
  | mov r12, machine->registers[REGISTER_A4]
  | mov r13, machine->registers[REGISTER_A5]
  | mov r14, machine->registers[REGISTER_A6]
  | mov r15, machine->registers[REGISTER_A7]
  return DASM_S_OK;
}

int asm_emit_epilogue(AsmContext* context)
{
  dasm_State** Dst = &context->d;
  | mov machine->registers[REGISTER_RA], rdx
  | mov machine->registers[REGISTER_SP], rbx
  | mov machine->registers[REGISTER_T0], rsi
  | mov machine->registers[REGISTER_T1], rdi
  | mov machine->registers[REGISTER_A0], r8
  | mov machine->registers[REGISTER_A1], r9
  | mov machine->registers[REGISTER_A2], r10
  | mov machine->registers[REGISTER_A3], r11
  | mov machine->registers[REGISTER_A4], r12
  | mov machine->registers[REGISTER_A5], r13
  | mov machine->registers[REGISTER_A6], r14
  | mov machine->registers[REGISTER_A7], r15
  return DASM_S_OK;
}

int asm_link(AsmContext* context, size_t *szp)
{
  dasm_State** Dst = &context->d;
  | pop rbp
  | pop rbx
  | pop r15
  | pop r14
  | pop r13
  | pop r12
  | ret
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
      | op2_r_r mov, target, value.value.reg
      break;
    case ASM_TAG_IMMEDIATE:
      | op2_r_imm mov, target, value.value.i
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
      | op2_r_r add, target, b.value.reg
      break;
    case ASM_TAG_IMMEDIATE:
      | op2_r_imm add, target, b.value.i
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
      | op2_r_r sub, target, b.value.reg
      break;
    case ASM_TAG_IMMEDIATE:
      | op2_r_imm sub, target, b.value.i
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
      | op2_x_r imul, rax, b.value.reg
      break;
    case ASM_TAG_IMMEDIATE:
      ret = asm_lock_x64_reg(context, X64_RCX, &rcx_used);
      if (ret != DASM_S_OK) { return ret; }
      ret = asm_mov_to_x64_reg(context, X64_RCX, a);
      if (ret != DASM_S_OK) { return ret; }
      | load_imm64 rcx, b.value.i
      | imul rax, rcx
      ret = asm_release_x64_reg(context, X64_RCX, rcx_used);
      if (ret != DASM_S_OK) { return ret; }
      break;
  }

  | op2_r_x mov, target, rax

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
  | mov rcx, rdx
  switch (b.tag) {
    case ASM_TAG_REGISTER:
      if (is_signed) {
        | op1_r imul, b.value.reg
      } else {
        | op1_r mul, b.value.reg
      }
      break;
    case ASM_TAG_IMMEDIATE:
      | push rcx
      | load_imm rcx, b.value.i
      if (is_signed) {
        | imul rcx
      } else {
        | mul rcx
      }
      | pop rcx
      break;
  }
  | mov rax, rdx
  | mov rdx, rcx
  | op2_r_x mov, target, rax

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

  | test rax, rax
  | jns >1
  /* calculate res = mulhu(-a, b), res is stored in rdx after this. */
  | neg rax
  | push rdx
  switch (b.tag) {
    case ASM_TAG_REGISTER:
      | op1_r mul, b.value.reg
      break;
    case ASM_TAG_IMMEDIATE:
      | load_imm rcx, b.value.i
      | mul rcx
      break;
  }
  /* calculate ~res and store it in rcx */
  | xor rdx, -1
  | mov rcx, rdx
  | pop rdx
  /*
   * calculate (a * b), then test (a * b == 0) and convert that to 1 or 0,
   * result is stored in rax after this.
   */
  ret = asm_mov_to_x64_reg(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }
  switch (b.tag) {
    case ASM_TAG_REGISTER:
      | op2_x_r imul, rax, b.value.reg
      break;
    case ASM_TAG_IMMEDIATE:
      | push rcx
      | load_imm rcx, b.value.i
      | imul rax, rcx
      | pop rcx
      break;
  }
  | test rax, rax
  | setz al
  | movzx rax, al
  /* calculate ~res + (a * b == 0) */
  | add rax, rcx
  | jmp >2
  /* just mulhu here */
  |1:
  | push rdx
  switch (b.tag) {
    case ASM_TAG_REGISTER:
      | op1_r mul, b.value.reg
      break;
    case ASM_TAG_IMMEDIATE:
      | load_imm rcx, b.value.i
      | mul rcx
      break;
  }
  /* calculate ~res and store it in rcx */
  | mov rax, rdx
  | pop rdx
  |2:
  | op2_r_x mov, target, rax

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
    | mov64 rax, INT64_MIN
    ret = asm_mov_to_x64_reg(context, X64_RCX, a);
    if (ret != DASM_S_OK) { return ret; }
    | cmp rax, rcx
    | jne >1
    | mov rax, -1
    ret = asm_mov_to_x64_reg(context, X64_RCX, b);
    if (ret != DASM_S_OK) { return ret; }
    | cmp rax, rcx
    | jne >1
    ret = asm_mov(context, target, a);
    if (ret != DASM_S_OK) { return ret; }
    | jmp >3
  }
  |1:
  | mov rax, 0
  ret = asm_mov_to_x64_reg(context, X64_RCX, b);
  if (ret != DASM_S_OK) { return ret; }
  | cmp rax, rcx
  | jne >2
  t.tag = ASM_TAG_IMMEDIATE;
  t.value.i = UINT64_MAX;
  ret = asm_mov(context, target, t);
  if (ret != DASM_S_OK) { return ret; }
  | jmp >3
  |2:
  ret = asm_mov_to_x64_reg(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }
  | mov rcx, rdx
  switch (b.tag) {
    case ASM_TAG_REGISTER:
      if (is_signed) {
        | cqo
        | op1_r idiv, b.value.reg
      } else {
        | xor rdx, rdx
        | op1_r div, b.value.reg
      }
      break;
    case ASM_TAG_IMMEDIATE:
      | push rcx
      | load_imm, rcx, b.value.i
      if (is_signed) {
        | cqo
        | idiv rcx
      } else {
        | xor rdx, rdx
        | div rcx
      }
      | pop rcx
      break;
  }
  | mov rdx, rcx
  | op2_r_x mov, target, rax
  |3:

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
    | mov64 rax, INT64_MIN
    ret = asm_mov_to_x64_reg(context, X64_RCX, a);
    if (ret != DASM_S_OK) { return ret; }
    | cmp rax, rcx
    | jne >1
    | mov rax, -1
    ret = asm_mov_to_x64_reg(context, X64_RCX, b);
    if (ret != DASM_S_OK) { return ret; }
    | cmp rax, rcx
    | jne >1
    t.tag = ASM_TAG_IMMEDIATE;
    t.value.i = 0;
    ret = asm_mov(context, target, t);
    if (ret != DASM_S_OK) { return ret; }
    | jmp >3
  }
  |1:
  | mov rax, 0
  ret = asm_mov_to_x64_reg(context, X64_RCX, b);
  if (ret != DASM_S_OK) { return ret; }
  | cmp rax, rcx
  | jne >2
  ret = asm_mov(context, target, a);
  if (ret != DASM_S_OK) { return ret; }
  | jmp >3
  |2:
  ret = asm_mov_to_x64_reg(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }
  | mov rcx, rdx
  switch (b.tag) {
    case ASM_TAG_REGISTER:
      if (is_signed) {
        | cqo
        | op1_r idiv, b.value.reg
      } else {
        | xor rdx, rdx
        | op1_r div, b.value.reg
      }
      break;
    case ASM_TAG_IMMEDIATE:
      | push rcx
      | load_imm, rcx, b.value.i
      if (is_signed) {
        | cqo
        | idiv rcx
      } else {
        | xor rdx, rdx
        | div rcx
      }
      | pop rcx
      break;
  }
  | mov rax, rdx
  | mov rdx, rcx
  | op2_r_x mov, target, rax
  |3:

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
      | op2_r_r and, target, b.value.reg
      break;
    case ASM_TAG_IMMEDIATE:
      | op2_r_imm and, target, b.value.i
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
      | op2_r_r or, target, b.value.reg
      break;
    case ASM_TAG_IMMEDIATE:
      | op2_r_imm or, target, b.value.i
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

  | op1_r not, target
  if (logical) {
    | op2_r_imm and, target, (uint64_t)1
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
      | op2_r_r xor, target, b.value.reg
      break;
    case ASM_TAG_IMMEDIATE:
      | op2_r_imm xor, target, b.value.i
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
      | op2_x_r mov, rcx, b.value.reg
      break;
    case ASM_TAG_IMMEDIATE:
      /*
       * shift operations only use cl as operand, there won't be any
       * overflowing issues.
       */
      | mov rcx, b.value.i
      break;
  }
  ret = asm_mov(context, target, a);
  if (ret != DASM_S_OK) { return ret; }

  | op2_r_x shl, target, cl

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
      | op2_x_r mov, rcx, b.value.reg
      break;
    case ASM_TAG_IMMEDIATE:
      /*
       * shift operations only use cl as operand, there won't be any
       * overflowing issues.
       */
      | mov rcx, b.value.i
      break;
  }
  ret = asm_mov(context, target, a);
  if (ret != DASM_S_OK) { return ret; }

  if (is_signed) {
    | op2_r_x sar, target, cl
  } else {
    | op2_r_x shr, target, cl
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
      | op2_x_r cmp, rcx, b.value.reg
      break;
    case ASM_TAG_IMMEDIATE:
      ret = asm_lock_x64_reg(context, X64_RAX, &rax_used);
      if (ret != DASM_S_OK) { return ret; }
      | load_imm rax, b.value.i
      | cmp rcx, rax
      ret = asm_release_x64_reg(context, X64_RAX, rax_used);
      if (ret != DASM_S_OK) { return ret; }
      break;
  }

  | sete cl
  | movzx rcx, cl
  | op2_r_x mov, target, rcx

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
      | op2_x_r cmp, rcx, b.value.reg
      break;
    case ASM_TAG_IMMEDIATE:
      ret = asm_lock_x64_reg(context, X64_RAX, &rax_used);
      if (ret != DASM_S_OK) { return ret; }
      | load_imm rax, b.value.i
      | cmp rcx, rax
      ret = asm_release_x64_reg(context, X64_RAX, rax_used);
      if (ret != DASM_S_OK) { return ret; }
      break;
  }

  if (is_signed) {
    | setl cl
  } else {
    | setb cl
  }
  | movzx rcx, cl
  | op2_r_x mov, target, rcx

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
      | op2_r_imm cmp, condition.value.reg, (uint64_t)1
      | jne >1
      ret = asm_mov(context, target, true_value);
      if (ret != DASM_S_OK) { return ret; }
      | jmp >2
      |1:
      ret = asm_mov(context, target, false_value);
      if (ret != DASM_S_OK) { return ret; }
      |2:
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
  | push Rq(riscv_reg_to_x64_reg(reg))
  return DASM_S_OK;
}

int asm_pop(AsmContext* context, uint32_t target)
{
  dasm_State** Dst = &context->d;
  | pop Rq(riscv_reg_to_x64_reg(target))
  return DASM_S_OK;
}

int asm_memory_write(AsmContext* context, AsmValue address, AsmValue v, uint32_t size)
{
  int ret, used;
  dasm_State** Dst = &context->d;
  ret = asm_emit_epilogue(context);
  if (ret != DASM_S_OK) { return ret; }
  | mov rdi, machine->m
  switch (address.tag) {
    case ASM_TAG_REGISTER:
      /* After epilogue, all RISC-V registers live in memory now */
      | mov rsi, machine->registers[address.value.reg]
      break;
    case ASM_TAG_IMMEDIATE:
      | load_imm, rsi, address.value.i
      break;
  }
  switch (v.tag) {
    case ASM_TAG_REGISTER:
      /* After epilogue, all RISC-V registers live in memory now */
      | mov rdx, machine->registers[v.value.reg]
      break;
    case ASM_TAG_IMMEDIATE:
      | load_imm, rdx, v.value.i
      break;
  }
  ret = asm_lock_x64_reg(context, X64_RAX, &used);
  if (ret != DASM_S_OK) { return ret; }
  /* Align rsp on a 16 byte boundary first, inspired from https://stackoverflow.com/a/9600102 */
  | push rsp
  | push qword [rsp]
  | and rsp, -0x10
  switch (size) {
    case 1:
      | mov64 rax, (ptrdiff_t)ckb_vm_jit_ffi_store8
      break;
    case 2:
      | mov64 rax, (ptrdiff_t)ckb_vm_jit_ffi_store16
      break;
    case 4:
      | mov64 rax, (ptrdiff_t)ckb_vm_jit_ffi_store32
      break;
    case 8:
      | mov64 rax, (ptrdiff_t)ckb_vm_jit_ffi_store64
      break;
    default:
      return ERROR_INVALID_SIZE;
  }
  | call rax
  | mov rsp, qword [rsp+8]
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
  | mov rdi, machine->m
  switch (address.tag) {
    case ASM_TAG_REGISTER:
      /* After epilogue, all RISC-V registers live in memory now */
      | mov rsi, machine->registers[address.value.reg]
      break;
    case ASM_TAG_IMMEDIATE:
      | load_imm, rsi, address.value.i
      break;
  }
  | lea rdx, machine->registers[target]
  ret = asm_lock_x64_reg(context, X64_RAX, &used);
  if (ret != DASM_S_OK) { return ret; }
  /* Align rsp on a 16 byte boundary first, inspired from https://stackoverflow.com/a/9600102 */
  | push rsp
  | push qword [rsp]
  | and rsp, -0x10
  switch (size) {
    case 1:
      | mov64 rax, (ptrdiff_t)ckb_vm_jit_ffi_load8
      break;
    case 2:
      | mov64 rax, (ptrdiff_t)ckb_vm_jit_ffi_load16
      break;
    case 4:
      | mov64 rax, (ptrdiff_t)ckb_vm_jit_ffi_load32
      break;
    case 8:
      | mov64 rax, (ptrdiff_t)ckb_vm_jit_ffi_load64
      break;
    default:
      return ERROR_INVALID_SIZE;
  }
  | call rax
  | mov rsp, qword [rsp+8]
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
    | push Rq(loc1)
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
    | pop Rq(loc1)
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
    | push Rq(x64_reg)
  } else {
    MARK_X64_REG_USED(context, x64_reg);
  }

  return DASM_S_OK;
}

static int asm_release_x64_reg(AsmContext* context, int32_t x64_reg, int used)
{
  dasm_State** Dst = &context->d;

  if (used) {
    | pop Rq(x64_reg)
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
      | op2_x_r mov, Rq(x64_reg), value.value.reg
      break;
    case ASM_TAG_IMMEDIATE:
      | load_imm, Rq(x64_reg), value.value.i
      break;
  }

  return DASM_S_OK;
}
