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
pub const OP_ADDI: u8 = 0;
pub const OP_ADD: u8 = 1;
pub const OP_ADDIW: u8 = 2;
pub const OP_ADDW: u8 = 3;
pub const OP_AND: u8 = 4;
pub const OP_ANDI: u8 = 5;
pub const OP_AUIPC: u8 = 6;
pub const OP_BEQ: u8 = 7;
pub const OP_BGE: u8 = 8;
pub const OP_BGEU: u8 = 9;
pub const OP_BLT: u8 = 10;
pub const OP_BLTU: u8 = 11;
pub const OP_BNE: u8 = 12;
pub const OP_DIV: u8 = 13;
pub const OP_DIVU: u8 = 14;
pub const OP_DIVUW: u8 = 15;
pub const OP_DIVW: u8 = 16;
pub const OP_EBREAK: u8 = 17;
pub const OP_ECALL: u8 = 18;
pub const OP_FENCE: u8 = 19;
pub const OP_FENCEI: u8 = 20;
pub const OP_JAL: u8 = 21;
pub const OP_JALR: u8 = 22;
pub const OP_LB: u8 = 23;
pub const OP_LBU: u8 = 24;
pub const OP_LD: u8 = 25;
pub const OP_LH: u8 = 26;
pub const OP_LHU: u8 = 27;
pub const OP_LUI: u8 = 28;
pub const OP_LW: u8 = 29;
pub const OP_LWU: u8 = 30;
pub const OP_MUL: u8 = 31;
pub const OP_MULH: u8 = 32;
pub const OP_MULHSU: u8 = 33;
pub const OP_MULHU: u8 = 34;
pub const OP_MULW: u8 = 35;
pub const OP_OR: u8 = 36;
pub const OP_ORI: u8 = 37;
pub const OP_REM: u8 = 38;
pub const OP_REMU: u8 = 39;
pub const OP_REMUW: u8 = 40;
pub const OP_REMW: u8 = 41;
pub const OP_SB: u8 = 42;
pub const OP_SD: u8 = 43;
pub const OP_SH: u8 = 44;
pub const OP_SLL: u8 = 45;
pub const OP_SLLI: u8 = 46;
pub const OP_SLLIW: u8 = 47;
pub const OP_SLLW: u8 = 48;
pub const OP_SLT: u8 = 49;
pub const OP_SLTI: u8 = 50;
pub const OP_SLTIU: u8 = 51;
pub const OP_SLTU: u8 = 52;
pub const OP_SRA: u8 = 53;
pub const OP_SRAI: u8 = 54;
pub const OP_SRAIW: u8 = 55;
pub const OP_SRAW: u8 = 56;
pub const OP_SRL: u8 = 57;
pub const OP_SRLI: u8 = 58;
pub const OP_SRLIW: u8 = 59;
pub const OP_SRLW: u8 = 60;
pub const OP_SUB: u8 = 61;
pub const OP_SUBW: u8 = 62;
pub const OP_SW: u8 = 63;
pub const OP_XOR: u8 = 64;
pub const OP_XORI: u8 = 65;
pub const OP_RVC_ADD: u8 = 66;
pub const OP_RVC_ADDI: u8 = 67;
pub const OP_RVC_ADDI16SP: u8 = 68;
pub const OP_RVC_ADDI4SPN: u8 = 69;
pub const OP_RVC_ADDIW: u8 = 70;
pub const OP_RVC_ADDW: u8 = 71;
pub const OP_RVC_AND: u8 = 72;
pub const OP_RVC_ANDI: u8 = 73;
pub const OP_RVC_BEQZ: u8 = 74;
pub const OP_RVC_BNEZ: u8 = 75;
pub const OP_RVC_EBREAK: u8 = 76;
pub const OP_RVC_J: u8 = 77;
pub const OP_RVC_JAL: u8 = 78;
pub const OP_RVC_JALR: u8 = 79;
pub const OP_RVC_JR: u8 = 80;
pub const OP_RVC_LD: u8 = 81;
pub const OP_RVC_LDSP: u8 = 82;
pub const OP_RVC_LI: u8 = 83;
pub const OP_RVC_LUI: u8 = 84;
pub const OP_RVC_LW: u8 = 85;
pub const OP_RVC_LWSP: u8 = 86;
pub const OP_RVC_MV: u8 = 87;
pub const OP_RVC_NOP: u8 = 88;
pub const OP_RVC_OR: u8 = 89;
pub const OP_RVC_SD: u8 = 90;
pub const OP_RVC_SDSP: u8 = 91;
pub const OP_RVC_SLLI: u8 = 92;
pub const OP_RVC_SLLI64: u8 = 93;
pub const OP_RVC_SRAI: u8 = 94;
pub const OP_RVC_SRAI64: u8 = 95;
pub const OP_RVC_SRLI: u8 = 96;
pub const OP_RVC_SRLI64: u8 = 97;
pub const OP_RVC_SUB: u8 = 98;
pub const OP_RVC_SUBW: u8 = 99;
pub const OP_RVC_SW: u8 = 100;
pub const OP_RVC_SWSP: u8 = 101;
pub const OP_RVC_XOR: u8 = 102;

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
