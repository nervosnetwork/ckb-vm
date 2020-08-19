// For fast decoding and cache friendly, RISC-V instruction is decoded
// into 64 bit unsigned integer in the following format:
//
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |           | rs1 | rs2 | res |     | rd  | op  | R-type
// +-----------+-----------------------------------+
// |    immediate    | rs1 | res |     | rd  | op  | I-type
// +-----------------------------------------------+
// |    immediate    | rs1 | res |     | rs2 | op  | S-type/B-type
// +-----------------+-----------------------------+
// |       immediate       | res |     | rd  | op  | U-type/J-type
// +-----+-----+-----+-----+-----+-----+-----+-----+
//
// +res+ here means reserved field that is not yet used.
//
// This way each op and register index are in full byte, accessing them
// will be much faster than the original compact form. Hence we will have
// a fast path where the interpreter loop reads instruction directly in this
// format, and a slow path where a full featured decoder decodes RISC-V
// instruction into the internal form here(much like how traces/micro-ops work.)
pub type Instruction = u64;

pub type InstructionOpcode = u8;

pub const OP_UNLOADED: InstructionOpcode = 0;
pub const OP_ADD: InstructionOpcode = 1;
pub const OP_ADDI: InstructionOpcode = 2;
pub const OP_ADDIW: InstructionOpcode = 3;
pub const OP_ADDW: InstructionOpcode = 4;
pub const OP_AND: InstructionOpcode = 5;
pub const OP_ANDI: InstructionOpcode = 6;
pub const OP_AUIPC: InstructionOpcode = 7;
pub const OP_BEQ: InstructionOpcode = 8;
pub const OP_BGE: InstructionOpcode = 9;
pub const OP_BGEU: InstructionOpcode = 10;
pub const OP_BLT: InstructionOpcode = 11;
pub const OP_BLTU: InstructionOpcode = 12;
pub const OP_BNE: InstructionOpcode = 13;
pub const OP_DIV: InstructionOpcode = 14;
pub const OP_DIVU: InstructionOpcode = 15;
pub const OP_DIVUW: InstructionOpcode = 16;
pub const OP_DIVW: InstructionOpcode = 17;
pub const OP_EBREAK: InstructionOpcode = 18;
pub const OP_ECALL: InstructionOpcode = 19;
pub const OP_FENCE: InstructionOpcode = 20;
pub const OP_FENCEI: InstructionOpcode = 21;
pub const OP_JAL: InstructionOpcode = 22;
pub const OP_JALR: InstructionOpcode = 23;
pub const OP_LB: InstructionOpcode = 24;
pub const OP_LBU: InstructionOpcode = 25;
pub const OP_LD: InstructionOpcode = 26;
pub const OP_LH: InstructionOpcode = 27;
pub const OP_LHU: InstructionOpcode = 28;
pub const OP_LUI: InstructionOpcode = 29;
pub const OP_LW: InstructionOpcode = 30;
pub const OP_LWU: InstructionOpcode = 31;
pub const OP_MUL: InstructionOpcode = 32;
pub const OP_MULH: InstructionOpcode = 33;
pub const OP_MULHSU: InstructionOpcode = 34;
pub const OP_MULHU: InstructionOpcode = 35;
pub const OP_MULW: InstructionOpcode = 36;
pub const OP_OR: InstructionOpcode = 37;
pub const OP_ORI: InstructionOpcode = 38;
pub const OP_REM: InstructionOpcode = 39;
pub const OP_REMU: InstructionOpcode = 40;
pub const OP_REMUW: InstructionOpcode = 41;
pub const OP_REMW: InstructionOpcode = 42;
pub const OP_SB: InstructionOpcode = 43;
pub const OP_SD: InstructionOpcode = 44;
pub const OP_SH: InstructionOpcode = 45;
pub const OP_SLL: InstructionOpcode = 46;
pub const OP_SLLI: InstructionOpcode = 47;
pub const OP_SLLIW: InstructionOpcode = 48;
pub const OP_SLLW: InstructionOpcode = 49;
pub const OP_SLT: InstructionOpcode = 50;
pub const OP_SLTI: InstructionOpcode = 51;
pub const OP_SLTIU: InstructionOpcode = 52;
pub const OP_SLTU: InstructionOpcode = 53;
pub const OP_SRA: InstructionOpcode = 54;
pub const OP_SRAI: InstructionOpcode = 55;
pub const OP_SRAIW: InstructionOpcode = 56;
pub const OP_SRAW: InstructionOpcode = 57;
pub const OP_SRL: InstructionOpcode = 58;
pub const OP_SRLI: InstructionOpcode = 59;
pub const OP_SRLIW: InstructionOpcode = 60;
pub const OP_SRLW: InstructionOpcode = 61;
pub const OP_SUB: InstructionOpcode = 62;
pub const OP_SUBW: InstructionOpcode = 63;
pub const OP_SW: InstructionOpcode = 64;
pub const OP_VERSION1_JALR: InstructionOpcode = 65;
pub const OP_XOR: InstructionOpcode = 66;
pub const OP_XORI: InstructionOpcode = 67;
pub const OP_RVC_ADD: InstructionOpcode = 68;
pub const OP_RVC_ADDI: InstructionOpcode = 69;
pub const OP_RVC_ADDI16SP: InstructionOpcode = 70;
pub const OP_RVC_ADDI4SPN: InstructionOpcode = 71;
pub const OP_RVC_ADDIW: InstructionOpcode = 72;
pub const OP_RVC_ADDW: InstructionOpcode = 73;
pub const OP_RVC_AND: InstructionOpcode = 74;
pub const OP_RVC_ANDI: InstructionOpcode = 75;
pub const OP_RVC_BEQZ: InstructionOpcode = 76;
pub const OP_RVC_BNEZ: InstructionOpcode = 77;
pub const OP_RVC_EBREAK: InstructionOpcode = 78;
pub const OP_RVC_J: InstructionOpcode = 79;
pub const OP_RVC_JAL: InstructionOpcode = 80;
pub const OP_RVC_JALR: InstructionOpcode = 81;
pub const OP_RVC_JR: InstructionOpcode = 82;
pub const OP_RVC_LD: InstructionOpcode = 83;
pub const OP_RVC_LDSP: InstructionOpcode = 84;
pub const OP_RVC_LI: InstructionOpcode = 85;
pub const OP_RVC_LUI: InstructionOpcode = 86;
pub const OP_RVC_LW: InstructionOpcode = 87;
pub const OP_RVC_LWSP: InstructionOpcode = 88;
pub const OP_RVC_MV: InstructionOpcode = 89;
pub const OP_RVC_NOP: InstructionOpcode = 90;
pub const OP_RVC_OR: InstructionOpcode = 91;
pub const OP_RVC_SD: InstructionOpcode = 92;
pub const OP_RVC_SDSP: InstructionOpcode = 93;
pub const OP_RVC_SLLI: InstructionOpcode = 94;
pub const OP_RVC_SLLI64: InstructionOpcode = 95;
pub const OP_RVC_SRAI: InstructionOpcode = 96;
pub const OP_RVC_SRAI64: InstructionOpcode = 97;
pub const OP_RVC_SRLI: InstructionOpcode = 98;
pub const OP_RVC_SRLI64: InstructionOpcode = 99;
pub const OP_RVC_SUB: InstructionOpcode = 100;
pub const OP_RVC_SUBW: InstructionOpcode = 101;
pub const OP_RVC_SW: InstructionOpcode = 102;
pub const OP_RVC_SWSP: InstructionOpcode = 103;
pub const OP_RVC_XOR: InstructionOpcode = 104;
pub const OP_VERSION1_RVC_JALR: InstructionOpcode = 105;
pub const OP_CUSTOM_LOAD_IMM: InstructionOpcode = 106;
pub const OP_CUSTOM_TRACE_END: InstructionOpcode = 107;

// Maximum opcode for instructions consuming 4 bytes. Any opcode
// larger than this one is treated as RVC instructions(which consume
// 2 bytes)
pub const MAXIMUM_OPCODE: InstructionOpcode = OP_CUSTOM_TRACE_END;

pub const MINIMAL_RVC_OPCODE: InstructionOpcode = OP_RVC_ADD;
pub const MAXIMUM_RVC_OPCODE: InstructionOpcode = OP_VERSION1_RVC_JALR;

pub const INSTRUCTION_OPCODE_NAMES: [&str; MAXIMUM_OPCODE as usize + 1] = [
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
    "VERSION1_JALR",
    "XOR",
    "XORI",
    "RVC_ADD",
    "RVC_ADDI",
    "RVC_ADDI16SP",
    "RVC_ADDI4SPN",
    "RVC_ADDIW",
    "RVC_ADDW",
    "RVC_AND",
    "RVC_ANDI",
    "RVC_BEQZ",
    "RVC_BNEZ",
    "RVC_EBREAK",
    "RVC_J",
    "RVC_JAL",
    "RVC_JALR",
    "RVC_JR",
    "RVC_LD",
    "RVC_LDSP",
    "RVC_LI",
    "RVC_LUI",
    "RVC_LW",
    "RVC_LWSP",
    "RVC_MV",
    "RVC_NOP",
    "RVC_OR",
    "RVC_SD",
    "RVC_SDSP",
    "RVC_SLLI",
    "RVC_SLLI64",
    "RVC_SRAI",
    "RVC_SRAI64",
    "RVC_SRLI",
    "RVC_SRLI64",
    "RVC_SUB",
    "RVC_SUBW",
    "RVC_SW",
    "RVC_SWSP",
    "RVC_XOR",
    "VERSION1_RVC_JALR",
    "CUSTOM_LOAD_IMM",
    "CUSTOM_TRACE_END",
];
