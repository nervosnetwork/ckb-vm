#![cfg(has_asm)]

#[cfg(has_aot)]
use ckb_vm::machine::{aot::AotCompilingMachine, asm::AotCode};

use bytes::Bytes;
use ckb_vm::{
    machine::{
        asm::{AsmCoreMachine, AsmMachine},
        trace::TraceMachine,
        DefaultCoreMachine, DefaultMachine, SupportMachine, VERSION0, VERSION1,
    },
    memory::{sparse::SparseMemory, wxorx::WXorXMemory},
    snapshot::{make_snapshot, resume, Snapshot},
    DefaultMachineBuilder, Error, Instruction, ISA_IMC,
};
use std::fs::File;
use std::io::Read;

#[test]
fn test_resume_interpreter_with_trace_2_asm() {
    resume_interpreter_with_trace_2_asm_inner(VERSION1, 8126917);
    resume_interpreter_with_trace_2_asm_inner(VERSION0, 8126917);
}

#[test]
#[cfg(has_aot)]
fn test_resume_aot_2_asm() {
    resume_aot_2_asm(VERSION1, 8126917);
    resume_aot_2_asm(VERSION0, 8126917);
}

#[test]
#[cfg(has_aot)]
fn test_resume_asm_2_aot() {
    resume_asm_2_aot(VERSION1, 8126917);
    resume_asm_2_aot(VERSION0, 8126917);
}

#[test]
fn test_resume_interpreter_2_asm() {
    resume_interpreter_2_asm(VERSION1, 8126917);
    resume_interpreter_2_asm(VERSION0, 8126917);
}

#[test]
fn test_resume_interpreter_2_interpreter() {
    resume_interpreter_2_interpreter(VERSION1, 8126917);
    resume_interpreter_2_interpreter(VERSION0, 8126917);
}

#[test]
fn test_resume_asm_2_interpreter() {
    resume_asm_2_interpreter(VERSION1, 8126917);
    resume_asm_2_interpreter(VERSION0, 8126917);
}

#[test]
fn test_resume_asm_2_asm_2_asm() {
    resume_asm_2_asm_2_asm(VERSION1, 8126917);
    resume_asm_2_asm_2_asm(VERSION0, 8126917);
}

#[test]
fn test_resume_asm_2_asm() {
    resume_asm_2_asm(VERSION1, 8126917);
    resume_asm_2_asm(VERSION0, 8126917);
}

fn dummy_cycle_func(_i: Instruction, _: u64, _: u64, _: bool) -> u64 {
    1
}

pub fn resume_asm_2_asm(version: u32, except_cycles: u64) {
    let buffer = load_program();

    // The cycles required for complete execution is 4194622
    let mut machine1 = MachineTy::Asm.build(version, except_cycles - 30);
    machine1
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    let result1 = machine1.run();
    let cycles1 = machine1.cycles();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::CyclesExceeded);
    let snapshot = machine1.snapshot().unwrap();

    let mut machine2 = MachineTy::Asm.build(version, 40);
    machine2.resume(&snapshot).unwrap();
    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles1 + cycles2, except_cycles);
}

pub fn resume_asm_2_asm_2_asm(version: u32, except_cycles: u64) {
    let buffer = load_program();

    let mut machine1 = MachineTy::Asm.build(version, 1000000);
    machine1
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    let result1 = machine1.run();
    let cycles1 = machine1.cycles();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::CyclesExceeded);
    let snapshot1 = machine1.snapshot().unwrap();

    let mut machine2 = MachineTy::Asm.build(version, 4000000);
    machine2.resume(&snapshot1).unwrap();
    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert!(result2.is_err());
    assert_eq!(result2.unwrap_err(), Error::CyclesExceeded);
    let snapshot2 = machine2.snapshot().unwrap();

    let mut machine3 = MachineTy::Asm.build(version, 4000000);
    machine3.resume(&snapshot2).unwrap();
    let result3 = machine3.run();
    let cycles3 = machine3.cycles();
    assert!(result3.is_ok());
    assert_eq!(result3.unwrap(), 0);
    assert_eq!(cycles1 + cycles2 + cycles3, except_cycles);
}

pub fn resume_asm_2_interpreter(version: u32, except_cycles: u64) {
    let buffer = load_program();

    let mut machine1 = MachineTy::Asm.build(version, except_cycles - 30);
    machine1
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    let result1 = machine1.run();
    let cycles1 = machine1.cycles();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::CyclesExceeded);
    let snapshot = machine1.snapshot().unwrap();

    let mut machine2 = MachineTy::Interpreter.build(version, 40);
    machine2.resume(&snapshot).unwrap();

    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles1 + cycles2, except_cycles);
}

pub fn resume_interpreter_2_interpreter(version: u32, except_cycles: u64) {
    let buffer = load_program();

    let mut machine1 = MachineTy::Interpreter.build(version, except_cycles - 30);
    machine1
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    let result1 = machine1.run();
    let cycles1 = machine1.cycles();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::CyclesExceeded);
    let snapshot = machine1.snapshot().unwrap();

    let mut machine2 = MachineTy::Interpreter.build(version, 30);
    machine2.resume(&snapshot).unwrap();
    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles1 + cycles2, except_cycles);
}

pub fn resume_interpreter_2_asm(version: u32, except_cycles: u64) {
    let buffer = load_program();

    let mut machine1 = MachineTy::Interpreter.build(version, except_cycles - 30);
    machine1
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    let result1 = machine1.run();
    let cycles1 = machine1.cycles();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::CyclesExceeded);
    let snapshot = machine1.snapshot().unwrap();

    let mut machine2 = MachineTy::Asm.build(version, 30);
    machine2.resume(&snapshot).unwrap();
    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles1 + cycles2, except_cycles);
}

#[cfg(has_aot)]
pub fn resume_aot_2_asm(version: u32, except_cycles: u64) {
    let buffer = load_program();

    let mut aot_machine =
        AotCompilingMachine::load(&buffer, Some(Box::new(dummy_cycle_func)), ISA_IMC, VERSION1)
            .unwrap();
    let code = aot_machine.compile().unwrap();
    let mut machine1 = AotMachine::build(version, except_cycles - 30, Some(&code));
    machine1
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    let result1 = machine1.run();
    let cycles1 = machine1.cycles();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::CyclesExceeded);
    let snapshot = machine1.snapshot().unwrap();

    let mut machine2 = MachineTy::Asm.build(version, 40);
    machine2.resume(&snapshot).unwrap();
    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles1 + cycles2, except_cycles);
}

#[cfg(has_aot)]
pub fn resume_asm_2_aot(version: u32, except_cycles: u64) {
    let buffer = load_program();

    let mut machine1 = MachineTy::Asm.build(version, except_cycles - 30);
    machine1
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    let result1 = machine1.run();
    let cycles1 = machine1.cycles();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::CyclesExceeded);
    let snapshot = machine1.snapshot().unwrap();

    let mut aot_machine =
        AotCompilingMachine::load(&buffer, Some(Box::new(dummy_cycle_func)), ISA_IMC, VERSION1)
            .unwrap();
    let code = aot_machine.compile().unwrap();
    let mut machine2 = AotMachine::build(version, 40, Some(&code));
    machine2.resume(&snapshot).unwrap();
    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles1 + cycles2, except_cycles);
}

pub fn resume_interpreter_with_trace_2_asm_inner(version: u32, except_cycles: u64) {
    let buffer = load_program();

    let mut machine1 = MachineTy::InterpreterWithTrace.build(version, except_cycles - 30);
    machine1
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    let result1 = machine1.run();
    let cycles1 = machine1.cycles();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::CyclesExceeded);
    let snapshot = machine1.snapshot().unwrap();

    let mut machine2 = MachineTy::Asm.build(version, 30);
    machine2.resume(&snapshot).unwrap();
    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles1 + cycles2, except_cycles);
}

fn load_program() -> Bytes {
    let mut file = File::open("tests/programs/alloc_many").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer.into()
}

enum MachineTy {
    Asm,
    Interpreter,
    InterpreterWithTrace,
}

impl MachineTy {
    fn build<'a>(self, version: u32, max_cycles: u64) -> Machine {
        match self {
            MachineTy::Asm => {
                let asm_core1 = AsmCoreMachine::new(ISA_IMC, version, max_cycles);
                let core1 = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core1)
                    .instruction_cycle_func(Box::new(dummy_cycle_func))
                    .build();
                Machine::Asm(AsmMachine::new(core1, None))
            }
            MachineTy::Interpreter => {
                let core_machine1 = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
                    ISA_IMC, version, max_cycles,
                );
                Machine::Interpreter(
                    DefaultMachineBuilder::<DefaultCoreMachine<u64, WXorXMemory<SparseMemory<u64>>>>::new(
                        core_machine1,
                    )
                    .instruction_cycle_func(Box::new(dummy_cycle_func))
                    .build(),
                )
            }
            MachineTy::InterpreterWithTrace => {
                let core_machine1 = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
                    ISA_IMC, version, max_cycles,
                );
                Machine::InterpreterWithTrace(
                    TraceMachine::new(
                        DefaultMachineBuilder::<
                            DefaultCoreMachine<u64, WXorXMemory<SparseMemory<u64>>>,
                        >::new(core_machine1)
                        .instruction_cycle_func(Box::new(dummy_cycle_func))
                        .build(),
                    ),
                )
            }
        }
    }
}

enum Machine {
    Asm(AsmMachine<'static>),
    Interpreter(DefaultMachine<'static, DefaultCoreMachine<u64, WXorXMemory<SparseMemory<u64>>>>),
    InterpreterWithTrace(
        TraceMachine<'static, DefaultCoreMachine<u64, WXorXMemory<SparseMemory<u64>>>>,
    ),
}

impl Machine {
    fn load_program(&mut self, program: &Bytes, args: &[Bytes]) -> Result<u64, Error> {
        use Machine::*;
        match self {
            Asm(inner) => inner.load_program(program, args),
            // Aot(inner) => inner.load_program(program, args),
            Interpreter(inner) => inner.load_program(program, args),
            InterpreterWithTrace(inner) => inner.load_program(program, args),
        }
    }

    fn run(&mut self) -> Result<i8, Error> {
        use Machine::*;
        match self {
            Asm(inner) => inner.run(),
            // Aot(inner) => inner.run(),
            Interpreter(inner) => inner.run(),
            InterpreterWithTrace(inner) => inner.run(),
        }
    }

    fn cycles(&self) -> u64 {
        use Machine::*;
        match self {
            Asm(inner) => inner.machine.cycles(),
            // Aot(inner) => inner.machine.cycles(),
            Interpreter(inner) => inner.cycles(),
            InterpreterWithTrace(inner) => inner.machine.cycles(),
        }
    }

    fn snapshot(&mut self) -> Result<Snapshot, Error> {
        use Machine::*;
        match self {
            Asm(inner) => make_snapshot(&mut inner.machine),
            // Aot(inner) => make_snapshot(&mut inner.machine),
            Interpreter(inner) => make_snapshot(inner),
            InterpreterWithTrace(inner) => make_snapshot(&mut inner.machine),
        }
    }

    fn resume(&mut self, snap: &Snapshot) -> Result<(), Error> {
        use Machine::*;
        match self {
            Asm(inner) => resume(&mut inner.machine, snap),
            // Aot(inner) => resume(&mut inner.machine, snap),
            Interpreter(inner) => resume(inner, snap),
            InterpreterWithTrace(inner) => resume(&mut inner.machine, snap),
        }
    }
}

#[cfg(has_aot)]
struct AotMachine<'a>(AsmMachine<'a>);

#[cfg(has_aot)]
impl<'a> AotMachine<'a> {
    fn build(version: u32, max_cycles: u64, program: Option<&'a AotCode>) -> AotMachine<'a> {
        let asm_core1 = AsmCoreMachine::new(ISA_IMC, version, max_cycles);
        let core1 = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core1)
            .instruction_cycle_func(Box::new(dummy_cycle_func))
            .build();
        AotMachine(AsmMachine::new(core1, program))
    }

    fn load_program(&mut self, program: &Bytes, args: &[Bytes]) -> Result<u64, Error> {
        self.0.load_program(program, args)
    }

    fn run(&mut self) -> Result<i8, Error> {
        self.0.run()
    }

    fn cycles(&self) -> u64 {
        self.0.machine.cycles()
    }

    fn snapshot(&mut self) -> Result<Snapshot, Error> {
        make_snapshot(&mut self.0.machine)
    }

    fn resume(&mut self, snap: &Snapshot) -> Result<(), Error> {
        resume(&mut self.0.machine, snap)
    }
}
