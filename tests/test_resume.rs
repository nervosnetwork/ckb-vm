#![cfg(has_asm)]

use bytes::Bytes;
use ckb_vm::{
    machine::{
        aot::AotCompilingMachine,
        asm::{AsmCoreMachine, AsmMachine},
        DefaultCoreMachine, SupportMachine, VERSION1,
    },
    memory::sparse::SparseMemory,
    snapshot::{make_snapshot, resume},
    DefaultMachineBuilder, Error, Instruction, ISA_IMC,
};
use std::fs::File;
use std::io::Read;

fn dummy_cycle_func(_i: Instruction) -> u64 {
    1
}

#[test]
pub fn test_resume_asm_2_asm() {
    let mut file = File::open("tests/programs/alloc_many").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    // The cycles required for complete execution is 4194622
    let asm_core1 = AsmCoreMachine::new(ISA_IMC, VERSION1, 4194600);
    let core1 = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core1)
        .instruction_cycle_func(Box::new(dummy_cycle_func))
        .build();
    let mut machine1 = AsmMachine::new(core1, None);
    machine1
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    let result1 = machine1.run();
    let cycles1 = machine1.machine.cycles();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::InvalidCycles);
    let snapshot = make_snapshot(&mut machine1.machine).unwrap();

    let asm_core2 = AsmCoreMachine::new(ISA_IMC, VERSION1, 30);
    let core2 = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core2)
        .instruction_cycle_func(Box::new(dummy_cycle_func))
        .build();
    let mut machine2 = AsmMachine::new(core2, None);
    machine2
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    resume(&mut machine2.machine, &snapshot).unwrap();
    let result2 = machine2.run();
    let cycles2 = machine2.machine.cycles();
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles1 + cycles2, 4194622);
}

#[test]
pub fn test_resume_asm_2_asm_2_asm() {
    let mut file = File::open("tests/programs/alloc_many").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let asm_core1 = AsmCoreMachine::new(ISA_IMC, VERSION1, 1000000);
    let core1 = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core1)
        .instruction_cycle_func(Box::new(dummy_cycle_func))
        .build();
    let mut machine1 = AsmMachine::new(core1, None);
    machine1
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    let result1 = machine1.run();
    let cycles1 = machine1.machine.cycles();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::InvalidCycles);
    let snapshot1 = make_snapshot(&mut machine1.machine).unwrap();

    let asm_core2 = AsmCoreMachine::new(ISA_IMC, VERSION1, 2000000);
    let core2 = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core2)
        .instruction_cycle_func(Box::new(dummy_cycle_func))
        .build();
    let mut machine2 = AsmMachine::new(core2, None);
    machine2
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    resume(&mut machine2.machine, &snapshot1).unwrap();
    let result2 = machine2.run();
    let cycles2 = machine2.machine.cycles();
    assert!(result2.is_err());
    assert_eq!(result2.unwrap_err(), Error::InvalidCycles);
    let snapshot2 = make_snapshot(&mut machine2.machine).unwrap();

    let asm_core3 = AsmCoreMachine::new(ISA_IMC, VERSION1, 2000000);
    let core3 = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core3)
        .instruction_cycle_func(Box::new(dummy_cycle_func))
        .build();
    let mut machine3 = AsmMachine::new(core3, None);
    machine3
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    resume(&mut machine3.machine, &snapshot2).unwrap();
    let result3 = machine3.run();
    let cycles3 = machine3.machine.cycles();
    assert!(result3.is_ok());
    assert_eq!(result3.unwrap(), 0);
    assert_eq!(cycles1 + cycles2 + cycles3, 4194622);
}

#[test]
pub fn test_resume_asm_2_interpreter() {
    let mut file = File::open("tests/programs/alloc_many").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let asm_core1 = AsmCoreMachine::new(ISA_IMC, VERSION1, 4194600);
    let core1 = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core1)
        .instruction_cycle_func(Box::new(dummy_cycle_func))
        .build();
    let mut machine1 = AsmMachine::new(core1, None);
    machine1
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    let result1 = machine1.run();
    let cycles1 = machine1.machine.cycles();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::InvalidCycles);
    let snapshot = make_snapshot(&mut machine1.machine).unwrap();

    let core_machine2 = DefaultCoreMachine::<u64, SparseMemory<u64>>::new(ISA_IMC, VERSION1, 30);
    let mut machine2 =
        DefaultMachineBuilder::<DefaultCoreMachine<u64, SparseMemory<u64>>>::new(core_machine2)
            .instruction_cycle_func(Box::new(dummy_cycle_func))
            .build();
    machine2
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    resume(&mut machine2, &snapshot).unwrap();

    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles1 + cycles2, 4194622);
}

#[test]
pub fn test_resume_interpreter_2_interpreter() {
    let mut file = File::open("tests/programs/alloc_many").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let core_machine1 =
        DefaultCoreMachine::<u64, SparseMemory<u64>>::new(ISA_IMC, VERSION1, 4194600);
    let mut machine1 =
        DefaultMachineBuilder::<DefaultCoreMachine<u64, SparseMemory<u64>>>::new(core_machine1)
            .instruction_cycle_func(Box::new(dummy_cycle_func))
            .build();
    machine1
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    let result1 = machine1.run();
    let cycles1 = machine1.cycles();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::InvalidCycles);
    let snapshot = make_snapshot(&mut machine1).unwrap();

    let core_machine2 = DefaultCoreMachine::<u64, SparseMemory<u64>>::new(ISA_IMC, VERSION1, 30);
    let mut machine2 =
        DefaultMachineBuilder::<DefaultCoreMachine<u64, SparseMemory<u64>>>::new(core_machine2)
            .instruction_cycle_func(Box::new(dummy_cycle_func))
            .build();
    machine2
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    resume(&mut machine2, &snapshot).unwrap();
    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles1 + cycles2, 4194622);
}

#[test]
pub fn test_resume_interpreter_2_asm() {
    let mut file = File::open("tests/programs/alloc_many").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let core_machine1 =
        DefaultCoreMachine::<u64, SparseMemory<u64>>::new(ISA_IMC, VERSION1, 4194600);
    let mut machine1 =
        DefaultMachineBuilder::<DefaultCoreMachine<u64, SparseMemory<u64>>>::new(core_machine1)
            .instruction_cycle_func(Box::new(dummy_cycle_func))
            .build();
    machine1
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    let result1 = machine1.run();
    let cycles1 = machine1.cycles();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::InvalidCycles);
    let snapshot = make_snapshot(&mut machine1).unwrap();

    let asm_core2 = AsmCoreMachine::new(ISA_IMC, VERSION1, 30);
    let core2 = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core2)
        .instruction_cycle_func(Box::new(dummy_cycle_func))
        .build();
    let mut machine2 = AsmMachine::new(core2, None);
    machine2
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    resume(&mut machine2.machine, &snapshot).unwrap();
    let result2 = machine2.run();
    let cycles2 = machine2.machine.cycles();
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles1 + cycles2, 4194622);
}

#[test]
pub fn test_resume_aot_2_asm() {
    let mut file = File::open("tests/programs/alloc_many").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let mut aot_machine =
        AotCompilingMachine::load(&buffer, Some(Box::new(dummy_cycle_func)), ISA_IMC, VERSION1)
            .unwrap();
    let code = aot_machine.compile().unwrap();
    let asm_core1 = AsmCoreMachine::new(ISA_IMC, VERSION1, 4194600);
    let core1 = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core1)
        .instruction_cycle_func(Box::new(dummy_cycle_func))
        .build();
    let mut machine1 = AsmMachine::new(core1, Some(&code));
    machine1
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    let result1 = machine1.run();
    let cycles1 = machine1.machine.cycles();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::InvalidCycles);
    let snapshot = make_snapshot(&mut machine1.machine).unwrap();

    let asm_core2 = AsmCoreMachine::new(ISA_IMC, VERSION1, 30);
    let core2 = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core2)
        .instruction_cycle_func(Box::new(dummy_cycle_func))
        .build();
    let mut machine2 = AsmMachine::new(core2, None);
    machine2
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    resume(&mut machine2.machine, &snapshot).unwrap();
    let result2 = machine2.run();
    let cycles2 = machine2.machine.cycles();
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles1 + cycles2, 4194622);
}
