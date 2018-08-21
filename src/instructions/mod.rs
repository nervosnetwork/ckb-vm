mod utils;

pub mod c;
pub mod rv32i;
pub mod rv32m;

use super::machine::Machine;
use super::memory::Memory;
use super::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Instruction {
    RV32I(rv32i::Instruction),
    RV32M(rv32m::Instruction),
    C(c::Instruction),
}

impl Instruction {
    pub fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<(), Error> {
        match self {
            Instruction::RV32I(instruction) => instruction.execute(machine),
            Instruction::RV32M(instruction) => instruction.execute(machine),
            Instruction::C(instruction) => instruction.execute(machine),
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
