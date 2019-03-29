mod common;
mod register;
mod utils;

pub mod i;
pub mod m;
pub mod rvc;

pub use self::register::Register;
use super::machine::Machine;
use super::Error;
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub enum Instruction {
    RVC(rvc::Instruction),
    I(i::Instruction),
    M(m::Instruction),
}

impl Default for Instruction {
    fn default() -> Self {
        // Default instruction is NOP, note that we don't use NOP from RVC,
        // since it's more likely every chip will implement I than RVC.
        i::nop()
    }
}

impl Instruction {
    pub fn execute<Mac: Machine>(&self, machine: &mut Mac) -> Result<(), Error> {
        match self {
            Instruction::I(instruction) => instruction.execute(machine),
            Instruction::M(instruction) => instruction.execute(machine),
            Instruction::RVC(instruction) => instruction.execute(machine),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: change to real disasm feature instead of simply delegating
        // to std::fmt::Debug
        write!(f, "{:?}", self)
    }
}

pub type InstructionFactory = fn(instruction_bits: u32) -> Option<Instruction>;

// Instruction execution trait
pub trait Execute {
    fn execute<Mac: Machine>(&self, machine: &mut Mac) -> Result<Option<Mac::REG>, Error>;
}

type RegisterIndex = u8;
type Immediate = i32;
type UImmediate = u32;
type UShortImmediate = u16;

//
//  31       27 26 25 24     20 19    15 14    12 11          7 6      0
// ======================================================================
// | funct7          |   rs2   |   rs1  | funct3 |  rd         | opcode | R-type
// +--------------------------------------------------------------------+
// |          imm[11:0]        |   rs1  | funct3 |  rd         | opcode | I-type
// +--------------------------------------------------------------------+
// |   imm[11:5]     |   rs2   |   rs1  | funct3 | imm[4:0]    | opcode | S-type
// +--------------------------------------------------------------------+
// |   imm[12|10:5]  |   rs2   |   rs1  | funct3 | imm[4:1|11] | opcode | B-type
// +--------------------------------------------------------------------+
// |             imm[31:12]                      |  rd         | opcode | U-type
// +--------------------------------------------------------------------+
// |             imm[20|10:1|11|19:12]           |  rd         | opcode | J-type
// ======================================================================
//

#[derive(Debug, Clone)]
pub struct Rtype<I> {
    rs2: RegisterIndex,
    rs1: RegisterIndex,
    rd: RegisterIndex,
    inst: I,
}

impl<I> Rtype<I> {
    pub fn inst(&self) -> &I {
        &self.inst
    }
}

#[derive(Debug, Clone)]
pub struct Itype<M, I> {
    rs1: RegisterIndex,
    rd: RegisterIndex,
    imm: M,
    inst: I,
}

impl<M, I> Itype<M, I> {
    pub fn inst(&self) -> &I {
        &self.inst
    }
}

#[derive(Debug, Clone)]
pub struct ItypeShift<M, I> {
    rs1: RegisterIndex,
    rd: RegisterIndex,
    shamt: M,
    inst: I,
}

impl<M, I> ItypeShift<M, I> {
    pub fn inst(&self) -> &I {
        &self.inst
    }
}

#[derive(Debug, Clone)]
pub struct Stype<M, I> {
    rs2: RegisterIndex,
    rs1: RegisterIndex,
    imm: M,
    inst: I,
}

impl<M, I> Stype<M, I> {
    pub fn inst(&self) -> &I {
        &self.inst
    }
}

#[derive(Debug, Clone)]
pub struct Btype<M, I> {
    rs2: RegisterIndex,
    rs1: RegisterIndex,
    imm: M,
    inst: I,
}

impl<M, I> Btype<M, I> {
    pub fn inst(&self) -> &I {
        &self.inst
    }
}

#[derive(Debug, Clone)]
pub struct Utype<M, I> {
    rd: RegisterIndex,
    imm: M,
    inst: I,
}

impl<M, I> Utype<M, I> {
    pub fn inst(&self) -> &I {
        &self.inst
    }
}

pub fn is_basic_block_end_instruction(i: &Instruction) -> bool {
    match i {
        Instruction::I(i) => match i {
            i::Instruction::I(i) => match i.inst() {
                i::ItypeInstruction::JALR => true,
                _ => false,
            },
            i::Instruction::B(_) => true,
            i::Instruction::Env(_) => true,
            i::Instruction::JAL { .. } => true,
            _ => false,
        },
        Instruction::RVC(i) => match i {
            rvc::Instruction::BEQZ { .. } => true,
            rvc::Instruction::BNEZ { .. } => true,
            rvc::Instruction::JAL { .. } => true,
            rvc::Instruction::J { .. } => true,
            rvc::Instruction::JR { .. } => true,
            rvc::Instruction::JALR { .. } => true,
            rvc::Instruction::EBREAK => true,
            _ => false,
        },
        Instruction::M(_) => false,
    }
}

pub fn instruction_length(i: &Instruction) -> usize {
    match i {
        Instruction::RVC(_) => 2,
        _ => 4,
    }
}
