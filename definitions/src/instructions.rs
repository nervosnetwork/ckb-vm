// For fast decoding and cache friendly, RISC-V instruction is decoded
// into 64 bit unsigned integer in the following format:
//
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |           | rs2 | rs1 | flg | op2 | rd  | op  | R-type
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |     | rs3 | rs2 | rs1 | flg | op2 | rd  | op  | R4-type
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |   uimm    | rs2 | rs1 | flg | op2 | rd  | op  | RU-type
// +-----------+-----------------------------------+
// |    immediate    | rs1 | flg | op2 | rd  | op  | I-type
// +-----------------------------------------------+
// |    immediate    | rs1 | flg | op2 | rs2 | op  | S-type/B-type
// +-----------------+-----------------------------+
// |       immediate       | flg | op2 | rd  | op  | U-type/J-type
// +-----------------+-----------------------------+
// |           | imm | vs2 | flg | op2 | rd  | op  | VI-type
// +-----+-----+-----+-----+-----+-----+-----+-----+
//
// +flg+ here means a combination of flags, Its format is as follows:
//
// +---+---+---+---+---+---+---+---+
// | 7 | 6 | 5 | 4 | length >> 1   |
// +---+---+---+---+---+---+---+---+
//
// This way each op and register index are in full byte, accessing them
// will be much faster than the original compact form. Hence we will have
// a fast path where the interpreter loop reads instruction directly in this
// format, and a slow path where a full featured decoder decodes RISC-V
// instruction into the internal form here(much like how traces/micro-ops work.)
//
// About +op+ and +op2+:
// When the op value is 0-239, it expresses a first-level instruction under fast
// path, at this time the value of op2 is ignored.
// When the op value is 240-255, op and op2 are combined to express a
// second-level instruction under slow path.
pub type Instruction = u64;

pub type InstructionOpcode = u16;

// IMC
pub const OP_UNLOADED: InstructionOpcode = 0x00;
pub const OP_ADD: InstructionOpcode = 0x01;
pub const OP_ADDI: InstructionOpcode = 0x02;
pub const OP_ADDIW: InstructionOpcode = 0x03;
pub const OP_ADDW: InstructionOpcode = 0x04;
pub const OP_AND: InstructionOpcode = 0x05;
pub const OP_ANDI: InstructionOpcode = 0x06;
pub const OP_AUIPC: InstructionOpcode = 0x07;
pub const OP_BEQ: InstructionOpcode = 0x08;
pub const OP_BGE: InstructionOpcode = 0x09;
pub const OP_BGEU: InstructionOpcode = 0x0A;
pub const OP_BLT: InstructionOpcode = 0x0B;
pub const OP_BLTU: InstructionOpcode = 0x0C;
pub const OP_BNE: InstructionOpcode = 0x0D;
pub const OP_DIV: InstructionOpcode = 0x0E;
pub const OP_DIVU: InstructionOpcode = 0x0F;
pub const OP_DIVUW: InstructionOpcode = 0x10;
pub const OP_DIVW: InstructionOpcode = 0x11;
pub const OP_EBREAK: InstructionOpcode = 0x12;
pub const OP_ECALL: InstructionOpcode = 0x13;
pub const OP_FENCE: InstructionOpcode = 0x14;
pub const OP_FENCEI: InstructionOpcode = 0x15;
pub const OP_JAL: InstructionOpcode = 0x16;
pub const OP_JALR: InstructionOpcode = 0x17;
pub const OP_LB: InstructionOpcode = 0x18;
pub const OP_LBU: InstructionOpcode = 0x19;
pub const OP_LD: InstructionOpcode = 0x1A;
pub const OP_LH: InstructionOpcode = 0x1B;
pub const OP_LHU: InstructionOpcode = 0x1C;
pub const OP_LUI: InstructionOpcode = 0x1D;
pub const OP_LW: InstructionOpcode = 0x1E;
pub const OP_LWU: InstructionOpcode = 0x1F;
pub const OP_MUL: InstructionOpcode = 0x20;
pub const OP_MULH: InstructionOpcode = 0x21;
pub const OP_MULHSU: InstructionOpcode = 0x22;
pub const OP_MULHU: InstructionOpcode = 0x23;
pub const OP_MULW: InstructionOpcode = 0x24;
pub const OP_OR: InstructionOpcode = 0x25;
pub const OP_ORI: InstructionOpcode = 0x26;
pub const OP_REM: InstructionOpcode = 0x27;
pub const OP_REMU: InstructionOpcode = 0x28;
pub const OP_REMUW: InstructionOpcode = 0x29;
pub const OP_REMW: InstructionOpcode = 0x2A;
pub const OP_SB: InstructionOpcode = 0x2B;
pub const OP_SD: InstructionOpcode = 0x2C;
pub const OP_SH: InstructionOpcode = 0x2D;
pub const OP_SLL: InstructionOpcode = 0x2E;
pub const OP_SLLI: InstructionOpcode = 0x2F;
pub const OP_SLLIW: InstructionOpcode = 0x30;
pub const OP_SLLW: InstructionOpcode = 0x31;
pub const OP_SLT: InstructionOpcode = 0x32;
pub const OP_SLTI: InstructionOpcode = 0x33;
pub const OP_SLTIU: InstructionOpcode = 0x34;
pub const OP_SLTU: InstructionOpcode = 0x35;
pub const OP_SRA: InstructionOpcode = 0x36;
pub const OP_SRAI: InstructionOpcode = 0x37;
pub const OP_SRAIW: InstructionOpcode = 0x38;
pub const OP_SRAW: InstructionOpcode = 0x39;
pub const OP_SRL: InstructionOpcode = 0x3A;
pub const OP_SRLI: InstructionOpcode = 0x3B;
pub const OP_SRLIW: InstructionOpcode = 0x3C;
pub const OP_SRLW: InstructionOpcode = 0x3D;
pub const OP_SUB: InstructionOpcode = 0x3E;
pub const OP_SUBW: InstructionOpcode = 0x3F;
pub const OP_SW: InstructionOpcode = 0x40;
pub const OP_XOR: InstructionOpcode = 0x41;
pub const OP_XORI: InstructionOpcode = 0x42;
// B
pub const OP_ADDUW: InstructionOpcode = 0x43;
pub const OP_ANDN: InstructionOpcode = 0x44;
pub const OP_BCLR: InstructionOpcode = 0x45;
pub const OP_BCLRI: InstructionOpcode = 0x46;
pub const OP_BEXT: InstructionOpcode = 0x47;
pub const OP_BEXTI: InstructionOpcode = 0x48;
pub const OP_BINV: InstructionOpcode = 0x49;
pub const OP_BINVI: InstructionOpcode = 0x4a;
pub const OP_BSET: InstructionOpcode = 0x4b;
pub const OP_BSETI: InstructionOpcode = 0x4c;
pub const OP_CLMUL: InstructionOpcode = 0x4d;
pub const OP_CLMULH: InstructionOpcode = 0x4e;
pub const OP_CLMULR: InstructionOpcode = 0x4f;
pub const OP_CLZ: InstructionOpcode = 0x50;
pub const OP_CLZW: InstructionOpcode = 0x51;
pub const OP_CPOP: InstructionOpcode = 0x52;
pub const OP_CPOPW: InstructionOpcode = 0x53;
pub const OP_CTZ: InstructionOpcode = 0x54;
pub const OP_CTZW: InstructionOpcode = 0x55;
pub const OP_MAX: InstructionOpcode = 0x56;
pub const OP_MAXU: InstructionOpcode = 0x57;
pub const OP_MIN: InstructionOpcode = 0x58;
pub const OP_MINU: InstructionOpcode = 0x59;
pub const OP_ORCB: InstructionOpcode = 0x5a;
pub const OP_ORN: InstructionOpcode = 0x5b;
pub const OP_REV8: InstructionOpcode = 0x5c;
pub const OP_ROL: InstructionOpcode = 0x5d;
pub const OP_ROLW: InstructionOpcode = 0x5e;
pub const OP_ROR: InstructionOpcode = 0x5f;
pub const OP_RORI: InstructionOpcode = 0x60;
pub const OP_RORIW: InstructionOpcode = 0x61;
pub const OP_RORW: InstructionOpcode = 0x62;
pub const OP_SEXTB: InstructionOpcode = 0x63;
pub const OP_SEXTH: InstructionOpcode = 0x64;
pub const OP_SH1ADD: InstructionOpcode = 0x65;
pub const OP_SH1ADDUW: InstructionOpcode = 0x66;
pub const OP_SH2ADD: InstructionOpcode = 0x67;
pub const OP_SH2ADDUW: InstructionOpcode = 0x68;
pub const OP_SH3ADD: InstructionOpcode = 0x69;
pub const OP_SH3ADDUW: InstructionOpcode = 0x6a;
pub const OP_SLLIUW: InstructionOpcode = 0x6b;
pub const OP_XNOR: InstructionOpcode = 0x6c;
pub const OP_ZEXTH: InstructionOpcode = 0x6d;
// Mop
pub const OP_WIDE_MUL: InstructionOpcode = 0x6e;
pub const OP_WIDE_MULU: InstructionOpcode = 0x6f;
pub const OP_WIDE_MULSU: InstructionOpcode = 0x70;
pub const OP_WIDE_DIV: InstructionOpcode = 0x71;
pub const OP_WIDE_DIVU: InstructionOpcode = 0x72;
pub const OP_FAR_JUMP_REL: InstructionOpcode = 0x73;
pub const OP_FAR_JUMP_ABS: InstructionOpcode = 0x74;
pub const OP_LD_SIGN_EXTENDED_32_CONSTANT: InstructionOpcode = 0x75;
pub const OP_ADC: InstructionOpcode = 0x76;
pub const OP_SBB: InstructionOpcode = 0x77;
pub const OP_CUSTOM_LOAD_IMM: InstructionOpcode = 0x78;
pub const OP_CUSTOM_TRACE_END: InstructionOpcode = 0x79;
// V
pub const OP_VSETVLI: InstructionOpcode = 0x00f0;
pub const OP_VSETIVLI: InstructionOpcode = 0x01f0;
pub const OP_VSETVL: InstructionOpcode = 0x02f0;
pub const OP_VLM_V: InstructionOpcode = 0x03f0;
pub const OP_VLE8_V: InstructionOpcode = 0x04f0;
pub const OP_VLE16_V: InstructionOpcode = 0x05f0;
pub const OP_VLE32_V: InstructionOpcode = 0x06f0;
pub const OP_VLE64_V: InstructionOpcode = 0x07f0;
pub const OP_VLE128_V: InstructionOpcode = 0x08f0;
pub const OP_VLE256_V: InstructionOpcode = 0x09f0;
pub const OP_VLE512_V: InstructionOpcode = 0x0af0;
pub const OP_VLE1024_V: InstructionOpcode = 0x0bf0;
pub const OP_VSM_V: InstructionOpcode = 0x0cf0;
pub const OP_VSE8_V: InstructionOpcode = 0x0df0;
pub const OP_VSE16_V: InstructionOpcode = 0x0ef0;
pub const OP_VSE32_V: InstructionOpcode = 0x0ff0;
pub const OP_VSE64_V: InstructionOpcode = 0x10f0;
pub const OP_VSE128_V: InstructionOpcode = 0x11f0;
pub const OP_VSE256_V: InstructionOpcode = 0x12f0;
pub const OP_VSE512_V: InstructionOpcode = 0x13f0;
pub const OP_VSE1024_V: InstructionOpcode = 0x14f0;
pub const OP_VADD_VV: InstructionOpcode = 0x15f0;
pub const OP_VADD_VX: InstructionOpcode = 0x16f0;
pub const OP_VADD_VI: InstructionOpcode = 0x17f0;
pub const OP_VSUB_VV: InstructionOpcode = 0x18f0;
pub const OP_VSUB_VX: InstructionOpcode = 0x19f0;
pub const OP_VRSUB_VX: InstructionOpcode = 0x1af0;
pub const OP_VRSUB_VI: InstructionOpcode = 0x1bf0;
pub const OP_VMUL_VV: InstructionOpcode = 0x1cf0;
pub const OP_VMUL_VX: InstructionOpcode = 0x1df0;
pub const OP_VDIV_VV: InstructionOpcode = 0x1ef0;
pub const OP_VDIV_VX: InstructionOpcode = 0x1ff0;
pub const OP_VDIVU_VV: InstructionOpcode = 0x20f0;
pub const OP_VDIVU_VX: InstructionOpcode = 0x21f0;
pub const OP_VREM_VV: InstructionOpcode = 0x22f0;
pub const OP_VREM_VX: InstructionOpcode = 0x23f0;
pub const OP_VREMU_VV: InstructionOpcode = 0x24f0;
pub const OP_VREMU_VX: InstructionOpcode = 0x25f0;
pub const OP_VSLL_VV: InstructionOpcode = 0x26f0;
pub const OP_VSLL_VX: InstructionOpcode = 0x27f0;
pub const OP_VSLL_VI: InstructionOpcode = 0x28f0;
pub const OP_VSRL_VV: InstructionOpcode = 0x29f0;
pub const OP_VSRL_VX: InstructionOpcode = 0x2af0;
pub const OP_VSRL_VI: InstructionOpcode = 0x2bf0;
pub const OP_VSRA_VV: InstructionOpcode = 0x2cf0;
pub const OP_VSRA_VX: InstructionOpcode = 0x2df0;
pub const OP_VSRA_VI: InstructionOpcode = 0x2ef0;
pub const OP_VMSEQ_VV: InstructionOpcode = 0x2ff0;
pub const OP_VMSEQ_VX: InstructionOpcode = 0x30f0;
pub const OP_VMSEQ_VI: InstructionOpcode = 0x31f0;
pub const OP_VMSNE_VV: InstructionOpcode = 0x32f0;
pub const OP_VMSNE_VX: InstructionOpcode = 0x33f0;
pub const OP_VMSNE_VI: InstructionOpcode = 0x34f0;
pub const OP_VMSLTU_VV: InstructionOpcode = 0x35f0;
pub const OP_VMSLTU_VX: InstructionOpcode = 0x36f0;
pub const OP_VMSLT_VV: InstructionOpcode = 0x37f0;
pub const OP_VMSLT_VX: InstructionOpcode = 0x38f0;
pub const OP_VMSLEU_VV: InstructionOpcode = 0x39f0;
pub const OP_VMSLEU_VX: InstructionOpcode = 0x3af0;
pub const OP_VMSLEU_VI: InstructionOpcode = 0x3bf0;
pub const OP_VMSLE_VV: InstructionOpcode = 0x3cf0;
pub const OP_VMSLE_VX: InstructionOpcode = 0x3df0;
pub const OP_VMSLE_VI: InstructionOpcode = 0x3ef0;
pub const OP_VMSGTU_VX: InstructionOpcode = 0x3ff0;
pub const OP_VMSGTU_VI: InstructionOpcode = 0x40f0;
pub const OP_VMSGT_VX: InstructionOpcode = 0x41f0;
pub const OP_VMSGT_VI: InstructionOpcode = 0x42f0;
pub const OP_VMINU_VV: InstructionOpcode = 0x43f0;
pub const OP_VMINU_VX: InstructionOpcode = 0x44f0;
pub const OP_VMIN_VV: InstructionOpcode = 0x45f0;
pub const OP_VMIN_VX: InstructionOpcode = 0x46f0;
pub const OP_VMAXU_VV: InstructionOpcode = 0x47f0;
pub const OP_VMAXU_VX: InstructionOpcode = 0x48f0;
pub const OP_VMAX_VV: InstructionOpcode = 0x49f0;
pub const OP_VMAX_VX: InstructionOpcode = 0x4af0;
pub const OP_VWADDU_VV: InstructionOpcode = 0x4bf0;
pub const OP_VWADDU_VX: InstructionOpcode = 0x4cf0;
pub const OP_VWSUBU_VV: InstructionOpcode = 0x4df0;
pub const OP_VWSUBU_VX: InstructionOpcode = 0x4ef0;
pub const OP_VWADD_VV: InstructionOpcode = 0x4ff0;
pub const OP_VWADD_VX: InstructionOpcode = 0x50f0;
pub const OP_VWSUB_VV: InstructionOpcode = 0x51f0;
pub const OP_VWSUB_VX: InstructionOpcode = 0x52f0;
pub const OP_VWADDU_WV: InstructionOpcode = 0x53f0;
pub const OP_VWADDU_WX: InstructionOpcode = 0x54f0;
pub const OP_VWSUBU_WV: InstructionOpcode = 0x55f0;
pub const OP_VWSUBU_WX: InstructionOpcode = 0x56f0;
pub const OP_VWADD_WV: InstructionOpcode = 0x57f0;
pub const OP_VWADD_WX: InstructionOpcode = 0x58f0;
pub const OP_VWSUB_WV: InstructionOpcode = 0x59f0;
pub const OP_VWSUB_WX: InstructionOpcode = 0x5af0;
pub const OP_VZEXT_VF8: InstructionOpcode = 0x5bf0;
pub const OP_VSEXT_VF8: InstructionOpcode = 0x5cf0;
pub const OP_VZEXT_VF4: InstructionOpcode = 0x5df0;
pub const OP_VSEXT_VF4: InstructionOpcode = 0x5ef0;
pub const OP_VZEXT_VF2: InstructionOpcode = 0x5ff0;
pub const OP_VSEXT_VF2: InstructionOpcode = 0x60f0;
pub const OP_VADC_VVM: InstructionOpcode = 0x61f0;
pub const OP_VADC_VXM: InstructionOpcode = 0x62f0;
pub const OP_VADC_VIM: InstructionOpcode = 0x63f0;
pub const OP_VMADC_VVM: InstructionOpcode = 0x64f0;
pub const OP_VMADC_VXM: InstructionOpcode = 0x65f0;
pub const OP_VMADC_VIM: InstructionOpcode = 0x66f0;
pub const OP_VMADC_VV: InstructionOpcode = 0x67f0;
pub const OP_VMADC_VX: InstructionOpcode = 0x68f0;
pub const OP_VMADC_VI: InstructionOpcode = 0x69f0;
pub const OP_VSBC_VVM: InstructionOpcode = 0x6af0;
pub const OP_VSBC_VXM: InstructionOpcode = 0x6bf0;
pub const OP_VMSBC_VVM: InstructionOpcode = 0x6cf0;
pub const OP_VMSBC_VXM: InstructionOpcode = 0x6df0;
pub const OP_VMSBC_VV: InstructionOpcode = 0x6ef0;
pub const OP_VMSBC_VX: InstructionOpcode = 0x6ff0;
pub const OP_VAND_VV: InstructionOpcode = 0x70f0;
pub const OP_VAND_VI: InstructionOpcode = 0x71f0;
pub const OP_VAND_VX: InstructionOpcode = 0x72f0;
pub const OP_VOR_VV: InstructionOpcode = 0x73f0;
pub const OP_VOR_VX: InstructionOpcode = 0x74f0;
pub const OP_VOR_VI: InstructionOpcode = 0x75f0;
pub const OP_VXOR_VV: InstructionOpcode = 0x76f0;
pub const OP_VXOR_VX: InstructionOpcode = 0x77f0;
pub const OP_VXOR_VI: InstructionOpcode = 0x78f0;
pub const OP_VNSRL_WV: InstructionOpcode = 0x79f0;
pub const OP_VNSRL_WX: InstructionOpcode = 0x7af0;
pub const OP_VNSRL_WI: InstructionOpcode = 0x7bf0;
pub const OP_VNSRA_WV: InstructionOpcode = 0x7cf0;
pub const OP_VNSRA_WX: InstructionOpcode = 0x7df0;
pub const OP_VNSRA_WI: InstructionOpcode = 0x7ef0;
pub const OP_VMULH_VV: InstructionOpcode = 0x7ff0;
pub const OP_VMULH_VX: InstructionOpcode = 0x80f0;
pub const OP_VMULHU_VV: InstructionOpcode = 0x81f0;
pub const OP_VMULHU_VX: InstructionOpcode = 0x82f0;
pub const OP_VMULHSU_VV: InstructionOpcode = 0x83f0;
pub const OP_VMULHSU_VX: InstructionOpcode = 0x84f0;
pub const OP_VWMULU_VV: InstructionOpcode = 0x85f0;
pub const OP_VWMULU_VX: InstructionOpcode = 0x86f0;
pub const OP_VWMULSU_VV: InstructionOpcode = 0x87f0;
pub const OP_VWMULSU_VX: InstructionOpcode = 0x88f0;
pub const OP_VWMUL_VV: InstructionOpcode = 0x89f0;
pub const OP_VWMUL_VX: InstructionOpcode = 0x8af0;
pub const OP_VMV_V_V: InstructionOpcode = 0x8bf0;
pub const OP_VMV_V_X: InstructionOpcode = 0x8cf0;
pub const OP_VMV_V_I: InstructionOpcode = 0x8df0;
pub const OP_VSADDU_VV: InstructionOpcode = 0x8ef0;
pub const OP_VSADDU_VX: InstructionOpcode = 0x8ff0;
pub const OP_VSADDU_VI: InstructionOpcode = 0x90f0;
pub const OP_VSADD_VV: InstructionOpcode = 0x91f0;
pub const OP_VSADD_VX: InstructionOpcode = 0x92f0;
pub const OP_VSADD_VI: InstructionOpcode = 0x93f0;
pub const OP_VSSUBU_VV: InstructionOpcode = 0x94f0;
pub const OP_VSSUBU_VX: InstructionOpcode = 0x95f0;
pub const OP_VSSUB_VV: InstructionOpcode = 0x96f0;
pub const OP_VSSUB_VX: InstructionOpcode = 0x97f0;
pub const OP_VAADDU_VV: InstructionOpcode = 0x98f0;
pub const OP_VAADDU_VX: InstructionOpcode = 0x99f0;
pub const OP_VAADD_VV: InstructionOpcode = 0x9af0;
pub const OP_VAADD_VX: InstructionOpcode = 0x9bf0;
pub const OP_VASUBU_VV: InstructionOpcode = 0x9cf0;
pub const OP_VASUBU_VX: InstructionOpcode = 0x9df0;
pub const OP_VASUB_VV: InstructionOpcode = 0x9ef0;
pub const OP_VASUB_VX: InstructionOpcode = 0x9ff0;
pub const OP_VMV1R_V: InstructionOpcode = 0xa0f0;
pub const OP_VMV2R_V: InstructionOpcode = 0xa1f0;
pub const OP_VMV4R_V: InstructionOpcode = 0xa2f0;
pub const OP_VMV8R_V: InstructionOpcode = 0xa3f0;
pub const OP_VFIRST_M: InstructionOpcode = 0xa4f0;
pub const OP_VMAND_MM: InstructionOpcode = 0xa5f0;
pub const OP_VMNAND_MM: InstructionOpcode = 0xa6f0;
pub const OP_VMANDNOT_MM: InstructionOpcode = 0xa7f0;
pub const OP_VMXOR_MM: InstructionOpcode = 0xa8f0;
pub const OP_VMOR_MM: InstructionOpcode = 0xa9f0;
pub const OP_VMNOR_MM: InstructionOpcode = 0xaaf0;
pub const OP_VMORNOT_MM: InstructionOpcode = 0xabf0;
pub const OP_VMXNOR_MM: InstructionOpcode = 0xacf0;
pub const OP_VLSE8_V: InstructionOpcode = 0xadf0;
pub const OP_VLSE16_V: InstructionOpcode = 0xaef0;
pub const OP_VLSE32_V: InstructionOpcode = 0xaff0;
pub const OP_VLSE64_V: InstructionOpcode = 0xb0f0;
pub const OP_VLSE128_V: InstructionOpcode = 0xb1f0;
pub const OP_VLSE256_V: InstructionOpcode = 0xb2f0;
pub const OP_VLSE512_V: InstructionOpcode = 0xb3f0;
pub const OP_VLSE1024_V: InstructionOpcode = 0xb4f0;
pub const OP_VSSE8_V: InstructionOpcode = 0xb5f0;
pub const OP_VSSE16_V: InstructionOpcode = 0xb6f0;
pub const OP_VSSE32_V: InstructionOpcode = 0xb7f0;
pub const OP_VSSE64_V: InstructionOpcode = 0xb8f0;
pub const OP_VSSE128_V: InstructionOpcode = 0xb9f0;
pub const OP_VSSE256_V: InstructionOpcode = 0xbaf0;
pub const OP_VSSE512_V: InstructionOpcode = 0xbbf0;
pub const OP_VSSE1024_V: InstructionOpcode = 0xbcf0;
pub const OP_VLUXEI8_V: InstructionOpcode = 0xbdf0;
pub const OP_VLUXEI16_V: InstructionOpcode = 0xbef0;
pub const OP_VLUXEI32_V: InstructionOpcode = 0xbff0;
pub const OP_VLUXEI64_V: InstructionOpcode = 0xc0f0;
pub const OP_VLOXEI8_V: InstructionOpcode = 0xc1f0;
pub const OP_VLOXEI16_V: InstructionOpcode = 0xc2f0;
pub const OP_VLOXEI32_V: InstructionOpcode = 0xc3f0;
pub const OP_VLOXEI64_V: InstructionOpcode = 0xc4f0;
pub const OP_VSUXEI8_V: InstructionOpcode = 0xc5f0;
pub const OP_VSUXEI16_V: InstructionOpcode = 0xc6f0;
pub const OP_VSUXEI32_V: InstructionOpcode = 0xc7f0;
pub const OP_VSUXEI64_V: InstructionOpcode = 0xc8f0;
pub const OP_VSOXEI8_V: InstructionOpcode = 0xc9f0;
pub const OP_VSOXEI16_V: InstructionOpcode = 0xcaf0;
pub const OP_VSOXEI32_V: InstructionOpcode = 0xcbf0;
pub const OP_VSOXEI64_V: InstructionOpcode = 0xccf0;
pub const OP_VL1RE8_V: InstructionOpcode = 0xcdf0;
pub const OP_VL1RE16_V: InstructionOpcode = 0xcef0;
pub const OP_VL1RE32_V: InstructionOpcode = 0xcff0;
pub const OP_VL1RE64_V: InstructionOpcode = 0xd0f0;
pub const OP_VL2RE8_V: InstructionOpcode = 0xd1f0;
pub const OP_VL2RE16_V: InstructionOpcode = 0xd2f0;
pub const OP_VL2RE32_V: InstructionOpcode = 0xd3f0;
pub const OP_VL2RE64_V: InstructionOpcode = 0xd4f0;
pub const OP_VL4RE8_V: InstructionOpcode = 0xd5f0;
pub const OP_VL4RE16_V: InstructionOpcode = 0xd6f0;
pub const OP_VL4RE32_V: InstructionOpcode = 0xd7f0;
pub const OP_VL4RE64_V: InstructionOpcode = 0xd8f0;
pub const OP_VL8RE8_V: InstructionOpcode = 0xd9f0;
pub const OP_VL8RE16_V: InstructionOpcode = 0xdaf0;
pub const OP_VL8RE32_V: InstructionOpcode = 0xdbf0;
pub const OP_VL8RE64_V: InstructionOpcode = 0xdcf0;
pub const OP_VS1R_V: InstructionOpcode = 0xddf0;
pub const OP_VS2R_V: InstructionOpcode = 0xdef0;
pub const OP_VS4R_V: InstructionOpcode = 0xdff0;
pub const OP_VS8R_V: InstructionOpcode = 0xe0f0;
pub const OP_VMACC_VV: InstructionOpcode = 0xe1f0;
pub const OP_VMACC_VX: InstructionOpcode = 0xe2f0;
pub const OP_VNMSAC_VV: InstructionOpcode = 0xe3f0;
pub const OP_VNMSAC_VX: InstructionOpcode = 0xe4f0;
pub const OP_VMADD_VV: InstructionOpcode = 0xe5f0;
pub const OP_VMADD_VX: InstructionOpcode = 0xe6f0;
pub const OP_VNMSUB_VV: InstructionOpcode = 0xe7f0;
pub const OP_VNMSUB_VX: InstructionOpcode = 0xe8f0;
pub const OP_VSSRL_VV: InstructionOpcode = 0xe9f0;
pub const OP_VSSRL_VX: InstructionOpcode = 0xeaf0;
pub const OP_VSSRL_VI: InstructionOpcode = 0xebf0;
pub const OP_VSSRA_VV: InstructionOpcode = 0xecf0;
pub const OP_VSSRA_VX: InstructionOpcode = 0xedf0;
pub const OP_VSSRA_VI: InstructionOpcode = 0xeef0;
pub const OP_VSMUL_VV: InstructionOpcode = 0xeff0;
pub const OP_VSMUL_VX: InstructionOpcode = 0xf0f0;
pub const OP_VWMACCU_VV: InstructionOpcode = 0xf1f0;
pub const OP_VWMACCU_VX: InstructionOpcode = 0xf2f0;
pub const OP_VWMACC_VV: InstructionOpcode = 0xf3f0;
pub const OP_VWMACC_VX: InstructionOpcode = 0xf4f0;
pub const OP_VWMACCSU_VV: InstructionOpcode = 0xf5f0;
pub const OP_VWMACCSU_VX: InstructionOpcode = 0xf6f0;
pub const OP_VWMACCUS_VX: InstructionOpcode = 0xf7f0;
pub const OP_VMERGE_VVM: InstructionOpcode = 0xf8f0;
pub const OP_VMERGE_VXM: InstructionOpcode = 0xf9f0;
pub const OP_VMERGE_VIM: InstructionOpcode = 0xfaf0;
pub const OP_VNCLIPU_WV: InstructionOpcode = 0xfbf0;
pub const OP_VNCLIPU_WX: InstructionOpcode = 0xfcf0;
pub const OP_VNCLIPU_WI: InstructionOpcode = 0xfdf0;
pub const OP_VNCLIP_WV: InstructionOpcode = 0xfef0;
pub const OP_VNCLIP_WX: InstructionOpcode = 0xfff0;
pub const OP_VNCLIP_WI: InstructionOpcode = 0x00f1;
pub const OP_VREDSUM_VS: InstructionOpcode = 0x01f1;
pub const OP_VREDAND_VS: InstructionOpcode = 0x02f1;
pub const OP_VREDOR_VS: InstructionOpcode = 0x03f1;
pub const OP_VREDXOR_VS: InstructionOpcode = 0x04f1;
pub const OP_VREDMINU_VS: InstructionOpcode = 0x05f1;
pub const OP_VREDMIN_VS: InstructionOpcode = 0x06f1;
pub const OP_VREDMAXU_VS: InstructionOpcode = 0x07f1;
pub const OP_VREDMAX_VS: InstructionOpcode = 0x08f1;
pub const OP_VWREDSUMU_VS: InstructionOpcode = 0x09f1;
pub const OP_VWREDSUM_VS: InstructionOpcode = 0x0af1;
pub const OP_VCPOP_M: InstructionOpcode = 0x0bf1;
pub const OP_VMSBF_M: InstructionOpcode = 0x0cf1;
pub const OP_VMSOF_M: InstructionOpcode = 0x0df1;
pub const OP_VMSIF_M: InstructionOpcode = 0x0ef1;
pub const OP_VIOTA_M: InstructionOpcode = 0x0ff1;
pub const OP_VID_V: InstructionOpcode = 0x10f1;
pub const OP_VMV_X_S: InstructionOpcode = 0x11f1;
pub const OP_VMV_S_X: InstructionOpcode = 0x12f1;
pub const OP_VCOMPRESS_VM: InstructionOpcode = 0x13f1;
pub const OP_VSLIDE1UP_VX: InstructionOpcode = 0x14f1;
pub const OP_VSLIDEUP_VX: InstructionOpcode = 0x15f1;
pub const OP_VSLIDEUP_VI: InstructionOpcode = 0x16f1;
pub const OP_VSLIDE1DOWN_VX: InstructionOpcode = 0x17f1;
pub const OP_VSLIDEDOWN_VX: InstructionOpcode = 0x18f1;
pub const OP_VSLIDEDOWN_VI: InstructionOpcode = 0x19f1;
pub const OP_VRGATHER_VX: InstructionOpcode = 0x1af1;
pub const OP_VRGATHER_VV: InstructionOpcode = 0x1bf1;
pub const OP_VRGATHEREI16_VV: InstructionOpcode = 0x1cf1;
pub const OP_VRGATHER_VI: InstructionOpcode = 0x1df1;

pub const MINIMAL_LEVEL1_OPCODE: InstructionOpcode = OP_UNLOADED;
pub const MAXIMUM_LEVEL1_OPCODE: InstructionOpcode = OP_CUSTOM_TRACE_END;
pub const LEVEL2_V_OPCODE: InstructionOpcode = 0xf0;
pub const MINIMAL_LEVEL2_OPCODE: InstructionOpcode = 0x00;
pub const MAXIMUM_LEVEL2_OPCODE: InstructionOpcode = 0x11d;

pub const INSTRUCTION_OPCODE_NAMES_LEVEL1: [&str; MAXIMUM_LEVEL1_OPCODE as usize + 1] = [
    "UNLOADED",
    "ADD",
    "ADDI",
    "ADDIW",
    "ADDW",
    "AND",
    "ANDI",
    "AUIPC",
    "BEQ",
    "BGE",
    "BGEU",
    "BLT",
    "BLTU",
    "BNE",
    "DIV",
    "DIVU",
    "DIVUW",
    "DIVW",
    "EBREAK",
    "ECALL",
    "FENCE",
    "FENCEI",
    "JAL",
    "JALR",
    "LB",
    "LBU",
    "LD",
    "LH",
    "LHU",
    "LUI",
    "LW",
    "LWU",
    "MUL",
    "MULH",
    "MULHSU",
    "MULHU",
    "MULW",
    "OR",
    "ORI",
    "REM",
    "REMU",
    "REMUW",
    "REMW",
    "SB",
    "SD",
    "SH",
    "SLL",
    "SLLI",
    "SLLIW",
    "SLLW",
    "SLT",
    "SLTI",
    "SLTIU",
    "SLTU",
    "SRA",
    "SRAI",
    "SRAIW",
    "SRAW",
    "SRL",
    "SRLI",
    "SRLIW",
    "SRLW",
    "SUB",
    "SUBW",
    "SW",
    "XOR",
    "XORI",
    "ADDUW",
    "ANDN",
    "BCLR",
    "BCLRI",
    "BEXT",
    "BEXTI",
    "BINV",
    "BINVI",
    "BSET",
    "BSETI",
    "CLMUL",
    "CLMULH",
    "CLMULR",
    "CLZ",
    "CLZW",
    "CPOP",
    "CPOPW",
    "CTZ",
    "CTZW",
    "MAX",
    "MAXU",
    "MIN",
    "MINU",
    "ORCB",
    "ORN",
    "REV8",
    "ROL",
    "ROLW",
    "ROR",
    "RORI",
    "RORIW",
    "RORW",
    "SEXTB",
    "SEXTH",
    "SH1ADD",
    "SH1ADDUW",
    "SH2ADD",
    "SH2ADDUW",
    "SH3ADD",
    "SH3ADDUW",
    "SLLIUW",
    "XNOR",
    "ZEXTH",
    "WIDE_MUL",
    "WIDE_MULU",
    "WIDE_MULSU",
    "WIDE_DIV",
    "WIDE_DIVU",
    "FAR_JUMP_REL",
    "FAR_JUMP_ABS",
    "LD_SIGN_EXTENDED_32_CONSTANT",
    "ADC",
    "SBB",
    "CUSTOM_LOAD_IMM",
    "CUSTOM_TRACE_END",
];

pub const INSTRUCTION_OPCODE_NAMES_LEVEL2: [&str;
    (MAXIMUM_LEVEL2_OPCODE - MINIMAL_LEVEL2_OPCODE) as usize + 1] = [
    "VSETVLI",
    "VSETIVLI",
    "VSETVL",
    "VLM_V",
    "VLE8_V",
    "VLE16_V",
    "VLE32_V",
    "VLE64_V",
    "VLE128_V",
    "VLE256_V",
    "VLE512_V",
    "VLE1024_V",
    "VSM_V",
    "VSE8_V",
    "VSE16_V",
    "VSE32_V",
    "VSE64_V",
    "VSE128_V",
    "VSE256_V",
    "VSE512_V",
    "VSE1024_V",
    "VADD_VV",
    "VADD_VX",
    "VADD_VI",
    "VSUB_VV",
    "VSUB_VX",
    "VRSUB_VX",
    "VRSUB_VI",
    "VMUL_VV",
    "VMUL_VX",
    "VDIV_VV",
    "VDIV_VX",
    "VDIVU_VV",
    "VDIVU_VX",
    "VREM_VV",
    "VREM_VX",
    "VREMU_VV",
    "VREMU_VX",
    "VSLL_VV",
    "VSLL_VX",
    "VSLL_VI",
    "VSRL_VV",
    "VSRL_VX",
    "VSRL_VI",
    "VSRA_VV",
    "VSRA_VX",
    "VSRA_VI",
    "VMSEQ_VV",
    "VMSEQ_VX",
    "VMSEQ_VI",
    "VMSNE_VV",
    "VMSNE_VX",
    "VMSNE_VI",
    "VMSLTU_VV",
    "VMSLTU_VX",
    "VMSLT_VV",
    "VMSLT_VX",
    "VMSLEU_VV",
    "VMSLEU_VX",
    "VMSLEU_VI",
    "VMSLE_VV",
    "VMSLE_VX",
    "VMSLE_VI",
    "VMSGTU_VX",
    "VMSGTU_VI",
    "VMSGT_VX",
    "VMSGT_VI",
    "VMINU_VV",
    "VMINU_VX",
    "VMIN_VV",
    "VMIN_VX",
    "VMAXU_VV",
    "VMAXU_VX",
    "VMAX_VV",
    "VMAX_VX",
    "VWADDU_VV",
    "VWADDU_VX",
    "VWSUBU_VV",
    "VWSUBU_VX",
    "VWADD_VV",
    "VWADD_VX",
    "VWSUB_VV",
    "VWSUB_VX",
    "VWADDU_WV",
    "VWADDU_WX",
    "VWSUBU_WV",
    "VWSUBU_WX",
    "VWADD_WV",
    "VWADD_WX",
    "VWSUB_WV",
    "VWSUB_WX",
    "VZEXT_VF8",
    "VSEXT_VF8",
    "VZEXT_VF4",
    "VSEXT_VF4",
    "VZEXT_VF2",
    "VSEXT_VF2",
    "VADC_VVM",
    "VADC_VXM",
    "VADC_VIM",
    "VMADC_VVM",
    "VMADC_VXM",
    "VMADC_VIM",
    "VMADC_VV",
    "VMADC_VX",
    "VMADC_VI",
    "VSBC_VVM",
    "VSBC_VXM",
    "VMSBC_VVM",
    "VMSBC_VXM",
    "VMSBC_VV",
    "VMSBC_VX",
    "VAND_VV",
    "VAND_VI",
    "VAND_VX",
    "VOR_VV",
    "VOR_VX",
    "VOR_VI",
    "VXOR_VV",
    "VXOR_VX",
    "VXOR_VI",
    "VNSRL_WV",
    "VNSRL_WX",
    "VNSRL_WI",
    "VNSRA_WV",
    "VNSRA_WX",
    "VNSRA_WI",
    "VMULH_VV",
    "VMULH_VX",
    "VMULHU_VV",
    "VMULHU_VX",
    "VMULHSU_VV",
    "VMULHSU_VX",
    "VWMULU_VV",
    "VWMULU_VX",
    "VWMULSU_VV",
    "VWMULSU_VX",
    "VWMUL_VV",
    "VWMUL_VX",
    "VMV_VV",
    "VMV_VX",
    "VMV_VI",
    "VSADDU_VV",
    "VSADDU_VX",
    "VSADDU_VI",
    "VSADD_VV",
    "VSADD_VX",
    "VSADD_VI",
    "VSSUBU_VV",
    "VSSUBU_VX",
    "VSSUB_VV",
    "VSSUB_VX",
    "VAADDU_VV",
    "VAADDU_VX",
    "VAADD_VV",
    "VAADD_VX",
    "VASUBU_VV",
    "VASUBU_VX",
    "VASUB_VV",
    "VASUB_VX",
    "VMV1R_V",
    "VMV2R_V",
    "VMV4R_V",
    "VMV8R_V",
    "VFIRST_M",
    "VMAND_MM",
    "VMNAND_MM",
    "VMANDNOT_MM",
    "VMXOR_MM",
    "VMOR_MM",
    "VMNOR_MM",
    "VMORNOT_MM",
    "VMXNOR_MM",
    "VLSE8_V",
    "VLSE16_V",
    "VLSE32_V",
    "VLSE64_V",
    "VLSE128_V",
    "VLSE256_V",
    "VLSE512_V",
    "VLSE1024_V",
    "VSSE8_V",
    "VSSE16_V",
    "VSSE32_V",
    "VSSE64_V",
    "VSSE128_V",
    "VSSE256_V",
    "VSSE512_V",
    "VSSE1024_V",
    "VLUXEI8_V",
    "VLUXEI16_V",
    "VLUXEI32_V",
    "VLUXEI64_V",
    "VLOXEI8_V",
    "VLOXEI16_V",
    "VLOXEI32_V",
    "VLOXEI64_V",
    "VSUXEI8_V",
    "VSUXEI16_V",
    "VSUXEI32_V",
    "VSUXEI64_V",
    "VSOXEI8_V",
    "VSOXEI16_V",
    "VSOXEI32_V",
    "VSOXEI64_V",
    "VL1RE8_V",
    "VL1RE16_V",
    "VL1RE32_V",
    "VL1RE64_V",
    "VL2RE8_V",
    "VL2RE16_V",
    "VL2RE32_V",
    "VL2RE64_V",
    "VL4RE8_V",
    "VL4RE16_V",
    "VL4RE32_V",
    "VL4RE64_V",
    "VL8RE8_V",
    "VL8RE16_V",
    "VL8RE32_V",
    "VL8RE64_V",
    "VS1R_V",
    "VS2R_V",
    "VS4R_V",
    "VS8R_V",
    "VMACC_VV",
    "VMACC_VX",
    "VNMSAC_VV",
    "VNMSAC_VX",
    "VMADD_VV",
    "VMADD_VX",
    "VNMSUB_VV",
    "VNMSUB_VX",
    "VSSRL_VV",
    "VSSRL_VX",
    "VSSRL_VI",
    "VSSRA_VV",
    "VSSRA_VX",
    "VSSRA_VI",
    "VSMUL_VV",
    "VSMUL_VX",
    "VWMACCU_VV",
    "VWMACCU_VX",
    "VWMACC_VV",
    "VWMACC_VX",
    "VWMACCSU_VV",
    "VWMACCSU_VX",
    "VWMACCUS_VX",
    "VMERGE_VVM",
    "VMERGE_VXM",
    "VMERGE_VIM",
    "VNCLIPU_WV",
    "VNCLIPU_WX",
    "VNCLIPU_WI",
    "VNCLIP_WV",
    "VNCLIP_WX",
    "VNCLIP_WI",
    "VREDSUM_VS",
    "VREDAND_VS",
    "VREDOR_VS",
    "VREDXOR_VS",
    "VREDMINU_VS",
    "VREDMIN_VS",
    "VREDMAXU_VS",
    "VREDMAX_VS",
    "VWREDSUMU_VS",
    "VWREDSUM_VS",
    "VCPOP_M",
    "VMSBF_M",
    "VMSOF_M",
    "VMSIF_M",
    "VIOTA_M",
    "VID_V",
    "VMV_X_S",
    "VMV_S_X",
    "VCOMPRESS_VM",
    "VSLIDE1UP_VX",
    "VSLIDEUP_VX",
    "VSLIDEUP_VI",
    "VSLIDE1DOWN_VX",
    "VSLIDEDOWN_VX",
    "VSLIDEDOWN_VI",
    "VRGATHER_VX",
    "VRGATHER_VV",
    "VRGATHEREI16_VV",
    "VRGATHER_VI",
];

pub fn instruction_opcode_name(i: InstructionOpcode) -> &'static str {
    if i >= 0xf0 {
        INSTRUCTION_OPCODE_NAMES_LEVEL2[(((i & 0xf) << 8) | (i >> 8)) as usize]
    } else {
        INSTRUCTION_OPCODE_NAMES_LEVEL1[i as usize]
    }
}
