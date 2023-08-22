#![cfg(has_asm)]
use bytes::Bytes;
use ckb_vm::cost_model::constant_cycles;
use ckb_vm::decoder::build_decoder;
use ckb_vm::machine::asm::traces::{MemoizedDynamicTraceDecoder, MemoizedFixedTraceDecoder};
use ckb_vm::machine::asm::{AsmCoreMachine, AsmMachine};
use ckb_vm::machine::{CoreMachine, VERSION0, VERSION1, VERSION2};
use ckb_vm::memory::Memory;
use ckb_vm::registers::{A0, A1, A2, A3, A4, A5, A7};
use ckb_vm::{Debugger, DefaultMachineBuilder, Error, Register, SupportMachine, Syscalls, ISA_IMC};
use std::fs;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::thread;
pub mod machine_build;

#[test]
pub fn test_asm_simple64() {
    let buffer = fs::read("tests/programs/simple64").unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec!["simple".into()])
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
    let buffer = fs::read("tests/programs/syscall64").unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core)
        .syscall(Box::new(CustomSyscall {}))
        .build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec!["syscall".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 39);
}

pub struct CustomDebugger {
    pub value: Arc<AtomicU8>,
}

impl<Mac: SupportMachine> Debugger<Mac> for CustomDebugger {
    fn initialize(&mut self, _machine: &mut Mac) -> Result<(), Error> {
        self.value.store(1, Ordering::Relaxed);
        Ok(())
    }

    fn ebreak(&mut self, _machine: &mut Mac) -> Result<(), Error> {
        self.value.store(2, Ordering::Relaxed);
        Ok(())
    }
}

#[test]
pub fn test_asm_ebreak() {
    let buffer = fs::read("tests/programs/ebreak64").unwrap().into();
    let value = Arc::new(AtomicU8::new(0));

    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core)
        .debugger(Box::new(CustomDebugger {
            value: Arc::clone(&value),
        }))
        .build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec!["ebreak".into()])
        .unwrap();
    assert_eq!(value.load(Ordering::Relaxed), 1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(value.load(Ordering::Relaxed), 2);
}

#[test]
pub fn test_asm_simple_cycles() {
    let buffer = fs::read("tests/programs/simple64").unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, 708);
    let core = DefaultMachineBuilder::new(asm_core)
        .instruction_cycle_func(Box::new(constant_cycles))
        .build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec!["syscall".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    assert_eq!(SupportMachine::cycles(&machine.machine), 708);
}

#[test]
pub fn test_asm_simple_max_cycles_reached() {
    let buffer = fs::read("tests/programs/simple64").unwrap().into();
    // Running simple64 should consume 708 cycles using dummy cycle func
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, 700);
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
        .instruction_cycle_func(Box::new(constant_cycles))
        .build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec!["syscall".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::CyclesExceeded);
}

#[test]
pub fn test_asm_trace() {
    let buffer = fs::read("tests/programs/trace64").unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec!["simple".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::MemWriteOnExecutablePage));
}

#[test]
pub fn test_asm_jump0() {
    let buffer = fs::read("tests/programs/jump0_64").unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec!["jump0_64".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::MemWriteOnExecutablePage));
}

#[test]
pub fn test_asm_write_large_address() {
    let buffer = fs::read("tests/programs/write_large_address64")
        .unwrap()
        .into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec!["write_large_address64".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::MemOutOfBound));
}

#[test]
pub fn test_misaligned_jump64() {
    let buffer = fs::read("tests/programs/misaligned_jump64").unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec!["misaligned_jump64".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
}

#[test]
pub fn test_mulw64() {
    let buffer = fs::read("tests/programs/mulw64").unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec!["mulw64".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_invalid_read64() {
    let buffer = fs::read("tests/programs/invalid_read64").unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec!["invalid_read64".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::MemOutOfBound));
}

#[test]
pub fn test_asm_load_elf_crash_64() {
    let buffer = fs::read("tests/programs/load_elf_crash_64").unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec!["load_elf_crash_64".into()])
        .unwrap();
    let result = machine.run();
    assert_eq!(result.err(), Some(Error::MemWriteOnExecutablePage));
}

#[test]
pub fn test_asm_wxorx_crash_64() {
    let buffer = fs::read("tests/programs/wxorx_crash_64").unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec!["wxorx_crash_64".into()])
        .unwrap();
    let result = machine.run();
    assert_eq!(result.err(), Some(Error::MemOutOfBound));
}

#[test]
pub fn test_asm_alloc_many() {
    let buffer = fs::read("tests/programs/alloc_many").unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    let result = machine.run();
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_chaos_seed() {
    let buffer = fs::read("tests/programs/read_memory").unwrap().into();
    let mut asm_core1 = AsmCoreMachine::new(ISA_IMC, VERSION1, u64::max_value());
    asm_core1.chaos_mode = 1;
    asm_core1.chaos_seed = 100;
    let core1 = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core1).build();
    let mut machine1 = AsmMachine::new(core1);
    machine1
        .load_program(&buffer, &vec!["read_memory".into()])
        .unwrap();
    let result1 = machine1.run();
    let exit1 = result1.unwrap();

    let mut asm_core2 = AsmCoreMachine::new(ISA_IMC, VERSION1, u64::max_value());
    asm_core2.chaos_mode = 1;
    asm_core2.chaos_seed = 100;
    let core2 = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core2).build();
    let mut machine2 = AsmMachine::new(core2);
    machine2
        .load_program(&buffer, &vec!["read_memory".into()])
        .unwrap();
    let result2 = machine2.run();
    let exit2 = result2.unwrap();

    assert_eq!(exit1, exit2);
    // Read 8 bytes from 0x300000, it is very unlikely that they are both 0.
    assert!(machine1.machine.memory_mut().load64(&0x300000).unwrap() != 0);
    assert!(machine2.machine.memory_mut().load64(&0x300000).unwrap() != 0);
}

#[test]
pub fn test_asm_rvc_pageend() {
    let buffer = fs::read("tests/programs/rvc_pageend").unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec!["rvc_pageend".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

pub struct OutOfCyclesSyscall {}

impl<Mac: SupportMachine> Syscalls<Mac> for OutOfCyclesSyscall {
    fn initialize(&mut self, _machine: &mut Mac) -> Result<(), Error> {
        Ok(())
    }

    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, Error> {
        let code = &machine.registers()[A7];
        if code.to_i32() != 1111 {
            return Ok(false);
        }
        machine.add_cycles_no_checking(100)?;
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
pub fn test_asm_outofcycles_in_syscall() {
    let buffer = fs::read("tests/programs/syscall64").unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, 20);
    let core = DefaultMachineBuilder::new(asm_core)
        .instruction_cycle_func(Box::new(constant_cycles))
        .syscall(Box::new(OutOfCyclesSyscall {}))
        .build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec!["syscall".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::CyclesExceeded);
    assert_eq!(machine.machine.cycles(), 108);
    assert_eq!(machine.machine.registers()[A0], 39);
}

#[test]
pub fn test_asm_cycles_overflow() {
    let buffer = fs::read("tests/programs/simple64").unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::MAX);
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
        .instruction_cycle_func(Box::new(constant_cycles))
        .build();
    let mut machine = AsmMachine::new(core);
    machine.machine.set_cycles(u64::MAX - 10);
    machine
        .load_program(&buffer, &vec!["simple64".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::CyclesOverflow);
}

#[test]
pub fn test_decoder_instructions_cache_pc_out_of_bound_timeout() {
    let buffer = fs::read("tests/programs/decoder_instructions_cache_pc_out_of_bound_timeout")
        .unwrap()
        .into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::MAX);
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
        .instruction_cycle_func(Box::new(constant_cycles))
        .build();
    let mut machine = AsmMachine::new(core);
    machine.machine.set_cycles(u64::MAX - 10);
    machine
        .load_program(&buffer, &vec!["simple64".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::MemOutOfBound);
}

#[test]
fn test_asm_step() {
    let buffer = fs::read("tests/programs/simple64").unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec!["simple64".into()])
        .unwrap();

    let result = || -> Result<i8, Error> {
        let mut decoder = build_decoder::<u64>(ISA_IMC, VERSION0);
        machine.machine.set_running(true);
        while machine.machine.running() {
            machine.step(&mut decoder)?;
        }
        Ok(machine.machine.exit_code())
    }();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_asm_thread_safe() {
    let buffer = fs::read("tests/programs/mulw64").unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec!["mulw64".into()])
        .unwrap();
    let thread_join_handle = thread::spawn(move || {
        let result = machine.run();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    });
    thread_join_handle.join().unwrap();
}

#[test]
fn test_zero_address() {
    let buffer = fs::read("tests/programs/zero_address").unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION1, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    machine.load_program(&buffer, &vec!["zero".into()]).unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_memoized_secp256k1() {
    let isa = ISA_IMC;
    let version = VERSION1;
    let buffer = fs::read("benches/data/secp256k1_bench").unwrap().into();
    let asm_core = AsmCoreMachine::new(isa, version, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    let args: Vec<Bytes> = vec!["secp256k1_bench",
                                      "033f8cf9c4d51a33206a6c1c6b27d2cc5129daa19dbd1fc148d395284f6b26411f",
                                      "304402203679d909f43f073c7c1dcf8468a485090589079ee834e6eed92fea9b09b06a2402201e46f1075afa18f306715e7db87493e7b7e779569aa13c64ab3d09980b3560a3",
                                      "foo",
                                      "bar"].into_iter().map(|a| a.into()).collect();
    machine.load_program(&buffer, &args).unwrap();
    let mut decoder = MemoizedFixedTraceDecoder::new(build_decoder::<u64>(isa, version));
    let result = machine.run_with_decoder(&mut decoder);
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_memoized_dynamic_secp256k1() {
    let isa = ISA_IMC;
    let version = VERSION1;
    let buffer = fs::read("benches/data/secp256k1_bench").unwrap().into();
    let asm_core = AsmCoreMachine::new(isa, version, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    let args: Vec<Bytes> = vec!["secp256k1_bench",
                                      "033f8cf9c4d51a33206a6c1c6b27d2cc5129daa19dbd1fc148d395284f6b26411f",
                                      "304402203679d909f43f073c7c1dcf8468a485090589079ee834e6eed92fea9b09b06a2402201e46f1075afa18f306715e7db87493e7b7e779569aa13c64ab3d09980b3560a3",
                                      "foo",
                                      "bar"].into_iter().map(|a| a.into()).collect();
    machine.load_program(&buffer, &args).unwrap();
    let mut decoder = MemoizedDynamicTraceDecoder::new(build_decoder::<u64>(isa, version));
    let result = machine.run_with_decoder(&mut decoder);
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_big_binary() {
    let buffer = fs::read("tests/programs/big_binary").unwrap().into();
    let asm_core = AsmCoreMachine::new_with_memory(ISA_IMC, VERSION2, u64::max_value(), 1024 * 512);
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    let result = machine.load_program(&buffer, &vec!["simple".into()]);
    assert_eq!(result, Err(Error::MemOutOfBound));
}

#[cfg(not(feature = "enable-chaos-mode-by-default"))]
#[test]
fn test_fast_memory_initialization_bug() {
    let isa = ISA_IMC;
    let version = VERSION1;
    let buffer = fs::read("benches/data/secp256k1_bench").unwrap().into();
    let mut asm_core = AsmCoreMachine::new(isa, version, u64::max_value());
    asm_core.memory[0] = 0x1;
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    machine.load_program(&buffer, &[]).unwrap();
    assert_eq!(machine.machine.memory_mut().load8(&0).unwrap(), 0);
}
