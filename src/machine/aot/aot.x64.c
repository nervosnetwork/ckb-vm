#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "dasm_proto.h"
#include "dasm_x86.h"

#include "cdefinitions_generated.h"

||#if (defined(_WIN32) != WIN)
#error "Wrong DynASM flags used: pass -D WIN to dynasm.lua to generate windows specific file"
#endif

#define ERROR_INVALID_MEMORY_SIZE 0xFFFFFF00
#define ERROR_NOT_ENOUGH_LABELS 0xFFFFFF01
#define ERROR_INVALID_VALUE 0xFFFFFF02

|.arch x64
|.section code
|.globals lbl_
|.actionlist bf_actions

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

|.type machine, AsmMachine, rdi

|.macro load_imm64, x64_reg, imm64
| mov64 x64_reg, imm64
|.endmacro

/* We can leverage sign extension to save bits when handling negative integers */
|.macro load_imm, x64_reg, imm
||if ((imm >> (context->version >= 1? 31: 32)) > 0 && ((imm & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
|   load_imm64 x64_reg, imm
||} else {
|   mov x64_reg, imm
||}
|.endmacro

/* r_r means both operands here are RISC-V registers */
|.macro op2_r_r, op, target, source, x64_temp_reg
||loc1 = riscv_reg_to_x64_reg(target);
||loc2 = riscv_reg_to_x64_reg(source);
||if (VALID_X64_REGISTER(loc1) && VALID_X64_REGISTER(loc2)) {
|  op Rq(loc1), Rq(loc2)
||} else if (VALID_X64_REGISTER(loc1)) {
|  op Rq(loc1), machine->registers[source]
||} else if (VALID_X64_REGISTER(loc2)) {
|  op machine->registers[target], Rq(loc2)
||} else {
|  mov x64_temp_reg, qword machine->registers[source]
|  op qword machine->registers[target], x64_temp_reg
||}
|.endmacro

|.macro op1_r, op, reg
||loc1 = riscv_reg_to_x64_reg(reg);
||if (VALID_X64_REGISTER(loc1)) {
|  op Rq(loc1)
||} else {
|  op qword machine->registers[reg]
||}
|.endmacro

/* r_x means that the first operand is RISC-V register, the second is X86 one */
|.macro op2_r_x, op, target, x64_source
||loc1 = riscv_reg_to_x64_reg(target);
||if (VALID_X64_REGISTER(loc1)) {
|  op Rq(loc1), x64_source
||} else {
|  op qword machine->registers[target], x64_source
||}
|.endmacro

/*
 * In version 0, imm 0x80000000 will be wrongly treated as 0xffffffff80000000.
 * It is feasible to directly compare imm with 0x7FFFFFFF or 0xFFFFFFFF, but
 * this will trigger a gcc warning when imm is 0 or UINT64_MAX: comparison is
 * always true due to limited range of data type.
 */
|.macro op2_r_imm, op, target, imm, x64_temp_reg
||if ((imm >> (context->version >= 1? 31: 32)) > 0 && ((imm & 0xFFFFFFFF80000000) != 0xFFFFFFFF80000000)) {
||  loc1 = riscv_reg_to_x64_reg(target);
|   load_imm64 x64_temp_reg, imm
||  if (VALID_X64_REGISTER(loc1)) {
|     op Rq(loc1), x64_temp_reg
||  } else {
|     op qword machine->registers[target], x64_temp_reg
||  }
||} else {
||  loc1 = riscv_reg_to_x64_reg(target);
||  if (VALID_X64_REGISTER(loc1)) {
|     op Rq(loc1), imm
||  } else {
|     op qword machine->registers[target], imm
||  }
||}
|.endmacro

|.macro op2_x_r, op, x64_target, source
||loc1 = riscv_reg_to_x64_reg(source);
||if (VALID_X64_REGISTER(loc1)) {
|  op x64_target, Rq(loc1)
||} else {
|  op x64_target, machine->registers[source]
||}
|.endmacro

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

  |.if WIN
    |.define rArg1, rcx
    |.define rArg2, rdx
  |.else
    |.define rArg1, rdi
    |.define rArg2, rsi
  |.endif
  |.macro prepcall
    | push rdi
    | push rsi
    | push rax
    | push rcx
    | push rdx
    | push r8
    | push r9
    | push r10
    | push rbp
    | mov rbp, rsp
    | and rsp, -16
    |.if WIN
      | sub rsp, 32
    |.endif
  |.endmacro
  |.macro postcall
    |.if WIN
      | add rsp, 32
    |.endif
    | mov rsp, rbp
    | pop rbp
    | pop r10
    | pop r9
    | pop r8
    | pop rdx
    | pop rcx
    | pop rax
    | pop rsi
    | pop rdi
  |.endmacro
  |.macro call_inited_memory
    | prepcall
    | mov rArg2, machine
    | mov rArg1, rcx
    | mov64 rax, (uint64_t)inited_memory
    | call rax
    | postcall
  |.endmacro

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
  |.code
  | push r12
  | push r13
  | push r14
  | push r15
  | push rbx
  | push rbp
  |.if WIN
    | push rdi
    | push rsi
    | mov rdi, rcx
    | mov rax, rdx
  |.else
    | mov rax, rsi
  |.endif
  | mov rsi, machine->registers[REGISTER_RA]
  | mov r8, machine->registers[REGISTER_SP]
  | mov r9, machine->registers[REGISTER_A0]
  | jmp rax
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
  |->check_write:
  | push rsi
  | push r8
  | mov rsi, rdx
  | mov rcx, rax
  | shr rcx, CKB_VM_ASM_RISCV_PAGE_SHIFTS
  /*
   * Test if the page stored in rcx is out of bound, and if the page has
   * correct write permissions
   */
  | cmp rcx, CKB_VM_ASM_RISCV_PAGES
  | jae >3
  | lea rdx, machine->flags
  | movzx edx, byte [rdx+rcx]
  | and edx, CKB_VM_ASM_MEMORY_FLAG_WXORX_BIT
  | cmp edx, CKB_VM_ASM_MEMORY_FLAG_WRITABLE
  | jne >4
  /*
   * Set the page as dirty
   */
  | lea rdx, machine->flags
  | movzx r8d, byte [rdx+rcx]
  | or r8d, CKB_VM_ASM_MEMORY_FLAG_DIRTY
  | mov byte [rdx+rcx], r8b
  /*
   * If the frame not initialized, then initialize it.
   */
  | shr rcx, CKB_VM_ASM_MEMORY_FRAME_PAGE_SHIFTS
  | lea rdx, machine->frames
  | movzx r8d, byte [rdx+rcx]
  | cmp r8d, 0
  | jne >1
  | mov byte [rdx+rcx], 1
  | call_inited_memory
  |1:
  /* Check if the write spans to a second memory page */
  | mov rdx, rax
  | add rdx, rsi
  | sub rdx, 1
  | shr rdx, CKB_VM_ASM_RISCV_PAGE_SHIFTS
  | add rcx, 1
  | cmp rcx, rdx
  | jne >2
  /*
   * Test if the page stored in rcx is out of bound, and if the page has
   * correct write permissions
   */
  | cmp rcx, CKB_VM_ASM_RISCV_PAGES
  | jae >3
  | lea rdx, machine->flags
  | movzx edx, byte [rdx+rcx]
  | and edx, CKB_VM_ASM_MEMORY_FLAG_WXORX_BIT
  | cmp edx, CKB_VM_ASM_MEMORY_FLAG_WRITABLE
  | jne >4
  /*
   * Set the page as dirty
   */
  | lea rdx, machine->flags
  | movzx r8d, byte [rdx+rcx]
  | or r8d, CKB_VM_ASM_MEMORY_FLAG_DIRTY
  | mov byte [rdx+rcx], r8b
  | shr rcx, CKB_VM_ASM_MEMORY_FRAME_PAGE_SHIFTS
  | lea rdx, machine->frames
  | movzx r8d, byte [rdx+rcx]
  | cmp r8d, 0
  | jne >2
  | mov byte [rdx+rcx], 1
  | call_inited_memory
  |2:
  | mov rdx, 0
  | pop r8
  | pop rsi
  | ret
  |3:
  | mov rdx, CKB_VM_ASM_RET_OUT_OF_BOUND
  | pop r8
  | pop rsi
  | ret
  |4:
  | mov rdx, CKB_VM_ASM_RET_INVALID_PERMISSION
  | pop r8
  | pop rsi
  | ret
  /*
   * Zeroed frame by memory address and length if it's necessary.
   *
   * rax: the memory address to read/write
   * rdx: length of memory to read/write
   */
  |->check_read:
  | push rsi
  | push r8
  | mov rcx, rax
  | shr rcx, CKB_VM_ASM_MEMORY_FRAME_SHIFTS
  | cmp rcx, CKB_VM_ASM_MEMORY_FRAMES
  | jae >3
  | lea rsi, machine->frames
  | movzx r8d, byte [rsi+rcx]
  | cmp r8d, 0
  | jne >1
  | mov byte [rsi+rcx], 1
  | call_inited_memory
  |1:
  | mov rcx, rax
  | add rcx, rdx
  | sub rcx, 1
  | shr rcx, CKB_VM_ASM_MEMORY_FRAME_SHIFTS
  | cmp rcx, CKB_VM_ASM_MEMORY_FRAMES
  | jae >3
  | movzx r8d, byte [rsi+rcx]
  | cmp r8d, 0
  | jne >2
  | mov byte [rsi+rcx], 1
  | call_inited_memory
  | jmp >2
  |2:
  | mov rdx, 0
  | pop r8
  | pop rsi
  | ret
  |3:
  | mov rdx, CKB_VM_ASM_RET_OUT_OF_BOUND
  | pop r8
  | pop rsi
  | ret
  /* rax should store the return value here */
  |->exit:
  | mov machine->registers[REGISTER_RA], rsi
  | mov machine->registers[REGISTER_SP], r8
  | mov machine->registers[REGISTER_A0], r9
  |.if WIN
    | pop rsi
    | pop rdi
  |.endif
  | pop rbp
  | pop rbx
  | pop r15
  | pop r14
  | pop r13
  | pop r12
  | ret
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
  |=>label:
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
      | op2_r_r add, target, b.value.reg, rax
      break;
    case AOT_TAG_IMMEDIATE:
      | op2_r_imm add, target, b.value.i, rax
      break;
    case AOT_TAG_X64_REGISTER:
      | op2_r_x add, target, Rq(b.value.x64_reg)
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
      | op2_r_r sub, target, b.value.reg, rax
      break;
    case AOT_TAG_IMMEDIATE:
      | op2_r_imm sub, target, b.value.i, rax
      break;
    case AOT_TAG_X64_REGISTER:
      | op2_r_x sub, target, Rq(b.value.x64_reg)
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
      | op2_x_r imul, rax, b.value.reg
      break;
    case AOT_TAG_IMMEDIATE:
      | load_imm rcx, b.value.i
      | imul rax, rcx
      break;
    case AOT_TAG_X64_REGISTER:
      | imul rax, Rq(b.value.x64_reg)
      break;
  }

  | op2_r_x mov, target, rax

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
        | op1_r imul, b.value.reg
      } else {
        | op1_r mul, b.value.reg
      }
      break;
    case AOT_TAG_IMMEDIATE:
      | load_imm rcx, b.value.i
      if (is_signed) {
        | imul rcx
      } else {
        | mul rcx
      }
      break;
    case AOT_TAG_X64_REGISTER:
      if (is_signed) {
        | imul Rq(b.value.x64_reg)
      } else {
        | mul Rq(b.value.x64_reg)
      }
      break;
  }
  | op2_r_x mov, target, rdx

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

  | test rax, rax
  | jns >1
  /* calculate res = mulhu(-a, b), res is stored in rdx after this. */
  | neg rax
  switch (b.tag) {
    case AOT_TAG_REGISTER:
      | op1_r mul, b.value.reg
      break;
    case AOT_TAG_IMMEDIATE:
      | load_imm rcx, b.value.i
      | mul rcx
      break;
    case AOT_TAG_X64_REGISTER:
      | mul Rq(b.value.x64_reg)
      break;
  }
  /* calculate ~res and store it in rcx */
  | xor rdx, -1
  | mov rcx, rdx
  /*
   * calculate (a * b), then test (a * b == 0) and convert that to 1 or 0,
   * result is stored in rax after this.
   */
  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }
  switch (b.tag) {
    case AOT_TAG_REGISTER:
      | op2_x_r imul, rax, b.value.reg
      break;
    case AOT_TAG_IMMEDIATE:
      | load_imm rdx, b.value.i
      | imul rax, rdx
      break;
    case AOT_TAG_X64_REGISTER:
      | imul rax, Rq(b.value.x64_reg)
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
  switch (b.tag) {
    case AOT_TAG_REGISTER:
      | op1_r mul, b.value.reg
      break;
    case AOT_TAG_IMMEDIATE:
      | load_imm rcx, b.value.i
      | mul rcx
      break;
    case AOT_TAG_X64_REGISTER:
      | mul Rq(b.value.x64_reg)
      break;
  }
  | mov rax, rdx
  |2:
  | op2_r_x mov, target, rax

  return DASM_S_OK;
}

int aot_div(AotContext* context, riscv_register_t target, AotValue a, AotValue b, int is_signed)
{
  int ret;
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  if (is_signed) {
    | mov64 rax, INT64_MIN
    ret = aot_mov_x64(context, X64_RCX, a);
    if (ret != DASM_S_OK) { return ret; }
    | cmp rax, rcx
    | jne >1
    | mov rax, -1
    ret = aot_mov_x64(context, X64_RCX, b);
    if (ret != DASM_S_OK) { return ret; }
    | cmp rax, rcx
    | jne >1
    ret = aot_mov_internal(context, target, a, X64_RAX);
    if (ret != DASM_S_OK) { return ret; }
    | jmp >3
  }
  |1:
  | mov rax, 0
  ret = aot_mov_x64(context, X64_RCX, b);
  if (ret != DASM_S_OK) { return ret; }
  | cmp rax, rcx
  | jne >2
  | op2_r_imm mov, target, (uint64_t)UINT64_MAX, rax
  | jmp >3
  |2:
  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }
  switch (b.tag) {
    case AOT_TAG_REGISTER:
      if (is_signed) {
        | cqo
        | op1_r idiv, b.value.reg
      } else {
        | xor rdx, rdx
        | op1_r div, b.value.reg
      }
      break;
    case AOT_TAG_IMMEDIATE:
      | load_imm, rcx, b.value.i
      if (is_signed) {
        | cqo
        | idiv rcx
      } else {
        | xor rdx, rdx
        | div rcx
      }
      break;
    case AOT_TAG_X64_REGISTER:
      if (is_signed) {
        | cqo
        | idiv Rq(b.value.x64_reg)
      } else {
        | xor rdx, rdx
        | div Rq(b.value.x64_reg)
      }
      break;
  }
  | op2_r_x mov, target, rax
  |3:

  return DASM_S_OK;
}

int aot_rem(AotContext* context, riscv_register_t target, AotValue a, AotValue b, int is_signed)
{
  int ret;
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  if (is_signed) {
    | mov64 rax, INT64_MIN
    ret = aot_mov_x64(context, X64_RCX, a);
    if (ret != DASM_S_OK) { return ret; }
    | cmp rax, rcx
    | jne >1
    | mov rax, -1
    ret = aot_mov_x64(context, X64_RCX, b);
    if (ret != DASM_S_OK) { return ret; }
    | cmp rax, rcx
    | jne >1
    | op2_r_imm mov, target, (uint64_t)0, rax
    | jmp >3
  }
  |1:
  | mov rax, 0
  ret = aot_mov_x64(context, X64_RCX, b);
  if (ret != DASM_S_OK) { return ret; }
  | cmp rax, rcx
  | jne >2
  ret = aot_mov_internal(context, target, a, X64_RAX);
  if (ret != DASM_S_OK) { return ret; }
  | jmp >3
  |2:
  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }
  switch (b.tag) {
    case AOT_TAG_REGISTER:
      if (is_signed) {
        | cqo
        | op1_r idiv, b.value.reg
      } else {
        | xor rdx, rdx
        | op1_r div, b.value.reg
      }
      break;
    case AOT_TAG_IMMEDIATE:
      | load_imm, rcx, b.value.i
      if (is_signed) {
        | cqo
        | idiv rcx
      } else {
        | xor rdx, rdx
        | div rcx
      }
      break;
    case AOT_TAG_X64_REGISTER:
      if (is_signed) {
        | cqo
        | idiv Rq(b.value.x64_reg)
      } else {
        | xor rdx, rdx
        | div Rq(b.value.x64_reg)
      }
      break;
  }
  | op2_r_x mov, target, rdx
  |3:

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
      | op2_r_r and, target, b.value.reg, rax
      break;
    case AOT_TAG_IMMEDIATE:
      | op2_r_imm and, target, b.value.i, rax
      break;
    case AOT_TAG_X64_REGISTER:
      | op2_r_x and, target, Rq(b.value.x64_reg)
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
      | op2_r_r or, target, b.value.reg, rax
      break;
    case AOT_TAG_IMMEDIATE:
      | op2_r_imm or, target, b.value.i, rax
      break;
    case AOT_TAG_X64_REGISTER:
      | op2_r_x or, target, Rq(b.value.x64_reg)
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

  | op1_r not, target
  if (logical) {
    | op2_r_imm and, target, (uint64_t)1, rax
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
      | op2_r_r xor, target, b.value.reg, rax
      break;
    case AOT_TAG_IMMEDIATE:
      | op2_r_imm xor, target, b.value.i, rax
      break;
    case AOT_TAG_X64_REGISTER:
      | op2_r_x xor, target, Rq(b.value.x64_reg)
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
      | op2_x_r mov, rcx, b.value.reg
      break;
    case AOT_TAG_IMMEDIATE:
      /*
       * shift operations only use cl as operand, there won't be any
       * overflowing issues.
       */
      | mov ecx, b.value.i
      break;
    case AOT_TAG_X64_REGISTER:
      | mov rcx, Rq(b.value.x64_reg)
      break;
  }

  | op2_r_x shl, target, cl

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
      | op2_x_r mov, rcx, b.value.reg
      break;
    case AOT_TAG_IMMEDIATE:
      /*
       * shift operations only use cl as operand, there won't be any
       * overflowing issues.
       */
      | mov ecx, b.value.i
      break;
    case AOT_TAG_X64_REGISTER:
      | mov rcx, Rq(b.value.x64_reg)
      break;
  }

  if (is_signed) {
    | op2_r_x sar, target, cl
  } else {
    | op2_r_x shr, target, cl
  }

  return DASM_S_OK;
}

int aot_clmul(AotContext* context, riscv_register_t target, AotValue a, AotValue b) {
  int ret;
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  | xor ecx, ecx
  | xor r10, r10
  |1:
  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }
  ret = aot_mov_x64(context, X64_RDX, b);
  if (ret != DASM_S_OK) { return ret; }
  | shl rax, cl
  | shr rdx, cl
  | xor rax, r10
  | and rdx, 1
  | cmovne r10, rax
  | add rcx, 1
  | cmp rcx, 64
  | jne <1
  | op2_r_x mov, target, r10

  return DASM_S_OK;
}

int aot_clmulh(AotContext* context, riscv_register_t target, AotValue a, AotValue b) {
  int ret;
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  | mov r11, 1
  | xor r10, r10
  |1:
  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }
  ret = aot_mov_x64(context, X64_RDX, b);
  if (ret != DASM_S_OK) { return ret; }
  | mov rcx, 64
  | sub rcx, r11
  | shr rax, cl
  | mov rcx, r11
  | shr rdx, cl
  | xor rax, r10
  | and rdx, 1
  | cmovne r10, rax
  | add r11, 1
  | cmp r11, 64
  | jne <1
  | op2_r_x mov, target, r10

  return DASM_S_OK;
}

int aot_clmulr(AotContext* context, riscv_register_t target, AotValue a, AotValue b)
{
  int ret;
  uint32_t loc1;
  dasm_State** Dst = &context->d;

  | xor r11, r11
  | xor r10, r10
  |1:
  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }
  ret = aot_mov_x64(context, X64_RDX, b);
  if (ret != DASM_S_OK) { return ret; }
  | mov rcx, 63
  | sub rcx, r11
  | shr rax, cl
  | mov rcx, r11
  | shr rdx, cl
  | xor rax, r10
  | and rdx, 1
  | cmovne r10, rax
  | add r11, 1
  | cmp r11, 64
  | jne <1
  | op2_r_x mov, target, r10

  return DASM_S_OK;
}

int aot_orcb(AotContext* context, riscv_register_t target, AotValue a)
{
  uint32_t loc1;
  int ret;
  dasm_State** Dst = &context->d;

  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }

  | xor rdx, rdx
  | mov64 r10, (uint64_t)0x00000000000000ff
  | mov r11, r10
  | and r10, rax
  | je >1
  | or rdx, r11
  |1:
  | mov64 r10, (uint64_t)0x000000000000ff00
  | mov r11, r10
  | and r10, rax
  | je >2
  | or rdx, r11
  |2:
  | mov64 r10, (uint64_t)0x0000000000ff0000
  | mov r11, r10
  | and r10, rax
  | je >3
  | or rdx, r11
  |3:
  | mov64 r10, (uint64_t)0x00000000ff000000
  | mov r11, r10
  | and r10, rax
  | je >4
  | or rdx, r11
  |4:
  | mov64 r10, (uint64_t)0x000000ff00000000
  | mov r11, r10
  | and r10, rax
  | je >5
  | or rdx, r11
  |5:
  | mov64 r10, (uint64_t)0x0000ff0000000000
  | mov r11, r10
  | and r10, rax
  | je >6
  | or rdx, r11
  |6:
  | mov64 r10, (uint64_t)0x00ff000000000000
  | mov r11, r10
  | and r10, rax
  | je >7
  | or rdx, r11
  |7:
  | mov64 r10, (uint64_t)0xff00000000000000
  | mov r11, r10
  | and r10, rax
  | je >8
  | or rdx, r11
  |8:
  | op2_r_x mov, target, rdx

  return DASM_S_OK;
}

int aot_rev8(AotContext* context, riscv_register_t target, AotValue a)
{
  uint32_t loc1;
  int ret;
  dasm_State** Dst = &context->d;

  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }

  | xor rdx, rdx
  | mov64 r10, (uint64_t)0x00000000000000ff
  | and r10, rax
  | shl r10, 56
  | or rdx, r10
  | mov64 r10, (uint64_t)0x000000000000ff00
  | and r10, rax
  | shl r10, 40
  | or rdx, r10
  | mov64 r10, (uint64_t)0x0000000000ff0000
  | and r10, rax
  | shl r10, 24
  | or rdx, r10
  | mov64 r10, (uint64_t)0x00000000ff000000
  | and r10, rax
  | shl r10, 8
  | or rdx, r10
  | mov64 r10, (uint64_t)0x000000ff00000000
  | and r10, rax
  | shr r10, 8
  | or rdx, r10
  | mov64 r10, (uint64_t)0x0000ff0000000000
  | and r10, rax
  | shr r10, 24
  | or rdx, r10
  | mov64 r10, (uint64_t)0x00ff000000000000
  | and r10, rax
  | shr r10, 40
  | or rdx, r10
  | mov64 r10, (uint64_t)0xff00000000000000
  | and r10, rax
  | shr r10, 56
  | or rdx, r10
  | op2_r_x mov, target, rdx

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
      | op2_x_r mov, rcx, b.value.reg
      break;
    case AOT_TAG_IMMEDIATE:
      /*
       * shift operations only use cl as operand, there won't be any
       * overflowing issues.
       */
      | mov ecx, b.value.i
      break;
    case AOT_TAG_X64_REGISTER:
      | mov rcx, Rq(b.value.x64_reg)
      break;
  }

  | op2_r_x rol, target, cl

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
      | op2_x_r mov, rcx, b.value.reg
      break;
    case AOT_TAG_IMMEDIATE:
      /*
       * shift operations only use cl as operand, there won't be any
       * overflowing issues.
       */
      | mov ecx, b.value.i
      break;
    case AOT_TAG_X64_REGISTER:
      | mov rcx, Rq(b.value.x64_reg)
      break;
  }

  | op2_r_x ror, target, cl

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
      | op2_x_r cmp, rcx, b.value.reg
      break;
    case AOT_TAG_IMMEDIATE:
      | load_imm rax, b.value.i
      | cmp rcx, rax
      break;
    case AOT_TAG_X64_REGISTER:
      | cmp rcx, Rq(b.value.x64_reg)
      break;
  }

  | sete cl
  | movzx rcx, cl
  | op2_r_x mov, target, rcx

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
      | op2_x_r cmp, rcx, b.value.reg
      break;
    case AOT_TAG_IMMEDIATE:
      | load_imm rax, b.value.i
      | cmp rcx, rax
      break;
    case AOT_TAG_X64_REGISTER:
      | cmp rcx, Rq(b.value.x64_reg)
      break;
  }

  if (is_signed) {
    | setl cl
  } else {
    | setb cl
  }
  | movzx rcx, cl
  | op2_r_x mov, target, rcx

  return DASM_S_OK;
}

int aot_cond(AotContext* context, riscv_register_t target, AotValue condition, AotValue true_value, AotValue false_value) {
  uint32_t loc1;
  int ret;
  dasm_State** Dst = &context->d;

  switch (condition.tag) {
    case AOT_TAG_REGISTER:
      | op2_r_imm cmp, condition.value.reg, (uint64_t)1, rax
      | jne >1
      ret = aot_mov_internal(context, target, true_value, X64_RAX);
      if (ret != DASM_S_OK) { return ret; }
      | jmp >2
      |1:
      ret = aot_mov_internal(context, target, false_value, X64_RAX);
      if (ret != DASM_S_OK) { return ret; }
      |2:
      break;
    case AOT_TAG_IMMEDIATE:
      ret = aot_mov_internal(context, target, (condition.value.i == 1) ? true_value : false_value, X64_RAX);
      if (ret != DASM_S_OK) { return ret; }
      break;
    case AOT_TAG_X64_REGISTER:
      | cmp Rq(condition.value.x64_reg), 1
      | jne >1
      ret = aot_mov_internal(context, target, true_value, X64_RAX);
      if (ret != DASM_S_OK) { return ret; }
      | jmp >2
      |1:
      ret = aot_mov_internal(context, target, false_value, X64_RAX);
      if (ret != DASM_S_OK) { return ret; }
      |2:
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

  | cmp rax, 0
  | je >1
  | bsr rax, rax
  | neg rax
  | add rax, 63
  | op2_r_x mov, target, rax
  | jmp >2
  |1:
  | op2_r_imm mov, target, (uint64_t)64, rax
  | jmp >2
  |2:

  return DASM_S_OK;
}

int aot_ctz(AotContext* context, riscv_register_t target, AotValue a)
{
  uint32_t loc1;
  int ret;
  dasm_State** Dst = &context->d;

  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }

  | cmp rax, 0
  | je >1
  | bsf rax, rax
  | op2_r_x mov, target, rax
  | jmp >2
  |1:
  | op2_r_imm mov, target, (uint64_t)64, rax
  | jmp >2
  |2:

  return DASM_S_OK;
}

int aot_cpop(AotContext* context, riscv_register_t target, AotValue a)
{
  uint32_t loc1;
  int ret;
  dasm_State** Dst = &context->d;

  ret = aot_mov_x64(context, X64_RAX, a);
  if (ret != DASM_S_OK) { return ret; }

  | mov rdx, rax
  | shr rdx, 1
  | mov64 rcx, 0x5555555555555555
  | and rdx, rcx
  | sub rax, rdx
  | mov rdx, rax
  | mov64 rcx, 0x3333333333333333
  | and rdx, rcx
  | shr rax, 2
  | and rax, rcx
  | add rax, rdx
  | mov rdx, rax
  | shr rdx, 4
  | add rax, rdx
  | mov64 rcx, 0x0F0F0F0F0F0F0F0F
  | and rax, rcx
  | mov rdx, rax
  | shr rdx, 8
  | add rax, rdx
  | mov rdx, rax
  | shr rdx, 16
  | add rax, rdx
  | mov rdx, rax
  | shr rdx, 32
  | add rax, rdx
  | and rax, 0x7F
  | op2_r_x mov, target, rax

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
      | movsxd rax, eax
    } else {
      | mov eax, eax
    }
    | op2_r_x mov, target, rax
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

      | mov ecx, 64
      | and edx, 0x3F
      | sub ecx, edx
      | shl rax, cl
      | mov ecx, edx
      if (is_signed) {
        | sar rax, cl
      } else {
        | shr rax, cl
      }
      break;
    case AOT_TAG_IMMEDIATE:
      if (bits.value.i < 64) {
        | shl rax, (64 - bits.value.i)
        if (is_signed) {
          | sar rax, (64 - bits.value.i)
        } else {
          | shr rax, (64 - bits.value.i)
        }
      }
      break;
    case AOT_TAG_X64_REGISTER:
      | mov rdx, Rq(bits.value.x64_reg)
      | mov ecx, 64
      | and edx, 0x3F
      | sub ecx, edx
      | shl rax, cl
      | mov ecx, edx
      if (is_signed) {
        | sar rax, cl
      } else {
        | shr rax, cl
      }
      break;
  }

  | op2_r_x mov, target, rax

  return DASM_S_OK;
}

int aot_exit(AotContext* context, int code)
{
  dasm_State** Dst = &context->d;
  | mov rax, code
  | jmp ->exit
  return DASM_S_OK;
}

int aot_add_cycles(AotContext* context, uint64_t cycles)
{
  int ret;
  dasm_State** Dst = &context->d;
  if (cycles == 0) {
    return DASM_S_OK;
  }
  | load_imm rax, cycles
  | add rax, machine->cycles
  | jnc >1
  ret = aot_exit(context, CKB_VM_ASM_RET_CYCLES_OVERFLOW);
  if (ret != DASM_S_OK) { return ret; }
  |1:
  | cmp rax, machine->max_cycles
  | jna >2
  ret = aot_exit(context, CKB_VM_ASM_RET_MAX_CYCLES_EXCEEDED);
  if (ret != DASM_S_OK) { return ret; }
  |2:
  | mov machine->cycles, rax
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
      | mov machine->pc, rcx
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
          | load_imm rcx, (value.value.i & 0xFFFFFFFFFFFFFF)
          | mov qword machine->pc, rcx
          break;
        case 0x40:
          | mov qword machine->pc, ((uint32_t)(value.value.i & 0x7FFFFFFF))
          | jmp =>((value.value.i >> 32) ^ 0x40000000)
          break;
        case 0x0:
          | load_imm rcx, value.value.i
          | mov machine->pc, rcx
          ret = aot_exit(context, CKB_VM_ASM_RET_DYNAMIC_JUMP);
          if (ret != DASM_S_OK) { return ret; }
          break;
        default:
          return ERROR_INVALID_VALUE;
      }
      break;
    case AOT_TAG_X64_REGISTER:
      | mov machine->pc, Rq(value.value.x64_reg)
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
      | op2_r_imm cmp, condition.value.reg, (uint64_t)1, rax
      | jne >1
      ret = aot_mov_pc_internal(context, true_value);
      if (ret != DASM_S_OK) { return ret; }
      |1:
      ret = aot_mov_pc_internal(context, false_value);
      if (ret != DASM_S_OK) { return ret; }
      break;
    case AOT_TAG_IMMEDIATE:
      ret = aot_mov_pc_internal(context, (condition.value.i == 1) ? true_value : false_value);
      if (ret != DASM_S_OK) { return ret; }
      break;
    case AOT_TAG_X64_REGISTER:
      | cmp Rq(condition.value.x64_reg), 1
      | jne >1
      ret = aot_mov_pc_internal(context, true_value);
      if (ret != DASM_S_OK) { return ret; }
      |1:
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

  | mov rdx, size
  | call ->check_write
  | cmp rdx, 0
  | jne >1
  | lea rdx, machine->memory
  ret = aot_mov_x64(context, X64_RCX, v);
  if (ret != DASM_S_OK) { return ret; }
  switch (size) {
    case 1:
      | mov byte [rdx+rax], cl
      break;
    case 2:
      | mov word [rdx+rax], cx
      break;
    case 4:
      | mov dword [rdx+rax], ecx
      break;
    case 8:
      | mov qword [rdx+rax], rcx
      break;
    default:
      return ERROR_INVALID_MEMORY_SIZE;
  }
  | jmp >2
  |1:
  | mov rax, rdx
  | jmp ->exit
  |2:

  return DASM_S_OK;
}

int aot_memory_read(AotContext* context, uint32_t target, AotValue address, uint32_t size)
{
  uint32_t loc1;
  int ret;
  dasm_State** Dst = &context->d;

  ret = aot_mov_x64(context, X64_RAX, address);
  if (ret != DASM_S_OK) { return ret; }

  | mov rdx, size
  | call ->check_read
  | cmp rdx, 0
  | jne >1
  | mov rdx, rax
  | add rdx, size
  | jc >1
  | cmp rdx, CKB_VM_ASM_RISCV_MAX_MEMORY
  if (context->version >= 1) {
    | ja >1
  } else {
    | jae >1
  }
  | lea rdx, machine->memory
  switch (size) {
    case 1:
      | movzx ecx, byte [rdx+rax]
      break;
    case 2:
      | movzx ecx, word [rdx+rax]
      break;
    case 4:
      | mov ecx, dword [rdx+rax]
      break;
    case 8:
      | mov rcx, qword [rdx+rax]
      break;
    default:
      return ERROR_INVALID_MEMORY_SIZE;
  }
  | op2_r_x mov, target, rcx
  | jmp >2
  | 1:
  ret = aot_exit(context, CKB_VM_ASM_RET_OUT_OF_BOUND);
  if (ret != DASM_S_OK) { return ret; }
  | 2:

  return DASM_S_OK;
}

static int aot_mov_internal(AotContext* context, riscv_register_t target, AotValue value, x64_register_t x64_temp_reg)
{
  uint32_t loc1, loc2;
  dasm_State** Dst = &context->d;

  switch (value.tag) {
    case AOT_TAG_REGISTER:
      if (target == value.value.reg) { return DASM_S_OK; }
      | op2_r_r mov, target, value.value.reg, Rq(x64_temp_reg)
      break;
    case AOT_TAG_IMMEDIATE:
      | op2_r_imm mov, target, value.value.i, Rq(x64_temp_reg)
      break;
    case AOT_TAG_X64_REGISTER:
      | op2_r_x mov, target, Rq(value.value.x64_reg)
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
      | op2_x_r mov, Rq(x64_target), value.value.reg
      break;
    case AOT_TAG_IMMEDIATE:
      | load_imm Rq(x64_target), value.value.i
      break;
    case AOT_TAG_X64_REGISTER:
      if (x64_target == value.value.x64_reg) { return DASM_S_OK; }
      | mov Rq(x64_target), Rq(value.value.x64_reg)
      break;
  }
  return DASM_S_OK;
}
