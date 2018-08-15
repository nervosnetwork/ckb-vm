extern crate byteorder;
extern crate goblin;

mod decoder;
mod instructions;
mod machine;
mod memory;

use machine::Machine;
use std::io::Error as IOError;

pub const RISCV_PAGESIZE: usize = 1 << 12;
pub const RISCV_GENERAL_REGISTER_NUMBER: usize = 32;
// 128 MB
pub const RISCV_MAX_MEMORY: usize = 128 << 20;

#[derive(Debug)]
pub enum Error {
    ParseError,
    Alignment,
    OutOfBound,
    InvalidInstruction(u32),
    IO(IOError),
    Unimplemented,
}

pub fn run(program: &[u8], args: &[String]) -> Result<u8, Error> {
    let mut machine = Machine::default();
    machine.load(program)?;
    machine.run(args)
}
