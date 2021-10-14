use ckb_vm::machine::VERSION0;
use ckb_vm::{
    run, DefaultCoreMachine, DefaultMachineBuilder, Error, FlatMemory, Instruction, SparseMemory,
    SupportMachine, ISA_IMC,
};
use std::fs;

#[test]
pub fn test_simple_instructions() {
    let buffer = fs::read("tests/programs/simple").unwrap().into();
    let result = run::<u32, SparseMemory<u32>>(&buffer, &vec!["simple".into()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_simple_instructions_64() {
    let buffer = fs::read("tests/programs/simple64").unwrap().into();
    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["simple".into()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_simple_instructions_flatmemory() {
    let buffer = fs::read("tests/programs/simple").unwrap().into();
    let result = run::<u32, FlatMemory<u32>>(&buffer, &vec!["simple".into()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

fn dummy_cycle_func(_i: Instruction, _: u64, _: u64, _: bool) -> u64 {
    1
}

#[test]
pub fn test_simple_cycles() {
    let buffer = fs::read("tests/programs/simple64").unwrap().into();
    let core_machine = DefaultCoreMachine::<u64, SparseMemory<u64>>::new(ISA_IMC, VERSION0, 708);
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

    assert_eq!(SupportMachine::cycles(&machine), 708);
}

#[test]
pub fn test_simple_max_cycles_reached() {
    let buffer = fs::read("tests/programs/simple64").unwrap().into();
    // Running simple64 should consume 708 cycles using dummy cycle func
    let core_machine = DefaultCoreMachine::<u64, SparseMemory<u64>>::new(ISA_IMC, VERSION0, 700);
    let mut machine =
        DefaultMachineBuilder::<DefaultCoreMachine<u64, SparseMemory<u64>>>::new(core_machine)
            .instruction_cycle_func(Box::new(dummy_cycle_func))
            .build();
    machine
        .load_program(&buffer, &vec!["simple".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::CyclesExceeded);
}

#[test]
pub fn test_simple_invalid_bits() {
    let buffer = fs::read("tests/programs/simple").unwrap().into();
    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["simple".into()]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::ElfBits);
}

#[test]
pub fn test_simple_loaded_bytes() {
    let buffer = fs::read("tests/programs/simple64").unwrap().into();
    let core_machine =
        DefaultCoreMachine::<u64, SparseMemory<u64>>::new(ISA_IMC, VERSION0, u64::max_value());
    let mut machine = DefaultMachineBuilder::new(core_machine).build();
    let bytes = machine
        .load_program(&buffer, &vec!["simple".into()])
        .unwrap();
    assert_eq!(bytes, 3831);
}

#[test]
pub fn test_simple_cycles_overflow() {
    let buffer = fs::read("tests/programs/simple64").unwrap().into();
    let core_machine =
        DefaultCoreMachine::<u64, SparseMemory<u64>>::new(ISA_IMC, VERSION0, u64::MAX);
    let mut machine =
        DefaultMachineBuilder::<DefaultCoreMachine<u64, SparseMemory<u64>>>::new(core_machine)
            .instruction_cycle_func(Box::new(dummy_cycle_func))
            .build();
    machine.set_cycles(u64::MAX - 10);
    machine
        .load_program(&buffer, &vec!["simple".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::CyclesOverflow);
}
