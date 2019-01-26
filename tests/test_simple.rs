extern crate ckb_vm;

use ckb_vm::{
    interpreter_run, run, DefaultCoreMachine, DefaultMachine, Error, FlatMemory, Instruction,
    SparseMemory, SupportMachine,
};
use std::fs::File;
use std::io::Read;

#[test]
pub fn test_simple_instructions() {
    let mut file = File::open("tests/programs/simple").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let result = run::<u32, SparseMemory<u32>>(&buffer, &vec![b"simple".to_vec()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_simple_instructions_64() {
    let mut file = File::open("tests/programs/simple64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec![b"simple".to_vec()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_simple_instructions_flatmemory() {
    let mut file = File::open("tests/programs/simple").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let result = run::<u32, FlatMemory<u32>>(&buffer, &vec![b"simple".to_vec()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

fn dummy_cycle_func(_i: &Instruction) -> u64 {
    1
}

#[test]
pub fn test_simple_cycles() {
    let mut file = File::open("tests/programs/simple64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let mut machine =
        DefaultMachine::<DefaultCoreMachine<u64, SparseMemory<u64>>>::new_with_cost_model(
            Box::new(dummy_cycle_func),
            517,
        );
    machine = machine
        .load_program(&buffer, &vec![b"simple".to_vec()])
        .unwrap();
    let result = interpreter_run(&mut machine);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    assert_eq!(SupportMachine::cycles(&machine), 517);
}

#[test]
pub fn test_simple_max_cycles_reached() {
    let mut file = File::open("tests/programs/simple64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    // Running simple64 should consume 517 cycles using dummy cycle func
    let mut machine =
        DefaultMachine::<DefaultCoreMachine<u64, SparseMemory<u64>>>::new_with_cost_model(
            Box::new(dummy_cycle_func),
            500,
        );
    machine = machine
        .load_program(&buffer, &vec![b"simple".to_vec()])
        .unwrap();
    let result = interpreter_run(&mut machine);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidCycles);
}

#[test]
pub fn test_simple_invalid_bits() {
    let mut file = File::open("tests/programs/simple").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec![b"simple".to_vec()]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidElfBits);
}
