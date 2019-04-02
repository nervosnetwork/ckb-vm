mod common;
mod execute;
mod register;
mod utils;

pub mod i;
pub mod m;
pub mod rvc;

pub use self::register::Register;
use super::Error;
pub use execute::execute;

type RegisterIndex = usize;
type Immediate = i32;
type UImmediate = u32;

// For fast decoding and cache friendly, RISC-V instruction is decoded
// into 64 bit unsigned integer in the following format:
//
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |           | rs2 | rs1 | res |     | rd  | op  | R-type
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

// ADDI must have an opcode of 0, this way the default 0 value for u64
// happens to be a NOP operation in RISC-V format. Of course using ADD here
// should have the same effect as well but we just want to stick to RISC-V
// established convention here.
pub const OP_ADDI: InstructionOpcode = 0;
pub const OP_ADD: InstructionOpcode = 1;
pub const OP_ADDIW: InstructionOpcode = 2;
pub const OP_ADDW: InstructionOpcode = 3;
pub const OP_AND: InstructionOpcode = 4;
pub const OP_ANDI: InstructionOpcode = 5;
pub const OP_AUIPC: InstructionOpcode = 6;
pub const OP_BEQ: InstructionOpcode = 7;
pub const OP_BGE: InstructionOpcode = 8;
pub const OP_BGEU: InstructionOpcode = 9;
pub const OP_BLT: InstructionOpcode = 10;
pub const OP_BLTU: InstructionOpcode = 11;
pub const OP_BNE: InstructionOpcode = 12;
pub const OP_DIV: InstructionOpcode = 13;
pub const OP_DIVU: InstructionOpcode = 14;
pub const OP_DIVUW: InstructionOpcode = 15;
pub const OP_DIVW: InstructionOpcode = 16;
pub const OP_EBREAK: InstructionOpcode = 17;
pub const OP_ECALL: InstructionOpcode = 18;
pub const OP_FENCE: InstructionOpcode = 19;
pub const OP_FENCEI: InstructionOpcode = 20;
pub const OP_JAL: InstructionOpcode = 21;
pub const OP_JALR: InstructionOpcode = 22;
pub const OP_LB: InstructionOpcode = 23;
pub const OP_LBU: InstructionOpcode = 24;
pub const OP_LD: InstructionOpcode = 25;
pub const OP_LH: InstructionOpcode = 26;
pub const OP_LHU: InstructionOpcode = 27;
pub const OP_LUI: InstructionOpcode = 28;
pub const OP_LW: InstructionOpcode = 29;
pub const OP_LWU: InstructionOpcode = 30;
pub const OP_MUL: InstructionOpcode = 31;
pub const OP_MULH: InstructionOpcode = 32;
pub const OP_MULHSU: InstructionOpcode = 33;
pub const OP_MULHU: InstructionOpcode = 34;
pub const OP_MULW: InstructionOpcode = 35;
pub const OP_OR: InstructionOpcode = 36;
pub const OP_ORI: InstructionOpcode = 37;
pub const OP_REM: InstructionOpcode = 38;
pub const OP_REMU: InstructionOpcode = 39;
pub const OP_REMUW: InstructionOpcode = 40;
pub const OP_REMW: InstructionOpcode = 41;
pub const OP_SB: InstructionOpcode = 42;
pub const OP_SD: InstructionOpcode = 43;
pub const OP_SH: InstructionOpcode = 44;
pub const OP_SLL: InstructionOpcode = 45;
pub const OP_SLLI: InstructionOpcode = 46;
pub const OP_SLLIW: InstructionOpcode = 47;
pub const OP_SLLW: InstructionOpcode = 48;
pub const OP_SLT: InstructionOpcode = 49;
pub const OP_SLTI: InstructionOpcode = 50;
pub const OP_SLTIU: InstructionOpcode = 51;
pub const OP_SLTU: InstructionOpcode = 52;
pub const OP_SRA: InstructionOpcode = 53;
pub const OP_SRAI: InstructionOpcode = 54;
pub const OP_SRAIW: InstructionOpcode = 55;
pub const OP_SRAW: InstructionOpcode = 56;
pub const OP_SRL: InstructionOpcode = 57;
pub const OP_SRLI: InstructionOpcode = 58;
pub const OP_SRLIW: InstructionOpcode = 59;
pub const OP_SRLW: InstructionOpcode = 60;
pub const OP_SUB: InstructionOpcode = 61;
pub const OP_SUBW: InstructionOpcode = 62;
pub const OP_SW: InstructionOpcode = 63;
pub const OP_XOR: InstructionOpcode = 64;
pub const OP_XORI: InstructionOpcode = 65;
pub const OP_RVC_ADD: InstructionOpcode = 66;
pub const OP_RVC_ADDI: InstructionOpcode = 67;
pub const OP_RVC_ADDI16SP: InstructionOpcode = 68;
pub const OP_RVC_ADDI4SPN: InstructionOpcode = 69;
pub const OP_RVC_ADDIW: InstructionOpcode = 70;
pub const OP_RVC_ADDW: InstructionOpcode = 71;
pub const OP_RVC_AND: InstructionOpcode = 72;
pub const OP_RVC_ANDI: InstructionOpcode = 73;
pub const OP_RVC_BEQZ: InstructionOpcode = 74;
pub const OP_RVC_BNEZ: InstructionOpcode = 75;
pub const OP_RVC_EBREAK: InstructionOpcode = 76;
pub const OP_RVC_J: InstructionOpcode = 77;
pub const OP_RVC_JAL: InstructionOpcode = 78;
pub const OP_RVC_JALR: InstructionOpcode = 79;
pub const OP_RVC_JR: InstructionOpcode = 80;
pub const OP_RVC_LD: InstructionOpcode = 81;
pub const OP_RVC_LDSP: InstructionOpcode = 82;
pub const OP_RVC_LI: InstructionOpcode = 83;
pub const OP_RVC_LUI: InstructionOpcode = 84;
pub const OP_RVC_LW: InstructionOpcode = 85;
pub const OP_RVC_LWSP: InstructionOpcode = 86;
pub const OP_RVC_MV: InstructionOpcode = 87;
pub const OP_RVC_NOP: InstructionOpcode = 88;
pub const OP_RVC_OR: InstructionOpcode = 89;
pub const OP_RVC_SD: InstructionOpcode = 90;
pub const OP_RVC_SDSP: InstructionOpcode = 91;
pub const OP_RVC_SLLI: InstructionOpcode = 92;
pub const OP_RVC_SLLI64: InstructionOpcode = 93;
pub const OP_RVC_SRAI: InstructionOpcode = 94;
pub const OP_RVC_SRAI64: InstructionOpcode = 95;
pub const OP_RVC_SRLI: InstructionOpcode = 96;
pub const OP_RVC_SRLI64: InstructionOpcode = 97;
pub const OP_RVC_SUB: InstructionOpcode = 98;
pub const OP_RVC_SUBW: InstructionOpcode = 99;
pub const OP_RVC_SW: InstructionOpcode = 100;
pub const OP_RVC_SWSP: InstructionOpcode = 101;
pub const OP_RVC_XOR: InstructionOpcode = 102;

// Maximum opcode for instructions consuming 4 bytes. Any opcode
// larger than this one is treated as RVC instructions(which consume
// 2 bytes)
pub const MAXIMUM_NORMAL_OPCODE: InstructionOpcode = OP_XORI;

pub fn extract_opcode(i: Instruction) -> InstructionOpcode {
    i as u8
}

pub type InstructionFactory = fn(instruction_bits: u32) -> Option<Instruction>;

// Blank instructions need no register indices nor immediates, they only have opcode
// and module bit set.
pub fn blank_instruction(op: InstructionOpcode) -> Instruction {
    (u64::from(op as u8))
}

#[derive(Debug, Clone, Copy)]
pub struct Rtype(Instruction);

impl Rtype {
    pub fn new(
        op: InstructionOpcode,
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    ) -> Self {
        Rtype(
            u64::from(op as u8)
                | (u64::from(rd as u8) << 8)
                | (u64::from(rs1 as u8) << 32)
                | (u64::from(rs2 as u8) << 40),
        )
    }

    pub fn op(self) -> InstructionOpcode {
        self.0 as InstructionOpcode
    }

    pub fn rd(self) -> RegisterIndex {
        (self.0 >> 8) as u8 as RegisterIndex
    }

    pub fn rs1(self) -> RegisterIndex {
        (self.0 >> 32) as u8 as RegisterIndex
    }

    pub fn rs2(self) -> RegisterIndex {
        (self.0 >> 40) as u8 as RegisterIndex
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Itype(Instruction);

impl Itype {
    pub fn new(
        op: InstructionOpcode,
        rd: RegisterIndex,
        rs1: RegisterIndex,
        immediate: UImmediate,
    ) -> Self {
        Itype(
            u64::from(op as u8) |
              (u64::from(rd as u8) << 8) |
              (u64::from(rs1 as u8) << 32) |
              // Per RISC-V spec, I-type uses 12 bits at most, so it's perfectly
              // fine we store them in 3-byte location.
              (u64::from(immediate) << 40),
        )
    }

    pub fn new_s(
        op: InstructionOpcode,
        rd: RegisterIndex,
        rs1: RegisterIndex,
        immediate: Immediate,
    ) -> Self {
        Self::new(op, rd, rs1, immediate as UImmediate)
    }

    pub fn op(self) -> InstructionOpcode {
        self.0 as InstructionOpcode
    }

    pub fn rd(self) -> RegisterIndex {
        (self.0 >> 8) as u8 as RegisterIndex
    }

    pub fn rs1(self) -> RegisterIndex {
        (self.0 >> 32) as u8 as RegisterIndex
    }

    pub fn immediate(self) -> UImmediate {
        self.immediate_s() as UImmediate
    }

    pub fn immediate_s(self) -> Immediate {
        ((self.0 as i64) >> 40) as Immediate
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Stype(Instruction);

impl Stype {
    pub fn new(
        op: InstructionOpcode,
        immediate: UImmediate,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    ) -> Self {
        Stype(
            u64::from(op as u8) |
              (u64::from(rs2 as u8) << 8) |
              (u64::from(rs1 as u8) << 32) |
              // Per RISC-V spec, S/B type uses 13 bits at most, so it's perfectly
              // fine we store them in 3-byte location.
              (u64::from(immediate) << 40),
        )
    }

    pub fn new_s(
        op: InstructionOpcode,
        immediate: Immediate,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    ) -> Self {
        Self::new(op, immediate as UImmediate, rs1, rs2)
    }

    pub fn op(self) -> InstructionOpcode {
        self.0 as InstructionOpcode
    }

    pub fn rs1(self) -> RegisterIndex {
        (self.0 >> 32) as u8 as RegisterIndex
    }

    pub fn rs2(self) -> RegisterIndex {
        (self.0 >> 8) as u8 as RegisterIndex
    }

    pub fn immediate(self) -> UImmediate {
        self.immediate_s() as UImmediate
    }

    pub fn immediate_s(self) -> Immediate {
        ((self.0 as i64) >> 40) as Immediate
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Utype(Instruction);

impl Utype {
    pub fn new(op: InstructionOpcode, rd: RegisterIndex, immediate: UImmediate) -> Self {
        Utype(u64::from(op as u8) | (u64::from(rd as u8) << 8) | (u64::from(immediate) << 32))
    }

    pub fn new_s(op: InstructionOpcode, rd: RegisterIndex, immediate: Immediate) -> Self {
        Self::new(op, rd, immediate as UImmediate)
    }

    pub fn op(self) -> InstructionOpcode {
        self.0 as InstructionOpcode
    }

    pub fn rd(self) -> RegisterIndex {
        (self.0 >> 8) as u8 as RegisterIndex
    }

    pub fn immediate(self) -> UImmediate {
        self.immediate_s() as UImmediate
    }

    pub fn immediate_s(self) -> Immediate {
        ((self.0 as i64) >> 32) as Immediate
    }
}

pub fn is_basic_block_end_instruction(i: Instruction) -> bool {
    match extract_opcode(i) {
        OP_JALR => true,
        OP_BEQ => true,
        OP_BNE => true,
        OP_BLT => true,
        OP_BGE => true,
        OP_BLTU => true,
        OP_BGEU => true,
        OP_ECALL => true,
        OP_EBREAK => true,
        OP_JAL => true,
        OP_RVC_EBREAK => true,
        OP_RVC_BEQZ => true,
        OP_RVC_BNEZ => true,
        OP_RVC_J => true,
        OP_RVC_JAL => true,
        OP_RVC_JALR => true,
        OP_RVC_JR => true,
        _ => false,
    }
}

pub fn instruction_length(i: Instruction) -> usize {
    if extract_opcode(i) <= MAXIMUM_NORMAL_OPCODE {
        4
    } else {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn test_instruction_op_should_fit_in_byte() {
        assert_eq!(1, size_of::<InstructionOpcode>());
    }
}
