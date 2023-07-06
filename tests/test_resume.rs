#![cfg(has_asm)]
pub mod machine_build;
use bytes::Bytes;
use ckb_vm::cost_model::constant_cycles;
use ckb_vm::machine::asm::{AsmCoreMachine, AsmMachine};
use ckb_vm::machine::trace::TraceMachine;
use ckb_vm::machine::{DefaultCoreMachine, DefaultMachine, SupportMachine, VERSION0, VERSION1};
use ckb_vm::memory::{sparse::SparseMemory, wxorx::WXorXMemory};
use ckb_vm::snapshot::{make_snapshot, resume, Snapshot};
use ckb_vm::{DefaultMachineBuilder, Error, ExecutionContext, ISA_IMC};
use std::fs::File;
use std::io::Read;

#[test]
fn test_resume_interpreter_with_trace_2_asm() {
    resume_interpreter_with_trace_2_asm_inner(VERSION1, 8126917);
    resume_interpreter_with_trace_2_asm_inner(VERSION0, 8126917);
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
    fn build(self, version: u32, max_cycles: u64) -> Machine {
        match self {
            MachineTy::Asm => {
                let asm_core1 = AsmCoreMachine::new(ISA_IMC, version, max_cycles);
                let core1 = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core1)
                    .context(ConstantCyclesCtx)
                    .build();
                Machine::Asm(AsmMachine::new(core1))
            }
            MachineTy::Interpreter => {
                let core_machine1 = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
                    ISA_IMC, version, max_cycles,
                );
                Machine::Interpreter(
                    DefaultMachineBuilder::<DefaultCoreMachine<u64, WXorXMemory<SparseMemory<u64>>>>::new(
                        core_machine1,
                    )
                    .context(ConstantCyclesCtx)
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
                        .context(ConstantCyclesCtx)
                        .build(),
                    ),
                )
            }
        }
    }
}

struct ConstantCyclesCtx;

impl<Mac: SupportMachine> ExecutionContext<Mac> for ConstantCyclesCtx {
    fn instruction_cycles(&self, inst: ckb_vm::Instruction) -> u64 {
        constant_cycles(inst)
    }
}

enum Machine {
    Asm(AsmMachine<ConstantCyclesCtx>),
    Interpreter(
        DefaultMachine<DefaultCoreMachine<u64, WXorXMemory<SparseMemory<u64>>>, ConstantCyclesCtx>,
    ),
    InterpreterWithTrace(
        TraceMachine<DefaultCoreMachine<u64, WXorXMemory<SparseMemory<u64>>>, ConstantCyclesCtx>,
    ),
}

impl Machine {
    fn load_program(&mut self, program: &Bytes, args: &[Bytes]) -> Result<u64, Error> {
        use Machine::*;
        match self {
            Asm(inner) => inner.load_program(program, args),
            Interpreter(inner) => inner.load_program(program, args),
            InterpreterWithTrace(inner) => inner.load_program(program, args),
        }
    }

    fn run(&mut self) -> Result<i8, Error> {
        use Machine::*;
        match self {
            Asm(inner) => inner.run(),
            Interpreter(inner) => inner.run(),
            InterpreterWithTrace(inner) => inner.run(),
        }
    }

    fn cycles(&self) -> u64 {
        use Machine::*;
        match self {
            Asm(inner) => inner.machine.cycles(),
            Interpreter(inner) => inner.cycles(),
            InterpreterWithTrace(inner) => inner.machine.cycles(),
        }
    }

    fn snapshot(&mut self) -> Result<Snapshot, Error> {
        use Machine::*;
        match self {
            Asm(inner) => make_snapshot(&mut inner.machine),
            Interpreter(inner) => make_snapshot(inner),
            InterpreterWithTrace(inner) => make_snapshot(&mut inner.machine),
        }
    }

    fn resume(&mut self, snap: &Snapshot) -> Result<(), Error> {
        use Machine::*;
        match self {
            Asm(inner) => resume(&mut inner.machine, snap),
            Interpreter(inner) => resume(inner, snap),
            InterpreterWithTrace(inner) => resume(&mut inner.machine, snap),
        }
    }
}
