pub mod bits;
pub mod decoder;
pub mod instructions;
pub mod machine;
pub mod memory;
pub mod syscalls;

pub use crate::{
    instructions::{Instruction, Register},
    machine::{CoreMachine, DefaultMachine, Machine},
    memory::{flat::FlatMemory, mmu::Mmu, sparse::SparseMemory, Memory},
    syscalls::Syscalls,
};
use std::io::{Error as IOError, ErrorKind};

pub const RISCV_PAGESIZE: usize = 1 << 12;
pub const RISCV_GENERAL_REGISTER_NUMBER: usize = 32;
// 128 MB
pub const RISCV_MAX_MEMORY: usize = 16 << 20;
pub const DEFAULT_STACK_SIZE: usize = 2 << 20;

// Register ABI names
pub const ZERO: usize = 0;
pub const RA: usize = 1;
pub const SP: usize = 2;
pub const GP: usize = 3;
pub const FP: usize = 8;
pub const TP: usize = 4;
pub const T0: usize = 5;
pub const T1: usize = 6;
pub const T2: usize = 7;
pub const T3: usize = 28;
pub const T4: usize = 29;
pub const T5: usize = 30;
pub const T6: usize = 31;
pub const S0: usize = 8;
pub const S1: usize = 9;
pub const S2: usize = 18;
pub const S3: usize = 19;
pub const S4: usize = 20;
pub const S5: usize = 21;
pub const S6: usize = 22;
pub const S7: usize = 23;
pub const S8: usize = 24;
pub const S9: usize = 25;
pub const S10: usize = 26;
pub const S11: usize = 27;
pub const A0: usize = 10;
pub const A1: usize = 11;
pub const A2: usize = 12;
pub const A3: usize = 13;
pub const A4: usize = 14;
pub const A5: usize = 15;
pub const A6: usize = 16;
pub const A7: usize = 17;

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const REGISTER_ABI_NAMES: [&str; 32] = [
    "zero", "ra", "sp", "gp",
    "tp", "t0", "t1", "t2",
    "s0", "s1", "a0", "a1",
    "a2", "a3", "a4", "a5",
    "a6", "a7", "s2", "s3",
    "s4", "s5", "s6", "s7",
    "s8", "s9", "s10", "s11",
    "t3", "t4", "t5", "t6",
];

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum Error {
    ParseError,
    Unaligned,
    OutOfBound,
    InvalidCycles,
    InvalidInstruction(u32),
    InvalidEcall(u64),
    InvalidElfBits,
    IO(ErrorKind),
    MaximumMmappingReached,
    InvalidPermission,
    Unimplemented,
}

impl From<IOError> for Error {
    fn from(error: IOError) -> Self {
        Error::IO(error.kind())
    }
}

pub fn run<R: Register, M: Memory + Default>(
    program: &[u8],
    args: &[Vec<u8>],
) -> Result<u8, Error> {
    DefaultMachine::<R, M>::default().run(program, args)
}
