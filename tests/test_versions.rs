#![cfg(has_asm)]

extern crate ckb_vm;

use bytes::Bytes;
use ckb_vm::{
    machine::{
        aot::{AotCode, AotCompilingMachine},
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

fn compile_aot_code(program: String, version: u32) -> AotCode {
    let mut file = File::open(format!("tests/programs/{}", program)).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let mut aot_machine = AotCompilingMachine::load(&buffer.clone(), None, version).unwrap();
    aot_machine.compile().unwrap()
}

fn create_aot_machine<'a>(program: String, code: &'a AotCode, version: u32) -> AsmMachine<'a> {
    let mut file = File::open(format!("tests/programs/{}", program)).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let asm_core = AsmCoreMachine::new(version, u64::max_value());
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core).build();
    let mut machine = AsmMachine::new(core, Some(code));
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

#[test]
pub fn test_aot_version0_argv_null() {
    let code = compile_aot_code("argv_null_test".to_string(), VERSION0);
    let mut machine = create_aot_machine("argv_null_test".to_string(), &code, VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[test]
pub fn test_aot_version0_sp_alignment() {
    let code = compile_aot_code("sp_alignment_test".to_string(), VERSION0);
    let mut machine = create_aot_machine("sp_alignment_test".to_string(), &code, VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[test]
pub fn test_aot_version0_jalr_bug() {
    let code = compile_aot_code("jalr_bug".to_string(), VERSION0);
    let mut machine = create_aot_machine("jalr_bug".to_string(), &code, VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), -1);
}

#[test]
pub fn test_aot_version0_jalr_bug_noc() {
    let code = compile_aot_code("jalr_bug_noc".to_string(), VERSION0);
    let mut machine = create_aot_machine("jalr_bug_noc".to_string(), &code, VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), -1);
}

#[test]
pub fn test_aot_version0_read_at_boundary() {
    let code = compile_aot_code("read_at_boundary64".to_string(), VERSION0);
    let mut machine = create_aot_machine("read_at_boundary64".to_string(), &code, VERSION0);
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::OutOfBound));
}

#[test]
pub fn test_aot_version0_write_at_boundary() {
    let code = compile_aot_code("write_at_boundary64".to_string(), VERSION0);
    let mut machine = create_aot_machine("write_at_boundary64".to_string(), &code, VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_aot_version1_argv_null() {
    let code = compile_aot_code("argv_null_test".to_string(), VERSION1);
    let mut machine = create_aot_machine("argv_null_test".to_string(), &code, VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_aot_version1_sp_alignment() {
    let code = compile_aot_code("sp_alignment_test".to_string(), VERSION1);
    let mut machine = create_aot_machine("sp_alignment_test".to_string(), &code, VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_aot_version1_jalr_bug() {
    let code = compile_aot_code("jalr_bug".to_string(), VERSION1);
    let mut machine = create_aot_machine("jalr_bug".to_string(), &code, VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_aot_version1_jalr_bug_noc() {
    let code = compile_aot_code("jalr_bug_noc".to_string(), VERSION1);
    let mut machine = create_aot_machine("jalr_bug_noc".to_string(), &code, VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_aot_version1_read_at_boundary() {
    let code = compile_aot_code("read_at_boundary64".to_string(), VERSION1);
    let mut machine = create_aot_machine("read_at_boundary64".to_string(), &code, VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_aot_version1_write_at_boundary() {
    let code = compile_aot_code("write_at_boundary64".to_string(), VERSION1);
    let mut machine = create_aot_machine("write_at_boundary64".to_string(), &code, VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_version0_unaligned64() {
    let program = "unaligned64";
    let mut file = File::open(format!("tests/programs/{}", program)).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let asm_core = AsmCoreMachine::new(VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core).build();
    let mut machine = AsmMachine::new(core, None);
    let result = machine.load_program(&buffer, &vec![program.into()]);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::InvalidPermission));
}

#[test]
pub fn test_asm_version1_unaligned64() {
    let mut machine = create_asm_machine("unaligned64".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_aot_version0_unaligned64() {
    let program = "unaligned64";
    let code = compile_aot_code(program.to_string(), VERSION1);
    let mut file = File::open(format!("tests/programs/{}", program)).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let asm_core = AsmCoreMachine::new(VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core).build();
    let mut machine = AsmMachine::new(core, Some(&code));
    let result = machine.load_program(&buffer, &vec![program.into()]);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::InvalidPermission));
}

#[test]
pub fn test_aot_version1_unaligned64() {
    let code = compile_aot_code("unaligned64".to_string(), VERSION1);
    let mut machine = create_aot_machine("unaligned64".to_string(), &code, VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}
