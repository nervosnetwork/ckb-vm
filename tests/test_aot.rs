#![cfg(has_aot)]

use ckb_vm::{
    machine::{
        aot::AotCompilingMachine,
        asm::{AsmCoreMachine, AsmMachine},
        CoreMachine, VERSION0, VERSION1,
    },
    memory::Memory,
    registers::{A0, A1, A2, A3, A4, A5, A7},
    Debugger, DefaultMachineBuilder, Error, Instruction, Register, SupportMachine, Syscalls,
    ISA_IMC,
};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::{fs, u64};

#[test]
pub fn test_aot_simple64() {
    let buffer = fs::read("tests/programs/simple64").unwrap().into();
    let mut aot_machine = AotCompilingMachine::load(&buffer, None, ISA_IMC, VERSION0).unwrap();
    let code = aot_machine.compile().unwrap();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core, Some(&code));
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
pub fn test_aot_with_custom_syscall() {
    let buffer = fs::read("tests/programs/syscall64").unwrap().into();
    let mut aot_machine = AotCompilingMachine::load(&buffer, None, ISA_IMC, VERSION0).unwrap();
    let code = aot_machine.compile().unwrap();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core)
        .syscall(Box::new(CustomSyscall {}))
        .build();
    let mut machine = AsmMachine::new(core, Some(&code));
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
pub fn test_aot_ebreak() {
    let buffer = fs::read("tests/programs/ebreak64").unwrap().into();
    let value = Arc::new(AtomicU8::new(0));
    let mut aot_machine = AotCompilingMachine::load(&buffer, None, ISA_IMC, VERSION0).unwrap();
    let code = aot_machine.compile().unwrap();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core)
        .debugger(Box::new(CustomDebugger {
            value: Arc::clone(&value),
        }))
        .build();
    let mut machine = AsmMachine::new(core, Some(&code));
    machine
        .load_program(&buffer, &vec!["ebreak".into()])
        .unwrap();
    assert_eq!(value.load(Ordering::Relaxed), 1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(value.load(Ordering::Relaxed), 2);
}

fn dummy_cycle_func(_i: Instruction, _: u64, _: u64, _: bool) -> u64 {
    1
}

#[test]
pub fn test_aot_simple_cycles() {
    let buffer = fs::read("tests/programs/simple64").unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, 708);
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
        .instruction_cycle_func(Box::new(dummy_cycle_func))
        .build();
    let mut aot_machine =
        AotCompilingMachine::load(&buffer, Some(Box::new(dummy_cycle_func)), ISA_IMC, VERSION0)
            .unwrap();
    let code = aot_machine.compile().unwrap();
    let mut machine = AsmMachine::new(core, Some(&code));
    machine
        .load_program(&buffer, &vec!["syscall".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    assert_eq!(SupportMachine::cycles(&machine.machine), 708);
}

#[test]
pub fn test_aot_simple_max_cycles_reached() {
    let buffer = fs::read("tests/programs/simple64").unwrap().into();
    // Running simple64 should consume 708 cycles using dummy cycle func
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, 700);
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
        .instruction_cycle_func(Box::new(dummy_cycle_func))
        .build();
    let mut aot_machine =
        AotCompilingMachine::load(&buffer, Some(Box::new(dummy_cycle_func)), ISA_IMC, VERSION0)
            .unwrap();
    let code = aot_machine.compile().unwrap();
    let mut machine = AsmMachine::new(core, Some(&code));
    machine
        .load_program(&buffer, &vec!["syscall".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::CyclesExceeded);
}

#[test]
pub fn test_aot_trace() {
    let buffer = fs::read("tests/programs/trace64").unwrap().into();
    let mut aot_machine = AotCompilingMachine::load(&buffer, None, ISA_IMC, VERSION0).unwrap();
    let code = aot_machine.compile().unwrap();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core, Some(&code));
    machine
        .load_program(&buffer, &vec!["simple".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::MemWriteOnExecutablePage));
}

#[test]
pub fn test_aot_jump0() {
    let buffer = fs::read("tests/programs/jump0_64").unwrap().into();
    let mut aot_machine = AotCompilingMachine::load(&buffer, None, ISA_IMC, VERSION0).unwrap();
    let code = aot_machine.compile().unwrap();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core, Some(&code));
    machine
        .load_program(&buffer, &vec!["jump0_64".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::MemWriteOnExecutablePage));
}

#[test]
pub fn test_aot_write_large_address() {
    let buffer = fs::read("tests/programs/write_large_address64")
        .unwrap()
        .into();
    let mut aot_machine = AotCompilingMachine::load(&buffer, None, ISA_IMC, VERSION0).unwrap();
    let code = aot_machine.compile().unwrap();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core, Some(&code));
    machine
        .load_program(&buffer, &vec!["write_large_address64".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::MemOutOfBound));
}

#[test]
pub fn test_aot_misaligned_jump64() {
    let buffer = fs::read("tests/programs/misaligned_jump64").unwrap().into();
    let mut aot_machine = AotCompilingMachine::load(&buffer, None, ISA_IMC, VERSION0).unwrap();
    let code = aot_machine.compile().unwrap();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core, Some(&code));
    machine
        .load_program(&buffer, &vec!["write_large_address64".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
}

#[test]
pub fn test_aot_mulw64() {
    let buffer = fs::read("tests/programs/mulw64").unwrap().into();
    let mut aot_machine = AotCompilingMachine::load(&buffer, None, ISA_IMC, VERSION0).unwrap();
    let code = aot_machine.compile().unwrap();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core, Some(&code));
    machine
        .load_program(&buffer, &vec!["mulw64".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_aot_invalid_read64() {
    let buffer = fs::read("tests/programs/invalid_read64").unwrap().into();
    let mut aot_machine = AotCompilingMachine::load(&buffer, None, ISA_IMC, VERSION0).unwrap();
    let code = aot_machine.compile().unwrap();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core, Some(&code));
    machine
        .load_program(&buffer, &vec!["invalid_read64".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::MemOutOfBound));
}

#[test]
pub fn test_aot_load_elf_crash_64() {
    let buffer = fs::read("tests/programs/load_elf_crash_64").unwrap().into();
    let mut aot_machine = AotCompilingMachine::load(&buffer, None, ISA_IMC, VERSION0).unwrap();
    let code = aot_machine.compile().unwrap();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core, Some(&code));
    machine
        .load_program(&buffer, &vec!["load_elf_crash_64".into()])
        .unwrap();
    let result = machine.run();
    assert_eq!(result.err(), Some(Error::MemWriteOnExecutablePage));
}

#[test]
pub fn test_aot_wxorx_crash_64() {
    let buffer = fs::read("tests/programs/wxorx_crash_64").unwrap().into();
    let mut aot_machine = AotCompilingMachine::load(&buffer, None, ISA_IMC, VERSION0).unwrap();
    let code = aot_machine.compile().unwrap();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core, Some(&code));
    machine
        .load_program(&buffer, &vec!["wxorx_crash_64".into()])
        .unwrap();
    let result = machine.run();
    assert_eq!(result.err(), Some(Error::MemOutOfBound));
}

#[test]
pub fn test_aot_load_elf_section_crash_64() {
    let buffer = fs::read("tests/programs/load_elf_section_crash_64")
        .unwrap()
        .into();
    let result = AotCompilingMachine::load(&buffer, None, ISA_IMC, VERSION0);
    assert_eq!(result.err(), Some(Error::AotSectionIsEmpty));
}

#[test]
pub fn test_aot_load_malformed_elf_crash_64() {
    let buffer = fs::read("tests/programs/load_malformed_elf_crash_64")
        .unwrap()
        .into();
    let result = AotCompilingMachine::load(&buffer, None, ISA_IMC, VERSION0);
    assert!(matches!(result.err(), Some(Error::ElfParseError(_))));
}

#[test]
pub fn test_aot_flat_crash_64() {
    let buffer = fs::read("tests/programs/flat_crash_64").unwrap().into();
    let result = AotCompilingMachine::load(&buffer, None, ISA_IMC, VERSION0);
    assert_eq!(result.err(), Some(Error::MemOutOfBound));
}

#[test]
pub fn test_aot_alloc_many() {
    let buffer = fs::read("tests/programs/alloc_many").unwrap().into();
    let mut aot_machine = AotCompilingMachine::load(&buffer, None, ISA_IMC, VERSION0).unwrap();
    let code = aot_machine.compile().unwrap();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core, Some(&code));
    machine
        .load_program(&buffer, &vec!["alloc_many".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_aot_chaos_seed() {
    let buffer = fs::read("tests/programs/read_memory").unwrap().into();
    let mut aot_machine1 = AotCompilingMachine::load(&buffer, None, ISA_IMC, VERSION1).unwrap();
    let code1 = aot_machine1.compile().unwrap();
    let mut asm_core1 = AsmCoreMachine::new(ISA_IMC, VERSION1, u64::max_value());
    asm_core1.chaos_mode = 1;
    asm_core1.chaos_seed = 100;
    let core1 = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core1).build();
    let mut machine1 = AsmMachine::new(core1, Some(&code1));
    machine1
        .load_program(&buffer, &vec!["read_memory".into()])
        .unwrap();
    let result1 = machine1.run();
    let exit1 = result1.unwrap();

    let mut aot_machine2 = AotCompilingMachine::load(&buffer, None, ISA_IMC, VERSION1).unwrap();
    let code2 = aot_machine2.compile().unwrap();
    let mut asm_core2 = AsmCoreMachine::new(ISA_IMC, VERSION1, u64::max_value());
    asm_core2.chaos_mode = 1;
    asm_core2.chaos_seed = 100;
    let core2 = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core2).build();
    let mut machine2 = AsmMachine::new(core2, Some(&code2));
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
pub fn test_aot_rvc_pageend() {
    let buffer = fs::read("tests/programs/rvc_pageend").unwrap().into();
    let mut aot_machine = AotCompilingMachine::load(&buffer, None, ISA_IMC, VERSION0).unwrap();
    let code = aot_machine.compile().unwrap();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core, Some(&code));
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
pub fn test_aot_outofcycles_in_syscall() {
    let buffer = fs::read("tests/programs/syscall64").unwrap().into();
    let mut aot_machine =
        AotCompilingMachine::load(&buffer, Some(Box::new(|_, _, _, _| 1)), ISA_IMC, VERSION0)
            .unwrap();
    let code = aot_machine.compile().unwrap();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, 20);
    let core = DefaultMachineBuilder::new(asm_core)
        .instruction_cycle_func(Box::new(|_, _, _, _| 1))
        .syscall(Box::new(OutOfCyclesSyscall {}))
        .build();
    let mut machine = AsmMachine::new(core, Some(&code));
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
pub fn test_aot_cycles_overflow() {
    let buffer = fs::read("tests/programs/simple64").unwrap().into();
    let mut aot_machine =
        AotCompilingMachine::load(&buffer, Some(Box::new(|_, _, _, _| 1)), ISA_IMC, VERSION1)
            .unwrap();
    let code = aot_machine.compile().unwrap();

    let mut asm_core = AsmCoreMachine::new(ISA_IMC, VERSION1, u64::MAX);
    asm_core.cycles = u64::MAX - 10;
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
        .instruction_cycle_func(Box::new(|_, _, _, _| 1))
        .build();
    let mut machine = AsmMachine::new(core, Some(&code));
    machine
        .load_program(&buffer, &vec!["simple64".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::CyclesOverflow);
}
