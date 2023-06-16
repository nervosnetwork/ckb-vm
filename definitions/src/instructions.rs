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
    ((0, $callback:ident), $(($name:ident, $code:expr)),*) => {
        $crate::instructions::paste! {
            $(
                $callback!([< OP_ $name >], $name, $code);
            )*
        }
    };
    ((1, $x:ident, $callback:ident), $(($name:ident, $code:expr)),*) => {
        $crate::instructions::paste! {
            $(
                $callback!([< OP_ $name >], $name, $code, $x);
            )*
        }
    };
    ((2, $x:ident, $y:ident, $callback:ident), $(($name:ident, $code:expr)),*) => {
        $crate::instructions::paste! {
            $(
                $callback!([< OP_ $name >], $name, $code, $x, $y);
            )*
        }
    };
    ((100, $res:ident, $val:expr, $callback:ident, $others:expr), $(($name:ident, $code:expr)),*) => {
        $crate::instructions::paste! {
            let $res = match $val {
                $( $code => $callback!([< OP_ $name >], $name, $code), )*
                _ => $others
            };
        }
    };
    ((101, $x:ident, $res:ident, $val:expr, $callback:ident, $others:expr), $(($name:ident, $code:expr)),*) => {
        $crate::instructions::paste! {
            let $res = match $val {
                $( $code => $callback!([< OP_ $name >], $name, $code, $x), )*
                _ => $others
            };
        }
    };
    ((102, $x:ident, $y:ident, $res:ident, $val:expr, $callback:ident, $others:expr), $(($name:ident, $code:expr)),*) => {
        $crate::instructions::paste! {
            let $res = match $val {
                $( $code => $callback!([< OP_ $name >], $name, $code, $x, $y), )*
                _ => $others
            };
        }
    };
    ((200, $res:ident, $callback:ident), $(($name:ident, $code:expr)),*) => {
        $crate::instructions::paste! {
            let $res = [
                $( $callback!([< OP_ $name >], $name, $code), )*
            ];
        }
    };
    ((201, $x:ident, $res:ident, $callback:ident), $(($name:ident, $code:expr)),*) => {
        $crate::instructions::paste! {
            let $res = [
                $( $callback!([< OP_ $name >], $name, $code, $x), )*
            ];
        }
    };
    ((202, $x:ident, $y:ident, $res:ident, $callback:ident), $(($name:ident, $code:expr)),*) => {
        $crate::instructions::paste! {
            let $res = [
                $( $callback!([< OP_ $name >], $name, $code, $x, $y), )*
            ];
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
            (DIV, 0x17),
            (DIVU, 0x18),
            (DIVUW, 0x19),
            (DIVW, 0x1a),
            (LB_VERSION0, 0x1b),
            (LB_VERSION1, 0x1c),
            (LBU_VERSION0, 0x1d),
            (LBU_VERSION1, 0x1e),
            (LD_VERSION0, 0x1f),
            (LD_VERSION1, 0x20),
            (LH_VERSION0, 0x21),
            (LH_VERSION1, 0x22),
            (LHU_VERSION0, 0x23),
            (LHU_VERSION1, 0x24),
            (LUI, 0x25),
            (LW_VERSION0, 0x26),
            (LW_VERSION1, 0x27),
            (LWU_VERSION0, 0x28),
            (LWU_VERSION1, 0x29),
            (MUL, 0x2a),
            (MULH, 0x2b),
            (MULHSU, 0x2c),
            (MULHU, 0x2d),
            (MULW, 0x2e),
            (OR, 0x2f),
            (ORI, 0x30),
            (REM, 0x31),
            (REMU, 0x32),
            (REMUW, 0x33),
            (REMW, 0x34),
            (SB, 0x35),
            (SD, 0x36),
            (SH, 0x37),
            (SLL, 0x38),
            (SLLI, 0x39),
            (SLLIW, 0x3a),
            (SLLW, 0x3b),
            (SLT, 0x3c),
            (SLTI, 0x3d),
            (SLTIU, 0x3e),
            (SLTU, 0x3f),
            (SRA, 0x40),
            (SRAI, 0x41),
            (SRAIW, 0x42),
            (SRAW, 0x43),
            (SRL, 0x44),
            (SRLI, 0x45),
            (SRLIW, 0x46),
            (SRLW, 0x47),
            (SUB, 0x48),
            (SUBW, 0x49),
            (SW, 0x4a),
            (XOR, 0x4b),
            (XORI, 0x4c),
            // A
            (LR_W, 0x4d),
            (SC_W, 0x4e),
            (AMOSWAP_W, 0x4f),
            (AMOADD_W, 0x50),
            (AMOXOR_W, 0x51),
            (AMOAND_W, 0x52),
            (AMOOR_W, 0x53),
            (AMOMIN_W, 0x54),
            (AMOMAX_W, 0x55),
            (AMOMINU_W, 0x56),
            (AMOMAXU_W, 0x57),
            (LR_D, 0x58),
            (SC_D, 0x59),
            (AMOSWAP_D, 0x5a),
            (AMOADD_D, 0x5b),
            (AMOXOR_D, 0x5c),
            (AMOAND_D, 0x5d),
            (AMOOR_D, 0x5e),
            (AMOMIN_D, 0x5f),
            (AMOMAX_D, 0x60),
            (AMOMINU_D, 0x61),
            (AMOMAXU_D, 0x62),
            // B
            (ADDUW, 0x63),
            (ANDN, 0x64),
            (BCLR, 0x65),
            (BCLRI, 0x66),
            (BEXT, 0x67),
            (BEXTI, 0x68),
            (BINV, 0x69),
            (BINVI, 0x6a),
            (BSET, 0x6b),
            (BSETI, 0x6c),
            (CLMUL, 0x6d),
            (CLMULH, 0x6e),
            (CLMULR, 0x6f),
            (CLZ, 0x70),
            (CLZW, 0x71),
            (CPOP, 0x72),
            (CPOPW, 0x73),
            (CTZ, 0x74),
            (CTZW, 0x75),
            (MAX, 0x76),
            (MAXU, 0x77),
            (MIN, 0x78),
            (MINU, 0x79),
            (ORCB, 0x7a),
            (ORN, 0x7b),
            (REV8, 0x7c),
            (ROL, 0x7d),
            (ROLW, 0x7e),
            (ROR, 0x7f),
            (RORI, 0x80),
            (RORIW, 0x81),
            (RORW, 0x82),
            (SEXTB, 0x83),
            (SEXTH, 0x84),
            (SH1ADD, 0x85),
            (SH1ADDUW, 0x86),
            (SH2ADD, 0x87),
            (SH2ADDUW, 0x88),
            (SH3ADD, 0x89),
            (SH3ADDUW, 0x8a),
            (SLLIUW, 0x8b),
            (XNOR, 0x8c),
            (ZEXTH, 0x8d),
            // Mop
            (WIDE_MUL, 0x8e),
            (WIDE_MULU, 0x8f),
            (WIDE_MULSU, 0x90),
            (WIDE_DIV, 0x91),
            (WIDE_DIVU, 0x92),
            (ADC, 0x93),
            (SBB, 0x94),
            (ADCS, 0x95),
            (SBBS, 0x96),
            (ADD3A, 0x97),
            (ADD3B, 0x98),
            (ADD3C, 0x99),
            (CUSTOM_LOAD_UIMM, 0x9a),
            (CUSTOM_LOAD_IMM, 0x9b),
            // All branches
            (AUIPC, 0x9c),
            (BEQ, 0x9d),
            (BGE, 0x9e),
            (BGEU, 0x9f),
            (BLT, 0xa0),
            (BLTU, 0xa1),
            (BNE, 0xa2),
            (EBREAK, 0xa3),
            (ECALL, 0xa4),
            (FENCE, 0xa5),
            (FENCEI, 0xa6),
            (JAL, 0xa7),
            (JALR_VERSION0, 0xa8),
            (JALR_VERSION1, 0xa9),
            (FAR_JUMP_REL, 0xaa),
            (FAR_JUMP_ABS, 0xab),
            (CUSTOM_ASM_TRACE_JUMP, 0xac),
            (CUSTOM_TRACE_END, 0xad)
        );
    };
}

/// Generates a possible definition for each instruction, it leverages
/// a callback macro that takes (at least) 3 arguments:
///
/// 1. $name: an identifier containing the full defined opcode name,
/// e.g., OP_ADD
/// 2. $real_name: an identifier containing just the opcode part, e.g., ADD
/// 3. $code: an expr containing the actual opcode number
///
/// Free variables are attached to the variants ending with inst1, inst2, etc.
/// Those free variables will also be appended as arguments to the callback macro.
#[macro_export]
macro_rules! for_each_inst {
    ($callback:ident) => {
        $crate::__for_each_inst_inner!((0, $callback));
    };
}

#[macro_export]
macro_rules! for_each_inst1 {
    ($callback:ident, $x:ident) => {
        $crate::__for_each_inst_inner!((1, $x, $callback));
    };
}

#[macro_export]
macro_rules! for_each_inst2 {
    ($callback:ident, $x:ident, $y:ident) => {
        $crate::__for_each_inst_inner!((2, $x, $y, $callback));
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
///
/// * Free variables are attached to the variants ending with match1, match2, etc.
#[macro_export]
macro_rules! for_each_inst_match {
    ($callback:ident, $val:expr, $others:expr) => {{
        $crate::__for_each_inst_inner!((100, __res__, $val, $callback, $others));
        __res__
    }};
}

#[macro_export]
macro_rules! for_each_inst_match1 {
    ($callback:ident, $val:expr, $others:expr, $x:ident) => {{
        $crate::__for_each_inst_inner!((101, $x, __res__, $val, $callback, $others));
        __res__
    }};
}

#[macro_export]
macro_rules! for_each_inst_match2 {
    ($callback:ident, $val:expr, $others:expr, $x:ident, $y:ident) => {{
        $crate::__for_each_inst_inner!((102, $x, $y, __res__, $val, $callback, $others));
        __res__
    }};
}

/// Generates an array on all instructions
///
/// * A callback macro that takes the exact same arguments as callback
/// macro in +for_each_inst+
///
/// * Free variables are attached to the variants ending with fold1, fold2, etc.
#[macro_export]
macro_rules! for_each_inst_array {
    ($callback:ident) => {{
        $crate::__for_each_inst_inner!((200, __res__, $callback));
        __res__
    }};
}

#[macro_export]
macro_rules! for_each_inst_array1 {
    ($callback:ident, $x:ident) => {{
        $crate::__for_each_inst_inner!((201, $x, __res__, $callback));
        __res__
    }};
}

#[macro_export]
macro_rules! for_each_inst_array2 {
    ($callback:ident, $x:ident, $y:ident) => {{
        $crate::__for_each_inst_inner!((202, $x, $y, __res__, $callback));
        __res__
    }};
}

// Define the actual opcodes
macro_rules! define_instruction {
    ($name:ident, $real_name:ident, $code:expr) => {
        pub const $name: InstructionOpcode = $code;
    };
}
for_each_inst!(define_instruction);

pub const MINIMAL_OPCODE: InstructionOpcode = OP_UNLOADED;
pub const MAXIMUM_OPCODE: InstructionOpcode = OP_CUSTOM_TRACE_END;

pub const MINIMAL_BASIC_BLOCK_END_OPCODE: InstructionOpcode = OP_AUIPC;
pub const MAXIMUM_BASIC_BLOCK_END_OPCODE: InstructionOpcode = OP_FAR_JUMP_ABS;

macro_rules! inst_real_name {
    ($name:ident, $real_name:ident, $code:expr) => {
        stringify!($real_name)
    };
}

pub fn instruction_opcode_name(i: InstructionOpcode) -> &'static str {
    for_each_inst_match!(inst_real_name, i, "UNKNOWN_INSTRUCTION!")
}
