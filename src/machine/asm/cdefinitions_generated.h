#define CKB_VM_ASM_RISCV_MAX_MEMORY 4194304
#define CKB_VM_ASM_RISCV_PAGE_SHIFTS 12
#define CKB_VM_ASM_RISCV_PAGE_SIZE 4096
#define CKB_VM_ASM_RISCV_PAGE_MASK 4095
#define CKB_VM_ASM_RISCV_PAGES 1024
#define CKB_VM_ASM_MEMORY_FRAME_SHIFTS 18
#define CKB_VM_ASM_MEMORY_FRAMESIZE 262144
#define CKB_VM_ASM_MEMORY_FRAMES 16
#define CKB_VM_ASM_MEMORY_FRAME_PAGE_SHIFTS 6

#define CKB_VM_ASM_MAXIMUM_TRACE_ADDRESS_LENGTH 64

#define CKB_VM_ASM_RET_DECODE_TRACE 1
#define CKB_VM_ASM_RET_ECALL 2
#define CKB_VM_ASM_RET_EBREAK 3
#define CKB_VM_ASM_RET_DYNAMIC_JUMP 4
#define CKB_VM_ASM_RET_MAX_CYCLES_EXCEEDED 5
#define CKB_VM_ASM_RET_CYCLES_OVERFLOW 6
#define CKB_VM_ASM_RET_OUT_OF_BOUND 7
#define CKB_VM_ASM_RET_INVALID_PERMISSION 8
#define CKB_VM_ASM_RET_SLOWPATH 9

#define CKB_VM_ASM_REGISTER_RA 1
#define CKB_VM_ASM_REGISTER_SP 2

#define CKB_VM_ASM_MEMORY_FLAG_FREEZED 1
#define CKB_VM_ASM_MEMORY_FLAG_EXECUTABLE 2
#define CKB_VM_ASM_MEMORY_FLAG_WXORX_BIT 2
#define CKB_VM_ASM_MEMORY_FLAG_WRITABLE 0
#define CKB_VM_ASM_MEMORY_FLAG_DIRTY 4

#define CKB_VM_ASM_TRACE_STRUCT_SIZE 296
#define CKB_VM_ASM_TRACE_OFFSET_ADDRESS 0
#define CKB_VM_ASM_TRACE_OFFSET_LENGTH 8
#define CKB_VM_ASM_TRACE_OFFSET_CYCLES 16
#define CKB_VM_ASM_TRACE_OFFSET_INSTRUCTIONS 24
#define CKB_VM_ASM_TRACE_OFFSET_THREAD 160

#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS 0
#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_PC 256
#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_CYCLES 280
#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MAX_CYCLES 288
#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_CHAOS_MODE 296
#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_CHAOS_SEED 300
#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_LOAD_RESERVATION_ADDRESS 304
#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY_SIZE 320
#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FRAMES_SIZE 328
#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FLAGS_SIZE 336
#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_LAST_READ_FRAME 344
#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_LAST_WRITE_PAGE 352
#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FLAGS 360
#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY 2426232
#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_TRACES 1400
#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_FRAMES 1384

#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY_H 2424832
#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY_L 1400

#define CKB_VM_ASM_OP_UNLOADED 16
#define CKB_VM_ASM_OP_ADD 17
#define CKB_VM_ASM_OP_ADDI 18
#define CKB_VM_ASM_OP_ADDIW 19
#define CKB_VM_ASM_OP_ADDW 20
#define CKB_VM_ASM_OP_AND 21
#define CKB_VM_ASM_OP_ANDI 22
#define CKB_VM_ASM_OP_DIV 23
#define CKB_VM_ASM_OP_DIVU 24
#define CKB_VM_ASM_OP_DIVUW 25
#define CKB_VM_ASM_OP_DIVW 26
#define CKB_VM_ASM_OP_LB_VERSION0 27
#define CKB_VM_ASM_OP_LB_VERSION1 28
#define CKB_VM_ASM_OP_LBU_VERSION0 29
#define CKB_VM_ASM_OP_LBU_VERSION1 30
#define CKB_VM_ASM_OP_LD_VERSION0 31
#define CKB_VM_ASM_OP_LD_VERSION1 32
#define CKB_VM_ASM_OP_LH_VERSION0 33
#define CKB_VM_ASM_OP_LH_VERSION1 34
#define CKB_VM_ASM_OP_LHU_VERSION0 35
#define CKB_VM_ASM_OP_LHU_VERSION1 36
#define CKB_VM_ASM_OP_LUI 37
#define CKB_VM_ASM_OP_LW_VERSION0 38
#define CKB_VM_ASM_OP_LW_VERSION1 39
#define CKB_VM_ASM_OP_LWU_VERSION0 40
#define CKB_VM_ASM_OP_LWU_VERSION1 41
#define CKB_VM_ASM_OP_MUL 42
#define CKB_VM_ASM_OP_MULH 43
#define CKB_VM_ASM_OP_MULHSU 44
#define CKB_VM_ASM_OP_MULHU 45
#define CKB_VM_ASM_OP_MULW 46
#define CKB_VM_ASM_OP_OR 47
#define CKB_VM_ASM_OP_ORI 48
#define CKB_VM_ASM_OP_REM 49
#define CKB_VM_ASM_OP_REMU 50
#define CKB_VM_ASM_OP_REMUW 51
#define CKB_VM_ASM_OP_REMW 52
#define CKB_VM_ASM_OP_SB 53
#define CKB_VM_ASM_OP_SD 54
#define CKB_VM_ASM_OP_SH 55
#define CKB_VM_ASM_OP_SLL 56
#define CKB_VM_ASM_OP_SLLI 57
#define CKB_VM_ASM_OP_SLLIW 58
#define CKB_VM_ASM_OP_SLLW 59
#define CKB_VM_ASM_OP_SLT 60
#define CKB_VM_ASM_OP_SLTI 61
#define CKB_VM_ASM_OP_SLTIU 62
#define CKB_VM_ASM_OP_SLTU 63
#define CKB_VM_ASM_OP_SRA 64
#define CKB_VM_ASM_OP_SRAI 65
#define CKB_VM_ASM_OP_SRAIW 66
#define CKB_VM_ASM_OP_SRAW 67
#define CKB_VM_ASM_OP_SRL 68
#define CKB_VM_ASM_OP_SRLI 69
#define CKB_VM_ASM_OP_SRLIW 70
#define CKB_VM_ASM_OP_SRLW 71
#define CKB_VM_ASM_OP_SUB 72
#define CKB_VM_ASM_OP_SUBW 73
#define CKB_VM_ASM_OP_SW 74
#define CKB_VM_ASM_OP_XOR 75
#define CKB_VM_ASM_OP_XORI 76
#define CKB_VM_ASM_OP_LR_W 77
#define CKB_VM_ASM_OP_SC_W 78
#define CKB_VM_ASM_OP_AMOSWAP_W 79
#define CKB_VM_ASM_OP_AMOADD_W 80
#define CKB_VM_ASM_OP_AMOXOR_W 81
#define CKB_VM_ASM_OP_AMOAND_W 82
#define CKB_VM_ASM_OP_AMOOR_W 83
#define CKB_VM_ASM_OP_AMOMIN_W 84
#define CKB_VM_ASM_OP_AMOMAX_W 85
#define CKB_VM_ASM_OP_AMOMINU_W 86
#define CKB_VM_ASM_OP_AMOMAXU_W 87
#define CKB_VM_ASM_OP_LR_D 88
#define CKB_VM_ASM_OP_SC_D 89
#define CKB_VM_ASM_OP_AMOSWAP_D 90
#define CKB_VM_ASM_OP_AMOADD_D 91
#define CKB_VM_ASM_OP_AMOXOR_D 92
#define CKB_VM_ASM_OP_AMOAND_D 93
#define CKB_VM_ASM_OP_AMOOR_D 94
#define CKB_VM_ASM_OP_AMOMIN_D 95
#define CKB_VM_ASM_OP_AMOMAX_D 96
#define CKB_VM_ASM_OP_AMOMINU_D 97
#define CKB_VM_ASM_OP_AMOMAXU_D 98
#define CKB_VM_ASM_OP_ADDUW 99
#define CKB_VM_ASM_OP_ANDN 100
#define CKB_VM_ASM_OP_BCLR 101
#define CKB_VM_ASM_OP_BCLRI 102
#define CKB_VM_ASM_OP_BEXT 103
#define CKB_VM_ASM_OP_BEXTI 104
#define CKB_VM_ASM_OP_BINV 105
#define CKB_VM_ASM_OP_BINVI 106
#define CKB_VM_ASM_OP_BSET 107
#define CKB_VM_ASM_OP_BSETI 108
#define CKB_VM_ASM_OP_CLMUL 109
#define CKB_VM_ASM_OP_CLMULH 110
#define CKB_VM_ASM_OP_CLMULR 111
#define CKB_VM_ASM_OP_CLZ 112
#define CKB_VM_ASM_OP_CLZW 113
#define CKB_VM_ASM_OP_CPOP 114
#define CKB_VM_ASM_OP_CPOPW 115
#define CKB_VM_ASM_OP_CTZ 116
#define CKB_VM_ASM_OP_CTZW 117
#define CKB_VM_ASM_OP_MAX 118
#define CKB_VM_ASM_OP_MAXU 119
#define CKB_VM_ASM_OP_MIN 120
#define CKB_VM_ASM_OP_MINU 121
#define CKB_VM_ASM_OP_ORCB 122
#define CKB_VM_ASM_OP_ORN 123
#define CKB_VM_ASM_OP_REV8 124
#define CKB_VM_ASM_OP_ROL 125
#define CKB_VM_ASM_OP_ROLW 126
#define CKB_VM_ASM_OP_ROR 127
#define CKB_VM_ASM_OP_RORI 128
#define CKB_VM_ASM_OP_RORIW 129
#define CKB_VM_ASM_OP_RORW 130
#define CKB_VM_ASM_OP_SEXTB 131
#define CKB_VM_ASM_OP_SEXTH 132
#define CKB_VM_ASM_OP_SH1ADD 133
#define CKB_VM_ASM_OP_SH1ADDUW 134
#define CKB_VM_ASM_OP_SH2ADD 135
#define CKB_VM_ASM_OP_SH2ADDUW 136
#define CKB_VM_ASM_OP_SH3ADD 137
#define CKB_VM_ASM_OP_SH3ADDUW 138
#define CKB_VM_ASM_OP_SLLIUW 139
#define CKB_VM_ASM_OP_XNOR 140
#define CKB_VM_ASM_OP_ZEXTH 141
#define CKB_VM_ASM_OP_WIDE_MUL 142
#define CKB_VM_ASM_OP_WIDE_MULU 143
#define CKB_VM_ASM_OP_WIDE_MULSU 144
#define CKB_VM_ASM_OP_WIDE_DIV 145
#define CKB_VM_ASM_OP_WIDE_DIVU 146
#define CKB_VM_ASM_OP_ADC 147
#define CKB_VM_ASM_OP_SBB 148
#define CKB_VM_ASM_OP_ADCS 149
#define CKB_VM_ASM_OP_SBBS 150
#define CKB_VM_ASM_OP_ADD3A 151
#define CKB_VM_ASM_OP_ADD3B 152
#define CKB_VM_ASM_OP_ADD3C 153
#define CKB_VM_ASM_OP_CUSTOM_LOAD_UIMM 154
#define CKB_VM_ASM_OP_CUSTOM_LOAD_IMM 155
#define CKB_VM_ASM_OP_AUIPC 156
#define CKB_VM_ASM_OP_BEQ 157
#define CKB_VM_ASM_OP_BGE 158
#define CKB_VM_ASM_OP_BGEU 159
#define CKB_VM_ASM_OP_BLT 160
#define CKB_VM_ASM_OP_BLTU 161
#define CKB_VM_ASM_OP_BNE 162
#define CKB_VM_ASM_OP_EBREAK 163
#define CKB_VM_ASM_OP_ECALL 164
#define CKB_VM_ASM_OP_FENCE 165
#define CKB_VM_ASM_OP_FENCEI 166
#define CKB_VM_ASM_OP_JAL 167
#define CKB_VM_ASM_OP_JALR_VERSION0 168
#define CKB_VM_ASM_OP_JALR_VERSION1 169
#define CKB_VM_ASM_OP_FAR_JUMP_REL 170
#define CKB_VM_ASM_OP_FAR_JUMP_ABS 171

#ifdef CKB_VM_ASM_GENERATE_LABEL_TABLES
#ifdef __APPLE__
.global _ckb_vm_asm_labels
_ckb_vm_asm_labels:
#else
.global ckb_vm_asm_labels
ckb_vm_asm_labels:
#endif
.CKB_VM_ASM_LABEL_TABLE:
	.long	.exit_slowpath - .CKB_VM_ASM_LABEL_TABLE
	.long	.exit_slowpath - .CKB_VM_ASM_LABEL_TABLE
	.long	.exit_slowpath - .CKB_VM_ASM_LABEL_TABLE
	.long	.exit_slowpath - .CKB_VM_ASM_LABEL_TABLE
	.long	.exit_slowpath - .CKB_VM_ASM_LABEL_TABLE
	.long	.exit_slowpath - .CKB_VM_ASM_LABEL_TABLE
	.long	.exit_slowpath - .CKB_VM_ASM_LABEL_TABLE
	.long	.exit_slowpath - .CKB_VM_ASM_LABEL_TABLE
	.long	.exit_slowpath - .CKB_VM_ASM_LABEL_TABLE
	.long	.exit_slowpath - .CKB_VM_ASM_LABEL_TABLE
	.long	.exit_slowpath - .CKB_VM_ASM_LABEL_TABLE
	.long	.exit_slowpath - .CKB_VM_ASM_LABEL_TABLE
	.long	.exit_slowpath - .CKB_VM_ASM_LABEL_TABLE
	.long	.exit_slowpath - .CKB_VM_ASM_LABEL_TABLE
	.long	.exit_slowpath - .CKB_VM_ASM_LABEL_TABLE
	.long	.exit_slowpath - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_UNLOADED - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ADD - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ADDI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ADDIW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ADDW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_AND - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ANDI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_DIV - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_DIVU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_DIVUW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_DIVW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LB_VERSION0 - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LB_VERSION1 - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LBU_VERSION0 - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LBU_VERSION1 - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LD_VERSION0 - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LD_VERSION1 - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LH_VERSION0 - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LH_VERSION1 - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LHU_VERSION0 - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LHU_VERSION1 - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LUI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LW_VERSION0 - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LW_VERSION1 - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LWU_VERSION0 - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LWU_VERSION1 - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_MUL - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_MULH - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_MULHSU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_MULHU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_MULW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_OR - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ORI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_REM - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_REMU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_REMUW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_REMW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SB - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SD - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SH - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SLL - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SLLI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SLLIW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SLLW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SLT - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SLTI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SLTIU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SLTU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SRA - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SRAI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SRAIW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SRAW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SRL - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SRLI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SRLIW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SRLW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SUB - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SUBW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_XOR - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_XORI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LR_W - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SC_W - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_AMOSWAP_W - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_AMOADD_W - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_AMOXOR_W - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_AMOAND_W - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_AMOOR_W - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_AMOMIN_W - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_AMOMAX_W - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_AMOMINU_W - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_AMOMAXU_W - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_LR_D - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SC_D - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_AMOSWAP_D - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_AMOADD_D - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_AMOXOR_D - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_AMOAND_D - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_AMOOR_D - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_AMOMIN_D - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_AMOMAX_D - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_AMOMINU_D - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_AMOMAXU_D - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ADDUW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ANDN - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_BCLR - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_BCLRI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_BEXT - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_BEXTI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_BINV - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_BINVI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_BSET - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_BSETI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_CLMUL - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_CLMULH - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_CLMULR - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_CLZ - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_CLZW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_CPOP - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_CPOPW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_CTZ - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_CTZW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_MAX - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_MAXU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_MIN - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_MINU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ORCB - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ORN - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_REV8 - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ROL - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ROLW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ROR - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RORI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RORIW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_RORW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SEXTB - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SEXTH - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SH1ADD - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SH1ADDUW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SH2ADD - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SH2ADDUW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SH3ADD - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SH3ADDUW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SLLIUW - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_XNOR - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ZEXTH - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_WIDE_MUL - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_WIDE_MULU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_WIDE_MULSU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_WIDE_DIV - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_WIDE_DIVU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ADC - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SBB - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ADCS - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_SBBS - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ADD3A - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ADD3B - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ADD3C - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_CUSTOM_LOAD_UIMM - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_CUSTOM_LOAD_IMM - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_AUIPC - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_BEQ - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_BGE - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_BGEU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_BLT - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_BLTU - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_BNE - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_EBREAK - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_ECALL - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_FENCE - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_FENCEI - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_JAL - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_JALR_VERSION0 - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_JALR_VERSION1 - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_FAR_JUMP_REL - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_FAR_JUMP_ABS - .CKB_VM_ASM_LABEL_TABLE
	.long	.CKB_VM_ASM_LABEL_OP_CUSTOM_TRACE_END - .CKB_VM_ASM_LABEL_TABLE
#endif /* CKB_VM_ASM_GENERATE_LABEL_TABLES */
