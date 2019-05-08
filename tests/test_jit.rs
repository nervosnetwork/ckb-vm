#![cfg(feature = "jit")]

use bytes::Bytes;
use ckb_vm::{
    default_jit_machine,
    registers::{A0, A1, A2, A3, A4, A5, A7},
    BaselineJitMachine, BaselineJitRunData, Error, Instruction, Register, SupportMachine, Syscalls,
    TcgTracer,
};
use std::fs::File;
use std::io::Read;

#[test]
pub fn test_tcg_simple64() {
    let mut file = File::open("tests/programs/simple64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let machine = BaselineJitMachine::new(buffer, Box::new(TcgTracer::default()));
    let result = machine.run(&vec!["simple".into()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().0, 0);
}

#[test]
pub fn test_jit_simple64() {
    let mut file = File::open("tests/programs/simple64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let mut pair = (-1, default_jit_machine(&buffer));

    // Run the program 20 times to make sure JIT is triggered.
    for _ in 1..20 {
        pair = pair.1.run(&vec!["simple".into()]).unwrap();
        assert_eq!(pair.0, 0);
        pair.0 = -1;
    }
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
pub fn test_jit_with_custom_syscall() {
    let mut file = File::open("tests/programs/syscall64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let run_data = BaselineJitRunData::default().syscall(Box::new(CustomSyscall {}));
    let machine = BaselineJitMachine::new(buffer, Box::new(TcgTracer::default()));
    let result = machine.run_with_data(&vec!["syscall".into()], run_data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().0, 39);
}

fn dummy_cycle_func(_i: Instruction) -> u64 {
    1
}

#[test]
pub fn test_jit_simple_cycles() {
    let mut file = File::open("tests/programs/simple64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let run_data = BaselineJitRunData::default()
        .max_cycles(517)
        .instruction_cycle_func(Box::new(dummy_cycle_func));
    let machine = BaselineJitMachine::new(buffer, Box::new(TcgTracer::default()));
    let result = machine.run_with_data(&vec!["simple".into()], run_data);
    assert!(result.is_ok());
    let pair = result.unwrap();
    assert_eq!(pair.0, 0);

    assert_eq!(SupportMachine::cycles(&pair.1), 517);
}

#[test]
pub fn test_jit_simple_max_cycles_reached() {
    let mut file = File::open("tests/programs/simple64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    // Running simple64 should consume 517 cycles using dummy cycle func
    let run_data = BaselineJitRunData::default()
        .max_cycles(500)
        .instruction_cycle_func(Box::new(dummy_cycle_func));
    let machine = BaselineJitMachine::new(buffer, Box::new(TcgTracer::default()));
    let result = machine.run_with_data(&vec!["simple".into()], run_data);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::InvalidCycles));
}
