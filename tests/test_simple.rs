extern crate ckb_vm;

use bytes::Bytes;
use ckb_vm::machine::VERSION0;
use ckb_vm::{
    run, DefaultCoreMachine, DefaultMachine, DefaultMachineBuilder, Error, FlatMemory, Instruction,
    SparseMemory, SupportMachine, ISA_IMC,
};
use std::fs::File;
use std::io::Read;

#[test]
pub fn test_simple_instructions() {
    let mut file = File::open("tests/programs/simple").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u32, SparseMemory<u32>>(&buffer, &vec!["simple".into()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_simple_instructions_64() {
    let mut file = File::open("tests/programs/simple64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["simple".into()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_simple_instructions_flatmemory() {
    let mut file = File::open("tests/programs/simple").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u32, FlatMemory<u32>>(&buffer, &vec!["simple".into()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

fn dummy_cycle_func(_i: Instruction) -> u64 {
    1
}

#[test]
pub fn test_simple_cycles() {
    let mut file = File::open("tests/programs/simple64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let core_machine = DefaultCoreMachine::<u64, SparseMemory<u64>>::new(ISA_IMC, VERSION0, 517);
    let mut machine =
        DefaultMachineBuilder::<DefaultCoreMachine<u64, SparseMemory<u64>>>::new(core_machine)
            .instruction_cycle_func(Box::new(dummy_cycle_func))
            .build();
    machine
        .load_program(&buffer, &vec!["simple".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    assert_eq!(SupportMachine::cycles(&machine), 517);
}

#[test]
pub fn test_simple_max_cycles_reached() {
    let mut file = File::open("tests/programs/simple64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    // Running simple64 should consume 517 cycles using dummy cycle func
    let core_machine = DefaultCoreMachine::<u64, SparseMemory<u64>>::new(ISA_IMC, VERSION0, 500);
    let mut machine =
        DefaultMachineBuilder::<DefaultCoreMachine<u64, SparseMemory<u64>>>::new(core_machine)
            .instruction_cycle_func(Box::new(dummy_cycle_func))
            .build();
    machine
        .load_program(&buffer, &vec!["simple".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidCycles);
}

#[test]
pub fn test_simple_invalid_bits() {
    let mut file = File::open("tests/programs/simple").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["simple".into()]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidElfBits);
}

#[test]
pub fn test_simple_loaded_bytes() {
    let mut file = File::open("tests/programs/simple64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let mut machine = DefaultMachine::<DefaultCoreMachine<u64, SparseMemory<u64>>>::default();
    let bytes = machine
        .load_program(&buffer, &vec!["simple".into()])
        .unwrap();
    assert_eq!(bytes, 4055);
}
