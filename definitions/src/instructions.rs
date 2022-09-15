// For fast decoding and cache friendly, RISC-V instruction is decoded
// into 64 bit unsigned integer in the following format:
//
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |           | rs2 | rs1 | flg | rd  |    op     | R-type
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |     | rs3 | rs2 | rs1 | flg | rd  |    op     | R4-type
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |   uimm    | rs2 | rs1 | flg | rd  |    op     | RU-type
// +-----------+-----------------------------------+
// |    immediate    | rs1 | flg | rd  |    op     | I-type
// +-----------------------------------------------+
// |    immediate    | rs1 | flg | rs2 |    op     | S-type/B-type
// +-----------------+-----------------------------+
// |       immediate       | flg | rd  |    op     | U-type/J-type
// +-----------------+-----------------------------+
// |           | imm | vs2 | flg | rd  |    op     | VI-type
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
pub const OP_DIV: InstructionOpcode = 0x07;
pub const OP_DIVU: InstructionOpcode = 0x08;
pub const OP_DIVUW: InstructionOpcode = 0x09;
pub const OP_DIVW: InstructionOpcode = 0x0a;
pub const OP_FENCE: InstructionOpcode = 0x0b;
pub const OP_FENCEI: InstructionOpcode = 0x0c;
pub const OP_LB: InstructionOpcode = 0x0d;
pub const OP_LBU: InstructionOpcode = 0x0e;
pub const OP_LD: InstructionOpcode = 0x0f;
pub const OP_LH: InstructionOpcode = 0x10;
pub const OP_LHU: InstructionOpcode = 0x11;
pub const OP_LUI: InstructionOpcode = 0x12;
pub const OP_LW: InstructionOpcode = 0x13;
pub const OP_LWU: InstructionOpcode = 0x14;
pub const OP_MUL: InstructionOpcode = 0x15;
pub const OP_MULH: InstructionOpcode = 0x16;
pub const OP_MULHSU: InstructionOpcode = 0x17;
pub const OP_MULHU: InstructionOpcode = 0x18;
pub const OP_MULW: InstructionOpcode = 0x19;
pub const OP_OR: InstructionOpcode = 0x1a;
pub const OP_ORI: InstructionOpcode = 0x1b;
pub const OP_REM: InstructionOpcode = 0x1c;
pub const OP_REMU: InstructionOpcode = 0x1d;
pub const OP_REMUW: InstructionOpcode = 0x1e;
pub const OP_REMW: InstructionOpcode = 0x1f;
pub const OP_SB: InstructionOpcode = 0x20;
pub const OP_SD: InstructionOpcode = 0x21;
pub const OP_SH: InstructionOpcode = 0x22;
pub const OP_SLL: InstructionOpcode = 0x23;
pub const OP_SLLI: InstructionOpcode = 0x24;
pub const OP_SLLIW: InstructionOpcode = 0x25;
pub const OP_SLLW: InstructionOpcode = 0x26;
pub const OP_SLT: InstructionOpcode = 0x27;
pub const OP_SLTI: InstructionOpcode = 0x28;
pub const OP_SLTIU: InstructionOpcode = 0x29;
pub const OP_SLTU: InstructionOpcode = 0x2a;
pub const OP_SRA: InstructionOpcode = 0x2b;
pub const OP_SRAI: InstructionOpcode = 0x2c;
pub const OP_SRAIW: InstructionOpcode = 0x2d;
pub const OP_SRAW: InstructionOpcode = 0x2e;
pub const OP_SRL: InstructionOpcode = 0x2f;
pub const OP_SRLI: InstructionOpcode = 0x30;
pub const OP_SRLIW: InstructionOpcode = 0x31;
pub const OP_SRLW: InstructionOpcode = 0x32;
pub const OP_SUB: InstructionOpcode = 0x33;
pub const OP_SUBW: InstructionOpcode = 0x34;
pub const OP_SW: InstructionOpcode = 0x35;
pub const OP_XOR: InstructionOpcode = 0x36;
pub const OP_XORI: InstructionOpcode = 0x37;
// B
pub const OP_ADDUW: InstructionOpcode = 0x38;
pub const OP_ANDN: InstructionOpcode = 0x39;
pub const OP_BCLR: InstructionOpcode = 0x3a;
pub const OP_BCLRI: InstructionOpcode = 0x3b;
pub const OP_BEXT: InstructionOpcode = 0x3c;
pub const OP_BEXTI: InstructionOpcode = 0x3d;
pub const OP_BINV: InstructionOpcode = 0x3e;
pub const OP_BINVI: InstructionOpcode = 0x3f;
pub const OP_BSET: InstructionOpcode = 0x40;
pub const OP_BSETI: InstructionOpcode = 0x41;
pub const OP_CLMUL: InstructionOpcode = 0x42;
pub const OP_CLMULH: InstructionOpcode = 0x43;
pub const OP_CLMULR: InstructionOpcode = 0x44;
pub const OP_CLZ: InstructionOpcode = 0x45;
pub const OP_CLZW: InstructionOpcode = 0x46;
pub const OP_CPOP: InstructionOpcode = 0x47;
pub const OP_CPOPW: InstructionOpcode = 0x48;
pub const OP_CTZ: InstructionOpcode = 0x49;
pub const OP_CTZW: InstructionOpcode = 0x4a;
pub const OP_MAX: InstructionOpcode = 0x4b;
pub const OP_MAXU: InstructionOpcode = 0x4c;
pub const OP_MIN: InstructionOpcode = 0x4d;
pub const OP_MINU: InstructionOpcode = 0x4e;
pub const OP_ORCB: InstructionOpcode = 0x4f;
pub const OP_ORN: InstructionOpcode = 0x50;
pub const OP_REV8: InstructionOpcode = 0x51;
pub const OP_ROL: InstructionOpcode = 0x52;
pub const OP_ROLW: InstructionOpcode = 0x53;
pub const OP_ROR: InstructionOpcode = 0x54;
pub const OP_RORI: InstructionOpcode = 0x55;
pub const OP_RORIW: InstructionOpcode = 0x56;
pub const OP_RORW: InstructionOpcode = 0x57;
pub const OP_SEXTB: InstructionOpcode = 0x58;
pub const OP_SEXTH: InstructionOpcode = 0x59;
pub const OP_SH1ADD: InstructionOpcode = 0x5a;
pub const OP_SH1ADDUW: InstructionOpcode = 0x5b;
pub const OP_SH2ADD: InstructionOpcode = 0x5c;
pub const OP_SH2ADDUW: InstructionOpcode = 0x5d;
pub const OP_SH3ADD: InstructionOpcode = 0x5e;
pub const OP_SH3ADDUW: InstructionOpcode = 0x5f;
pub const OP_SLLIUW: InstructionOpcode = 0x60;
pub const OP_XNOR: InstructionOpcode = 0x61;
pub const OP_ZEXTH: InstructionOpcode = 0x62;
// Mop
pub const OP_WIDE_MUL: InstructionOpcode = 0x63;
pub const OP_WIDE_MULU: InstructionOpcode = 0x64;
pub const OP_WIDE_MULSU: InstructionOpcode = 0x65;
pub const OP_WIDE_DIV: InstructionOpcode = 0x66;
pub const OP_WIDE_DIVU: InstructionOpcode = 0x67;
pub const OP_ADC: InstructionOpcode = 0x68;
pub const OP_SBB: InstructionOpcode = 0x69;
pub const OP_CUSTOM_LOAD_UIMM: InstructionOpcode = 0x6a;
pub const OP_CUSTOM_LOAD_IMM: InstructionOpcode = 0x6b;
// Basic block ends
pub const OP_AUIPC: InstructionOpcode = 0x6c;
pub const OP_BEQ: InstructionOpcode = 0x6d;
pub const OP_BGE: InstructionOpcode = 0x6e;
pub const OP_BGEU: InstructionOpcode = 0x6f;
pub const OP_BLT: InstructionOpcode = 0x70;
pub const OP_BLTU: InstructionOpcode = 0x71;
pub const OP_BNE: InstructionOpcode = 0x72;
pub const OP_EBREAK: InstructionOpcode = 0x73;
pub const OP_ECALL: InstructionOpcode = 0x74;
pub const OP_JAL: InstructionOpcode = 0x75;
pub const OP_JALR: InstructionOpcode = 0x76;
pub const OP_FAR_JUMP_REL: InstructionOpcode = 0x77;
pub const OP_FAR_JUMP_ABS: InstructionOpcode = 0x78;
pub const OP_CUSTOM_TRACE_END: InstructionOpcode = 0x79;
// V
pub const OP_VSETVLI: InstructionOpcode = 0x007a;
pub const OP_VSETIVLI: InstructionOpcode = 0x007b;
pub const OP_VSETVL: InstructionOpcode = 0x007c;
pub const OP_VLM_V: InstructionOpcode = 0x007d;
pub const OP_VLE8_V: InstructionOpcode = 0x007e;
pub const OP_VLE16_V: InstructionOpcode = 0x007f;
pub const OP_VLE32_V: InstructionOpcode = 0x0080;
pub const OP_VLE64_V: InstructionOpcode = 0x0081;
pub const OP_VLE128_V: InstructionOpcode = 0x0082;
pub const OP_VLE256_V: InstructionOpcode = 0x0083;
pub const OP_VLE512_V: InstructionOpcode = 0x0084;
pub const OP_VLE1024_V: InstructionOpcode = 0x0085;
pub const OP_VSM_V: InstructionOpcode = 0x0086;
pub const OP_VSE8_V: InstructionOpcode = 0x0087;
pub const OP_VSE16_V: InstructionOpcode = 0x0088;
pub const OP_VSE32_V: InstructionOpcode = 0x0089;
pub const OP_VSE64_V: InstructionOpcode = 0x008a;
pub const OP_VSE128_V: InstructionOpcode = 0x008b;
pub const OP_VSE256_V: InstructionOpcode = 0x008c;
pub const OP_VSE512_V: InstructionOpcode = 0x008d;
pub const OP_VSE1024_V: InstructionOpcode = 0x008e;
pub const OP_VADD_VV: InstructionOpcode = 0x008f;
pub const OP_VADD_VX: InstructionOpcode = 0x0090;
pub const OP_VADD_VI: InstructionOpcode = 0x0091;
pub const OP_VSUB_VV: InstructionOpcode = 0x0092;
pub const OP_VSUB_VX: InstructionOpcode = 0x0093;
pub const OP_VRSUB_VX: InstructionOpcode = 0x0094;
pub const OP_VRSUB_VI: InstructionOpcode = 0x0095;
pub const OP_VMUL_VV: InstructionOpcode = 0x0096;
pub const OP_VMUL_VX: InstructionOpcode = 0x0097;
pub const OP_VDIV_VV: InstructionOpcode = 0x0098;
pub const OP_VDIV_VX: InstructionOpcode = 0x0099;
pub const OP_VDIVU_VV: InstructionOpcode = 0x009a;
pub const OP_VDIVU_VX: InstructionOpcode = 0x009b;
pub const OP_VREM_VV: InstructionOpcode = 0x009c;
pub const OP_VREM_VX: InstructionOpcode = 0x009d;
pub const OP_VREMU_VV: InstructionOpcode = 0x009e;
pub const OP_VREMU_VX: InstructionOpcode = 0x009f;
pub const OP_VSLL_VV: InstructionOpcode = 0x00a0;
pub const OP_VSLL_VX: InstructionOpcode = 0x00a1;
pub const OP_VSLL_VI: InstructionOpcode = 0x00a2;
pub const OP_VSRL_VV: InstructionOpcode = 0x00a3;
pub const OP_VSRL_VX: InstructionOpcode = 0x00a4;
pub const OP_VSRL_VI: InstructionOpcode = 0x00a5;
pub const OP_VSRA_VV: InstructionOpcode = 0x00a6;
pub const OP_VSRA_VX: InstructionOpcode = 0x00a7;
pub const OP_VSRA_VI: InstructionOpcode = 0x00a8;
pub const OP_VMSEQ_VV: InstructionOpcode = 0x00a9;
pub const OP_VMSEQ_VX: InstructionOpcode = 0x00aa;
pub const OP_VMSEQ_VI: InstructionOpcode = 0x00ab;
pub const OP_VMSNE_VV: InstructionOpcode = 0x00ac;
pub const OP_VMSNE_VX: InstructionOpcode = 0x00ad;
pub const OP_VMSNE_VI: InstructionOpcode = 0x00ae;
pub const OP_VMSLTU_VV: InstructionOpcode = 0x00af;
pub const OP_VMSLTU_VX: InstructionOpcode = 0x00b0;
pub const OP_VMSLT_VV: InstructionOpcode = 0x00b1;
pub const OP_VMSLT_VX: InstructionOpcode = 0x00b2;
pub const OP_VMSLEU_VV: InstructionOpcode = 0x00b3;
pub const OP_VMSLEU_VX: InstructionOpcode = 0x00b4;
pub const OP_VMSLEU_VI: InstructionOpcode = 0x00b5;
pub const OP_VMSLE_VV: InstructionOpcode = 0x00b6;
pub const OP_VMSLE_VX: InstructionOpcode = 0x00b7;
pub const OP_VMSLE_VI: InstructionOpcode = 0x00b8;
pub const OP_VMSGTU_VX: InstructionOpcode = 0x00b9;
pub const OP_VMSGTU_VI: InstructionOpcode = 0x00ba;
pub const OP_VMSGT_VX: InstructionOpcode = 0x00bb;
pub const OP_VMSGT_VI: InstructionOpcode = 0x00bc;
pub const OP_VMINU_VV: InstructionOpcode = 0x00bd;
pub const OP_VMINU_VX: InstructionOpcode = 0x00be;
pub const OP_VMIN_VV: InstructionOpcode = 0x00bf;
pub const OP_VMIN_VX: InstructionOpcode = 0x00c0;
pub const OP_VMAXU_VV: InstructionOpcode = 0x00c1;
pub const OP_VMAXU_VX: InstructionOpcode = 0x00c2;
pub const OP_VMAX_VV: InstructionOpcode = 0x00c3;
pub const OP_VMAX_VX: InstructionOpcode = 0x00c4;
pub const OP_VWADDU_VV: InstructionOpcode = 0x00c5;
pub const OP_VWADDU_VX: InstructionOpcode = 0x00c6;
pub const OP_VWSUBU_VV: InstructionOpcode = 0x00c7;
pub const OP_VWSUBU_VX: InstructionOpcode = 0x00c8;
pub const OP_VWADD_VV: InstructionOpcode = 0x00c9;
pub const OP_VWADD_VX: InstructionOpcode = 0x00ca;
pub const OP_VWSUB_VV: InstructionOpcode = 0x00cb;
pub const OP_VWSUB_VX: InstructionOpcode = 0x00cc;
pub const OP_VWADDU_WV: InstructionOpcode = 0x00cd;
pub const OP_VWADDU_WX: InstructionOpcode = 0x00ce;
pub const OP_VWSUBU_WV: InstructionOpcode = 0x00cf;
pub const OP_VWSUBU_WX: InstructionOpcode = 0x00d0;
pub const OP_VWADD_WV: InstructionOpcode = 0x00d1;
pub const OP_VWADD_WX: InstructionOpcode = 0x00d2;
pub const OP_VWSUB_WV: InstructionOpcode = 0x00d3;
pub const OP_VWSUB_WX: InstructionOpcode = 0x00d4;
pub const OP_VZEXT_VF8: InstructionOpcode = 0x00d5;
pub const OP_VSEXT_VF8: InstructionOpcode = 0x00d6;
pub const OP_VZEXT_VF4: InstructionOpcode = 0x00d7;
pub const OP_VSEXT_VF4: InstructionOpcode = 0x00d8;
pub const OP_VZEXT_VF2: InstructionOpcode = 0x00d9;
pub const OP_VSEXT_VF2: InstructionOpcode = 0x00da;
pub const OP_VADC_VVM: InstructionOpcode = 0x00db;
pub const OP_VADC_VXM: InstructionOpcode = 0x00dc;
pub const OP_VADC_VIM: InstructionOpcode = 0x00dd;
pub const OP_VMADC_VVM: InstructionOpcode = 0x00de;
pub const OP_VMADC_VXM: InstructionOpcode = 0x00df;
pub const OP_VMADC_VIM: InstructionOpcode = 0x00e0;
pub const OP_VMADC_VV: InstructionOpcode = 0x00e1;
pub const OP_VMADC_VX: InstructionOpcode = 0x00e2;
pub const OP_VMADC_VI: InstructionOpcode = 0x00e3;
pub const OP_VSBC_VVM: InstructionOpcode = 0x00e4;
pub const OP_VSBC_VXM: InstructionOpcode = 0x00e5;
pub const OP_VMSBC_VVM: InstructionOpcode = 0x00e6;
pub const OP_VMSBC_VXM: InstructionOpcode = 0x00e7;
pub const OP_VMSBC_VV: InstructionOpcode = 0x00e8;
pub const OP_VMSBC_VX: InstructionOpcode = 0x00e9;
pub const OP_VAND_VV: InstructionOpcode = 0x00ea;
pub const OP_VAND_VI: InstructionOpcode = 0x00eb;
pub const OP_VAND_VX: InstructionOpcode = 0x00ec;
pub const OP_VOR_VV: InstructionOpcode = 0x00ed;
pub const OP_VOR_VX: InstructionOpcode = 0x00ee;
pub const OP_VOR_VI: InstructionOpcode = 0x00ef;
pub const OP_VXOR_VV: InstructionOpcode = 0x00f0;
pub const OP_VXOR_VX: InstructionOpcode = 0x00f1;
pub const OP_VXOR_VI: InstructionOpcode = 0x00f2;
pub const OP_VNSRL_WV: InstructionOpcode = 0x00f3;
pub const OP_VNSRL_WX: InstructionOpcode = 0x00f4;
pub const OP_VNSRL_WI: InstructionOpcode = 0x00f5;
pub const OP_VNSRA_WV: InstructionOpcode = 0x00f6;
pub const OP_VNSRA_WX: InstructionOpcode = 0x00f7;
pub const OP_VNSRA_WI: InstructionOpcode = 0x00f8;
pub const OP_VMULH_VV: InstructionOpcode = 0x00f9;
pub const OP_VMULH_VX: InstructionOpcode = 0x00fa;
pub const OP_VMULHU_VV: InstructionOpcode = 0x00fb;
pub const OP_VMULHU_VX: InstructionOpcode = 0x00fc;
pub const OP_VMULHSU_VV: InstructionOpcode = 0x00fd;
pub const OP_VMULHSU_VX: InstructionOpcode = 0x00fe;
pub const OP_VWMULU_VV: InstructionOpcode = 0x00ff;
pub const OP_VWMULU_VX: InstructionOpcode = 0x0100;
pub const OP_VWMULSU_VV: InstructionOpcode = 0x0101;
pub const OP_VWMULSU_VX: InstructionOpcode = 0x0102;
pub const OP_VWMUL_VV: InstructionOpcode = 0x0103;
pub const OP_VWMUL_VX: InstructionOpcode = 0x0104;
pub const OP_VMV_V_V: InstructionOpcode = 0x0105;
pub const OP_VMV_V_X: InstructionOpcode = 0x0106;
pub const OP_VMV_V_I: InstructionOpcode = 0x0107;
pub const OP_VSADDU_VV: InstructionOpcode = 0x0108;
pub const OP_VSADDU_VX: InstructionOpcode = 0x0109;
pub const OP_VSADDU_VI: InstructionOpcode = 0x010a;
pub const OP_VSADD_VV: InstructionOpcode = 0x010b;
pub const OP_VSADD_VX: InstructionOpcode = 0x010c;
pub const OP_VSADD_VI: InstructionOpcode = 0x010d;
pub const OP_VSSUBU_VV: InstructionOpcode = 0x010e;
pub const OP_VSSUBU_VX: InstructionOpcode = 0x010f;
pub const OP_VSSUB_VV: InstructionOpcode = 0x0110;
pub const OP_VSSUB_VX: InstructionOpcode = 0x0111;
pub const OP_VAADDU_VV: InstructionOpcode = 0x0112;
pub const OP_VAADDU_VX: InstructionOpcode = 0x0113;
pub const OP_VAADD_VV: InstructionOpcode = 0x0114;
pub const OP_VAADD_VX: InstructionOpcode = 0x0115;
pub const OP_VASUBU_VV: InstructionOpcode = 0x0116;
pub const OP_VASUBU_VX: InstructionOpcode = 0x0117;
pub const OP_VASUB_VV: InstructionOpcode = 0x0118;
pub const OP_VASUB_VX: InstructionOpcode = 0x0119;
pub const OP_VMV1R_V: InstructionOpcode = 0x011a;
pub const OP_VMV2R_V: InstructionOpcode = 0x011b;
pub const OP_VMV4R_V: InstructionOpcode = 0x011c;
pub const OP_VMV8R_V: InstructionOpcode = 0x011d;
pub const OP_VFIRST_M: InstructionOpcode = 0x011e;
pub const OP_VMAND_MM: InstructionOpcode = 0x011f;
pub const OP_VMNAND_MM: InstructionOpcode = 0x0120;
pub const OP_VMANDNOT_MM: InstructionOpcode = 0x0121;
pub const OP_VMXOR_MM: InstructionOpcode = 0x0122;
pub const OP_VMOR_MM: InstructionOpcode = 0x0123;
pub const OP_VMNOR_MM: InstructionOpcode = 0x0124;
pub const OP_VMORNOT_MM: InstructionOpcode = 0x0125;
pub const OP_VMXNOR_MM: InstructionOpcode = 0x0126;
pub const OP_VLSE8_V: InstructionOpcode = 0x0127;
pub const OP_VLSE16_V: InstructionOpcode = 0x0128;
pub const OP_VLSE32_V: InstructionOpcode = 0x0129;
pub const OP_VLSE64_V: InstructionOpcode = 0x012a;
pub const OP_VLSE128_V: InstructionOpcode = 0x012b;
pub const OP_VLSE256_V: InstructionOpcode = 0x012c;
pub const OP_VLSE512_V: InstructionOpcode = 0x012d;
pub const OP_VLSE1024_V: InstructionOpcode = 0x012e;
pub const OP_VSSE8_V: InstructionOpcode = 0x012f;
pub const OP_VSSE16_V: InstructionOpcode = 0x0130;
pub const OP_VSSE32_V: InstructionOpcode = 0x0131;
pub const OP_VSSE64_V: InstructionOpcode = 0x0132;
pub const OP_VSSE128_V: InstructionOpcode = 0x0133;
pub const OP_VSSE256_V: InstructionOpcode = 0x0134;
pub const OP_VSSE512_V: InstructionOpcode = 0x0135;
pub const OP_VSSE1024_V: InstructionOpcode = 0x0136;
pub const OP_VLUXEI8_V: InstructionOpcode = 0x0137;
pub const OP_VLUXEI16_V: InstructionOpcode = 0x0138;
pub const OP_VLUXEI32_V: InstructionOpcode = 0x0139;
pub const OP_VLUXEI64_V: InstructionOpcode = 0x013a;
pub const OP_VLUXEI128_V: InstructionOpcode = 0x013b;
pub const OP_VLUXEI256_V: InstructionOpcode = 0x013c;
pub const OP_VLUXEI512_V: InstructionOpcode = 0x013d;
pub const OP_VLUXEI1024_V: InstructionOpcode = 0x013e;
pub const OP_VLOXEI8_V: InstructionOpcode = 0x013f;
pub const OP_VLOXEI16_V: InstructionOpcode = 0x0140;
pub const OP_VLOXEI32_V: InstructionOpcode = 0x0141;
pub const OP_VLOXEI64_V: InstructionOpcode = 0x0142;
pub const OP_VLOXEI128_V: InstructionOpcode = 0x0143;
pub const OP_VLOXEI256_V: InstructionOpcode = 0x0144;
pub const OP_VLOXEI512_V: InstructionOpcode = 0x0145;
pub const OP_VLOXEI1024_V: InstructionOpcode = 0x0146;
pub const OP_VSUXEI8_V: InstructionOpcode = 0x0147;
pub const OP_VSUXEI16_V: InstructionOpcode = 0x0148;
pub const OP_VSUXEI32_V: InstructionOpcode = 0x0149;
pub const OP_VSUXEI64_V: InstructionOpcode = 0x014a;
pub const OP_VSUXEI128_V: InstructionOpcode = 0x014b;
pub const OP_VSUXEI256_V: InstructionOpcode = 0x014c;
pub const OP_VSUXEI512_V: InstructionOpcode = 0x014d;
pub const OP_VSUXEI1024_V: InstructionOpcode = 0x014e;
pub const OP_VSOXEI8_V: InstructionOpcode = 0x014f;
pub const OP_VSOXEI16_V: InstructionOpcode = 0x0150;
pub const OP_VSOXEI32_V: InstructionOpcode = 0x0151;
pub const OP_VSOXEI64_V: InstructionOpcode = 0x0152;
pub const OP_VSOXEI128_V: InstructionOpcode = 0x0153;
pub const OP_VSOXEI256_V: InstructionOpcode = 0x0154;
pub const OP_VSOXEI512_V: InstructionOpcode = 0x0155;
pub const OP_VSOXEI1024_V: InstructionOpcode = 0x0156;
pub const OP_VL1RE8_V: InstructionOpcode = 0x0157;
pub const OP_VL1RE16_V: InstructionOpcode = 0x0158;
pub const OP_VL1RE32_V: InstructionOpcode = 0x0159;
pub const OP_VL1RE64_V: InstructionOpcode = 0x015a;
pub const OP_VL2RE8_V: InstructionOpcode = 0x015b;
pub const OP_VL2RE16_V: InstructionOpcode = 0x015c;
pub const OP_VL2RE32_V: InstructionOpcode = 0x015d;
pub const OP_VL2RE64_V: InstructionOpcode = 0x015e;
pub const OP_VL4RE8_V: InstructionOpcode = 0x015f;
pub const OP_VL4RE16_V: InstructionOpcode = 0x0160;
pub const OP_VL4RE32_V: InstructionOpcode = 0x0161;
pub const OP_VL4RE64_V: InstructionOpcode = 0x0162;
pub const OP_VL8RE8_V: InstructionOpcode = 0x0163;
pub const OP_VL8RE16_V: InstructionOpcode = 0x0164;
pub const OP_VL8RE32_V: InstructionOpcode = 0x0165;
pub const OP_VL8RE64_V: InstructionOpcode = 0x0166;
pub const OP_VS1R_V: InstructionOpcode = 0x0167;
pub const OP_VS2R_V: InstructionOpcode = 0x0168;
pub const OP_VS4R_V: InstructionOpcode = 0x0169;
pub const OP_VS8R_V: InstructionOpcode = 0x016a;
pub const OP_VMACC_VV: InstructionOpcode = 0x016b;
pub const OP_VMACC_VX: InstructionOpcode = 0x016c;
pub const OP_VNMSAC_VV: InstructionOpcode = 0x016d;
pub const OP_VNMSAC_VX: InstructionOpcode = 0x016e;
pub const OP_VMADD_VV: InstructionOpcode = 0x016f;
pub const OP_VMADD_VX: InstructionOpcode = 0x0170;
pub const OP_VNMSUB_VV: InstructionOpcode = 0x0171;
pub const OP_VNMSUB_VX: InstructionOpcode = 0x0172;
pub const OP_VSSRL_VV: InstructionOpcode = 0x0173;
pub const OP_VSSRL_VX: InstructionOpcode = 0x0174;
pub const OP_VSSRL_VI: InstructionOpcode = 0x0175;
pub const OP_VSSRA_VV: InstructionOpcode = 0x0176;
pub const OP_VSSRA_VX: InstructionOpcode = 0x0177;
pub const OP_VSSRA_VI: InstructionOpcode = 0x0178;
pub const OP_VSMUL_VV: InstructionOpcode = 0x0179;
pub const OP_VSMUL_VX: InstructionOpcode = 0x017a;
pub const OP_VWMACCU_VV: InstructionOpcode = 0x017b;
pub const OP_VWMACCU_VX: InstructionOpcode = 0x017c;
pub const OP_VWMACC_VV: InstructionOpcode = 0x017d;
pub const OP_VWMACC_VX: InstructionOpcode = 0x017e;
pub const OP_VWMACCSU_VV: InstructionOpcode = 0x017f;
pub const OP_VWMACCSU_VX: InstructionOpcode = 0x0180;
pub const OP_VWMACCUS_VX: InstructionOpcode = 0x0181;
pub const OP_VMERGE_VVM: InstructionOpcode = 0x0182;
pub const OP_VMERGE_VXM: InstructionOpcode = 0x0183;
pub const OP_VMERGE_VIM: InstructionOpcode = 0x0184;
pub const OP_VNCLIPU_WV: InstructionOpcode = 0x0185;
pub const OP_VNCLIPU_WX: InstructionOpcode = 0x0186;
pub const OP_VNCLIPU_WI: InstructionOpcode = 0x0187;
pub const OP_VNCLIP_WV: InstructionOpcode = 0x0188;
pub const OP_VNCLIP_WX: InstructionOpcode = 0x0189;
pub const OP_VNCLIP_WI: InstructionOpcode = 0x018a;
pub const OP_VREDSUM_VS: InstructionOpcode = 0x018b;
pub const OP_VREDAND_VS: InstructionOpcode = 0x018c;
pub const OP_VREDOR_VS: InstructionOpcode = 0x018d;
pub const OP_VREDXOR_VS: InstructionOpcode = 0x018e;
pub const OP_VREDMINU_VS: InstructionOpcode = 0x018f;
pub const OP_VREDMIN_VS: InstructionOpcode = 0x0190;
pub const OP_VREDMAXU_VS: InstructionOpcode = 0x0191;
pub const OP_VREDMAX_VS: InstructionOpcode = 0x0192;
pub const OP_VWREDSUMU_VS: InstructionOpcode = 0x0193;
pub const OP_VWREDSUM_VS: InstructionOpcode = 0x0194;
pub const OP_VCPOP_M: InstructionOpcode = 0x0195;
pub const OP_VMSBF_M: InstructionOpcode = 0x0196;
pub const OP_VMSOF_M: InstructionOpcode = 0x0197;
pub const OP_VMSIF_M: InstructionOpcode = 0x0198;
pub const OP_VIOTA_M: InstructionOpcode = 0x0199;
pub const OP_VID_V: InstructionOpcode = 0x019a;
pub const OP_VMV_X_S: InstructionOpcode = 0x019b;
pub const OP_VMV_S_X: InstructionOpcode = 0x019c;
pub const OP_VCOMPRESS_VM: InstructionOpcode = 0x019d;
pub const OP_VSLIDE1UP_VX: InstructionOpcode = 0x019e;
pub const OP_VSLIDEUP_VX: InstructionOpcode = 0x019f;
pub const OP_VSLIDEUP_VI: InstructionOpcode = 0x01a0;
pub const OP_VSLIDE1DOWN_VX: InstructionOpcode = 0x01a1;
pub const OP_VSLIDEDOWN_VX: InstructionOpcode = 0x01a2;
pub const OP_VSLIDEDOWN_VI: InstructionOpcode = 0x01a3;
pub const OP_VRGATHER_VX: InstructionOpcode = 0x01a4;
pub const OP_VRGATHER_VV: InstructionOpcode = 0x01a5;
pub const OP_VRGATHEREI16_VV: InstructionOpcode = 0x01a6;
pub const OP_VRGATHER_VI: InstructionOpcode = 0x01a7;

pub const MINIMAL_LEVEL1_OPCODE: InstructionOpcode = OP_UNLOADED;
pub const MAXIMUM_LEVEL1_OPCODE: InstructionOpcode = OP_CUSTOM_TRACE_END;
pub const MINIMAL_LEVEL2_OPCODE: InstructionOpcode = OP_VSETVLI;
pub const MAXIMUM_LEVEL2_OPCODE: InstructionOpcode = OP_VRGATHER_VI;

pub const INSTRUCTION_OPCODE_NAMES: [&str; OP_VRGATHER_VI as usize + 1] = [
    "UNLOADED",
    "ADD",
    "ADDI",
    "ADDIW",
    "ADDW",
    "AND",
    "ANDI",
    "DIV",
    "DIVU",
    "DIVUW",
    "DIVW",
    "FENCE",
    "FENCEI",
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
    "ADC",
    "SBB",
    "CUSTOM_LOAD_UIMM",
    "CUSTOM_LOAD_IMM",
    "AUIPC",
    "BEQ",
    "BGE",
    "BGEU",
    "BLT",
    "BLTU",
    "BNE",
    "EBREAK",
    "ECALL",
    "JAL",
    "JALR",
    "FAR_JUMP_REL",
    "FAR_JUMP_ABS",
    "CUSTOM_TRACE_END",
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
    "VLUXEI128_V",
    "VLUXEI256_V",
    "VLUXEI512_V",
    "VLUXEI1024_V",
    "VLOXEI8_V",
    "VLOXEI16_V",
    "VLOXEI32_V",
    "VLOXEI64_V",
    "VLOXEI128_V",
    "VLOXEI256_V",
    "VLOXEI512_V",
    "VLOXEI1024_V",
    "VSUXEI8_V",
    "VSUXEI16_V",
    "VSUXEI32_V",
    "VSUXEI64_V",
    "VSUXEI128_V",
    "VSUXEI256_V",
    "VSUXEI512_V",
    "VSUXEI1024_V",
    "VSOXEI8_V",
    "VSOXEI16_V",
    "VSOXEI32_V",
    "VSOXEI64_V",
    "VSOXEI128_V",
    "VSOXEI256_V",
    "VSOXEI512_V",
    "VSOXEI1024_V",
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
    INSTRUCTION_OPCODE_NAMES[i as usize]
}
