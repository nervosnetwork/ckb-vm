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
pub const OP_VSETVLI: InstructionOpcode = 0x7a;
pub const OP_VSETIVLI: InstructionOpcode = 0x7b;
pub const OP_VSETVL: InstructionOpcode = 0x7c;
pub const OP_VLM_V: InstructionOpcode = 0x7d;
pub const OP_VLE8_V: InstructionOpcode = 0x7e;
pub const OP_VLE16_V: InstructionOpcode = 0x7f;
pub const OP_VLE32_V: InstructionOpcode = 0x80;
pub const OP_VLE64_V: InstructionOpcode = 0x81;
pub const OP_VLE128_V: InstructionOpcode = 0x82;
pub const OP_VLE256_V: InstructionOpcode = 0x83;
pub const OP_VLE512_V: InstructionOpcode = 0x84;
pub const OP_VLE1024_V: InstructionOpcode = 0x85;
pub const OP_VSM_V: InstructionOpcode = 0x86;
pub const OP_VSE8_V: InstructionOpcode = 0x87;
pub const OP_VSE16_V: InstructionOpcode = 0x88;
pub const OP_VSE32_V: InstructionOpcode = 0x89;
pub const OP_VSE64_V: InstructionOpcode = 0x8a;
pub const OP_VSE128_V: InstructionOpcode = 0x8b;
pub const OP_VSE256_V: InstructionOpcode = 0x8c;
pub const OP_VSE512_V: InstructionOpcode = 0x8d;
pub const OP_VSE1024_V: InstructionOpcode = 0x8e;
pub const OP_VADD_VV: InstructionOpcode = 0x8f;
pub const OP_VADD_VX: InstructionOpcode = 0x90;
pub const OP_VADD_VI: InstructionOpcode = 0x91;
pub const OP_VSUB_VV: InstructionOpcode = 0x92;
pub const OP_VSUB_VX: InstructionOpcode = 0x93;
pub const OP_VRSUB_VX: InstructionOpcode = 0x94;
pub const OP_VRSUB_VI: InstructionOpcode = 0x95;
pub const OP_VMUL_VV: InstructionOpcode = 0x96;
pub const OP_VMUL_VX: InstructionOpcode = 0x97;
pub const OP_VDIV_VV: InstructionOpcode = 0x98;
pub const OP_VDIV_VX: InstructionOpcode = 0x99;
pub const OP_VDIVU_VV: InstructionOpcode = 0x9a;
pub const OP_VDIVU_VX: InstructionOpcode = 0x9b;
pub const OP_VREM_VV: InstructionOpcode = 0x9c;
pub const OP_VREM_VX: InstructionOpcode = 0x9d;
pub const OP_VREMU_VV: InstructionOpcode = 0x9e;
pub const OP_VREMU_VX: InstructionOpcode = 0x9f;
pub const OP_VSLL_VV: InstructionOpcode = 0xa0;
pub const OP_VSLL_VX: InstructionOpcode = 0xa1;
pub const OP_VSLL_VI: InstructionOpcode = 0xa2;
pub const OP_VSRL_VV: InstructionOpcode = 0xa3;
pub const OP_VSRL_VX: InstructionOpcode = 0xa4;
pub const OP_VSRL_VI: InstructionOpcode = 0xa5;
pub const OP_VSRA_VV: InstructionOpcode = 0xa6;
pub const OP_VSRA_VX: InstructionOpcode = 0xa7;
pub const OP_VSRA_VI: InstructionOpcode = 0xa8;
pub const OP_VMSEQ_VV: InstructionOpcode = 0xa9;
pub const OP_VMSEQ_VX: InstructionOpcode = 0xaa;
pub const OP_VMSEQ_VI: InstructionOpcode = 0xab;
pub const OP_VMSNE_VV: InstructionOpcode = 0xac;
pub const OP_VMSNE_VX: InstructionOpcode = 0xad;
pub const OP_VMSNE_VI: InstructionOpcode = 0xae;
pub const OP_VMSLTU_VV: InstructionOpcode = 0xaf;
pub const OP_VMSLTU_VX: InstructionOpcode = 0xb0;
pub const OP_VMSLT_VV: InstructionOpcode = 0xb1;
pub const OP_VMSLT_VX: InstructionOpcode = 0xb2;
pub const OP_VMSLEU_VV: InstructionOpcode = 0xb3;
pub const OP_VMSLEU_VX: InstructionOpcode = 0xb4;
pub const OP_VMSLEU_VI: InstructionOpcode = 0xb5;
pub const OP_VMSLE_VV: InstructionOpcode = 0xb6;
pub const OP_VMSLE_VX: InstructionOpcode = 0xb7;
pub const OP_VMSLE_VI: InstructionOpcode = 0xb8;
pub const OP_VMSGTU_VX: InstructionOpcode = 0xb9;
pub const OP_VMSGTU_VI: InstructionOpcode = 0xba;
pub const OP_VMSGT_VX: InstructionOpcode = 0xbb;
pub const OP_VMSGT_VI: InstructionOpcode = 0xbc;
pub const OP_VMINU_VV: InstructionOpcode = 0xbd;
pub const OP_VMINU_VX: InstructionOpcode = 0xbe;
pub const OP_VMIN_VV: InstructionOpcode = 0xbf;
pub const OP_VMIN_VX: InstructionOpcode = 0xc0;
pub const OP_VMAXU_VV: InstructionOpcode = 0xc1;
pub const OP_VMAXU_VX: InstructionOpcode = 0xc2;
pub const OP_VMAX_VV: InstructionOpcode = 0xc3;
pub const OP_VMAX_VX: InstructionOpcode = 0xc4;
pub const OP_VWADDU_VV: InstructionOpcode = 0xc5;
pub const OP_VWADDU_VX: InstructionOpcode = 0xc6;
pub const OP_VWSUBU_VV: InstructionOpcode = 0xc7;
pub const OP_VWSUBU_VX: InstructionOpcode = 0xc8;
pub const OP_VWADD_VV: InstructionOpcode = 0xc9;
pub const OP_VWADD_VX: InstructionOpcode = 0xca;
pub const OP_VWSUB_VV: InstructionOpcode = 0xcb;
pub const OP_VWSUB_VX: InstructionOpcode = 0xcc;
pub const OP_VWADDU_WV: InstructionOpcode = 0xcd;
pub const OP_VWADDU_WX: InstructionOpcode = 0xce;
pub const OP_VWSUBU_WV: InstructionOpcode = 0xcf;
pub const OP_VWSUBU_WX: InstructionOpcode = 0xd0;
pub const OP_VWADD_WV: InstructionOpcode = 0xd1;
pub const OP_VWADD_WX: InstructionOpcode = 0xd2;
pub const OP_VWSUB_WV: InstructionOpcode = 0xd3;
pub const OP_VWSUB_WX: InstructionOpcode = 0xd4;
pub const OP_VZEXT_VF8: InstructionOpcode = 0xd5;
pub const OP_VSEXT_VF8: InstructionOpcode = 0xd6;
pub const OP_VZEXT_VF4: InstructionOpcode = 0xd7;
pub const OP_VSEXT_VF4: InstructionOpcode = 0xd8;
pub const OP_VZEXT_VF2: InstructionOpcode = 0xd9;
pub const OP_VSEXT_VF2: InstructionOpcode = 0xda;
pub const OP_VADC_VVM: InstructionOpcode = 0xdb;
pub const OP_VADC_VXM: InstructionOpcode = 0xdc;
pub const OP_VADC_VIM: InstructionOpcode = 0xdd;
pub const OP_VMADC_VVM: InstructionOpcode = 0xde;
pub const OP_VMADC_VXM: InstructionOpcode = 0xdf;
pub const OP_VMADC_VIM: InstructionOpcode = 0xe0;
pub const OP_VMADC_VV: InstructionOpcode = 0xe1;
pub const OP_VMADC_VX: InstructionOpcode = 0xe2;
pub const OP_VMADC_VI: InstructionOpcode = 0xe3;
pub const OP_VSBC_VVM: InstructionOpcode = 0xe4;
pub const OP_VSBC_VXM: InstructionOpcode = 0xe5;
pub const OP_VMSBC_VVM: InstructionOpcode = 0xe6;
pub const OP_VMSBC_VXM: InstructionOpcode = 0xe7;
pub const OP_VMSBC_VV: InstructionOpcode = 0xe8;
pub const OP_VMSBC_VX: InstructionOpcode = 0xe9;
pub const OP_VAND_VV: InstructionOpcode = 0xea;
pub const OP_VAND_VI: InstructionOpcode = 0xeb;
pub const OP_VAND_VX: InstructionOpcode = 0xec;
pub const OP_VOR_VV: InstructionOpcode = 0xed;
pub const OP_VOR_VX: InstructionOpcode = 0xee;
pub const OP_VOR_VI: InstructionOpcode = 0xef;
pub const OP_VXOR_VV: InstructionOpcode = 0xf0;
pub const OP_VXOR_VX: InstructionOpcode = 0xf1;
pub const OP_VXOR_VI: InstructionOpcode = 0xf2;
pub const OP_VNSRL_WV: InstructionOpcode = 0xf3;
pub const OP_VNSRL_WX: InstructionOpcode = 0xf4;
pub const OP_VNSRL_WI: InstructionOpcode = 0xf5;
pub const OP_VNSRA_WV: InstructionOpcode = 0xf6;
pub const OP_VNSRA_WX: InstructionOpcode = 0xf7;
pub const OP_VNSRA_WI: InstructionOpcode = 0xf8;
pub const OP_VMULH_VV: InstructionOpcode = 0xf9;
pub const OP_VMULH_VX: InstructionOpcode = 0xfa;
pub const OP_VMULHU_VV: InstructionOpcode = 0xfb;
pub const OP_VMULHU_VX: InstructionOpcode = 0xfc;
pub const OP_VMULHSU_VV: InstructionOpcode = 0xfd;
pub const OP_VMULHSU_VX: InstructionOpcode = 0xfe;
pub const OP_VWMULU_VV: InstructionOpcode = 0xff;
pub const OP_VWMULU_VX: InstructionOpcode = 0x100;
pub const OP_VWMULSU_VV: InstructionOpcode = 0x101;
pub const OP_VWMULSU_VX: InstructionOpcode = 0x102;
pub const OP_VWMUL_VV: InstructionOpcode = 0x103;
pub const OP_VWMUL_VX: InstructionOpcode = 0x104;
pub const OP_VMV_V_V: InstructionOpcode = 0x105;
pub const OP_VMV_V_X: InstructionOpcode = 0x106;
pub const OP_VMV_V_I: InstructionOpcode = 0x107;
pub const OP_VSADDU_VV: InstructionOpcode = 0x108;
pub const OP_VSADDU_VX: InstructionOpcode = 0x109;
pub const OP_VSADDU_VI: InstructionOpcode = 0x10a;
pub const OP_VSADD_VV: InstructionOpcode = 0x10b;
pub const OP_VSADD_VX: InstructionOpcode = 0x10c;
pub const OP_VSADD_VI: InstructionOpcode = 0x10d;
pub const OP_VSSUBU_VV: InstructionOpcode = 0x10e;
pub const OP_VSSUBU_VX: InstructionOpcode = 0x10f;
pub const OP_VSSUB_VV: InstructionOpcode = 0x110;
pub const OP_VSSUB_VX: InstructionOpcode = 0x111;
pub const OP_VAADDU_VV: InstructionOpcode = 0x112;
pub const OP_VAADDU_VX: InstructionOpcode = 0x113;
pub const OP_VAADD_VV: InstructionOpcode = 0x114;
pub const OP_VAADD_VX: InstructionOpcode = 0x115;
pub const OP_VASUBU_VV: InstructionOpcode = 0x116;
pub const OP_VASUBU_VX: InstructionOpcode = 0x117;
pub const OP_VASUB_VV: InstructionOpcode = 0x118;
pub const OP_VASUB_VX: InstructionOpcode = 0x119;
pub const OP_VMV1R_V: InstructionOpcode = 0x11a;
pub const OP_VMV2R_V: InstructionOpcode = 0x11b;
pub const OP_VMV4R_V: InstructionOpcode = 0x11c;
pub const OP_VMV8R_V: InstructionOpcode = 0x11d;
pub const OP_VFIRST_M: InstructionOpcode = 0x11e;
pub const OP_VMAND_MM: InstructionOpcode = 0x11f;
pub const OP_VMNAND_MM: InstructionOpcode = 0x120;
pub const OP_VMANDNOT_MM: InstructionOpcode = 0x121;
pub const OP_VMXOR_MM: InstructionOpcode = 0x122;
pub const OP_VMOR_MM: InstructionOpcode = 0x123;
pub const OP_VMNOR_MM: InstructionOpcode = 0x124;
pub const OP_VMORNOT_MM: InstructionOpcode = 0x125;
pub const OP_VMXNOR_MM: InstructionOpcode = 0x126;
pub const OP_VLSE8_V: InstructionOpcode = 0x127;
pub const OP_VLSE16_V: InstructionOpcode = 0x128;
pub const OP_VLSE32_V: InstructionOpcode = 0x129;
pub const OP_VLSE64_V: InstructionOpcode = 0x12a;
pub const OP_VLSE128_V: InstructionOpcode = 0x12b;
pub const OP_VLSE256_V: InstructionOpcode = 0x12c;
pub const OP_VLSE512_V: InstructionOpcode = 0x12d;
pub const OP_VLSE1024_V: InstructionOpcode = 0x12e;
pub const OP_VSSE8_V: InstructionOpcode = 0x12f;
pub const OP_VSSE16_V: InstructionOpcode = 0x130;
pub const OP_VSSE32_V: InstructionOpcode = 0x131;
pub const OP_VSSE64_V: InstructionOpcode = 0x132;
pub const OP_VSSE128_V: InstructionOpcode = 0x133;
pub const OP_VSSE256_V: InstructionOpcode = 0x134;
pub const OP_VSSE512_V: InstructionOpcode = 0x135;
pub const OP_VSSE1024_V: InstructionOpcode = 0x136;
pub const OP_VLUXEI8_V: InstructionOpcode = 0x137;
pub const OP_VLUXEI16_V: InstructionOpcode = 0x138;
pub const OP_VLUXEI32_V: InstructionOpcode = 0x139;
pub const OP_VLUXEI64_V: InstructionOpcode = 0x13a;
pub const OP_VLOXEI8_V: InstructionOpcode = 0x13b;
pub const OP_VLOXEI16_V: InstructionOpcode = 0x13c;
pub const OP_VLOXEI32_V: InstructionOpcode = 0x13d;
pub const OP_VLOXEI64_V: InstructionOpcode = 0x13e;
pub const OP_VSUXEI8_V: InstructionOpcode = 0x13f;
pub const OP_VSUXEI16_V: InstructionOpcode = 0x140;
pub const OP_VSUXEI32_V: InstructionOpcode = 0x141;
pub const OP_VSUXEI64_V: InstructionOpcode = 0x142;
pub const OP_VSOXEI8_V: InstructionOpcode = 0x143;
pub const OP_VSOXEI16_V: InstructionOpcode = 0x144;
pub const OP_VSOXEI32_V: InstructionOpcode = 0x145;
pub const OP_VSOXEI64_V: InstructionOpcode = 0x146;
pub const OP_VL1RE8_V: InstructionOpcode = 0x147;
pub const OP_VL1RE16_V: InstructionOpcode = 0x148;
pub const OP_VL1RE32_V: InstructionOpcode = 0x149;
pub const OP_VL1RE64_V: InstructionOpcode = 0x14a;
pub const OP_VL2RE8_V: InstructionOpcode = 0x14b;
pub const OP_VL2RE16_V: InstructionOpcode = 0x14c;
pub const OP_VL2RE32_V: InstructionOpcode = 0x14d;
pub const OP_VL2RE64_V: InstructionOpcode = 0x14e;
pub const OP_VL4RE8_V: InstructionOpcode = 0x14f;
pub const OP_VL4RE16_V: InstructionOpcode = 0x150;
pub const OP_VL4RE32_V: InstructionOpcode = 0x151;
pub const OP_VL4RE64_V: InstructionOpcode = 0x152;
pub const OP_VL8RE8_V: InstructionOpcode = 0x153;
pub const OP_VL8RE16_V: InstructionOpcode = 0x154;
pub const OP_VL8RE32_V: InstructionOpcode = 0x155;
pub const OP_VL8RE64_V: InstructionOpcode = 0x156;
pub const OP_VS1R_V: InstructionOpcode = 0x157;
pub const OP_VS2R_V: InstructionOpcode = 0x158;
pub const OP_VS4R_V: InstructionOpcode = 0x159;
pub const OP_VS8R_V: InstructionOpcode = 0x15a;
pub const OP_VMACC_VV: InstructionOpcode = 0x15b;
pub const OP_VMACC_VX: InstructionOpcode = 0x15c;
pub const OP_VNMSAC_VV: InstructionOpcode = 0x15d;
pub const OP_VNMSAC_VX: InstructionOpcode = 0x15e;
pub const OP_VMADD_VV: InstructionOpcode = 0x15f;
pub const OP_VMADD_VX: InstructionOpcode = 0x160;
pub const OP_VNMSUB_VV: InstructionOpcode = 0x161;
pub const OP_VNMSUB_VX: InstructionOpcode = 0x162;
pub const OP_VSSRL_VV: InstructionOpcode = 0x163;
pub const OP_VSSRL_VX: InstructionOpcode = 0x164;
pub const OP_VSSRL_VI: InstructionOpcode = 0x165;
pub const OP_VSSRA_VV: InstructionOpcode = 0x166;
pub const OP_VSSRA_VX: InstructionOpcode = 0x167;
pub const OP_VSSRA_VI: InstructionOpcode = 0x168;
pub const OP_VSMUL_VV: InstructionOpcode = 0x169;
pub const OP_VSMUL_VX: InstructionOpcode = 0x16a;
pub const OP_VWMACCU_VV: InstructionOpcode = 0x16b;
pub const OP_VWMACCU_VX: InstructionOpcode = 0x16c;
pub const OP_VWMACC_VV: InstructionOpcode = 0x16d;
pub const OP_VWMACC_VX: InstructionOpcode = 0x16e;
pub const OP_VWMACCSU_VV: InstructionOpcode = 0x16f;
pub const OP_VWMACCSU_VX: InstructionOpcode = 0x170;
pub const OP_VWMACCUS_VX: InstructionOpcode = 0x171;
pub const OP_VMERGE_VVM: InstructionOpcode = 0x172;
pub const OP_VMERGE_VXM: InstructionOpcode = 0x173;
pub const OP_VMERGE_VIM: InstructionOpcode = 0x174;
pub const OP_VNCLIPU_WV: InstructionOpcode = 0x175;
pub const OP_VNCLIPU_WX: InstructionOpcode = 0x176;
pub const OP_VNCLIPU_WI: InstructionOpcode = 0x177;
pub const OP_VNCLIP_WV: InstructionOpcode = 0x178;
pub const OP_VNCLIP_WX: InstructionOpcode = 0x179;
pub const OP_VNCLIP_WI: InstructionOpcode = 0x17a;
pub const OP_VREDSUM_VS: InstructionOpcode = 0x17b;
pub const OP_VREDAND_VS: InstructionOpcode = 0x17c;
pub const OP_VREDOR_VS: InstructionOpcode = 0x17d;
pub const OP_VREDXOR_VS: InstructionOpcode = 0x17e;
pub const OP_VREDMINU_VS: InstructionOpcode = 0x17f;
pub const OP_VREDMIN_VS: InstructionOpcode = 0x180;
pub const OP_VREDMAXU_VS: InstructionOpcode = 0x181;
pub const OP_VREDMAX_VS: InstructionOpcode = 0x182;
pub const OP_VWREDSUMU_VS: InstructionOpcode = 0x183;
pub const OP_VWREDSUM_VS: InstructionOpcode = 0x184;
pub const OP_VCPOP_M: InstructionOpcode = 0x185;
pub const OP_VMSBF_M: InstructionOpcode = 0x186;
pub const OP_VMSOF_M: InstructionOpcode = 0x187;
pub const OP_VMSIF_M: InstructionOpcode = 0x188;
pub const OP_VIOTA_M: InstructionOpcode = 0x189;
pub const OP_VID_V: InstructionOpcode = 0x18a;
pub const OP_VMV_X_S: InstructionOpcode = 0x18b;
pub const OP_VMV_S_X: InstructionOpcode = 0x18c;
pub const OP_VCOMPRESS_VM: InstructionOpcode = 0x18d;
pub const OP_VSLIDE1UP_VX: InstructionOpcode = 0x18e;
pub const OP_VSLIDEUP_VX: InstructionOpcode = 0x18f;
pub const OP_VSLIDEUP_VI: InstructionOpcode = 0x190;
pub const OP_VSLIDE1DOWN_VX: InstructionOpcode = 0x191;
pub const OP_VSLIDEDOWN_VX: InstructionOpcode = 0x192;
pub const OP_VSLIDEDOWN_VI: InstructionOpcode = 0x193;
pub const OP_VRGATHER_VX: InstructionOpcode = 0x194;
pub const OP_VRGATHER_VV: InstructionOpcode = 0x195;
pub const OP_VRGATHEREI16_VV: InstructionOpcode = 0x196;
pub const OP_VRGATHER_VI: InstructionOpcode = 0x197;

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
    INSTRUCTION_OPCODE_NAMES[i as usize]
}
