// For fast decoding and cache friendly, RISC-V instruction is decoded
// into 64 bit unsigned integer in the following format:
//
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |           | rs2 | rs1 | flg | op2 | rd  | op  | R-type
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |     | rs3 | rs2 | rs1 | flg | op2 | rd  | op  | R4-type
// +-----------+-----------------------------------+
// | rs4 | rs3 | rs2 | rs1 | flg | op2 | rd  | op  | R5-type
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
// When the op value is 0x10-0xff, it expresses a first-level instruction under fast
// path, at this time the value of op2 is ignored.
// When the op value is 0x00-0x0f, op and op2 are combined to express a
// second-level instruction under slow path.
//
// Notice that this module now uses macro-based techniques to define opcodes.
// To see a full list of opcodes as plain Rust source code, install
// [cargo-expand](https://github.com/dtolnay/cargo-expand) first, then use the
// following command:
//
// cargo expand --manifest-path=definitions/Cargo.toml --lib instructions
pub type Instruction = u64;

pub type InstructionOpcode = u16;

pub use paste::paste;

#[doc(hidden)]
#[macro_export]
macro_rules! __apply {
    ($callback:ident, $(($name:ident, $code:expr)),*) => {
        $crate::instructions::paste! {
            $(
                $callback!([< OP_ $name >], $name, $code);
            )*
        }
    };
    ((1, $res:ident, $x:expr, $callback:ident, $others:expr), $(($name:ident, $code:expr)),*) => {
        $crate::instructions::paste! {
            let $res = match $x {
                $( $code => $callback!([< OP_ $name >], $name, $code), )*
                _ => $others
            };
        }
    };
    ((2, $x:ident, $callback:ident), $(($name:ident, $code:expr)),*) => {
        $crate::instructions::paste! {
            $(
                $callback!([< OP_ $name >], $name, $code, $x);
            )*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __for_each_inst_inner {
    ($callback:tt) => {
        $crate::__apply!(
            $callback,
            // IMC
            (UNLOADED, 0x10),
            (ADD, 0x11),
            (ADDI, 0x12),
            (ADDIW, 0x13),
            (ADDW, 0x14),
            (AND, 0x15),
            (ANDI, 0x16),
            (AUIPC, 0x17),
            (BEQ, 0x18),
            (BGE, 0x19),
            (BGEU, 0x1a),
            (BLT, 0x1b),
            (BLTU, 0x1c),
            (BNE, 0x1d),
            (DIV, 0x1e),
            (DIVU, 0x1f),
            (DIVUW, 0x20),
            (DIVW, 0x21),
            (EBREAK, 0x22),
            (ECALL, 0x23),
            (FENCE, 0x24),
            (FENCEI, 0x25),
            (JAL, 0x26),
            (JALR_VERSION0, 0x27),
            (JALR_VERSION1, 0x28),
            (LB_VERSION0, 0x29),
            (LB_VERSION1, 0x2a),
            (LBU_VERSION0, 0x2b),
            (LBU_VERSION1, 0x2c),
            (LD_VERSION0, 0x2d),
            (LD_VERSION1, 0x2e),
            (LH_VERSION0, 0x2f),
            (LH_VERSION1, 0x30),
            (LHU_VERSION0, 0x31),
            (LHU_VERSION1, 0x32),
            (LUI, 0x33),
            (LW_VERSION0, 0x34),
            (LW_VERSION1, 0x35),
            (LWU_VERSION0, 0x36),
            (LWU_VERSION1, 0x37),
            (MUL, 0x38),
            (MULH, 0x39),
            (MULHSU, 0x3a),
            (MULHU, 0x3b),
            (MULW, 0x3c),
            (OR, 0x3d),
            (ORI, 0x3e),
            (REM, 0x3f),
            (REMU, 0x40),
            (REMUW, 0x41),
            (REMW, 0x42),
            (SB, 0x43),
            (SD, 0x44),
            (SH, 0x45),
            (SLL, 0x46),
            (SLLI, 0x47),
            (SLLIW, 0x48),
            (SLLW, 0x49),
            (SLT, 0x4a),
            (SLTI, 0x4b),
            (SLTIU, 0x4c),
            (SLTU, 0x4d),
            (SRA, 0x4e),
            (SRAI, 0x4f),
            (SRAIW, 0x50),
            (SRAW, 0x51),
            (SRL, 0x52),
            (SRLI, 0x53),
            (SRLIW, 0x54),
            (SRLW, 0x55),
            (SUB, 0x56),
            (SUBW, 0x57),
            (SW, 0x58),
            (XOR, 0x59),
            (XORI, 0x5a),
            // A
            (LR_W, 0x5b),
            (SC_W, 0x5c),
            (AMOSWAP_W, 0x5d),
            (AMOADD_W, 0x5e),
            (AMOXOR_W, 0x5f),
            (AMOAND_W, 0x60),
            (AMOOR_W, 0x61),
            (AMOMIN_W, 0x62),
            (AMOMAX_W, 0x63),
            (AMOMINU_W, 0x64),
            (AMOMAXU_W, 0x65),
            (LR_D, 0x66),
            (SC_D, 0x67),
            (AMOSWAP_D, 0x68),
            (AMOADD_D, 0x69),
            (AMOXOR_D, 0x6a),
            (AMOAND_D, 0x6b),
            (AMOOR_D, 0x6c),
            (AMOMIN_D, 0x6d),
            (AMOMAX_D, 0x6e),
            (AMOMINU_D, 0x6f),
            (AMOMAXU_D, 0x70),
            // B
            (ADDUW, 0x71),
            (ANDN, 0x72),
            (BCLR, 0x73),
            (BCLRI, 0x74),
            (BEXT, 0x75),
            (BEXTI, 0x76),
            (BINV, 0x77),
            (BINVI, 0x78),
            (BSET, 0x79),
            (BSETI, 0x7a),
            (CLMUL, 0x7b),
            (CLMULH, 0x7c),
            (CLMULR, 0x7d),
            (CLZ, 0x7e),
            (CLZW, 0x7f),
            (CPOP, 0x80),
            (CPOPW, 0x81),
            (CTZ, 0x82),
            (CTZW, 0x83),
            (MAX, 0x84),
            (MAXU, 0x85),
            (MIN, 0x86),
            (MINU, 0x87),
            (ORCB, 0x88),
            (ORN, 0x89),
            (REV8, 0x8a),
            (ROL, 0x8b),
            (ROLW, 0x8c),
            (ROR, 0x8d),
            (RORI, 0x8e),
            (RORIW, 0x8f),
            (RORW, 0x90),
            (SEXTB, 0x91),
            (SEXTH, 0x92),
            (SH1ADD, 0x93),
            (SH1ADDUW, 0x94),
            (SH2ADD, 0x95),
            (SH2ADDUW, 0x96),
            (SH3ADD, 0x97),
            (SH3ADDUW, 0x98),
            (SLLIUW, 0x99),
            (XNOR, 0x9a),
            (ZEXTH, 0x9b),
            // Mop
            (WIDE_MUL, 0x9c),
            (WIDE_MULU, 0x9d),
            (WIDE_MULSU, 0x9e),
            (WIDE_DIV, 0x9f),
            (WIDE_DIVU, 0xa0),
            (FAR_JUMP_REL, 0xa1),
            (FAR_JUMP_ABS, 0xa2),
            (ADC, 0xa3),
            (SBB, 0xa4),
            (ADCS, 0xa5),
            (SBBS, 0xa6),
            (ADD3A, 0xa7),
            (ADD3B, 0xa8),
            (ADD3C, 0xa9),
            (CUSTOM_LOAD_UIMM, 0xaa),
            (CUSTOM_LOAD_IMM, 0xab),
            (CUSTOM_TRACE_END, 0xac)
        );
    };
}

/// Generates a possible definition for each instruction, it leverages
/// a callback macro that takes 3 arguments:
///
/// 1. $name: an identifier containing the full defined opcode name,
/// e.g., OP_ADD
/// 2. $real_name: an identifier containing just the opcode part, e.g., ADD
/// 3. $code: an expr containing the actual opcode number
#[macro_export]
macro_rules! for_each_inst {
    ($callback:ident) => {
        $crate::__for_each_inst_inner!($callback);
    };
}

/// Generates a match expression containing all instructions, it takes 3
/// arguments:
///
/// * A callback macro that takes the exact same arguments as callback
/// macro in +for_each_inst+
/// * A value expression containing the actual value to match against.
/// * An expression used as wildcard matches when the passed value does
/// not match any opcode
#[macro_export]
macro_rules! for_each_inst_match {
    ($callback:ident, $val:expr, $others:expr) => {{
        $crate::__for_each_inst_inner!((1, __res__, $val, $callback, $others));
        __res__
    }};
}

/// Generates a match expression doing fold on all instructions
///
/// * A callback macro that takes 4 arguments: the first 3 arguments are
/// exactly the same as the 3 callback arguments in +for_each_inst+, the
/// 4th argument here is the identifier below to work with hygienic macros.
/// * An identifier for a mutable variable used for folding
#[macro_export]
macro_rules! for_each_inst_fold {
    ($callback:ident, $x:ident) => {
        $crate::__for_each_inst_inner!((2, $x, $callback));
    };
}

macro_rules! define_instruction {
    ($name:ident, $real_name:ident, $code:expr) => {
        pub const $name: InstructionOpcode = $code;
    };
}
for_each_inst!(define_instruction);

pub const MINIMAL_OPCODE: InstructionOpcode = OP_UNLOADED;
pub const MAXIMUM_OPCODE: InstructionOpcode = OP_CUSTOM_TRACE_END;

macro_rules! inst_real_name {
    ($name:ident, $real_name:ident, $code:expr) => {
        stringify!($real_name)
    };
}

pub fn instruction_opcode_name(i: InstructionOpcode) -> &'static str {
    for_each_inst_match!(inst_real_name, i, "UNKNOWN_INSTRUCTION!")
}
