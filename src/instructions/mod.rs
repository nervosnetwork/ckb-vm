mod common;
mod register;
mod utils;

pub mod i;
pub mod m;
pub mod rvc;

pub use self::register::Register;
use super::machine::Machine;
use super::Error;

type RegisterIndex = u8;
type Immediate = i32;
type UImmediate = u32;

// For fast decoding and cache friendly, RISC-V instruction is decoded
// into 64 bit unsigned integer in the following format:
//
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |           | rs2 | rs1 | res | mod | rd  | op  | R-type
// +-----------+-----------------------------------+
// |    immediate    | rs1 | res | mod | rd  | op  | I-type
// +-----------------------------------------------+
// |    immediate    | rs1 | res | mod | rs2 | op  | S-type/B-type
// +-----------------+-----------------------------+
// |       immediate       | res | mod | rd  | op  | U-type/J-type
// +-----+-----+-----+-----+-----+-----+-----+-----+
//
// +res+ here means reserved field that is not yet used.
// +mod+ includes the RISC-V module extension current instruction lives,
// theoretically we won't need those since op is fully flattened and should
// contain all instructions in every module. But having mod here can be handy
// when we want to quickly determine what RISC-V module one instruction lives in.
//
// This way each op and register index are in full byte, accessing them
// will be much faster than the original compact form. Hence we will have
// a fast path where the interpreter loop reads instruction directly in this
// format, and a slow path where a full featured decoder decodes RISC-V
// instruction into the internal form here(much like how traces/micro-ops work.)
pub type Instruction = u64;

pub type InstructionOpcode = u8;

// ADDI must have an opcode of 0, this way the default 0 value for u64
// happens to be a NOP operation in RISC-V format
pub const OP_ADDI: u8 = 0;
pub const OP_ADD: u8 = 1;
pub const OP_ADDI16SP: u8 = 2;
pub const OP_ADDI4SPN: u8 = 3;
pub const OP_ADDIW: u8 = 4;
pub const OP_ADDW: u8 = 5;
pub const OP_AND: u8 = 6;
pub const OP_ANDI: u8 = 7;
pub const OP_AUIPC: u8 = 8;
pub const OP_BEQ: u8 = 9;
pub const OP_BEQZ: u8 = 10;
pub const OP_BGE: u8 = 11;
pub const OP_BGEU: u8 = 12;
pub const OP_BLT: u8 = 13;
pub const OP_BLTU: u8 = 14;
pub const OP_BNE: u8 = 15;
pub const OP_BNEZ: u8 = 16;
pub const OP_DIV: u8 = 17;
pub const OP_DIVU: u8 = 18;
pub const OP_DIVUW: u8 = 19;
pub const OP_DIVW: u8 = 20;
pub const OP_EBREAK: u8 = 21;
pub const OP_ECALL: u8 = 22;
pub const OP_FENCE: u8 = 23;
pub const OP_FENCEI: u8 = 24;
pub const OP_J: u8 = 25;
pub const OP_JAL: u8 = 26;
pub const OP_JALR: u8 = 27;
pub const OP_JR: u8 = 28;
pub const OP_LB: u8 = 29;
pub const OP_LBU: u8 = 30;
pub const OP_LD: u8 = 31;
pub const OP_LDSP: u8 = 32;
pub const OP_LH: u8 = 33;
pub const OP_LHU: u8 = 34;
pub const OP_LI: u8 = 35;
pub const OP_LUI: u8 = 36;
pub const OP_LW: u8 = 37;
pub const OP_LWSP: u8 = 38;
pub const OP_LWU: u8 = 39;
pub const OP_MUL: u8 = 40;
pub const OP_MULH: u8 = 41;
pub const OP_MULHSU: u8 = 42;
pub const OP_MULHU: u8 = 43;
pub const OP_MULW: u8 = 44;
pub const OP_MV: u8 = 45;
pub const OP_NOP: u8 = 46;
pub const OP_OR: u8 = 47;
pub const OP_ORI: u8 = 48;
pub const OP_REM: u8 = 49;
pub const OP_REMU: u8 = 50;
pub const OP_REMUW: u8 = 51;
pub const OP_REMW: u8 = 52;
pub const OP_SB: u8 = 53;
pub const OP_SD: u8 = 54;
pub const OP_SDSP: u8 = 55;
pub const OP_SH: u8 = 56;
pub const OP_SLL: u8 = 57;
pub const OP_SLLI: u8 = 58;
pub const OP_SLLI64: u8 = 59;
pub const OP_SLLIW: u8 = 60;
pub const OP_SLLW: u8 = 61;
pub const OP_SLT: u8 = 62;
pub const OP_SLTI: u8 = 63;
pub const OP_SLTIU: u8 = 64;
pub const OP_SLTU: u8 = 65;
pub const OP_SRA: u8 = 66;
pub const OP_SRAI: u8 = 67;
pub const OP_SRAI64: u8 = 68;
pub const OP_SRAIW: u8 = 69;
pub const OP_SRAW: u8 = 70;
pub const OP_SRL: u8 = 71;
pub const OP_SRLI: u8 = 72;
pub const OP_SRLI64: u8 = 73;
pub const OP_SRLIW: u8 = 74;
pub const OP_SRLW: u8 = 75;
pub const OP_SUB: u8 = 76;
pub const OP_SUBW: u8 = 77;
pub const OP_SW: u8 = 78;
pub const OP_SWSP: u8 = 79;
pub const OP_XOR: u8 = 80;
pub const OP_XORI: u8 = 81;

pub fn extract_opcode(i: Instruction) -> InstructionOpcode {
    i as u8
}

pub type InstructionModule = u8;

pub const MODULE_I: u8 = 0;
pub const MODULE_M: u8 = 1;
pub const MODULE_RVC: u8 = 2;

pub fn extract_module(i: Instruction) -> InstructionModule {
    (i >> 16) as u8
}

pub fn execute<Mac: Machine>(i: Instruction, machine: &mut Mac) -> Result<(), Error> {
    match extract_module(i) {
        MODULE_I => i::execute(i, machine),
        MODULE_M => m::execute(i, machine),
        MODULE_RVC => rvc::execute(i, machine),
        _ => Err(Error::ParseError),
    }
}

pub type InstructionFactory = fn(instruction_bits: u32) -> Option<Instruction>;

// Blank instructions need no register indices nor immediates, they only have opcode
// and module bit set.
pub fn blank_instruction(op: InstructionOpcode, m: InstructionModule) -> Instruction {
    (u64::from(op as u8)) | ((u64::from(m as u8)) << 16)
}

#[derive(Debug, Clone, Copy)]
pub struct Rtype(Instruction);

impl Rtype {
    pub fn new(op: InstructionOpcode, rd: u8, rs1: u8, rs2: u8, m: InstructionModule) -> Self {
        Rtype(
            u64::from(op as u8)
                | (u64::from(rd) << 8)
                | (u64::from(m as u8) << 16)
                | (u64::from(rs1) << 32)
                | (u64::from(rs2) << 40),
        )
    }

    pub fn op(self) -> InstructionOpcode {
        self.0 as u8
    }

    pub fn rd(self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub fn rs1(self) -> u8 {
        (self.0 >> 32) as u8
    }

    pub fn rs2(self) -> u8 {
        (self.0 >> 40) as u8
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Itype(Instruction);

impl Itype {
    pub fn new(
        op: InstructionOpcode,
        rd: u8,
        rs1: u8,
        immediate: u32,
        m: InstructionModule,
    ) -> Self {
        Itype(
            u64::from(op as u8) |
              (u64::from(rd) << 8) |
              (u64::from(m as u8) << 16) |
              (u64::from(rs1) << 32) |
              // Per RISC-V spec, I-type uses 12 bits at most, so it's perfectly
              // fine we store them in 3-byte location.
              (u64::from(immediate) << 40),
        )
    }

    pub fn new_s(
        op: InstructionOpcode,
        rd: u8,
        rs1: u8,
        immediate: i32,
        m: InstructionModule,
    ) -> Self {
        Self::new(op, rd, rs1, immediate as u32, m)
    }

    pub fn op(self) -> InstructionOpcode {
        self.0 as u8
    }

    pub fn rd(self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub fn rs1(self) -> u8 {
        (self.0 >> 32) as u8
    }

    pub fn immediate(self) -> u32 {
        self.immediate_s() as u32
    }

    pub fn immediate_s(self) -> i32 {
        ((self.0 as i64) >> 40) as i32
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Stype(Instruction);

impl Stype {
    pub fn new(
        op: InstructionOpcode,
        immediate: u32,
        rs1: u8,
        rs2: u8,
        m: InstructionModule,
    ) -> Self {
        Stype(
            u64::from(op as u8) |
              (u64::from(rs2) << 8) |
              (u64::from(m as u8) << 16) |
              (u64::from(rs1) << 32) |
              // Per RISC-V spec, S/B type uses 13 bits at most, so it's perfectly
              // fine we store them in 3-byte location.
              (u64::from(immediate) << 40),
        )
    }

    pub fn new_s(
        op: InstructionOpcode,
        immediate: i32,
        rs1: u8,
        rs2: u8,
        m: InstructionModule,
    ) -> Self {
        Self::new(op, immediate as u32, rs1, rs2, m)
    }

    pub fn op(self) -> InstructionOpcode {
        self.0 as u8
    }

    pub fn rs1(self) -> u8 {
        (self.0 >> 32) as u8
    }

    pub fn rs2(self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub fn immediate(self) -> u32 {
        self.immediate_s() as u32
    }

    pub fn immediate_s(self) -> i32 {
        ((self.0 as i64) >> 40) as i32
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Utype(Instruction);

impl Utype {
    pub fn new(op: InstructionOpcode, rd: u8, immediate: u32, m: InstructionModule) -> Self {
        Utype(
            u64::from(op as u8)
                | (u64::from(rd) << 8)
                | (u64::from(m as u8) << 16)
                | (u64::from(immediate) << 32),
        )
    }

    pub fn new_s(op: InstructionOpcode, rd: u8, immediate: i32, m: InstructionModule) -> Self {
        Self::new(op, rd, immediate as u32, m)
    }

    pub fn op(self) -> InstructionOpcode {
        self.0 as u8
    }

    pub fn rd(self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub fn immediate(self) -> u32 {
        self.immediate_s() as u32
    }

    pub fn immediate_s(self) -> i32 {
        ((self.0 as i64) >> 32) as i32
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
        OP_BEQZ => true,
        OP_BNEZ => true,
        OP_J => true,
        OP_JR => true,
        _ => false,
    }
}

pub fn instruction_length(i: Instruction) -> usize {
    match extract_module(i) {
        MODULE_RVC => 2,
        _ => 4,
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

    #[test]
    fn test_module_should_fit_in_byte() {
        assert_eq!(1, size_of::<InstructionModule>());
    }
}
