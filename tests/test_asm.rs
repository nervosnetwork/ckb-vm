#![cfg(has_asm)]

pub mod machine_build;

use ckb_vm::{
    machine::{
        asm::{AsmCoreMachine, AsmMachine, AsmWrapMachine},
        CoreMachine, VERSION0,
    },
    memory::Memory,
    registers::{A0, A1, A2, A3, A4, A5, A7},
    Debugger, DefaultMachineBuilder, Error, Register, SupportMachine, Syscalls, ISA_IMC,
};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

#[test]
pub fn test_asm_simple64() {
    let mut machine = machine_build::asm_v0_imc("tests/programs/simple64");
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
    let path = "tests/programs/syscall64";
    let code = std::fs::read(path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let asm_wrap = AsmWrapMachine::new(asm_core, false);
    let core = DefaultMachineBuilder::new(asm_wrap)
        .syscall(Box::new(CustomSyscall {}))
        .build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&code, &vec!["syscall64".into()])
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
    let path = "tests/programs/ebreak64";
    let code = std::fs::read(path).unwrap().into();
    let value = Arc::new(AtomicU8::new(0));
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let asm_wrap = AsmWrapMachine::new(asm_core, false);
    let core = DefaultMachineBuilder::new(asm_wrap)
        .debugger(Box::new(CustomDebugger {
            value: Arc::clone(&value),
        }))
        .build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&code, &vec!["ebreak64".into()])
        .unwrap();
    assert_eq!(value.load(Ordering::Relaxed), 1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(value.load(Ordering::Relaxed), 2);
}

#[test]
pub fn test_asm_simple_cycles() {
    let mut machine = machine_build::asm_v0_imc("tests/programs/simple64");
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
    assert_eq!(SupportMachine::cycles(&machine.machine), 517);
}

#[test]
pub fn test_asm_simple_max_cycles_reached() {
    let mut machine = machine_build::asm_v0_imc("tests/programs/simple64");
    // Running simple64 should consume 517 cycles using dummy cycle func
    machine.machine.inner_mut().machine.max_cycles = 500;
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidCycles);
}

#[test]
pub fn test_asm_trace() {
    let mut machine = machine_build::asm_v0_imc("tests/programs/trace64");
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::InvalidPermission));
}

#[test]
pub fn test_asm_jump0() {
    let mut machine = machine_build::asm_v0_imc("tests/programs/jump0_64");
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::InvalidPermission));
}

#[test]
pub fn test_asm_write_large_address() {
    let mut machine = machine_build::asm_v0_imc("tests/programs/write_large_address64");
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::OutOfBound));
}

#[test]
pub fn test_asm_misaligned_jump64() {
    let mut machine = machine_build::asm_v0_imc("tests/programs/misaligned_jump64");
    let result = machine.run();
    assert!(result.is_ok());
}

#[test]
pub fn test_asm_mulw64() {
    let mut machine = machine_build::asm_v0_imc("tests/programs/mulw64");
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_invalid_read64() {
    let mut machine = machine_build::asm_v0_imc("tests/programs/invalid_read64");
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::OutOfBound));
}

#[test]
pub fn test_asm_load_elf_crash_64() {
    let mut machine = machine_build::asm_v0_imc("tests/programs/load_elf_crash_64");
    let result = machine.run();
    assert_eq!(result.err(), Some(Error::InvalidPermission));
}

#[test]
pub fn test_asm_wxorx_crash_64() {
    let mut machine = machine_build::asm_v0_imc("tests/programs/wxorx_crash_64");
    let result = machine.run();
    assert_eq!(result.err(), Some(Error::OutOfBound));
}

#[test]
pub fn test_asm_alloc_many() {
    let mut machine = machine_build::asm_v0_imc("tests/programs/alloc_many");
    let result = machine.run();
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_chaos_seed() {
    let mut machine1 = machine_build::asm_v0_imc("tests/programs/read_memory");
    machine1.machine.inner_mut().machine.chaos_mode = 1;
    machine1.machine.inner_mut().machine.chaos_seed = 100;
    let result1 = machine1.run();
    let exit1 = result1.unwrap();

    let mut machine2 = machine_build::asm_v0_imc("tests/programs/read_memory");
    machine2.machine.inner_mut().machine.chaos_mode = 1;
    machine2.machine.inner_mut().machine.chaos_seed = 100;
    let result2 = machine2.run();
    let exit2 = result2.unwrap();

    assert_eq!(exit1, exit2);
    // Read 8 bytes from 0x300000, it is very unlikely that they are both 0.
    assert!(machine1.machine.memory_mut().load64(&0x300000).unwrap() != 0);
    assert!(machine2.machine.memory_mut().load64(&0x300000).unwrap() != 0);
}

#[test]
pub fn test_asm_rvc_pageend() {
    let mut machine = machine_build::asm_v0_imc("tests/programs/rvc_pageend");
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}
