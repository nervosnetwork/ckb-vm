#![cfg(has_asm)]
pub mod machine_build;
use bytes::Bytes;
use ckb_vm::cost_model::constant_cycles;
use ckb_vm::elf::parse_elf;
use ckb_vm::machine::asm::{AsmCoreMachine, AsmMachine};
use ckb_vm::machine::trace::TraceMachine;
use ckb_vm::machine::{
    CoreMachine, DefaultCoreMachine, DefaultMachine, SupportMachine, VERSION0, VERSION1,
};
use ckb_vm::memory::{sparse::SparseMemory, wxorx::WXorXMemory};
use ckb_vm::registers::{A0, A1, A7};
use ckb_vm::snapshot2::{DataSource, Snapshot2, Snapshot2Context};
use ckb_vm::{DefaultMachineBuilder, Error, ExecutionContext, Register, ISA_IMC};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};

#[test]
fn test_resume2_interpreter_with_trace_2_asm() {
    resume_interpreter_with_trace_2_asm_inner(VERSION1, 8126917);
    resume_interpreter_with_trace_2_asm_inner(VERSION0, 8126917);
}

#[test]
fn test_resume2_interpreter_2_asm() {
    resume_interpreter_2_asm(VERSION1, 8126917);
    resume_interpreter_2_asm(VERSION0, 8126917);
}

#[test]
fn test_resume2_interpreter_2_interpreter() {
    resume_interpreter_2_interpreter(VERSION1, 8126917);
    resume_interpreter_2_interpreter(VERSION0, 8126917);
}

#[test]
fn test_resume2_asm_2_interpreter() {
    resume_asm_2_interpreter(VERSION1, 8126917);
    resume_asm_2_interpreter(VERSION0, 8126917);
}

#[test]
fn test_resume2_asm_2_asm_2_asm() {
    resume_asm_2_asm_2_asm(VERSION1, 8126917);
    resume_asm_2_asm_2_asm(VERSION0, 8126917);
}

#[test]
fn test_resume2_asm_2_asm() {
    resume_asm_2_asm(VERSION1, 8126917);
    resume_asm_2_asm(VERSION0, 8126917);
}

#[test]
fn test_resume2_secp256k1_asm_2_interpreter_2_asm() {
    let data_source = load_program("benches/data/secp256k1_bench");

    let version = VERSION1;
    let except_cycles = 613073;

    let mut machine1 = MachineTy::Asm.build(data_source.clone(), version);
    machine1.set_max_cycles(100000);
    machine1
        .load_program(&vec![
            "secp256k1_bench".into(),
            "033f8cf9c4d51a33206a6c1c6b27d2cc5129daa19dbd1fc148d395284f6b26411f".into(),
            "304402203679d909f43f073c7c1dcf8468a485090589079ee834e6eed92fea9b09b06a2402201e46f1075afa18f306715e7db87493e7b7e779569aa13c64ab3d09980b3560a3".into(),
            "foo".into(),
            "bar".into(),
        ])
        .unwrap();
    let result1 = machine1.run();
    assert_eq!(result1.unwrap_err(), Error::CyclesExceeded);
    let snapshot1 = machine1.snapshot().unwrap();
    assert!(!snapshot1.pages_from_source.is_empty());

    let mut machine2 = MachineTy::Interpreter.build(data_source.clone(), version);
    machine2.resume(snapshot1).unwrap();

    assert_eq!(machine1.cycles(), machine2.cycles());
    assert_eq!(machine1.full_registers(), machine2.full_registers());
    #[cfg(not(feature = "enable-chaos-mode-by-default"))]
    assert_eq!(machine1.full_memory(), machine2.full_memory());

    machine2.set_max_cycles(100000 + 200000);
    let result2 = machine2.run();
    assert_eq!(result2.unwrap_err(), Error::CyclesExceeded);
    let snapshot2 = machine2.snapshot().unwrap();
    assert!(!snapshot2.pages_from_source.is_empty());

    let mut machine3 = MachineTy::Asm.build(data_source, version);
    machine3.resume(snapshot2).unwrap();

    assert_eq!(machine2.cycles(), machine3.cycles());
    assert_eq!(machine2.full_registers(), machine3.full_registers());
    #[cfg(not(feature = "enable-chaos-mode-by-default"))]
    assert_eq!(machine2.full_memory(), machine3.full_memory());

    machine3.set_max_cycles(100000 + 200000 + 400000);
    let result3 = machine3.run();
    let cycles3 = machine3.cycles();
    assert_eq!(result3.unwrap(), 0);
    assert_eq!(cycles3, except_cycles);
}

#[test]
fn test_resume2_load_data_asm_2_interpreter() {
    let data_source = load_program("tests/programs/resume2_load_data");

    let version = VERSION1;
    let except_cycles = 1476715;

    let mut machine1 = MachineTy::Asm.build(data_source.clone(), version);
    machine1.set_max_cycles(300000);
    machine1
        .load_program(&vec!["resume2_load_data".into()])
        .unwrap();
    let result1 = machine1.run();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::CyclesExceeded);
    let snapshot = machine1.snapshot().unwrap();
    assert!(!snapshot.pages_from_source.is_empty());

    let mut machine2 = MachineTy::Interpreter.build(data_source, version);
    machine2.resume(snapshot).unwrap();

    assert_eq!(machine1.cycles(), machine2.cycles());
    assert_eq!(machine1.full_registers(), machine2.full_registers());
    #[cfg(not(feature = "enable-chaos-mode-by-default"))]
    assert_eq!(machine1.full_memory(), machine2.full_memory());

    machine2.set_max_cycles(except_cycles + 10);

    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles2, except_cycles);
}

#[test]
fn test_resume2_load_data_interpreter_2_asm() {
    let data_source = load_program("tests/programs/resume2_load_data");

    let version = VERSION1;
    let except_cycles = 1476715;

    let mut machine1 = MachineTy::Interpreter.build(data_source.clone(), version);
    machine1.set_max_cycles(300000);
    machine1
        .load_program(&vec!["resume2_load_data".into()])
        .unwrap();
    let result1 = machine1.run();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::CyclesExceeded);
    let snapshot = machine1.snapshot().unwrap();
    assert!(!snapshot.pages_from_source.is_empty());

    let mut machine2 = MachineTy::Asm.build(data_source, version);
    machine2.resume(snapshot).unwrap();

    assert_eq!(machine1.cycles(), machine2.cycles());
    assert_eq!(machine1.full_registers(), machine2.full_registers());
    #[cfg(not(feature = "enable-chaos-mode-by-default"))]
    assert_eq!(machine1.full_memory(), machine2.full_memory());

    machine2.set_max_cycles(except_cycles + 10);

    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles2, except_cycles);
}

pub fn resume_asm_2_asm(version: u32, except_cycles: u64) {
    let data_source = load_program("tests/programs/alloc_many");

    // The cycles required for complete execution is 4194622
    let mut machine1 = MachineTy::Asm.build(data_source.clone(), version);
    machine1.set_max_cycles(except_cycles - 30);
    machine1.load_program(&vec!["alloc_many".into()]).unwrap();
    let result1 = machine1.run();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::CyclesExceeded);
    let snapshot = machine1.snapshot().unwrap();

    let mut machine2 = MachineTy::Asm.build(data_source, version);
    machine2.resume(snapshot).unwrap();
    machine2.set_max_cycles(except_cycles + 10);
    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles2, except_cycles);
}

pub fn resume_asm_2_asm_2_asm(version: u32, except_cycles: u64) {
    let data_source = load_program("tests/programs/alloc_many");

    let mut machine1 = MachineTy::Asm.build(data_source.clone(), version);
    machine1.set_max_cycles(1000000);
    machine1.load_program(&vec!["alloc_many".into()]).unwrap();
    let result1 = machine1.run();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::CyclesExceeded);
    let snapshot1 = machine1.snapshot().unwrap();

    let mut machine2 = MachineTy::Asm.build(data_source.clone(), version);
    machine2.resume(snapshot1).unwrap();
    machine2.set_max_cycles(1000000 + 4000000);
    let result2 = machine2.run();
    assert!(result2.is_err());
    assert_eq!(result2.unwrap_err(), Error::CyclesExceeded);
    let snapshot2 = machine2.snapshot().unwrap();

    let mut machine3 = MachineTy::Asm.build(data_source, version);
    machine3.resume(snapshot2).unwrap();
    machine3.set_max_cycles(1000000 + 4000000 + 4000000);
    let result3 = machine3.run();
    let cycles3 = machine3.cycles();
    assert_eq!(result3.unwrap(), 0);
    assert_eq!(cycles3, except_cycles);
}

pub fn resume_asm_2_interpreter(version: u32, except_cycles: u64) {
    let data_source = load_program("tests/programs/alloc_many");

    let mut machine1 = MachineTy::Asm.build(data_source.clone(), version);
    machine1.set_max_cycles(except_cycles - 30);
    machine1.load_program(&vec!["alloc_many".into()]).unwrap();
    let result1 = machine1.run();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::CyclesExceeded);
    let snapshot = machine1.snapshot().unwrap();

    let mut machine2 = MachineTy::Interpreter.build(data_source, version);
    machine2.resume(snapshot).unwrap();
    machine2.set_max_cycles(except_cycles + 10);

    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles2, except_cycles);
}

pub fn resume_interpreter_2_interpreter(version: u32, except_cycles: u64) {
    let data_source = load_program("tests/programs/alloc_many");

    let mut machine1 = MachineTy::Interpreter.build(data_source.clone(), version);
    machine1.set_max_cycles(except_cycles - 30);
    machine1.load_program(&vec!["alloc_many".into()]).unwrap();
    let result1 = machine1.run();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::CyclesExceeded);
    let snapshot = machine1.snapshot().unwrap();

    let mut machine2 = MachineTy::Interpreter.build(data_source, version);
    machine2.resume(snapshot).unwrap();
    machine2.set_max_cycles(except_cycles + 10);
    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles2, except_cycles);
}

pub fn resume_interpreter_2_asm(version: u32, except_cycles: u64) {
    let data_source = load_program("tests/programs/alloc_many");

    let mut machine1 = MachineTy::Interpreter.build(data_source.clone(), version);
    machine1.set_max_cycles(except_cycles - 30);
    machine1.load_program(&vec!["alloc_many".into()]).unwrap();
    let result1 = machine1.run();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::CyclesExceeded);
    let snapshot = machine1.snapshot().unwrap();

    let mut machine2 = MachineTy::Asm.build(data_source, version);
    machine2.resume(snapshot).unwrap();
    machine2.set_max_cycles(except_cycles);
    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles2, except_cycles);
}

pub fn resume_interpreter_with_trace_2_asm_inner(version: u32, except_cycles: u64) {
    let data_source = load_program("tests/programs/alloc_many");

    let mut machine1 = MachineTy::InterpreterWithTrace.build(data_source.clone(), version);
    machine1.set_max_cycles(except_cycles - 30);
    machine1.load_program(&vec!["alloc_many".into()]).unwrap();
    let result1 = machine1.run();
    assert!(result1.is_err());
    assert_eq!(result1.unwrap_err(), Error::CyclesExceeded);
    let snapshot = machine1.snapshot().unwrap();

    let mut machine2 = MachineTy::Asm.build(data_source, version);
    machine2.resume(snapshot).unwrap();
    machine2.set_max_cycles(except_cycles);
    let result2 = machine2.run();
    let cycles2 = machine2.cycles();
    assert_eq!(result2.unwrap(), 0);
    assert_eq!(cycles2, except_cycles);
}

fn load_program(name: &str) -> TestSource {
    let mut file = File::open(name).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let program = buffer.into();

    let data = vec![7; 16 * 4096];
    let mut m = HashMap::default();
    m.insert(DATA_ID, data.into());
    m.insert(PROGRAM_ID, program);

    TestSource(m)
}

const PROGRAM_ID: u64 = 0x1234;
const DATA_ID: u64 = 0x2000;

#[derive(Clone)]
struct TestSource(HashMap<u64, Bytes>);

impl DataSource<u64> for TestSource {
    fn load_data(&self, id: &u64, offset: u64, length: u64) -> Result<Bytes, Error> {
        match self.0.get(id) {
            Some(data) => Ok(data.slice(
                offset as usize..(if length > 0 {
                    (offset + length) as usize
                } else {
                    data.len()
                }),
            )),
            None => Err(Error::Unexpected(format!(
                "Id {} is missing in source!",
                id
            ))),
        }
    }
}

struct InsertDataSyscall(Arc<Mutex<Snapshot2Context<u64, TestSource>>>);

impl<Mac: SupportMachine> ExecutionContext<Mac> for InsertDataSyscall {
    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, Error> {
        let code = &machine.registers()[A7];
        if code.to_i32() != 1111 {
            return Ok(false);
        }
        let addr = machine.registers()[A0].to_u64();
        let size = machine.registers()[A1].to_u64();

        self.0
            .lock()
            .unwrap()
            .store_bytes(machine, addr, &DATA_ID, 0, size)?;

        machine.add_cycles_no_checking(100000)?;

        machine.set_register(A0, Mac::REG::from_u64(0));
        Ok(true)
    }

    fn instruction_cycles(&self, inst: ckb_vm::Instruction) -> u64 {
        constant_cycles(inst)
    }
}

enum MachineTy {
    Asm,
    Interpreter,
    InterpreterWithTrace,
}

impl MachineTy {
    fn build(self, data_source: TestSource, version: u32) -> Machine {
        match self {
            MachineTy::Asm => {
                let context = Arc::new(Mutex::new(Snapshot2Context::new(data_source)));
                let asm_core1 = AsmCoreMachine::new(ISA_IMC, version, 0);
                let core1 = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core1)
                    .context(InsertDataSyscall(context.clone()))
                    .build();
                Machine::Asm(AsmMachine::new(core1), context)
            }
            MachineTy::Interpreter => {
                let context = Arc::new(Mutex::new(Snapshot2Context::new(data_source)));
                let core_machine1 = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
                    ISA_IMC, version, 0,
                );
                Machine::Interpreter(
                    DefaultMachineBuilder::<DefaultCoreMachine<u64, WXorXMemory<SparseMemory<u64>>>>::new(
                        core_machine1,
                    )
                    .context(InsertDataSyscall(context.clone()))
                    .build(),
                    context,
                )
            }
            MachineTy::InterpreterWithTrace => {
                let context = Arc::new(Mutex::new(Snapshot2Context::new(data_source)));
                let core_machine1 = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
                    ISA_IMC, version, 0,
                );
                Machine::InterpreterWithTrace(
                    TraceMachine::new(
                        DefaultMachineBuilder::<
                            DefaultCoreMachine<u64, WXorXMemory<SparseMemory<u64>>>,
                        >::new(core_machine1)
                        .context(InsertDataSyscall(context.clone()))
                        .build(),
                    ),
                    context,
                )
            }
        }
    }
}

enum Machine {
    Asm(
        AsmMachine<InsertDataSyscall>,
        Arc<Mutex<Snapshot2Context<u64, TestSource>>>,
    ),
    Interpreter(
        DefaultMachine<DefaultCoreMachine<u64, WXorXMemory<SparseMemory<u64>>>, InsertDataSyscall>,
        Arc<Mutex<Snapshot2Context<u64, TestSource>>>,
    ),
    InterpreterWithTrace(
        TraceMachine<DefaultCoreMachine<u64, WXorXMemory<SparseMemory<u64>>>, InsertDataSyscall>,
        Arc<Mutex<Snapshot2Context<u64, TestSource>>>,
    ),
}

impl Machine {
    fn load_program(&mut self, args: &[Bytes]) -> Result<u64, Error> {
        use Machine::*;
        match self {
            Asm(inner, context) => {
                let program = context
                    .lock()
                    .unwrap()
                    .data_source()
                    .load_data(&PROGRAM_ID, 0, 0)
                    .unwrap();
                let metadata = parse_elf::<u64>(&program, inner.machine.version())?;
                let bytes = inner.load_program_with_metadata(&program, &metadata, args)?;
                context.lock().unwrap().mark_program(
                    inner.machine.inner_mut(),
                    &metadata,
                    &PROGRAM_ID,
                    0,
                )?;
                Ok(bytes)
            }
            Interpreter(inner, context) => {
                let program = context
                    .lock()
                    .unwrap()
                    .data_source()
                    .load_data(&PROGRAM_ID, 0, 0)
                    .unwrap();
                let metadata = parse_elf::<u64>(&program, inner.version())?;
                let bytes = inner.load_program_with_metadata(&program, &metadata, args)?;
                context.lock().unwrap().mark_program(
                    inner.inner_mut(),
                    &metadata,
                    &PROGRAM_ID,
                    0,
                )?;
                Ok(bytes)
            }
            InterpreterWithTrace(inner, context) => {
                let program = context
                    .lock()
                    .unwrap()
                    .data_source()
                    .load_data(&PROGRAM_ID, 0, 0)
                    .unwrap();
                let metadata = parse_elf::<u64>(&program, inner.machine.version())?;
                let bytes = inner.load_program_with_metadata(&program, &metadata, args)?;
                context.lock().unwrap().mark_program(
                    inner.machine.inner_mut(),
                    &metadata,
                    &PROGRAM_ID,
                    0,
                )?;
                Ok(bytes)
            }
        }
    }

    fn run(&mut self) -> Result<i8, Error> {
        use Machine::*;
        match self {
            Asm(inner, _) => inner.run(),
            Interpreter(inner, _) => inner.run(),
            InterpreterWithTrace(inner, _) => inner.run(),
        }
    }

    fn set_max_cycles(&mut self, cycles: u64) {
        use Machine::*;
        match self {
            Asm(inner, _) => inner.machine.set_max_cycles(cycles),
            Interpreter(inner, _) => inner.set_max_cycles(cycles),
            InterpreterWithTrace(inner, _) => inner.machine.set_max_cycles(cycles),
        }
    }

    fn cycles(&self) -> u64 {
        use Machine::*;
        match self {
            Asm(inner, _) => inner.machine.cycles(),
            Interpreter(inner, _) => inner.cycles(),
            InterpreterWithTrace(inner, _) => inner.machine.cycles(),
        }
    }

    #[cfg(not(feature = "enable-chaos-mode-by-default"))]
    fn full_memory(&mut self) -> Result<Bytes, Error> {
        use ckb_vm::{Memory, RISCV_MAX_MEMORY};
        use Machine::*;
        match self {
            Asm(inner, _) => inner
                .machine
                .memory_mut()
                .load_bytes(0, RISCV_MAX_MEMORY as u64),
            Interpreter(inner, _) => inner.memory_mut().load_bytes(0, RISCV_MAX_MEMORY as u64),
            InterpreterWithTrace(inner, _) => inner
                .machine
                .memory_mut()
                .load_bytes(0, RISCV_MAX_MEMORY as u64),
        }
    }

    fn full_registers(&self) -> [u64; 33] {
        use Machine::*;
        let mut regs = [0u64; 33];
        match self {
            Asm(inner, _) => {
                regs[0..32].copy_from_slice(inner.machine.registers());
                regs[32] = *inner.machine.pc();
            }
            Interpreter(inner, _) => {
                regs[0..32].copy_from_slice(inner.registers());
                regs[32] = *inner.pc();
            }
            InterpreterWithTrace(inner, _) => {
                regs[0..32].copy_from_slice(inner.machine.registers());
                regs[32] = *inner.machine.pc();
            }
        };
        regs
    }

    fn snapshot(&mut self) -> Result<Snapshot2<u64>, Error> {
        use Machine::*;
        match self {
            Asm(inner, context) => {
                let context = context.lock().unwrap();
                Ok(context.make_snapshot(inner.machine.inner_mut())?)
            }
            Interpreter(inner, context) => {
                let context = context.lock().unwrap();
                Ok(context.make_snapshot(inner.inner_mut())?)
            }
            InterpreterWithTrace(inner, context) => {
                let context = context.lock().unwrap();
                Ok(context.make_snapshot(inner.machine.inner_mut())?)
            }
        }
    }

    fn resume(&mut self, snap: Snapshot2<u64>) -> Result<(), Error> {
        use Machine::*;
        match self {
            Asm(inner, context) => {
                context
                    .lock()
                    .unwrap()
                    .resume(inner.machine.inner_mut(), &snap)?;
            }
            Interpreter(inner, context) => {
                context.lock().unwrap().resume(inner.inner_mut(), &snap)?;
            }
            InterpreterWithTrace(inner, context) => {
                context
                    .lock()
                    .unwrap()
                    .resume(inner.machine.inner_mut(), &snap)?;
            }
        };
        Ok(())
    }
}
