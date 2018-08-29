mod common;
mod utils;

pub mod rv32i;
pub mod rv32m;
pub mod rvc;

use super::machine::Machine;
use super::memory::Memory;
use super::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Instruction {
    RVC(rvc::Instruction),
    RV32I(rv32i::Instruction),
    RV32M(rv32m::Instruction),
}

impl Instruction {
    pub fn execute<Mac: Machine<u32, M>, M: Memory>(&self, machine: &mut Mac) -> Result<(), Error> {
        match self {
            Instruction::RV32I(instruction) => instruction.execute(machine),
            Instruction::RV32M(instruction) => instruction.execute(machine),
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
    fn execute<Mac: Machine<u32, M>, M: Memory>(
        &self,
        machine: &mut Mac,
    ) -> Result<Option<NextPC>, Error>;
}

type RegisterIndex = usize;
type Immediate = i32;
type UImmediate = u32;
type NextPC = u32;

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

#[derive(Debug)]
pub struct Rtype<I> {
    rs2: RegisterIndex,
    rs1: RegisterIndex,
    rd: RegisterIndex,
    inst: I,
}

#[derive(Debug)]
pub struct Itype<M, I> {
    rs1: RegisterIndex,
    rd: RegisterIndex,
    imm: M,
    inst: I,
}

#[derive(Debug)]
pub struct ItypeShift<M, I> {
    rs1: RegisterIndex,
    rd: RegisterIndex,
    shamt: M,
    inst: I,
}

#[derive(Debug)]
pub struct Stype<M, I> {
    rs2: RegisterIndex,
    rs1: RegisterIndex,
    imm: M,
    inst: I,
}

#[derive(Debug)]
pub struct Btype<M, I> {
    rs2: RegisterIndex,
    rs1: RegisterIndex,
    imm: M,
    inst: I,
}

#[derive(Debug)]
pub struct Utype<M, I> {
    rd: RegisterIndex,
    imm: M,
    inst: I,
}
