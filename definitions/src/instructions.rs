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
// will be much faster than the original compact form. The decoder translates
// RISC-V instructions into this internal format first, then both the
// interpreter and the asm backend execute the decoded opcode stream directly.
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
            (UNLOADED, 0x00),
            (ADD, 0x01),
            (ADDI, 0x02),
            (ADDIW, 0x03),
            (ADDW, 0x04),
            (AND, 0x05),
            (ANDI, 0x06),
            (DIV, 0x07),
            (DIVU, 0x08),
            (DIVUW, 0x09),
            (DIVW, 0x0a),
            (LB_VERSION0, 0x0b),
            (LB_VERSION1, 0x0c),
            (LBU_VERSION0, 0x0d),
            (LBU_VERSION1, 0x0e),
            (LD_VERSION0, 0x0f),
            (LD_VERSION1, 0x10),
            (LH_VERSION0, 0x11),
            (LH_VERSION1, 0x12),
            (LHU_VERSION0, 0x13),
            (LHU_VERSION1, 0x14),
            (LUI, 0x15),
            (LW_VERSION0, 0x16),
            (LW_VERSION1, 0x17),
            (LWU_VERSION0, 0x18),
            (LWU_VERSION1, 0x19),
            (MUL, 0x1a),
            (MULH, 0x1b),
            (MULHSU, 0x1c),
            (MULHU, 0x1d),
            (MULW, 0x1e),
            (OR, 0x1f),
            (ORI, 0x20),
            (REM, 0x21),
            (REMU, 0x22),
            (REMUW, 0x23),
            (REMW, 0x24),
            (SB, 0x25),
            (SD, 0x26),
            (SH, 0x27),
            (SLL, 0x28),
            (SLLI, 0x29),
            (SLLIW, 0x2a),
            (SLLW, 0x2b),
            (SLT, 0x2c),
            (SLTI, 0x2d),
            (SLTIU, 0x2e),
            (SLTU, 0x2f),
            (SRA, 0x30),
            (SRAI, 0x31),
            (SRAIW, 0x32),
            (SRAW, 0x33),
            (SRL, 0x34),
            (SRLI, 0x35),
            (SRLIW, 0x36),
            (SRLW, 0x37),
            (SUB, 0x38),
            (SUBW, 0x39),
            (SW, 0x3a),
            (XOR, 0x3b),
            (XORI, 0x3c),
            // B
            (ADDUW, 0x3d),
            (ANDN, 0x3e),
            (BCLR, 0x3f),
            (BCLRI, 0x40),
            (BEXT, 0x41),
            (BEXTI, 0x42),
            (BINV, 0x43),
            (BINVI, 0x44),
            (BSET, 0x45),
            (BSETI, 0x46),
            (CLMUL, 0x47),
            (CLMULH, 0x48),
            (CLMULR, 0x49),
            (CLZ, 0x4a),
            (CLZW, 0x4b),
            (CPOP, 0x4c),
            (CPOPW, 0x4d),
            (CTZ, 0x4e),
            (CTZW, 0x4f),
            (MAX, 0x50),
            (MAXU, 0x51),
            (MIN, 0x52),
            (MINU, 0x53),
            (ORCB, 0x54),
            (ORN, 0x55),
            (REV8, 0x56),
            (ROL, 0x57),
            (ROLW, 0x58),
            (ROR, 0x59),
            (RORI, 0x5a),
            (RORIW, 0x5b),
            (RORW, 0x5c),
            (SEXTB, 0x5d),
            (SEXTH, 0x5e),
            (SH1ADD, 0x5f),
            (SH1ADDUW, 0x60),
            (SH2ADD, 0x61),
            (SH2ADDUW, 0x62),
            (SH3ADD, 0x63),
            (SH3ADDUW, 0x64),
            (SLLIUW, 0x65),
            (XNOR, 0x66),
            (ZEXTH, 0x67),
            // Mop
            (WIDE_MUL, 0x68),
            (WIDE_MULU, 0x69),
            (WIDE_MULSU, 0x6a),
            (WIDE_DIV, 0x6b),
            (WIDE_DIVU, 0x6c),
            (ADC, 0x6d),
            (SBB, 0x6e),
            (ADCS, 0x6f),
            (SBBS, 0x70),
            (ADD3A, 0x71),
            (ADD3B, 0x72),
            (ADD3C, 0x73),
            (CUSTOM_LOAD_UIMM, 0x74),
            (CUSTOM_LOAD_IMM, 0x75),
            // All branches
            (AUIPC, 0x76),
            (BEQ, 0x77),
            (BGE, 0x78),
            (BGEU, 0x79),
            (BLT, 0x7a),
            (BLTU, 0x7b),
            (BNE, 0x7c),
            (EBREAK, 0x7d),
            (ECALL, 0x7e),
            (FENCE, 0x7f),
            (FENCEI, 0x80),
            (JAL, 0x81),
            (JALR_VERSION0, 0x82),
            (JALR_VERSION1, 0x83),
            (FAR_JUMP_REL, 0x84),
            (FAR_JUMP_ABS, 0x85),
            (CUSTOM_ASM_TRACE_JUMP, 0x86),
            (CUSTOM_TRACE_END, 0x87)
        );
    };
}

/// Generates a possible definition for each instruction, it leverages
/// a callback macro that takes (at least) 3 arguments:
///
/// 1. $name: an identifier containing the full defined opcode name,
///    e.g., OP_ADD
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
///   macro in +for_each_inst+
/// * A value expression containing the actual value to match against.
/// * An expression used as wildcard matches when the passed value does
///   not match any opcode
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
///   macro in +for_each_inst+
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
