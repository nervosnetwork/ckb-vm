extern crate ckb_vm;

use bytes::Bytes;
use ckb_vm::{
    registers::{A0, A1, A2, A3, A4, A5, A7},
    run, DefaultCoreMachine, DefaultMachineBuilder, Error, Register, SparseMemory, SupportMachine,
    Syscalls,
};
use std::fs::File;
use std::io::Read;

#[test]
pub fn test_andi() {
    let mut file = File::open("tests/programs/andi").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u32, SparseMemory<u32>>(&buffer, &vec!["andi".into()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_nop() {
    let mut file = File::open("tests/programs/nop").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u32, SparseMemory<u32>>(&buffer, &vec!["nop".into()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

pub struct CustomSyscall {}

impl<Mac: SupportMachine> Syscalls<Mac> for CustomSyscall {
    fn initialize(&mut self, _machine: &mut Mac) -> Result<(), Error> {
        Ok(())
    }

    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, Error> {
        let code = &machine.registers()[A7];
        if code.to_i32() != 1111 {
            return Ok(false);
        }
        let result = machine.registers()[A0]
            .overflowing_add(&machine.registers()[A1])
            .overflowing_add(&machine.registers()[A2])
            .overflowing_add(&machine.registers()[A3])
            .overflowing_add(&machine.registers()[A4])
            .overflowing_add(&machine.registers()[A5]);
        machine.set_register(A0, result);
        Ok(true)
    }
}

#[test]
pub fn test_custom_syscall() {
    let mut file = File::open("tests/programs/syscall64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let mut machine =
        DefaultMachineBuilder::<DefaultCoreMachine<u64, SparseMemory<u64>>>::default()
            .syscall(Box::new(CustomSyscall {}))
            .build();
    machine
        .load_program(&buffer, &vec!["syscall".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 39);
}

#[test]
pub fn test_trace() {
    let mut file = File::open("tests/programs/trace64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["trace64".into()]);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::InvalidPermission));
}

#[test]
pub fn test_jump0() {
    let mut file = File::open("tests/programs/jump0_64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["jump0_64".into()]);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::InvalidPermission));
}

#[test]
pub fn test_misaligned_jump64() {
    let mut file = File::open("tests/programs/misaligned_jump64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["misaligned_jump64".into()]);
    assert!(result.is_ok());
}

#[test]
pub fn test_mulw64() {
    let mut file = File::open("tests/programs/mulw64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["mulw64".into()]);
    assert!(result.is_ok());
}
