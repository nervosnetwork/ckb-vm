mod utils;

pub mod rv32i;

use super::machine::Machine;
use super::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Instruction {
    RV32I(rv32i::Instruction),
}

impl Instruction {
    pub fn execute(&self, machine: &mut Machine) -> Result<(), Error> {
        match self {
            Instruction::RV32I(instruction) => instruction.execute(machine),
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
