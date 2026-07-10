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
            // A
            (LR_W, 0x3d),
            (SC_W, 0x3e),
            (AMOSWAP_W, 0x3f),
            (AMOADD_W, 0x40),
            (AMOXOR_W, 0x41),
            (AMOAND_W, 0x42),
            (AMOOR_W, 0x43),
            (AMOMIN_W, 0x44),
            (AMOMAX_W, 0x45),
            (AMOMINU_W, 0x46),
            (AMOMAXU_W, 0x47),
            (LR_D, 0x48),
            (SC_D, 0x49),
            (AMOSWAP_D, 0x4a),
            (AMOADD_D, 0x4b),
            (AMOXOR_D, 0x4c),
            (AMOAND_D, 0x4d),
            (AMOOR_D, 0x4e),
            (AMOMIN_D, 0x4f),
            (AMOMAX_D, 0x50),
            (AMOMINU_D, 0x51),
            (AMOMAXU_D, 0x52),
            // B
            (ADDUW, 0x53),
            (ANDN, 0x54),
            (BCLR, 0x55),
            (BCLRI, 0x56),
            (BEXT, 0x57),
            (BEXTI, 0x58),
            (BINV, 0x59),
            (BINVI, 0x5a),
            (BSET, 0x5b),
            (BSETI, 0x5c),
            (CLMUL, 0x5d),
            (CLMULH, 0x5e),
            (CLMULR, 0x5f),
            (CLZ, 0x60),
            (CLZW, 0x61),
            (CPOP, 0x62),
            (CPOPW, 0x63),
            (CTZ, 0x64),
            (CTZW, 0x65),
            (MAX, 0x66),
            (MAXU, 0x67),
            (MIN, 0x68),
            (MINU, 0x69),
            (ORCB, 0x6a),
            (ORN, 0x6b),
            (REV8, 0x6c),
            (ROL, 0x6d),
            (ROLW, 0x6e),
            (ROR, 0x6f),
            (RORI, 0x70),
            (RORIW, 0x71),
            (RORW, 0x72),
            (SEXTB, 0x73),
            (SEXTH, 0x74),
            (SH1ADD, 0x75),
            (SH1ADDUW, 0x76),
            (SH2ADD, 0x77),
            (SH2ADDUW, 0x78),
            (SH3ADD, 0x79),
            (SH3ADDUW, 0x7a),
            (SLLIUW, 0x7b),
            (XNOR, 0x7c),
            (ZEXTH, 0x7d),
            // Mop
            (WIDE_MUL, 0x7e),
            (WIDE_MULU, 0x7f),
            (WIDE_MULSU, 0x80),
            (WIDE_DIV, 0x81),
            (WIDE_DIVU, 0x82),
            (ADC, 0x83),
            (SBB, 0x84),
            (ADCS, 0x85),
            (SBBS, 0x86),
            (ADD3A, 0x87),
            (ADD3B, 0x88),
            (ADD3C, 0x89),
            (CUSTOM_LOAD_UIMM, 0x8a),
            (CUSTOM_LOAD_IMM, 0x8b),
            // All branches
            (AUIPC, 0x8c),
            (BEQ, 0x8d),
            (BGE, 0x8e),
            (BGEU, 0x8f),
            (BLT, 0x90),
            (BLTU, 0x91),
            (BNE, 0x92),
            (EBREAK, 0x93),
            (ECALL, 0x94),
            (FENCE, 0x95),
            (FENCEI, 0x96),
            (JAL, 0x97),
            (JALR_VERSION0, 0x98),
            (JALR_VERSION1, 0x99),
            (FAR_JUMP_REL, 0x9a),
            (FAR_JUMP_ABS, 0x9b),
            (CUSTOM_ASM_TRACE_JUMP, 0x9c),
            (CUSTOM_TRACE_END, 0x9d)
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
