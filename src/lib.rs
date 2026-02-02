#[macro_use]
extern crate derive_more;

pub mod bits;
pub mod cost_model;
pub mod debugger;
pub mod decoder;
pub mod elf;
pub mod error;
pub mod instructions;
pub mod machine;
pub mod memory;
pub mod rng;
pub mod snapshot;
pub mod snapshot2;
pub mod syscalls;

pub use bytes;
pub use ckb_vm_definitions;

pub use crate::{
    debugger::Debugger,
    instructions::{Instruction, Register},
    machine::{
        CoreMachine, DefaultCoreMachine, DefaultMachine, DefaultMachineRunner, FlattenedArgsReader,
        InstructionCycleFunc, Machine, RustDefaultMachineBuilder, SupportMachine,
        trace::TraceMachine,
    },
    memory::{Memory, flat::FlatMemory, sparse::SparseMemory, wxorx::WXorXMemory},
    syscalls::Syscalls,
};
pub use bytes::Bytes;

pub use ckb_vm_definitions::{
    DEFAULT_MEMORY_SIZE, ISA_A, ISA_B, ISA_IMC, ISA_MOP, MEMORY_FRAME_SHIFTS, MEMORY_FRAMESIZE,
    RISCV_GENERAL_REGISTER_NUMBER, RISCV_PAGE_SHIFTS, RISCV_PAGESIZE, registers,
};

pub use error::Error;

pub fn run<R: Register, M: Memory<REG = R>>(program: &Bytes, args: &[Bytes]) -> Result<i8, Error> {
    run_with_memory::<R, M>(program, args, DEFAULT_MEMORY_SIZE)
}

pub fn run_with_memory<R: Register, M: Memory<REG = R>>(
    program: &Bytes,
    args: &[Bytes],
    memory_size: usize,
) -> Result<i8, Error> {
    let core_machine = DefaultCoreMachine::<R, WXorXMemory<M>>::new_with_memory(
        ISA_IMC | ISA_B | ISA_MOP,
        machine::VERSION2,
        u64::MAX,
        memory_size,
    );
    let mut machine = TraceMachine::new(RustDefaultMachineBuilder::new(core_machine).build());
    machine.load_program(program, args.iter().map(|e| Ok(e.clone())))?;
    machine.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_memory_must_be_multiple_of_pages() {
        assert_eq!(DEFAULT_MEMORY_SIZE % RISCV_PAGESIZE, 0);
    }

    #[test]
    fn test_page_size_be_power_of_2() {
        assert!(RISCV_PAGESIZE.is_power_of_two());
    }
}
