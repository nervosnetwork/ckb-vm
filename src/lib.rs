pub mod bits;
pub mod decoder;
pub mod instructions;
pub mod machine;
pub mod memory;
pub mod syscalls;

#[cfg(feature = "jit")]
mod jit;

pub use crate::{
    instructions::{Instruction, Register},
    machine::{
        trace::TraceMachine, CoreMachine, DefaultCoreMachine, DefaultMachine,
        DefaultMachineBuilder, InstructionCycleFunc, Machine, SupportMachine,
    },
    memory::{flat::FlatMemory, sparse::SparseMemory, wxorx::WXorXMemory, Memory},
    syscalls::Syscalls,
};
use bytes::Bytes;
use std::io::{Error as IOError, ErrorKind};

#[cfg(feature = "jit")]
pub use crate::jit::{
    default_jit_machine, BaselineJitMachine, BaselineJitRunData, DefaultTracer, TcgTracer,
};
pub use ckb_vm_definitions::{
    registers, DEFAULT_STACK_SIZE, RISCV_GENERAL_REGISTER_NUMBER, RISCV_MAX_MEMORY, RISCV_PAGES,
    RISCV_PAGESIZE,
};

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum Error {
    ParseError,
    Unaligned,
    OutOfBound,
    InvalidCycles,
    InvalidInstruction(u32),
    InvalidEcall(u64),
    InvalidElfBits,
    InvalidOp(u8),
    IO(ErrorKind),
    Dynasm(i32),
    Asm(u8),
    MaximumMmappingReached,
    InvalidPermission,
    Unexpected,
    Unimplemented,
}

impl From<IOError> for Error {
    fn from(error: IOError) -> Self {
        Error::IO(error.kind())
    }
}

pub fn run<R: Register, M: Memory<R> + Default>(
    program: &Bytes,
    args: &[Bytes],
) -> Result<i8, Error> {
    let mut machine =
        TraceMachine::new(DefaultMachine::<DefaultCoreMachine<R, WXorXMemory<R, M>>>::default());
    machine.load_program(program, args)?;
    machine.run()
}

#[cfg(test)]
mod tests {
    use super::bits::power_of_2;
    use super::*;

    #[test]
    fn test_max_memory_must_be_multiple_of_pages() {
        assert_eq!(RISCV_MAX_MEMORY % RISCV_PAGESIZE, 0);
    }

    #[test]
    fn test_page_size_be_power_of_2() {
        assert!(power_of_2(RISCV_PAGESIZE));
    }
}
