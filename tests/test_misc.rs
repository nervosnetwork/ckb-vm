extern crate ckb_vm;

use ckb_vm::{
    run, CoreMachine, DefaultMachine, Error, SparseMemory, Syscalls, A0, A1, A2, A3, A4, A5, A7,
};
use std::fs::File;
use std::io::Read;

#[test]
pub fn test_andi() {
    let mut file = File::open("tests/programs/andi").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let result = run::<u32, SparseMemory>(&buffer, &vec![b"andi".to_vec()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_nop() {
    let mut file = File::open("tests/programs/nop").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let result = run::<u32, SparseMemory>(&buffer, &vec![b"nop".to_vec()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

pub struct CustomSyscall {}

impl Syscalls<u64, SparseMemory> for CustomSyscall {
    fn initialize(&mut self, _machine: &mut CoreMachine<u64, SparseMemory>) -> Result<(), Error> {
        Ok(())
    }

    fn ecall(&mut self, machine: &mut CoreMachine<u64, SparseMemory>) -> Result<bool, Error> {
        let code = machine.registers()[A7];
        if code != 1111 {
            return Ok(false);
        }
        let result = machine.registers()[A0]
            + machine.registers()[A1]
            + machine.registers()[A2]
            + machine.registers()[A3]
            + machine.registers()[A4]
            + machine.registers()[A5];
        machine.registers_mut()[A0] = result;
        Ok(true)
    }
}

#[test]
pub fn test_custom_syscall() {
    let mut file = File::open("tests/programs/syscall64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let mut machine = DefaultMachine::<u64, SparseMemory>::default();
    machine.add_syscall_module(Box::new(CustomSyscall {}));
    let result = machine.run(&buffer, &vec![b"syscall".to_vec()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 39);
}
