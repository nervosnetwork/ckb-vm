#define CKB_VM_ASM_GENERATE_LABEL_TABLES 1
#include "cdefinitions_generated.h"

#ifdef _WIN32
#define IS_WINDOWS 1
#endif

#define INT64_MIN 0x8000000000000000
#define UINT64_MAX 0xffffffffffffffff

#ifdef IS_WINDOWS
#define ARG1 %rcx
#define ARG2 %rdx
#else
#define ARG1 %rdi
#define ARG2 %rsi
#endif

#ifdef IS_WINDOWS
#define PREPCALL \
  push %rdi; \
  push %rsi; \
  push %rax; \
  push %rcx; \
  push %rdx; \
  push %r8; \
  push %r9; \
  push %r10; \
  push %rbp; \
  movq %rsp, %rbp; \
  andq $-16, %rsp; \
  sub $32, %rsp
#define POSTCALL \
  add $32, %rsp; \
  movq %rbp, %rsp; \
  pop %rbp; \
  pop %r10; \
  pop %r9; \
  pop %r8; \
  pop %rdx; \
  pop %rcx; \
  pop %rax; \
  pop %rsi; \
  pop %rdi
#else
#define PREPCALL \
  push %rdi; \
  push %rsi; \
  push %rax; \
  push %rcx; \
  push %rdx; \
  push %r8; \
  push %r9; \
  push %r10; \
  push %rbp; \
  movq %rsp, %rbp; \
  andq $-16, %rsp
#define POSTCALL \
  movq %rbp, %rsp; \
  pop %rbp; \
  pop %r10; \
  pop %r9; \
  pop %r8; \
  pop %rdx; \
  pop %rcx; \
  pop %rax; \
  pop %rsi; \
  pop %rdi
#endif

/* rax is used both in Microsoft x64 and System V AMD64 ABI */
#define ARG_RETd %eax

#define MACHINE %rsi
#define TRACE %rbx

/*
 * INST_PC contains the current address of decoded Instruction in
 * Trace item, which is different from the RISC-V PC
 */
#define INST_PC %r8
#define INST_ARGS %r9

/*
 * Rules to meet when considering register allocations:
 * * RD and TEMP1 cannot be %rcx to set aside %cl for shifts
 * * RS2r and TEMP1 cannot be %rax to allow using imul and idiv
 * * RS2r cannot be %rdx to allow using idiv
 */
#define RD_RS2s %rax
#define RD RD_RS2s
#define RS1 %rdx
#define RS2r %rbp
#define RS2s RD_RS2s
#define RS3 %r12
#define IMMEDIATE %rcx
#define TEMP1 %rdi
#define TEMP2 %r10
#define TEMP3 %r11
#define FLAG TEMP3

#define RDd_RS2sd %eax
#define RDd RDd_RS2sd
#define RS1d %edx
#define RS1h %dx
#define RS1b %dl
#define RS2rd %ebp
#define RS2rb %bpl
#define RS2sd %eax
#define RS2sb %al
#define RS2sh %ax
#define RS3d %r12d
#define IMMEDIATEb %cl
#define IMMEDIATEd %ecx
#define TEMP1b %dil
#define TEMP1d %edi
#define TEMP2d %r10d
#define TEMP2b %r10b
#define TEMP3d %r11d
#define TEMP3b %r11b

#define PC_ADDRESS \
  CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_PC(MACHINE)

#define ZERO_ADDRESS \
  CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(MACHINE)

#define RA_ADDRESS \
  (CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS + CKB_VM_ASM_REGISTER_RA * 8)(MACHINE)

#define SP_ADDRESS \
  (CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS + CKB_VM_ASM_REGISTER_SP * 8)(MACHINE)

#define REGISTER_ADDRESS(r) \
  CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS(MACHINE, r, 8)

#define VERSION_ADDRESS \
  CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_VERSION(MACHINE)

/*
 * Those macros provide slight abstractions over register allocations,
 * they might change when we alter register allocations. For example:
 *
 * * A push operation needs to do nothing if the variable to push is not
 * the register we need to save
 * * A move operation becomes nop if the variable is assigned the same
 * register as the target
 */
#define PUSH_RD_IF_RAX push %rax
#define PUSH_RD_IF_RDX
#define POP_RD_IF_RAX pop %rax
#define POP_RD_IF_RDX
#define PUSH_RS1_IF_RAX
#define PUSH_RS1_IF_RDX push %rdx
#define POP_RS1_IF_RAX
#define POP_RS1_IF_RDX pop %rdx

#define MOV_RS1_TO_RAX movq RS1, %rax
#define MOV_RAX_TO_RS1 movq %rax, RS1
#define MOV_RS2r_TO_RAX movq RS2r, %rax
#define MOV_RAX_TO_RS2r movq %rax, RS2r
#define MOV_RS1_TO_RDX
#define MOV_RDX_TO_RS1
#define MOV_RS2r_TO_RDX movq RS2r, %rdx
#define MOV_RDX_TO_RS2r movq %rdx, RS2r
#ifdef IS_WINDOWS
#define MOV_TEMP1_TO_ARG1 movq TEMP1, ARG1
#define MOV_MACHINE_TO_ARG2 movq MACHINE, ARG2
#else
#define MOV_TEMP1_TO_ARG1
#define MOV_MACHINE_TO_ARG2
#endif

#ifdef __APPLE__
#define CALL_INITED_MEMORY call _inited_memory
#elif defined IS_WINDOWS
#define CALL_INITED_MEMORY call inited_memory
#else
#define CALL_INITED_MEMORY call inited_memory@plt
#endif

#define CHECK_READ_BOUND_VERSION1(length) \
  movq REGISTER_ADDRESS(RS1), RS1; \
  addq IMMEDIATE, RS1; \
  movq RS1, TEMP1; \
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1; \
  jae .exit_out_of_bound; \
  addq length, TEMP1; \
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1; \
  ja .exit_out_of_bound

#define CHECK_READ(address_reg, length) \
  movq address_reg, TEMP1; \
  shr $CKB_VM_ASM_MEMORY_FRAME_SHIFTS, TEMP1; \
  cmp $CKB_VM_ASM_MEMORY_FRAMES, TEMP1; \
  jae .exit_out_of_bound; \
  movzbl CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FRAMES(MACHINE, TEMP1), TEMP2d; \
  cmp $0, TEMP2d; \
  jne 1f; \
  movb $1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FRAMES(MACHINE, TEMP1); \
  PREPCALL; \
  MOV_TEMP1_TO_ARG1; \
  MOV_MACHINE_TO_ARG2; \
  CALL_INITED_MEMORY; \
  POSTCALL; \
1: \
  movq address_reg, TEMP1; \
  addq $length, TEMP1; \
  subq $1, TEMP1; \
  shr $CKB_VM_ASM_MEMORY_FRAME_SHIFTS, TEMP1; \
  cmp $CKB_VM_ASM_MEMORY_FRAMES, TEMP1; \
  jae .exit_out_of_bound; \
  movzbl CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FRAMES(MACHINE, TEMP1), TEMP2d; \
  cmp $0, TEMP2d; \
  jne 2f; \
  movb $1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FRAMES(MACHINE, TEMP1);\
  PREPCALL; \
  MOV_TEMP1_TO_ARG1; \
  MOV_MACHINE_TO_ARG2; \
  CALL_INITED_MEMORY; \
  POSTCALL; \
2:

#define CHECK_WRITE(address_reg, temp_regd, length) \
  movq address_reg, TEMP1; \
  shr $CKB_VM_ASM_RISCV_PAGE_SHIFTS, TEMP1; \
  cmp $CKB_VM_ASM_RISCV_PAGES, TEMP1; \
  jae .exit_out_of_bound; \
  movzbl CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FLAGS(MACHINE, TEMP1), temp_regd; \
  mov temp_regd, TEMP2d; \
  and $CKB_VM_ASM_MEMORY_FLAG_WXORX_BIT, temp_regd; \
  cmp $CKB_VM_ASM_MEMORY_FLAG_WRITABLE, temp_regd; \
  jne .exit_invalid_permission; \
  or $CKB_VM_ASM_MEMORY_FLAG_DIRTY, TEMP2b; \
  movb TEMP2b, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FLAGS(MACHINE, TEMP1); \
  movq TEMP1, TEMP2; \
  shr $CKB_VM_ASM_MEMORY_FRAME_PAGE_SHIFTS, TEMP1; \
  movzbl CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FRAMES(MACHINE, TEMP1), temp_regd; \
  cmp $0, temp_regd; \
  jne 1f; \
  movb $1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FRAMES(MACHINE, TEMP1); \
  PREPCALL; \
  MOV_TEMP1_TO_ARG1; \
  MOV_MACHINE_TO_ARG2; \
  CALL_INITED_MEMORY; \
  POSTCALL; \
1: \
  movq TEMP2, TEMP1; \
  addq $1, TEMP1; \
  shl $CKB_VM_ASM_RISCV_PAGE_SHIFTS, TEMP1; \
  movq address_reg, TEMP2; \
  addq $length, TEMP2; \
  cmp TEMP1, TEMP2; \
  jbe 2f; \
  shr $CKB_VM_ASM_RISCV_PAGE_SHIFTS, TEMP1; \
  cmp $CKB_VM_ASM_RISCV_PAGES, TEMP1; \
  jae .exit_out_of_bound; \
  movzbl CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FLAGS(MACHINE, TEMP1), temp_regd; \
  mov temp_regd, TEMP2d; \
  and $CKB_VM_ASM_MEMORY_FLAG_WXORX_BIT, temp_regd; \
  cmp $CKB_VM_ASM_MEMORY_FLAG_WRITABLE, temp_regd; \
  jne .exit_invalid_permission; \
  or $CKB_VM_ASM_MEMORY_FLAG_DIRTY, TEMP2b; \
  movb TEMP2b, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FLAGS(MACHINE, TEMP1); \
  shr $CKB_VM_ASM_MEMORY_FRAME_PAGE_SHIFTS, TEMP1; \
  movzbl CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FRAMES(MACHINE, TEMP1), temp_regd; \
  cmp $0, temp_regd; \
  jne 2f; \
  movb $1, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FRAMES(MACHINE, TEMP1); \
  PREPCALL; \
  MOV_TEMP1_TO_ARG1; \
  MOV_MACHINE_TO_ARG2; \
  CALL_INITED_MEMORY; \
  POSTCALL; \
2:

#define ADDRESS_TO_SLOT_ADDRESS(r) \
  shr $5, r; \
  andq $8191, r; \
  imul $CKB_VM_ASM_TRACE_STRUCT_SIZE, r; \
  lea CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_TRACES(MACHINE, r), r

#define WRITE_RD_VALUE(v, temp_reg) \
  movq v, temp_reg; \
  movq temp_reg, REGISTER_ADDRESS(RD); \
  movq $0, ZERO_ADDRESS

#define WRITE_RD(v) \
  movq v, REGISTER_ADDRESS(RD); \
  movq $0, ZERO_ADDRESS

#define WRITE_RS1(v) \
  movq v, REGISTER_ADDRESS(RS1); \

#define WRITE_RS2r(v) \
  movq v, REGISTER_ADDRESS(RS2r); \

#define WRITE_RS3(v) \
  movq v, REGISTER_ADDRESS(RS3); \

#define NEXT_INST \
  movq (INST_ARGS), %rcx; \
  addq $8, INST_ARGS; \
  movzbl %ch, RDd_RS2sd; \
  sar $24, %rcx; \
  movzx %cl, FLAG; \
  sar $8, %rcx; \
  movq (INST_PC), TEMP1; \
  addq $8, INST_PC; \
  jmp *TEMP1

#define DECODE_R \
  movzbl %cl, RS1d; \
  movzbl %ch, RS2rd

#define DECODE_I \
  movzbl %cl, RS1d; \
  sar $8, %rcx

#define DECODE_S \
  movzbl %cl, RS1d; \
  sar $8, %rcx

#define DECODE_U

#define DECODE_R4 \
  movzbl %cl, RS1d; \
  movzbl %ch, RS2rd; \
  sar $16, %rcx; \
  movzbl %cl, RS3d

#ifdef __APPLE__
.globl _ckb_vm_x64_execute
_ckb_vm_x64_execute:
#else
.globl ckb_vm_x64_execute
ckb_vm_x64_execute:
#endif
#ifdef IS_WINDOWS
  push %rsi
  push %rdi
#endif
  push %rbp
  push %rbx
  push %r12
  mov ARG1, MACHINE
.p2align 3
.CKB_VM_ASM_LABEL_OP_CUSTOM_TRACE_END:
.prepare_trace:
  movq PC_ADDRESS, %rax
  mov %eax, %ecx
  shr $2, %eax
  andq $8191, %rax
  imul $CKB_VM_ASM_TRACE_STRUCT_SIZE, %eax
  lea CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_TRACES(MACHINE, %rax), TRACE
  movq CKB_VM_ASM_TRACE_OFFSET_ADDRESS(TRACE), %rdx
  cmp %rcx, %rdx
  jne .exit_trace
  movzbl CKB_VM_ASM_TRACE_OFFSET_LENGTH(TRACE), %edx
  cmp $0, %rdx
  je .exit_trace
  movq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_CYCLES(MACHINE), %rax
  addq CKB_VM_ASM_TRACE_OFFSET_CYCLES(TRACE), %rax
  jc .exit_cycles_overflow
  cmp CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MAX_CYCLES(MACHINE), %rax
  ja .exit_max_cycles_exceeded
  movq %rax, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_CYCLES(MACHINE)
  addq %rdx, PC_ADDRESS
  lea CKB_VM_ASM_TRACE_OFFSET_INSTRUCTIONS(TRACE), INST_ARGS
  lea CKB_VM_ASM_TRACE_OFFSET_THREAD(TRACE), INST_PC
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_ADDI:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_ADD:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  addq REGISTER_ADDRESS(RS2r), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_ADDIW:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  movslq RS1d, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_ADDW:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  addq REGISTER_ADDRESS(RS2r), RS1
  movslq RS1d, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_AND:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  andq REGISTER_ADDRESS(RS2r), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_ANDI:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  andq IMMEDIATE, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_AUIPC:
  DECODE_U
  movq PC_ADDRESS, RS1
  subq $4, RS1
  addq IMMEDIATE, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_BEQ:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2s), RS2s
  cmpq RS2s, RS1
  je .i_branch_success
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_BGE:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2s), RS2s
  cmpq RS2s, RS1
  jge .i_branch_success
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_BGEU:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2s), RS2s
  cmpq RS2s, RS1
  jae .i_branch_success
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_BLT:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2s), RS2s
  cmpq RS2s, RS1
  jl .i_branch_success
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_BLTU:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2s), RS2s
  cmpq RS2s, RS1
  jb .i_branch_success
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_BNE:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2s), RS2s
  cmpq RS2s, RS1
  jne .i_branch_success
  NEXT_INST
.i_branch_success:
  movq PC_ADDRESS, RS1
  andq $0xF, FLAG
  shl $1, FLAG
  subq FLAG, RS1
  addq IMMEDIATE, RS1
  movq RS1, PC_ADDRESS
  jmp .prepare_trace
.p2align 3
.CKB_VM_ASM_LABEL_OP_DIV:
  DECODE_R
  push RD
  movq $INT64_MIN, RD
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  cmp RD, RS1
  jne .div_branch1
  cmp $-1, RS2r
  jne .div_branch1
  jmp .div_branch3
.div_branch1:
  cmp $0, RS2r
  jne .div_branch2
  movq $UINT64_MAX, RS1
  jmp .div_branch3
.div_branch2:
  MOV_RS1_TO_RAX
  cqo
  idivq RS2r
  MOV_RAX_TO_RS1
.div_branch3:
  pop RD
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_DIVU:
  DECODE_R
  movq REGISTER_ADDRESS(RS2r), RS2r
  cmp $0, RS2r
  jne .divu_branch1
  WRITE_RD_VALUE($UINT64_MAX, RS2r)
  NEXT_INST
.divu_branch1:
  PUSH_RD_IF_RAX
  PUSH_RD_IF_RDX
  movq REGISTER_ADDRESS(RS1), %rax
  xorq %rdx, %rdx
  divq RS2r
  mov %rax, RS2r
  POP_RD_IF_RDX
  POP_RD_IF_RAX
  WRITE_RD(RS2r)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_DIVUW:
  DECODE_R
  movq REGISTER_ADDRESS(RS2r), RS2r
  mov RS2rd, RS2rd
  cmp $0, RS2r
  jne .divuw_branch1
  WRITE_RD_VALUE($UINT64_MAX, RS2r)
  NEXT_INST
.divuw_branch1:
  PUSH_RD_IF_RAX
  PUSH_RD_IF_RDX
  movq REGISTER_ADDRESS(RS1), %rax
  mov %eax, %eax
  xorq %rdx, %rdx
  divq RS2r
  mov %rax, RS2r
  POP_RD_IF_RDX
  POP_RD_IF_RAX
  movslq RS2rd, RS2r
  WRITE_RD(RS2r)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_DIVW:
  DECODE_R
  push RD
  movq $INT64_MIN, RD
  movq REGISTER_ADDRESS(RS1), RS1
  movslq RS1d, RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  movslq RS2rd, RS2r
  cmp RD, RS1
  jne .divw_branch1
  cmp $-1, RS2r
  jne .divw_branch1
  jmp .divw_branch3
.divw_branch1:
  cmp $0, RS2r
  jne .divw_branch2
  movq $UINT64_MAX, RS1
  jmp .divw_branch3
.divw_branch2:
  MOV_RS1_TO_RAX
  cqo
  idivq RS2r
  MOV_RAX_TO_RS1
.divw_branch3:
  pop RD
  movslq RS1d, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_EBREAK:
  DECODE_U
  mov $CKB_VM_ASM_RET_EBREAK, ARG_RETd
  jmp .exit
.p2align 3
.CKB_VM_ASM_LABEL_OP_ECALL:
  DECODE_U
  mov $CKB_VM_ASM_RET_ECALL, ARG_RETd
  jmp .exit
.p2align 3
.CKB_VM_ASM_LABEL_OP_FENCE:
.CKB_VM_ASM_LABEL_OP_FENCEI:
  DECODE_U
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_JAL:
  DECODE_U
  movq PC_ADDRESS, RS1
  WRITE_RD(RS1)
  andq $0xF, FLAG
  shl $1, FLAG
  subq FLAG, RS1
  addq IMMEDIATE, RS1
  movq RS1, PC_ADDRESS
  jmp .prepare_trace
.p2align 3
.CKB_VM_ASM_LABEL_OP_JALR:
  mov VERSION_ADDRESS, TEMP1d
  cmp $1, TEMP1d
  jge .CKB_VM_ASM_LABEL_OP_JALR_VERSION1
  jmp .CKB_VM_ASM_LABEL_OP_JALR_VERSION0
.p2align 3
.CKB_VM_ASM_LABEL_OP_JALR_VERSION0:
  DECODE_I
  movq PC_ADDRESS, TEMP1
  WRITE_RD(TEMP1)
  movq REGISTER_ADDRESS(RS1), TEMP1
  addq IMMEDIATE, TEMP1
  andq $-2, TEMP1
  movq TEMP1, PC_ADDRESS
  jmp .prepare_trace
/* RS2r is used as a temporary register here */
.p2align 3
.CKB_VM_ASM_LABEL_OP_JALR_VERSION1:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS2r
  movq PC_ADDRESS, TEMP1
  WRITE_RD(TEMP1)
  addq IMMEDIATE, RS2r
  andq $-2, RS2r
  movq RS2r, PC_ADDRESS
  jmp .prepare_trace
.p2align 3
.CKB_VM_ASM_LABEL_OP_LB:
  mov VERSION_ADDRESS, TEMP1d
  cmp $1, TEMP1d
  jge .CKB_VM_ASM_LABEL_OP_LB_VERSION1
  jmp .CKB_VM_ASM_LABEL_OP_LB_VERSION0
.p2align 3
.CKB_VM_ASM_LABEL_OP_LB_VERSION0:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  movq RS1, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  addq $1, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  CHECK_READ(RS1, 1)
  movsbq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_LB_VERSION1:
  DECODE_I
  CHECK_READ_BOUND_VERSION1($1)
  CHECK_READ(RS1, 1)
  movsbq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_LBU:
  mov VERSION_ADDRESS, TEMP1d
  cmp $1, TEMP1d
  jge .CKB_VM_ASM_LABEL_OP_LBU_VERSION1
  jmp .CKB_VM_ASM_LABEL_OP_LBU_VERSION0
.p2align 3
.CKB_VM_ASM_LABEL_OP_LBU_VERSION0:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  movq RS1, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  addq $1, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  CHECK_READ(RS1, 1)
  movzbq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_LBU_VERSION1:
  DECODE_I
  CHECK_READ_BOUND_VERSION1($1)
  CHECK_READ(RS1, 1)
  movzbq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_LD:
  mov VERSION_ADDRESS, TEMP1d
  cmp $1, TEMP1d
  jge .CKB_VM_ASM_LABEL_OP_LD_VERSION1
  jmp .CKB_VM_ASM_LABEL_OP_LD_VERSION0
.p2align 3
.CKB_VM_ASM_LABEL_OP_LD_VERSION0:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  movq RS1, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  addq $8, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  CHECK_READ(RS1, 8)
  movq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_LD_VERSION1:
  DECODE_I
  CHECK_READ_BOUND_VERSION1($8)
  CHECK_READ(RS1, 1)
  movq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_LH:
  mov VERSION_ADDRESS, TEMP1d
  cmp $1, TEMP1d
  jge .CKB_VM_ASM_LABEL_OP_LH_VERSION1
  jmp .CKB_VM_ASM_LABEL_OP_LH_VERSION0
.p2align 3
.CKB_VM_ASM_LABEL_OP_LH_VERSION0:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  movq RS1, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  addq $2, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  CHECK_READ(RS1, 2)
  movswq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_LH_VERSION1:
  DECODE_I
  CHECK_READ_BOUND_VERSION1($2)
  CHECK_READ(RS1, 1)
  movswq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_LHU:
  mov VERSION_ADDRESS, TEMP1d
  cmp $1, TEMP1d
  jge .CKB_VM_ASM_LABEL_OP_LHU_VERSION1
  jmp .CKB_VM_ASM_LABEL_OP_LHU_VERSION0
.p2align 3
.CKB_VM_ASM_LABEL_OP_LHU_VERSION0:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  movq RS1, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  addq $2, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  CHECK_READ(RS1, 2)
  movzwq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_LHU_VERSION1:
  DECODE_I
  CHECK_READ_BOUND_VERSION1($2)
  CHECK_READ(RS1, 1)
  movzwq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_LUI:
.CKB_VM_ASM_LABEL_OP_CUSTOM_LOAD_IMM:
.CKB_VM_ASM_LABEL_OP_LD_SIGN_EXTENDED_32_CONSTANT:
  DECODE_U
  WRITE_RD(IMMEDIATE)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_LW:
  mov VERSION_ADDRESS, TEMP1d
  cmp $1, TEMP1d
  jge .CKB_VM_ASM_LABEL_OP_LW_VERSION1
  jmp .CKB_VM_ASM_LABEL_OP_LW_VERSION0
.p2align 3
.CKB_VM_ASM_LABEL_OP_LW_VERSION0:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  movq RS1, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  addq $4, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  CHECK_READ(RS1, 4)
  movslq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_LW_VERSION1:
  DECODE_I
  CHECK_READ_BOUND_VERSION1($4)
  CHECK_READ(RS1, 1)
  movslq CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_LWU:
  mov VERSION_ADDRESS, TEMP1d
  cmp $1, TEMP1d
  jge .CKB_VM_ASM_LABEL_OP_LWU_VERSION1
  jmp .CKB_VM_ASM_LABEL_OP_LWU_VERSION0
.p2align 3
.CKB_VM_ASM_LABEL_OP_LWU_VERSION0:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  movq RS1, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  addq $4, TEMP1
  cmp $CKB_VM_ASM_RISCV_MAX_MEMORY, TEMP1
  jae .exit_out_of_bound
  CHECK_READ(RS1, 4)
  mov CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1d
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_LWU_VERSION1:
  DECODE_I
  CHECK_READ_BOUND_VERSION1($4)
  CHECK_READ(RS1, 1)
  mov CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1), RS1d
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_MUL:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  imul REGISTER_ADDRESS(RS2r), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_MULH:
  DECODE_R
  PUSH_RD_IF_RAX
  PUSH_RD_IF_RDX
  movq REGISTER_ADDRESS(RS1), %rax
  imulq REGISTER_ADDRESS(RS2r)
  MOV_RDX_TO_RS2r
  POP_RD_IF_RDX
  POP_RD_IF_RAX
  WRITE_RD(RS2r)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_MULHSU:
  DECODE_R
  PUSH_RD_IF_RAX
  PUSH_RD_IF_RDX
  PUSH_RS1_IF_RAX
  PUSH_RS1_IF_RDX
  movq REGISTER_ADDRESS(RS1), %rax
  test %rax, %rax
  jns .mulhsu_branch1
  neg %rax
  mulq REGISTER_ADDRESS(RS2r)
  xor $-1, %rdx
  movq %rdx, TEMP1
  POP_RS1_IF_RDX
  POP_RS1_IF_RAX
  movq REGISTER_ADDRESS(RS1), %rax
  imulq REGISTER_ADDRESS(RS2r)
  test %rax, %rax
  setz %al
  movzbl %al, %eax
  addq %rax, TEMP1
  POP_RD_IF_RDX
  POP_RD_IF_RAX
  WRITE_RD(TEMP1)
  NEXT_INST
.mulhsu_branch1:
  mulq REGISTER_ADDRESS(RS2r)
  movq %rdx, TEMP1
  POP_RS1_IF_RDX
  POP_RS1_IF_RAX
  POP_RD_IF_RDX
  POP_RD_IF_RAX
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_MULHU:
  DECODE_R
  PUSH_RD_IF_RAX
  PUSH_RD_IF_RDX
  movq REGISTER_ADDRESS(RS1), %rax
  mulq REGISTER_ADDRESS(RS2r)
  movq %rdx, RS2r
  POP_RD_IF_RDX
  POP_RD_IF_RAX
  WRITE_RD(RS2r)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_MULW:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  imul RS2rd, RS1d
  movslq RS1d, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_OR:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  orq REGISTER_ADDRESS(RS2r), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_ORI:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  orq IMMEDIATE, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_REM:
  DECODE_R
  push RD
  movq $INT64_MIN, RD
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  cmp RD, RS1
  jne .rem_branch1
  cmp $-1, RS2r
  jne .rem_branch1
  xorq RS1, RS1
  jmp .rem_branch3
.rem_branch1:
  cmp $0, RS2r
  jne .rem_branch2
  jmp .rem_branch3
.rem_branch2:
  MOV_RS1_TO_RAX
  cqo
  idivq RS2r
  MOV_RDX_TO_RS1
.rem_branch3:
  pop RD
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_REMU:
  DECODE_R
  movq REGISTER_ADDRESS(RS2r), RS2r
  cmp $0, RS2r
  jne .remu_branch2
  movq REGISTER_ADDRESS(RS1), RS1
  WRITE_RD(RS1)
  NEXT_INST
.remu_branch2:
  PUSH_RD_IF_RAX
  PUSH_RD_IF_RDX
  movq REGISTER_ADDRESS(RS1), %rax
  xorq %rdx, %rdx
  divq RS2r
  mov %rdx, RS2r
  POP_RD_IF_RDX
  POP_RD_IF_RAX
  WRITE_RD(RS2r)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_REMUW:
  DECODE_R
  movq REGISTER_ADDRESS(RS2r), RS2r
  mov RS2rd, RS2rd
  cmp $0, RS2r
  jne .remuw_branch2
  movq REGISTER_ADDRESS(RS1), RS1
  movslq RS1d, RS1
  WRITE_RD(RS1)
  NEXT_INST
.remuw_branch2:
  PUSH_RD_IF_RAX
  PUSH_RD_IF_RDX
  movq REGISTER_ADDRESS(RS1), %rax
  mov %eax, %eax
  xorq %rdx, %rdx
  divq RS2r
  mov %rdx, RS2r
  POP_RD_IF_RDX
  POP_RD_IF_RAX
  movslq RS2rd, RS2r
  WRITE_RD(RS2r)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_REMW:
  DECODE_R
  push RD
  movq $INT64_MIN, RD
  movq REGISTER_ADDRESS(RS1), RS1
  movslq RS1d, RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  movslq RS2rd, RS2r
  cmp RD, RS1
  jne .remw_branch1
  cmp $-1, RS2r
  jne .remw_branch1
  xorq RS1, RS1
  jmp .remw_branch3
.remw_branch1:
  cmp $0, RS2r
  jne .remw_branch2
  jmp .remw_branch3
.remw_branch2:
  MOV_RS1_TO_RAX
  cqo
  idivq RS2r
  MOV_RDX_TO_RS1
.remw_branch3:
  pop RD
  movslq RS1d, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SB:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  CHECK_WRITE(RS1, RS2rd, 1)
  movq REGISTER_ADDRESS(RS2s), RS2s
  mov RS2sb, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SD:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  CHECK_WRITE(RS1, RS2rd, 8)
  movq REGISTER_ADDRESS(RS2s), RS2s
  movq RS2s, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SH:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  CHECK_WRITE(RS1, RS2rd, 2)
  movq REGISTER_ADDRESS(RS2s), RS2s
  mov RS2sh, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SLL:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq REGISTER_ADDRESS(RS2r), %rcx
  shl %cl, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SLLI:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq IMMEDIATE, %rcx
  shl %cl, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SLLIW:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq IMMEDIATE, %rcx
  shl %cl, TEMP1
  movslq TEMP1d, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SLLW:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq REGISTER_ADDRESS(RS2r), %rcx
  and $0x1F, %ecx
  shl %cl, TEMP1
  movslq TEMP1d, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SLT:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  cmpq RS2r, RS1
  setl RS1b
  movzbl RS1b, RS1d
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SLTI:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  cmpq IMMEDIATE, RS1
  setl RS1b
  movzbl RS1b, RS1d
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SLTIU:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  cmpq IMMEDIATE, RS1
  setb RS1b
  movzbl RS1b, RS1d
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SLTU:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  cmpq RS2r, RS1
  setb RS1b
  movzbl RS1b, RS1d
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SRA:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq REGISTER_ADDRESS(RS2r), %rcx
  sar %cl, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SRAI:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq IMMEDIATE, %rcx
  sar %cl, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SRAIW:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq IMMEDIATE, %rcx
  sar %cl, TEMP1d
  movslq TEMP1d, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SRAW:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq REGISTER_ADDRESS(RS2r), %rcx
  and $0x1F, %ecx
  sar %cl, TEMP1d
  movslq TEMP1d, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SRL:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq REGISTER_ADDRESS(RS2r), %rcx
  shr %cl, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SRLI:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq IMMEDIATE, %rcx
  shr %cl, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SRLIW:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq IMMEDIATE, %rcx
  shr %cl, TEMP1d
  movslq TEMP1d, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SRLW:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), TEMP1
  movq REGISTER_ADDRESS(RS2r), %rcx
  and $0x1F, %ecx
  shr %cl, TEMP1d
  movslq TEMP1d, TEMP1
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SUB:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  subq REGISTER_ADDRESS(RS2r), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SUBW:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  subq REGISTER_ADDRESS(RS2r), RS1
  movslq RS1d, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SW:
  DECODE_S
  movq REGISTER_ADDRESS(RS1), RS1
  addq IMMEDIATE, RS1
  CHECK_WRITE(RS1, RS2rd, 4)
  movq REGISTER_ADDRESS(RS2s), RS2s
  mov RS2sd, CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY(MACHINE, RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_XOR:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  xorq REGISTER_ADDRESS(RS2r), RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_XORI:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  xorq IMMEDIATE, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_ADDUW:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  mov RS1d, RS1d
  addq RS2r, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_ANDN:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  not RS2r
  and RS2r, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_BCLR:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), %rcx
  movq $1, TEMP1
  shl %cl, TEMP1
  not TEMP1
  and TEMP1, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_BCLRI:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  movq IMMEDIATE, %rcx
  movq $1, TEMP1
  shl %cl, TEMP1
  not TEMP1
  and TEMP1, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_BEXT:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), %rcx
  shr %cl, RS1
  and $1, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_BEXTI:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  movq IMMEDIATE, %rcx
  shr %cl, RS1
  and $1, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_BINV:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), %rcx
  movq $1, TEMP1
  shl %cl, TEMP1
  xor TEMP1, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_BINVI:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  movq IMMEDIATE, %rcx
  movq $1, TEMP1
  shl %cl, TEMP1
  xor TEMP1, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_BSET:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), %rcx
  movq $1, TEMP1
  shl %cl, TEMP1
  or TEMP1, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_BSETI:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  movq IMMEDIATE, %rcx
  movq $1, TEMP1
  shl %cl, TEMP1
  or TEMP1, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_CLMUL:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  xor %ecx, %ecx
  xor TEMP3, TEMP3
.clmul_branch:
  movq RS1, TEMP1
  movq RS2r, TEMP2
  shl %cl, TEMP1
  shr %cl, TEMP2
  xor TEMP3, TEMP1
  and $1, TEMP2
  cmovne TEMP1, TEMP3
  add $1, %rcx
  cmp $64, %rcx
  jne .clmul_branch
  WRITE_RD(TEMP3)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_CLMULH:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  movq $1, RS3
  xor TEMP3, TEMP3
.clmulh_branch:
  movq RS1, TEMP1
  movq RS2r, TEMP2
  movq $64, %rcx
  subq RS3, %rcx
  shr %cl, TEMP1
  movq RS3, %rcx
  shr %cl, TEMP2
  xor TEMP3, TEMP1
  and $1, TEMP2
  cmovne TEMP1, TEMP3
  add $1, RS3
  cmp $64, RS3
  jne .clmulh_branch
  WRITE_RD(TEMP3)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_CLMULR:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  xor RS3, RS3
  xor TEMP3, TEMP3
.clmulr_branch:
  movq RS1, TEMP1
  movq RS2r, TEMP2
  movq $63, %rcx
  subq RS3, %rcx
  shr %cl, TEMP1
  movq RS3, %rcx
  shr %cl, TEMP2
  xor TEMP3, TEMP1
  and $1, TEMP2
  cmovne TEMP1, TEMP3
  add $1, RS3
  cmp $64, RS3
  jne .clmulr_branch
  WRITE_RD(TEMP3)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_CLZ:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  cmp $0, RS1
  je .clz_branch
  bsr RS1, RS1
  neg RS1
  addq $63, RS1
  WRITE_RD(RS1)
  NEXT_INST
.clz_branch:
  WRITE_RD($64)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_CLZW:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  cmp $0, RS1d
  je .clzw_branch
  bsr RS1d, RS1d
  neg RS1
  addq $31, RS1
  WRITE_RD(RS1)
  NEXT_INST
.clzw_branch:
  WRITE_RD($32)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_CPOP:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  /* Since we only plan to use SSE2, the popcnt instruction cannot be used.
   * As an alternative, a soft-popcnt is given below, which is described in
   * Hacker's delight Chapter 5, Counting Bits.
   *
   * int popcnt64(uint64_t x)
   * {
   *   x = x - (x >> 1) & 0x5555555555555555;
   *   x = (x & 0x3333333333333333) + ((x >> 2) & 0x3333333333333333);
   *   x = (x + (x >> 4)) & 0x0f0f0f0f0f0f0f0f;
   *   x = x + (x >> 8);
   *   x = x + (x >> 16);
   *   x = x + (x >> 32);
   *   return x & 0x7f;
   * }
   */
  movq RS1, RS2r
  shr $1, RS2r
  movabs $0x5555555555555555, TEMP1
  and TEMP1, RS2r
  sub RS2r, RS1
  movq RS1, RS2r
  movabs $0x3333333333333333, TEMP1
  and TEMP1, RS2r
  shr $2, RS1
  and TEMP1, RS1
  add RS2r, RS1
  movq RS1, RS2r
  shr $4, RS2r
  add RS2r, RS1
  movabs $0x0f0f0f0f0f0f0f0f, TEMP1
  and TEMP1, RS1
  movq RS1, RS2r
  shr $8, RS2r
  add RS2r, RS1
  movq RS1, RS2r
  shr $16, RS2r
  add RS2r, RS1
  movq RS1, RS2r
  shr $32, RS2r
  add RS2r, RS1
  and $0x7f, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_CPOPW:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  /*
   * int popcnt32(uint32_t x) {
   *   x = x - ((x >> 1) & 0x55555555);
   *   x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
   *   x = (x + (x >> 4)) & 0x0f0f0f0f;
   *   x = x + (x >> 8);
   *   x = x + (x >> 16);
   *   return x & 0x3f;
   * }
   */
  mov RS1d, RS2rd
  shr $1, RS2rd
  and $0x55555555, RS2rd
  sub RS2rd, RS1d
  mov RS1d, RS2rd
  and $0x33333333, RS2rd
  shr $2, RS1d
  and $0x33333333, RS1d
  add RS2rd, RS1d
  mov RS1d, RS2rd
  shr $4, RS2rd
  add RS2rd, RS1d
  and $0x0f0f0f0f, RS1d
  mov RS1d, RS2rd
  shr $8, RS2rd
  add RS2rd, RS1d
  mov RS1d, RS2rd
  shr $16, RS2rd
  add RS2rd, RS1d
  and $0x3f, RS1d
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_CTZ:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  cmp $0, RS1
  je .ctz_branch
  bsf RS1, RS1
  WRITE_RD(RS1)
  NEXT_INST
.ctz_branch:
  WRITE_RD($64)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_CTZW:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  cmp $0, RS1d
  je .ctzw_branch
  bsf RS1d, RS1d
  WRITE_RD(RS1)
  NEXT_INST
.ctzw_branch:
  WRITE_RD($32)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_MAX:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  cmp RS1, RS2r
  cmovge RS2r, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_MAXU:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  cmp RS1, RS2r
  cmovae RS2r, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_MIN:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  cmp RS1, RS2r
  cmovle RS2r, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_MINU:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  cmp RS1, RS2r
  cmovbe RS2r, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_ORCB:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  xor RS2r, RS2r
  movq $0x00000000000000ff, TEMP1
  movq TEMP1, TEMP2
  andq RS1, TEMP1
  je .orcb_branch1
  orq TEMP2, RS2r
.orcb_branch1:
  movq $0x000000000000ff00, TEMP1
  movq TEMP1, TEMP2
  andq RS1, TEMP1
  je .orcb_branch2
  orq TEMP2, RS2r
.orcb_branch2:
  movq $0x0000000000ff0000, TEMP1
  movq TEMP1, TEMP2
  andq RS1, TEMP1
  je .orcb_branch3
  orq TEMP2, RS2r
.orcb_branch3:
  movq $0x00000000ff000000, TEMP1
  movq TEMP1, TEMP2
  andq RS1, TEMP1
  je .orcb_branch4
  orq TEMP2, RS2r
.orcb_branch4:
  movq $0x000000ff00000000, TEMP1
  movq TEMP1, TEMP2
  andq RS1, TEMP1
  je .orcb_branch5
  orq TEMP2, RS2r
.orcb_branch5:
  movq $0x0000ff0000000000, TEMP1
  movq TEMP1, TEMP2
  andq RS1, TEMP1
  je .orcb_branch6
  orq TEMP2, RS2r
.orcb_branch6:
  movq $0x00ff000000000000, TEMP1
  movq TEMP1, TEMP2
  andq RS1, TEMP1
  je .orcb_branch7
  orq TEMP2, RS2r
.orcb_branch7:
  movq $0xff00000000000000, TEMP1
  movq TEMP1, TEMP2
  andq RS1, TEMP1
  je .orcb_branch8
  orq TEMP2, RS2r
.orcb_branch8:
  WRITE_RD(RS2r)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_ORN:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  not RS2r
  or RS2r, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_REV8:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  xorq RS2r, RS2r
  movq $0x00000000000000ff, TEMP1
  andq RS1, TEMP1
  shl $56, TEMP1
  orq TEMP1, RS2r
  movq $0x000000000000ff00, TEMP1
  andq RS1, TEMP1
  shl $40, TEMP1
  orq TEMP1, RS2r
  movq $0x0000000000ff0000, TEMP1
  andq RS1, TEMP1
  shl $24, TEMP1
  orq TEMP1, RS2r
  movq $0x00000000ff000000, TEMP1
  andq RS1, TEMP1
  shl $8, TEMP1
  orq TEMP1, RS2r
  movq $0x000000ff00000000, TEMP1
  andq RS1, TEMP1
  shr $8, TEMP1
  orq TEMP1, RS2r
  movq $0x0000ff0000000000, TEMP1
  andq RS1, TEMP1
  shr $24, TEMP1
  orq TEMP1, RS2r
  movq $0x00ff000000000000, TEMP1
  andq RS1, TEMP1
  shr $40, TEMP1
  orq TEMP1, RS2r
  movq $0xff00000000000000, TEMP1
  andq RS1, TEMP1
  shr $56, TEMP1
  orq TEMP1, RS2r
  WRITE_RD(RS2r)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_ROL:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), %rcx
  rol %cl, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_ROLW:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), %rcx
  rol %cl, RS1d
  movslq RS1d, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_ROR:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  movb RS2rb, %cl
  ror %cl, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_RORI:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  ror IMMEDIATEb, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_RORIW:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  ror IMMEDIATEb, RS1d
  movslq RS1d, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_RORW:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), %rcx
  ror %cl, RS1d
  movslq RS1d, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SEXTB:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movsx RS1b, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SEXTH:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movsx RS1h, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SH1ADD:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  shl $1, RS1
  add RS2r, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SH1ADDUW:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  mov RS1d, RS1d
  shl $1, RS1
  add RS2r, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SH2ADD:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  shl $2, RS1
  add RS2r, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SH2ADDUW:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  mov RS1d, RS1d
  shl $2, RS1
  add RS2r, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SH3ADD:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  shl $3, RS1
  add RS2r, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SH3ADDUW:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  mov RS1d, RS1d
  shl $3, RS1
  add RS2r, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SLLIUW:
  DECODE_I
  movq REGISTER_ADDRESS(RS1), RS1
  movq IMMEDIATE, %rcx
  mov RS1d, RS1d
  and $0x3f, %cl
  shl %cl, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_XNOR:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  not RS2r
  xor RS2r, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_ZEXTH:
  DECODE_R
  movq REGISTER_ADDRESS(RS1), RS1
  shl $16, RS1d
  shr $16, RS1d
  movslq RS1d, RS1
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_FAR_JUMP_ABS:
  DECODE_U
  movq IMMEDIATE, RS2r
  movq PC_ADDRESS, TEMP1
  movq TEMP1, RA_ADDRESS
  andq $-2, RS2r
  movq RS2r, PC_ADDRESS
  jmp .prepare_trace
.p2align 3
.CKB_VM_ASM_LABEL_OP_FAR_JUMP_REL:
  DECODE_U
  movq PC_ADDRESS, RS2r
  andq $0xF, FLAG
  shl $1, FLAG
  subq FLAG, RS2r
  addq IMMEDIATE, RS2r
  movq PC_ADDRESS, TEMP1
  movq TEMP1, RA_ADDRESS
  andq $-2, RS2r
  movq RS2r, PC_ADDRESS
  jmp .prepare_trace
.p2align 3
.CKB_VM_ASM_LABEL_OP_WIDE_MUL:
  DECODE_R4
  PUSH_RD_IF_RAX
  PUSH_RD_IF_RDX
  movq REGISTER_ADDRESS(RS1), %rax
  imulq REGISTER_ADDRESS(RS2r)
  MOV_RDX_TO_RS2r
  movq %rax, TEMP1
  POP_RD_IF_RDX
  POP_RD_IF_RAX
  WRITE_RS3(TEMP1)
  WRITE_RD(RS2r)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_WIDE_MULU:
  DECODE_R4
  PUSH_RD_IF_RAX
  PUSH_RD_IF_RDX
  movq REGISTER_ADDRESS(RS1), %rax
  mulq REGISTER_ADDRESS(RS2r)
  MOV_RDX_TO_RS2r
  movq %rax, TEMP1
  POP_RD_IF_RDX
  POP_RD_IF_RAX
  WRITE_RS3(TEMP1)
  WRITE_RD(RS2r)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_WIDE_MULSU:
  DECODE_R4
  PUSH_RD_IF_RAX
  PUSH_RD_IF_RDX
  PUSH_RS1_IF_RAX
  PUSH_RS1_IF_RDX
  movq REGISTER_ADDRESS(RS1), %rax
  test %rax, %rax
  jns .wide_mulsu_branch1
  neg %rax
  mulq REGISTER_ADDRESS(RS2r)
  xor $-1, %rdx
  movq %rdx, TEMP1
  POP_RS1_IF_RDX
  POP_RS1_IF_RAX
  movq REGISTER_ADDRESS(RS1), %rax
  imulq REGISTER_ADDRESS(RS2r)
  movq %rax, TEMP2
  test %rax, %rax
  setz %al
  movzbl %al, %eax
  addq %rax, TEMP1
  POP_RD_IF_RDX
  POP_RD_IF_RAX
  WRITE_RS3(TEMP2)
  WRITE_RD(TEMP1)
  NEXT_INST
.wide_mulsu_branch1:
  mulq REGISTER_ADDRESS(RS2r)
  MOV_RDX_TO_RS2r
  movq %rax, TEMP1
  POP_RS1_IF_RDX
  POP_RS1_IF_RAX
  POP_RD_IF_RDX
  POP_RD_IF_RAX
  WRITE_RS3(TEMP1)
  WRITE_RD(RS2r)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_WIDE_DIV:
  DECODE_R4
  push RD
  movq $INT64_MIN, RD
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  cmp RD, RS1
  jne .wide_div_branch1
  cmp $-1, RS2r
  jne .wide_div_branch1
  xorq TEMP1, TEMP1
  jmp .wide_div_branch3
.wide_div_branch1:
  cmp $0, RS2r
  jne .wide_div_branch2
  movq RS1, TEMP1
  movq $UINT64_MAX, RS1
  jmp .wide_div_branch3
.wide_div_branch2:
  MOV_RS1_TO_RAX
  cqo
  idivq RS2r
  mov %rdx, TEMP1
  MOV_RAX_TO_RS1
.wide_div_branch3:
  pop RD
  WRITE_RS3(TEMP1)
  WRITE_RD(RS1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_WIDE_DIVU:
  DECODE_R4
  movq REGISTER_ADDRESS(RS1), RS1
  movq REGISTER_ADDRESS(RS2r), RS2r
  cmp $0, RS2r
  jne .wide_divu_branch1
  WRITE_RS3(RS1)
  WRITE_RD_VALUE($UINT64_MAX, RS2r)
  NEXT_INST
.wide_divu_branch1:
  PUSH_RD_IF_RAX
  PUSH_RD_IF_RDX
  movq RS1, %rax
  xorq %rdx, %rdx
  divq RS2r
  mov %rax, TEMP1
  MOV_RDX_TO_RS2r
  POP_RD_IF_RDX
  POP_RD_IF_RAX
  WRITE_RS3(RS2r)
  WRITE_RD(TEMP1)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_ADC:
  DECODE_R
  xor TEMP1, TEMP1
  xor TEMP2, TEMP2
  movq REGISTER_ADDRESS(RD), %rcx
  addq REGISTER_ADDRESS(RS1), %rcx
  adc $0, TEMP1
  addq REGISTER_ADDRESS(RS2r), %rcx
  adc $0, TEMP2
  orq TEMP2, TEMP1
  WRITE_RS1(TEMP1)
  WRITE_RS2r(TEMP2)
  WRITE_RD(%rcx)
  NEXT_INST
.p2align 3
.CKB_VM_ASM_LABEL_OP_SBB:
  DECODE_R4
  xor TEMP1, TEMP1
  xor TEMP2, TEMP2
  movq REGISTER_ADDRESS(RD), %rcx
  subq REGISTER_ADDRESS(RS1), %rcx
  adc $0, TEMP1
  subq REGISTER_ADDRESS(RS2r), %rcx
  adc $0, TEMP2
  WRITE_RS2r(TEMP2)
  WRITE_RS3(TEMP1)
  orq TEMP2, TEMP1
  WRITE_RS1(TEMP1)
  WRITE_RD(%rcx)
  NEXT_INST
.p2align 3
.exit_out_of_bound:
  mov $CKB_VM_ASM_RET_OUT_OF_BOUND, ARG_RETd
  jmp .exit
.p2align 3
.exit_max_cycles_exceeded:
  mov $CKB_VM_ASM_RET_MAX_CYCLES_EXCEEDED, ARG_RETd
  jmp .exit
.p2align 3
.exit_cycles_overflow:
  mov $CKB_VM_ASM_RET_CYCLES_OVERFLOW, ARG_RETd
  jmp .exit
.p2align 3
.exit_invalid_permission:
  mov $CKB_VM_ASM_RET_INVALID_PERMISSION, ARG_RETd
  jmp .exit
/*
 * Some instructions that are difficult to implement will be interpreted and
 * executed by the rust interpreter
 */
.p2align 3
.exit_slowpath:
  mov $CKB_VM_ASM_RET_SLOWPATH, ARG_RETd
  jmp .exit
.p2align 3
.exit_trace:
.CKB_VM_ASM_LABEL_OP_UNLOADED:
  DECODE_U
  mov $CKB_VM_ASM_RET_DECODE_TRACE, ARG_RETd
  jmp .exit
.exit:
  pop %r12
  pop %rbx
  pop %rbp
#ifdef IS_WINDOWS
  pop %rdi
  pop %rsi
#endif
  retq
