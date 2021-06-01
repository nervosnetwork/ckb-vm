#![cfg(has_asm)]

use ckb_vm::{
    machine::{
        asm::{AsmCoreMachine, AsmMachine, AsmWrapMachine},
        trace::TraceMachine,
        DefaultCoreMachine, DefaultMachine, SupportMachine, VERSION0, VERSION1,
    },
    memory::sparse::SparseMemory,
    snapshot::{make_snapshot, resume, Snapshot},
    Bytes, DefaultMachineBuilder, Error, ISA_IMC,
};

#[test]
fn test_resume_interpreter_with_trace_2_asm() {
    resume_interpreter_with_trace_2_asm_inner(VERSION1, 4194622);
    resume_interpreter_with_trace_2_asm_inner(VERSION0, 4194622);
}

#[test]
fn test_resume_aot_2_asm() {
    resume_aot_2_asm(VERSION1, 4194622);
    resume_aot_2_asm(VERSION0, 4194622);
}

#[test]
fn test_resume_interpreter_2_asm() {
    resume_interpreter_2_asm(VERSION1, 4194622);
    resume_interpreter_2_asm(VERSION0, 4194622);
}

#[test]
fn test_resume_interpreter_2_interpreter() {
    resume_interpreter_2_interpreter(VERSION1, 4194622);
    resume_interpreter_2_interpreter(VERSION0, 4194622);
}

#[test]
fn test_resume_asm_2_interpreter() {
    resume_asm_2_interpreter(VERSION1, 4194622);
    resume_asm_2_interpreter(VERSION0, 4194622);
}

#[test]
fn test_resume_asm_2_asm_2_asm() {
    resume_asm_2_asm_2_asm(VERSION1, 4194622);
    resume_asm_2_asm_2_asm(VERSION0, 4194622);
}

#[test]
fn test_resume_asm_2_asm() {
    resume_asm_2_asm(VERSION1, 4194622);
    resume_asm_2_asm(VERSION0, 4194622);
}

pub fn resume_asm_2_asm(version: u32, except_cycles: u64) {
    let buffer = std::fs::read("tests/programs/alloc_many").unwrap().into();

    // The cycles required for complete execution is 4194622
    let mut machine1 = MachineTy::Asm.build(version, except_cycles - 30);
    machine1
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    let result1 = machine1.run();
    let cycles1 = machine1.cycles();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::InvalidCycles);
    let snapshot = machine1.snapshot().unwrap();

    let mut machine2 = MachineTy::Asm.build(version, 40);
    machine2
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    machine2.resume(&snapshot).unwrap();
    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles1 + cycles2, except_cycles);
}

pub fn resume_asm_2_asm_2_asm(version: u32, except_cycles: u64) {
    let buffer = std::fs::read("tests/programs/alloc_many").unwrap().into();

    let mut machine1 = MachineTy::Asm.build(version, 1000000);
    machine1
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    let result1 = machine1.run();
    let cycles1 = machine1.cycles();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::InvalidCycles);
    let snapshot1 = machine1.snapshot().unwrap();

    let mut machine2 = MachineTy::Asm.build(version, 2000000);
    machine2
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    machine2.resume(&snapshot1).unwrap();
    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert!(result2.is_err());
    assert_eq!(result2.unwrap_err(), Error::InvalidCycles);
    let snapshot2 = machine2.snapshot().unwrap();

    let mut machine3 = MachineTy::Asm.build(version, 2000000);
    machine3
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    machine3.resume(&snapshot2).unwrap();
    let result3 = machine3.run();
    let cycles3 = machine3.cycles();
    assert!(result3.is_ok());
    assert_eq!(result3.unwrap(), 0);
    assert_eq!(cycles1 + cycles2 + cycles3, except_cycles);
}

pub fn resume_asm_2_interpreter(version: u32, except_cycles: u64) {
    let buffer = std::fs::read("tests/programs/alloc_many").unwrap().into();

    let mut machine1 = MachineTy::Asm.build(version, except_cycles - 30);
    machine1
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    let result1 = machine1.run();
    let cycles1 = machine1.cycles();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::InvalidCycles);
    let snapshot = machine1.snapshot().unwrap();

    let mut machine2 = MachineTy::Interpreter.build(version, 40);
    machine2
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    machine2.resume(&snapshot).unwrap();

    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles1 + cycles2, except_cycles);
}

pub fn resume_interpreter_2_interpreter(version: u32, except_cycles: u64) {
    let buffer = std::fs::read("tests/programs/alloc_many").unwrap().into();

    let mut machine1 = MachineTy::Interpreter.build(version, except_cycles - 30);
    machine1
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    let result1 = machine1.run();
    let cycles1 = machine1.cycles();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::InvalidCycles);
    let snapshot = machine1.snapshot().unwrap();

    let mut machine2 = MachineTy::Interpreter.build(version, 30);
    machine2
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    machine2.resume(&snapshot).unwrap();
    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles1 + cycles2, except_cycles);
}

pub fn resume_interpreter_2_asm(version: u32, except_cycles: u64) {
    let buffer = std::fs::read("tests/programs/alloc_many").unwrap().into();

    let mut machine1 = MachineTy::Interpreter.build(version, except_cycles - 30);
    machine1
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    let result1 = machine1.run();
    let cycles1 = machine1.cycles();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::InvalidCycles);
    let snapshot = machine1.snapshot().unwrap();

    let mut machine2 = MachineTy::Asm.build(version, 30);
    machine2
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    machine2.resume(&snapshot).unwrap();
    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles1 + cycles2, except_cycles);
}

pub fn resume_aot_2_asm(version: u32, except_cycles: u64) {
    let buffer = std::fs::read("tests/programs/alloc_many").unwrap().into();

    let mut machine1 = MachineTy::Aot.build(version, except_cycles - 30);
    machine1
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    let result1 = machine1.run();
    let cycles1 = machine1.cycles();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::InvalidCycles);
    let snapshot = machine1.snapshot().unwrap();

    let mut machine2 = MachineTy::Asm.build(version, 40);
    machine2
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    machine2.resume(&snapshot).unwrap();
    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles1 + cycles2, except_cycles);
}

pub fn resume_interpreter_with_trace_2_asm_inner(version: u32, except_cycles: u64) {
    let buffer = std::fs::read("tests/programs/alloc_many").unwrap().into();

    let mut machine1 = MachineTy::InterpreterWithTrace.build(version, except_cycles - 30);
    machine1
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    let result1 = machine1.run();
    let cycles1 = machine1.cycles();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::InvalidCycles);
    let snapshot = machine1.snapshot().unwrap();

    let mut machine2 = MachineTy::Asm.build(version, 30);
    machine2
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    machine2.resume(&snapshot).unwrap();
    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles1 + cycles2, except_cycles);
}

enum MachineTy {
    Asm,
    Aot,
    Interpreter,
    InterpreterWithTrace,
}

impl MachineTy {
    fn build<'a>(self, version: u32, max_cycles: u64) -> Machine<'a> {
        match self {
            MachineTy::Asm => {
                let asm_core = AsmCoreMachine::new(ISA_IMC, version, max_cycles);
                let asm_wrap = AsmWrapMachine::new(asm_core, false);
                let core = DefaultMachineBuilder::new(asm_wrap)
                    .instruction_cycle_func(Box::new(|_| 1))
                    .build();
                Machine::Asm(AsmMachine::new(core))
            }
            MachineTy::Aot => {
                let asm_core = AsmCoreMachine::new(ISA_IMC, version, max_cycles);
                let asm_wrap = AsmWrapMachine::new(asm_core, true);
                let core = DefaultMachineBuilder::new(asm_wrap)
                    .instruction_cycle_func(Box::new(|_| 1))
                    .build();
                Machine::Aot(AsmMachine::new(core))
            }
            MachineTy::Interpreter => {
                let core_machine =
                    DefaultCoreMachine::<u64, SparseMemory<u64>>::new(ISA_IMC, version, max_cycles);
                Machine::Interpreter(
                    DefaultMachineBuilder::<DefaultCoreMachine<u64, SparseMemory<u64>>>::new(
                        core_machine,
                    )
                    .instruction_cycle_func(Box::new(|_| 1))
                    .build(),
                )
            }
            MachineTy::InterpreterWithTrace => {
                let core_machine =
                    DefaultCoreMachine::<u64, SparseMemory<u64>>::new(ISA_IMC, version, max_cycles);
                Machine::InterpreterWithTrace(TraceMachine::new(
                    DefaultMachineBuilder::<DefaultCoreMachine<u64, SparseMemory<u64>>>::new(
                        core_machine,
                    )
                    .instruction_cycle_func(Box::new(|_| 1))
                    .build(),
                ))
            }
        }
    }
}

enum Machine<'a> {
    Asm(AsmMachine<'static>),
    Aot(AsmMachine<'a>),
    Interpreter(DefaultMachine<'static, DefaultCoreMachine<u64, SparseMemory<u64>>>),
    InterpreterWithTrace(TraceMachine<'static, DefaultCoreMachine<u64, SparseMemory<u64>>>),
}

impl<'a> Machine<'a> {
    fn load_program(&mut self, program: &Bytes, args: &[Bytes]) -> Result<u64, Error> {
        use Machine::*;
        match self {
            Asm(inner) => inner.load_program(program, args),
            Aot(inner) => inner.load_program(program, args),
            Interpreter(inner) => inner.load_program(program, args),
            InterpreterWithTrace(inner) => inner.load_program(program, args),
        }
    }

    fn run(&mut self) -> Result<i8, Error> {
        use Machine::*;
        match self {
            Asm(inner) => inner.run(),
            Aot(inner) => inner.run(),
            Interpreter(inner) => inner.run(),
            InterpreterWithTrace(inner) => inner.run(),
        }
    }

    fn cycles(&self) -> u64 {
        use Machine::*;
        match self {
            Asm(inner) => inner.machine.cycles(),
            Aot(inner) => inner.machine.cycles(),
            Interpreter(inner) => inner.cycles(),
            InterpreterWithTrace(inner) => inner.machine.cycles(),
        }
    }

    fn snapshot(&mut self) -> Result<Snapshot, Error> {
        use Machine::*;
        match self {
            Asm(inner) => make_snapshot(&mut inner.machine),
            Aot(inner) => make_snapshot(&mut inner.machine),
            Interpreter(inner) => make_snapshot(inner),
            InterpreterWithTrace(inner) => make_snapshot(&mut inner.machine),
        }
    }

    fn resume(&mut self, snap: &Snapshot) -> Result<(), Error> {
        use Machine::*;
        match self {
            Asm(inner) => resume(&mut inner.machine, snap),
            Aot(inner) => resume(&mut inner.machine, snap),
            Interpreter(inner) => resume(inner, snap),
            InterpreterWithTrace(inner) => resume(&mut inner.machine, snap),
        }
    }
}
