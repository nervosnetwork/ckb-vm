// For fast decoding and cache friendly, RISC-V instruction is decoded
// into 64 bit unsigned integer in the following format:
//
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |           | rs2 | rs1 | flg | op2 | rd  | op  | R-type
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |     | rs3 | rs2 | rs1 | flg | op2 | rd  | op  | R4-type
// +-----------+-----------------------------------+
// |    immediate    | rs1 | flg | op2 | rd  | op  | I-type
// +-----------------------------------------------+
// |    immediate    | rs1 | flg | op2 | rs2 | op  | S-type/B-type
// +-----------------+-----------------------------+
// |       immediate       | flg | op2 | rd  | op  | U-type/J-type
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

// Level-1 IMC
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
// Level-1 B
pub const OP_CLZ: InstructionOpcode = 0x43;
pub const OP_CTZ: InstructionOpcode = 0x44;
pub const OP_PCNT: InstructionOpcode = 0x45;
pub const OP_CLZW: InstructionOpcode = 0x46;
pub const OP_CTZW: InstructionOpcode = 0x47;
pub const OP_PCNTW: InstructionOpcode = 0x48;
pub const OP_ANDN: InstructionOpcode = 0x49;
pub const OP_ORN: InstructionOpcode = 0x4A;
pub const OP_XNOR: InstructionOpcode = 0x4B;
pub const OP_PACK: InstructionOpcode = 0x4C;
pub const OP_PACKU: InstructionOpcode = 0x4D;
pub const OP_PACKH: InstructionOpcode = 0x4E;
pub const OP_PACKW: InstructionOpcode = 0x4F;
pub const OP_PACKUW: InstructionOpcode = 0x50;
pub const OP_MIN: InstructionOpcode = 0x51;
pub const OP_MAX: InstructionOpcode = 0x52;
pub const OP_MINU: InstructionOpcode = 0x53;
pub const OP_MAXU: InstructionOpcode = 0x54;
pub const OP_SEXTB: InstructionOpcode = 0x55;
pub const OP_SEXTH: InstructionOpcode = 0x56;
pub const OP_SBCLR: InstructionOpcode = 0x57;
pub const OP_SBSET: InstructionOpcode = 0x58;
pub const OP_SBINV: InstructionOpcode = 0x59;
pub const OP_SBEXT: InstructionOpcode = 0x5A;
pub const OP_SBCLRI: InstructionOpcode = 0x5B;
pub const OP_SBSETI: InstructionOpcode = 0x5C;
pub const OP_SBINVI: InstructionOpcode = 0x5D;
pub const OP_SBEXTI: InstructionOpcode = 0x5E;
pub const OP_SBCLRW: InstructionOpcode = 0x5F;
pub const OP_SBSETW: InstructionOpcode = 0x60;
pub const OP_SBINVW: InstructionOpcode = 0x61;
pub const OP_SBEXTW: InstructionOpcode = 0x62;
pub const OP_SBCLRIW: InstructionOpcode = 0x63;
pub const OP_SBSETIW: InstructionOpcode = 0x64;
pub const OP_SBINVIW: InstructionOpcode = 0x65;
pub const OP_SLO: InstructionOpcode = 0x66;
pub const OP_SRO: InstructionOpcode = 0x67;
pub const OP_SLOI: InstructionOpcode = 0x68;
pub const OP_SROI: InstructionOpcode = 0x69;
pub const OP_SLOW: InstructionOpcode = 0x6A;
pub const OP_SROW: InstructionOpcode = 0x6B;
pub const OP_SLOIW: InstructionOpcode = 0x6C;
pub const OP_SROIW: InstructionOpcode = 0x6D;
pub const OP_ROL: InstructionOpcode = 0x6E;
pub const OP_ROR: InstructionOpcode = 0x6F;
pub const OP_RORI: InstructionOpcode = 0x70;
pub const OP_ROLW: InstructionOpcode = 0x71;
pub const OP_RORW: InstructionOpcode = 0x72;
pub const OP_RORIW: InstructionOpcode = 0x73;
pub const OP_CMIX: InstructionOpcode = 0x74;
pub const OP_CMOV: InstructionOpcode = 0x75;
pub const OP_FSL: InstructionOpcode = 0x76;
pub const OP_FSR: InstructionOpcode = 0x77;
pub const OP_FSRI: InstructionOpcode = 0x78;
pub const OP_FSLW: InstructionOpcode = 0x79;
pub const OP_FSRW: InstructionOpcode = 0x7A;
pub const OP_FSRIW: InstructionOpcode = 0x7B;
pub const OP_SH1ADD: InstructionOpcode = 0x7C;
pub const OP_SH2ADD: InstructionOpcode = 0x7D;
pub const OP_SH3ADD: InstructionOpcode = 0x7E;
pub const OP_SH1ADDUW: InstructionOpcode = 0x7F;
pub const OP_SH2ADDUW: InstructionOpcode = 0x80;
pub const OP_SH3ADDUW: InstructionOpcode = 0x81;
pub const OP_ADDWU: InstructionOpcode = 0x82;
pub const OP_SUBWU: InstructionOpcode = 0x83;
pub const OP_ADDIWU: InstructionOpcode = 0x84;
pub const OP_ADDUW: InstructionOpcode = 0x85;
pub const OP_SUBUW: InstructionOpcode = 0x86;
pub const OP_SLLIUW: InstructionOpcode = 0x87;
// Level-1 Macro op fusion
pub const OP_WIDE_MUL: InstructionOpcode = 0x88;
pub const OP_WIDE_MULU: InstructionOpcode = 0x89;
pub const OP_WIDE_MULSU: InstructionOpcode = 0x8A;
pub const OP_WIDE_DIV: InstructionOpcode = 0x8B;
pub const OP_WIDE_DIVU: InstructionOpcode = 0x8C;
pub const OP_FAR_JUMP_REL: InstructionOpcode = 0x8D;
pub const OP_FAR_JUMP_ABS: InstructionOpcode = 0x8E;
pub const OP_LD_SIGN_EXTENDED_32_CONSTANT: InstructionOpcode = 0x8F;
pub const OP_ADC: InstructionOpcode = 0x90;
pub const OP_SBB: InstructionOpcode = 0x91;
pub const OP_TWINS_LD: InstructionOpcode = 0x92;
pub const OP_TWINS_SD: InstructionOpcode = 0x93;
// Level-1 Custom
pub const OP_CUSTOM_LOAD_IMM: InstructionOpcode = 0x94;
pub const OP_CUSTOM_TRACE_END: InstructionOpcode = 0x95;
// Level-2 B
pub const OP_GREV: InstructionOpcode = 0x00F0;
pub const OP_GREVI: InstructionOpcode = 0x01F0;
pub const OP_GREVW: InstructionOpcode = 0x02F0;
pub const OP_GREVIW: InstructionOpcode = 0x03F0;
pub const OP_SHFL: InstructionOpcode = 0x04F0;
pub const OP_UNSHFL: InstructionOpcode = 0x05F0;
pub const OP_SHFLI: InstructionOpcode = 0x06F0;
pub const OP_UNSHFLI: InstructionOpcode = 0x07F0;
pub const OP_SHFLW: InstructionOpcode = 0x08F0;
pub const OP_UNSHFLW: InstructionOpcode = 0x09F0;
pub const OP_GORC: InstructionOpcode = 0x0AF0;
pub const OP_GORCI: InstructionOpcode = 0x0BF0;
pub const OP_GORCW: InstructionOpcode = 0x0CF0;
pub const OP_GORCIW: InstructionOpcode = 0x0DF0;
pub const OP_BFP: InstructionOpcode = 0x0EF0;
pub const OP_BFPW: InstructionOpcode = 0x0FF0;
pub const OP_BDEP: InstructionOpcode = 0x10F0;
pub const OP_BEXT: InstructionOpcode = 0x11F0;
pub const OP_BDEPW: InstructionOpcode = 0x12F0;
pub const OP_BEXTW: InstructionOpcode = 0x13F0;
pub const OP_CLMUL: InstructionOpcode = 0x14F0;
pub const OP_CLMULR: InstructionOpcode = 0x15F0;
pub const OP_CLMULH: InstructionOpcode = 0x16F0;
pub const OP_CLMULW: InstructionOpcode = 0x17F0;
pub const OP_CLMULRW: InstructionOpcode = 0x18F0;
pub const OP_CLMULHW: InstructionOpcode = 0x19F0;
pub const OP_CRC32B: InstructionOpcode = 0x1AF0;
pub const OP_CRC32H: InstructionOpcode = 0x1BF0;
pub const OP_CRC32W: InstructionOpcode = 0x1CF0;
pub const OP_CRC32D: InstructionOpcode = 0x1DF0;
pub const OP_CRC32CB: InstructionOpcode = 0x1EF0;
pub const OP_CRC32CH: InstructionOpcode = 0x1FF0;
pub const OP_CRC32CW: InstructionOpcode = 0x20F0;
pub const OP_CRC32CD: InstructionOpcode = 0x21F0;
pub const OP_BMATOR: InstructionOpcode = 0x22F0;
pub const OP_BMATXOR: InstructionOpcode = 0x23F0;
pub const OP_BMATFLIP: InstructionOpcode = 0x24F0;

pub const MINIMAL_LEVEL1_OPCODE: InstructionOpcode = OP_UNLOADED;
pub const MAXIMUM_LEVEL1_OPCODE: InstructionOpcode = OP_CUSTOM_TRACE_END;
pub const LEVEL2_B_OPCODE: InstructionOpcode = 0xF0;
pub const MINIMAL_LEVEL2_B_OPCODE2: InstructionOpcode = 0x00;
pub const MAXIMUM_LEVEL2_B_OPCODE2: InstructionOpcode = 0x24;

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
    "CLZ",
    "CTZ",
    "PCNT",
    "CLZW",
    "CTZW",
    "PCNTW",
    "ANDN",
    "ORN",
    "XNOR",
    "PACK",
    "PACKU",
    "PACKH",
    "PACKW",
    "PACKUW",
    "MIN",
    "MAX",
    "MINU",
    "MAXU",
    "SEXTB",
    "SEXTH",
    "SBCLR",
    "SBSET",
    "SBINV",
    "SBEXT",
    "SBCLRI",
    "SBSETI",
    "SBINVI",
    "SBEXTI",
    "SBCLRW",
    "SBSETW",
    "SBINVW",
    "SBEXTW",
    "SBCLRIW",
    "SBSETIW",
    "SBINVIW",
    "SLO",
    "SRO",
    "SLOI",
    "SROI",
    "SLOW",
    "SROW",
    "SLOIW",
    "SROIW",
    "ROL",
    "ROR",
    "RORI",
    "ROLW",
    "RORW",
    "RORIW",
    "CMIX",
    "CMOV",
    "FSL",
    "FSR",
    "FSRI",
    "FSLW",
    "FSRW",
    "FSRIW",
    "SH1ADD",
    "SH2ADD",
    "SH3ADD",
    "SH1ADDUW",
    "SH2ADDUW",
    "SH3ADDUW",
    "ADDWU",
    "SUBWU",
    "ADDIWU",
    "ADDUW",
    "SUBUW",
    "SLLIUW",
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
    "TWINS_LD",
    "TWINS_SD",
    "CUSTOM_LOAD_IMM",
    "CUSTOM_TRACE_END",
];

pub const INSTRUCTION_OPCODE_NAMES_LEVEL2_B: [&str; MAXIMUM_LEVEL2_B_OPCODE2 as usize + 1] = [
    "GREV", "GREVI", "GREVW", "GREVIW", "SHFL", "UNSHFL", "SHFLI", "UNSHFLI", "SHFLW", "UNSHFLW",
    "GORC", "GORCI", "GORCW", "GORCIW", "BFP", "BFPW", "BDEP", "BEXT", "BDEPW", "BEXTW", "CLMUL",
    "CLMULR", "CLMULH", "CLMULW", "CLMULRW", "CLMULHW", "CRC32B", "CRC32H", "CRC32W", "CRC32D",
    "CRC32CB", "CRC32CH", "CRC32CW", "CRC32CD", "BMATOR", "BMATXOR", "BMATFLIP",
];

pub fn instruction_opcode_name(i: InstructionOpcode) -> &'static str {
    if i & 0xFF == LEVEL2_B_OPCODE {
        return INSTRUCTION_OPCODE_NAMES_LEVEL2_B[i as usize >> 8];
    }
    INSTRUCTION_OPCODE_NAMES_LEVEL1[i as usize]
}
