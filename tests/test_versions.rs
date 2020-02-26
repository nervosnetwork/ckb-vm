#![cfg(has_asm)]

extern crate ckb_vm;

use bytes::Bytes;
use ckb_vm::{
    machine::{
        asm::{AsmCoreMachine, AsmMachine},
        VERSION0, VERSION1,
    },
    DefaultCoreMachine, DefaultMachine, DefaultMachineBuilder, Error, SparseMemory,
};
use std::fs::File;
use std::io::Read;

fn create_rust_machine<'a>(
    program: String,
    version: u32,
) -> DefaultMachine<'a, DefaultCoreMachine<u64, SparseMemory<u64>>> {
    let mut file = File::open(format!("tests/programs/{}", program)).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let core_machine = DefaultCoreMachine::<u64, SparseMemory<u64>>::new(version, u64::max_value());
    let mut machine =
        DefaultMachineBuilder::<DefaultCoreMachine<u64, SparseMemory<u64>>>::new(core_machine)
            .build();
    machine
        .load_program(&buffer, &vec![program.into()])
        .unwrap();
    machine
}

fn create_asm_machine<'a>(program: String, version: u32) -> AsmMachine<'a> {
    let mut file = File::open(format!("tests/programs/{}", program)).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let asm_core = AsmCoreMachine::new(version, u64::max_value());
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core).build();
    let mut machine = AsmMachine::new(core, None);
    machine
        .load_program(&buffer, &vec![program.into()])
        .unwrap();
    machine
}

#[test]
pub fn test_rust_version0_argv_null() {
    let mut machine = create_rust_machine("argv_null_test".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[test]
pub fn test_rust_version0_sp_alignment() {
    let mut machine = create_rust_machine("sp_alignment_test".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[test]
pub fn test_rust_version1_argv_null() {
    let mut machine = create_rust_machine("argv_null_test".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_rust_version1_sp_alignment() {
    let mut machine = create_rust_machine("sp_alignment_test".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_version0_argv_null() {
    let mut machine = create_asm_machine("argv_null_test".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[test]
pub fn test_asm_version0_sp_alignment() {
    let mut machine = create_asm_machine("sp_alignment_test".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[test]
pub fn test_asm_version0_jalr_bug() {
    let mut machine = create_asm_machine("jalr_bug".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), -1);
}

#[test]
pub fn test_asm_version0_jalr_bug_noc() {
    let mut machine = create_asm_machine("jalr_bug_noc".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), -1);
}

#[test]
pub fn test_asm_version0_read_at_boundary() {
    let mut machine = create_asm_machine("read_at_boundary64".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::OutOfBound));
}

#[test]
pub fn test_asm_version0_write_at_boundary() {
    let mut machine = create_asm_machine("write_at_boundary64".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_version1_argv_null() {
    let mut machine = create_asm_machine("argv_null_test".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_version1_sp_alignment() {
    let mut machine = create_asm_machine("sp_alignment_test".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_version1_jalr_bug() {
    let mut machine = create_asm_machine("jalr_bug".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_version1_jalr_bug_noc() {
    let mut machine = create_asm_machine("jalr_bug_noc".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_version1_read_at_boundary() {
    let mut machine = create_asm_machine("read_at_boundary64".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_version1_write_at_boundary() {
    let mut machine = create_asm_machine("write_at_boundary64".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}
