#![cfg(feature = "asm")]

use ckb_vm::{
    machine::asm::{AsmCoreMachine, AsmMachine},
    registers::{A0, A1, A2, A3, A4, A5, A7},
    DefaultMachineBuilder, Error, Instruction, Register, SupportMachine, Syscalls,
};
use std::fs::File;
use std::io::Read;

#[test]
pub fn test_asm_simple64() {
    let mut file = File::open("tests/programs/simple64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let mut machine = AsmMachine::default();
    machine
        .load_program(&buffer, &vec![b"simple".to_vec()])
        .unwrap();
    let result = machine.run();
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
pub fn test_asm_with_custom_syscall() {
    let mut file = File::open("tests/programs/syscall64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::default()
        .syscall(Box::new(CustomSyscall {}))
        .build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec![b"syscall".to_vec()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 39);
}

fn dummy_cycle_func(_i: &Instruction) -> u64 {
    1
}

#[test]
pub fn test_asm_simple_cycles() {
    let mut file = File::open("tests/programs/simple64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let asm_core = AsmCoreMachine::new_with_max_cycles(517);
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
        .instruction_cycle_func(Box::new(dummy_cycle_func))
        .build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec![b"syscall".to_vec()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    assert_eq!(SupportMachine::cycles(&machine.machine), 517);
}

#[test]
pub fn test_asm_simple_max_cycles_reached() {
    let mut file = File::open("tests/programs/simple64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    // Running simple64 should consume 517 cycles using dummy cycle func
    let asm_core = AsmCoreMachine::new_with_max_cycles(500);
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
        .instruction_cycle_func(Box::new(dummy_cycle_func))
        .build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec![b"syscall".to_vec()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidCycles);
}

#[test]
pub fn test_asm_trace() {
    let mut file = File::open("tests/programs/trace64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let mut machine = AsmMachine::default();
    machine
        .load_program(&buffer, &vec![b"simple".to_vec()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 7);
}