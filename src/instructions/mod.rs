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

#[derive(Copy, Clone, Debug)]
pub enum InstructionOp {
    // ADDI must have an opcode of 0, this way the default 0 value for u64
    // happens to be a NOP operation in RISC-V format
    ADDI = 0,
    ADD = 1,
    ADDI16SP = 2,
    ADDI4SPN = 3,
    ADDIW = 4,
    ADDW = 5,
    AND = 6,
    ANDI = 7,
    AUIPC = 8,
    BEQ = 9,
    BEQZ = 10,
    BGE = 11,
    BGEU = 12,
    BLT = 13,
    BLTU = 14,
    BNE = 15,
    BNEZ = 16,
    DIV = 17,
    DIVU = 18,
    DIVUW = 19,
    DIVW = 20,
    EBREAK = 21,
    ECALL = 22,
    FENCE = 23,
    FENCEI = 24,
    J = 25,
    JAL = 26,
    JALR = 27,
    JR = 28,
    LB = 29,
    LBU = 30,
    LD = 31,
    LDSP = 32,
    LH = 33,
    LHU = 34,
    LI = 35,
    LUI = 36,
    LW = 37,
    LWSP = 38,
    LWU = 39,
    MUL = 40,
    MULH = 41,
    MULHSU = 42,
    MULHU = 43,
    MULW = 44,
    MV = 45,
    NOP = 46,
    OR = 47,
    ORI = 48,
    REM = 49,
    REMU = 50,
    REMUW = 51,
    REMW = 52,
    SB = 53,
    SD = 54,
    SDSP = 55,
    SH = 56,
    SLL = 57,
    SLLI = 58,
    SLLI64 = 59,
    SLLIW = 60,
    SLLW = 61,
    SLT = 62,
    SLTI = 63,
    SLTIU = 64,
    SLTU = 65,
    SRA = 66,
    SRAI = 67,
    SRAI64 = 68,
    SRAIW = 69,
    SRAW = 70,
    SRL = 71,
    SRLI = 72,
    SRLI64 = 73,
    SRLIW = 74,
    SRLW = 75,
    SUB = 76,
    SUBW = 77,
    SW = 78,
    SWSP = 79,
    XOR = 80,
    XORI = 81,
}

impl InstructionOp {
    fn from_u8(value: u8) -> Result<InstructionOp, Error> {
        match value {
            0 => Ok(InstructionOp::ADDI),
            1 => Ok(InstructionOp::ADD),
            2 => Ok(InstructionOp::ADDI16SP),
            3 => Ok(InstructionOp::ADDI4SPN),
            4 => Ok(InstructionOp::ADDIW),
            5 => Ok(InstructionOp::ADDW),
            6 => Ok(InstructionOp::AND),
            7 => Ok(InstructionOp::ANDI),
            8 => Ok(InstructionOp::AUIPC),
            9 => Ok(InstructionOp::BEQ),
            10 => Ok(InstructionOp::BEQZ),
            11 => Ok(InstructionOp::BGE),
            12 => Ok(InstructionOp::BGEU),
            13 => Ok(InstructionOp::BLT),
            14 => Ok(InstructionOp::BLTU),
            15 => Ok(InstructionOp::BNE),
            16 => Ok(InstructionOp::BNEZ),
            17 => Ok(InstructionOp::DIV),
            18 => Ok(InstructionOp::DIVU),
            19 => Ok(InstructionOp::DIVUW),
            20 => Ok(InstructionOp::DIVW),
            21 => Ok(InstructionOp::EBREAK),
            22 => Ok(InstructionOp::ECALL),
            23 => Ok(InstructionOp::FENCE),
            24 => Ok(InstructionOp::FENCEI),
            25 => Ok(InstructionOp::J),
            26 => Ok(InstructionOp::JAL),
            27 => Ok(InstructionOp::JALR),
            28 => Ok(InstructionOp::JR),
            29 => Ok(InstructionOp::LB),
            30 => Ok(InstructionOp::LBU),
            31 => Ok(InstructionOp::LD),
            32 => Ok(InstructionOp::LDSP),
            33 => Ok(InstructionOp::LH),
            34 => Ok(InstructionOp::LHU),
            35 => Ok(InstructionOp::LI),
            36 => Ok(InstructionOp::LUI),
            37 => Ok(InstructionOp::LW),
            38 => Ok(InstructionOp::LWSP),
            39 => Ok(InstructionOp::LWU),
            40 => Ok(InstructionOp::MUL),
            41 => Ok(InstructionOp::MULH),
            42 => Ok(InstructionOp::MULHSU),
            43 => Ok(InstructionOp::MULHU),
            44 => Ok(InstructionOp::MULW),
            45 => Ok(InstructionOp::MV),
            46 => Ok(InstructionOp::NOP),
            47 => Ok(InstructionOp::OR),
            48 => Ok(InstructionOp::ORI),
            49 => Ok(InstructionOp::REM),
            50 => Ok(InstructionOp::REMU),
            51 => Ok(InstructionOp::REMUW),
            52 => Ok(InstructionOp::REMW),
            53 => Ok(InstructionOp::SB),
            54 => Ok(InstructionOp::SD),
            55 => Ok(InstructionOp::SDSP),
            56 => Ok(InstructionOp::SH),
            57 => Ok(InstructionOp::SLL),
            58 => Ok(InstructionOp::SLLI),
            59 => Ok(InstructionOp::SLLI64),
            60 => Ok(InstructionOp::SLLIW),
            61 => Ok(InstructionOp::SLLW),
            62 => Ok(InstructionOp::SLT),
            63 => Ok(InstructionOp::SLTI),
            64 => Ok(InstructionOp::SLTIU),
            65 => Ok(InstructionOp::SLTU),
            66 => Ok(InstructionOp::SRA),
            67 => Ok(InstructionOp::SRAI),
            68 => Ok(InstructionOp::SRAI64),
            69 => Ok(InstructionOp::SRAIW),
            70 => Ok(InstructionOp::SRAW),
            71 => Ok(InstructionOp::SRL),
            72 => Ok(InstructionOp::SRLI),
            73 => Ok(InstructionOp::SRLI64),
            74 => Ok(InstructionOp::SRLIW),
            75 => Ok(InstructionOp::SRLW),
            76 => Ok(InstructionOp::SUB),
            77 => Ok(InstructionOp::SUBW),
            78 => Ok(InstructionOp::SW),
            79 => Ok(InstructionOp::SWSP),
            80 => Ok(InstructionOp::XOR),
            81 => Ok(InstructionOp::XORI),
            _ => Err(Error::ParseError),
        }
    }
}

pub fn extract_opcode(i: Instruction) -> Result<InstructionOp, Error> {
    InstructionOp::from_u8(i as u8)
}

#[derive(Copy, Clone, Debug)]
pub enum Module {
    I = 0,
    M = 1,
    RVC = 2,
}

impl Module {
    fn from_u8(value: u8) -> Result<Module, Error> {
        match value {
            0 => Ok(Module::I),
            1 => Ok(Module::M),
            2 => Ok(Module::RVC),
            _ => Err(Error::ParseError),
        }
    }

    fn from_instruction(instruction: Instruction) -> Result<Module, Error> {
        Module::from_u8((instruction >> 16) as u8)
    }
}

pub fn execute<Mac: Machine>(i: Instruction, machine: &mut Mac) -> Result<(), Error> {
    match Module::from_instruction(i)? {
        Module::I => i::execute(i, machine),
        Module::M => m::execute(i, machine),
        Module::RVC => rvc::execute(i, machine),
    }
}

pub type InstructionFactory = fn(instruction_bits: u32) -> Option<Instruction>;

pub fn assemble_no_argument_instruction(op: InstructionOp, m: Module) -> Instruction {
    (u64::from(op as u8)) | ((u64::from(m as u8)) << 16)
}

#[derive(Debug, Clone, Copy)]
pub struct Rtype(Instruction);

impl Rtype {
    pub fn assemble(op: InstructionOp, rd: u8, rs1: u8, rs2: u8, m: Module) -> Self {
        Rtype(
            u64::from(op as u8)
                | (u64::from(rd) << 8)
                | (u64::from(m as u8) << 16)
                | (u64::from(rs1) << 32)
                | (u64::from(rs2) << 40),
        )
    }

    pub fn op(self) -> Result<InstructionOp, Error> {
        InstructionOp::from_u8(self.0 as u8)
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
    pub fn assemble(op: InstructionOp, rd: u8, rs1: u8, immediate: u32, m: Module) -> Self {
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

    pub fn assemble_s(op: InstructionOp, rd: u8, rs1: u8, immediate: i32, m: Module) -> Self {
        Self::assemble(op, rd, rs1, immediate as u32, m)
    }

    pub fn op(self) -> Result<InstructionOp, Error> {
        InstructionOp::from_u8(self.0 as u8)
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
    pub fn assemble(op: InstructionOp, immediate: u32, rs1: u8, rs2: u8, m: Module) -> Self {
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

    pub fn assemble_s(op: InstructionOp, immediate: i32, rs1: u8, rs2: u8, m: Module) -> Self {
        Self::assemble(op, immediate as u32, rs1, rs2, m)
    }

    pub fn op(self) -> Result<InstructionOp, Error> {
        InstructionOp::from_u8(self.0 as u8)
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
    pub fn assemble(op: InstructionOp, rd: u8, immediate: u32, m: Module) -> Self {
        Utype(
            u64::from(op as u8)
                | (u64::from(rd) << 8)
                | (u64::from(m as u8) << 16)
                | (u64::from(immediate) << 32),
        )
    }

    pub fn assemble_s(op: InstructionOp, rd: u8, immediate: i32, m: Module) -> Self {
        Self::assemble(op, rd, immediate as u32, m)
    }

    pub fn op(self) -> Result<InstructionOp, Error> {
        InstructionOp::from_u8(self.0 as u8)
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
        Ok(InstructionOp::JALR) => true,
        Ok(InstructionOp::BEQ) => true,
        Ok(InstructionOp::BNE) => true,
        Ok(InstructionOp::BLT) => true,
        Ok(InstructionOp::BGE) => true,
        Ok(InstructionOp::BLTU) => true,
        Ok(InstructionOp::BGEU) => true,
        Ok(InstructionOp::ECALL) => true,
        Ok(InstructionOp::EBREAK) => true,
        Ok(InstructionOp::JAL) => true,
        Ok(InstructionOp::BEQZ) => true,
        Ok(InstructionOp::BNEZ) => true,
        Ok(InstructionOp::J) => true,
        Ok(InstructionOp::JR) => true,
        _ => false,
    }
}

pub fn instruction_length(i: Instruction) -> usize {
    match Module::from_instruction(i) {
        Ok(Module::RVC) => 2,
        _ => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn test_instruction_op_should_fit_in_byte() {
        assert_eq!(1, size_of::<InstructionOp>());
    }

    #[test]
    fn test_module_should_fit_in_byte() {
        assert_eq!(1, size_of::<Module>());
    }
}
