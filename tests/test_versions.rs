#![cfg(has_asm)]
use ckb_vm::cost_model::constant_cycles;
use ckb_vm::error::OutOfBoundKind;
use ckb_vm::machine::asm::{AsmCoreMachine, AsmMachine};
use ckb_vm::machine::{VERSION0, VERSION1, VERSION2};
use ckb_vm::memory::{FLAG_DIRTY, FLAG_FREEZED};
use ckb_vm::{
    CoreMachine, DefaultCoreMachine, DefaultMachine, DefaultMachineBuilder, Error, Memory,
    SparseMemory, TraceMachine, WXorXMemory, ISA_A, ISA_B, ISA_IMC, ISA_MOP, RISCV_PAGESIZE,
};
use std::fs;

type Mem = WXorXMemory<SparseMemory<u64>>;

fn create_rust_machine(
    program: String,
    version: u32,
) -> DefaultMachine<DefaultCoreMachine<u64, Mem>> {
    let path = format!("tests/programs/{}", program);
    let buffer = fs::read(path).unwrap().into();
    let core_machine = DefaultCoreMachine::<u64, Mem>::new(ISA_IMC, version, u64::MAX);
    let mut machine =
        DefaultMachineBuilder::<DefaultCoreMachine<u64, Mem>>::new(core_machine).build();
    machine
        .load_program(&buffer, &vec![program.into()])
        .unwrap();
    machine
}

fn create_asm_machine(program: String, version: u32) -> AsmMachine {
    let path = format!("tests/programs/{}", program);
    let buffer = fs::read(path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, version, u64::MAX);
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec![program.into()])
        .unwrap();
    machine
}

#[test]
pub fn test_rust_version0_argv_null() {
    let mut machine = create_rust_machine("argv_null_test".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[test]
pub fn test_rust_version0_sp_alignment() {
    let mut machine = create_rust_machine("sp_alignment_test".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[test]
pub fn test_rust_version0_jalr_bug() {
    let mut machine = create_rust_machine("jalr_bug".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), -1);
}

#[test]
pub fn test_rust_version0_jalr_bug_noc() {
    let mut machine = create_rust_machine("jalr_bug_noc".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), -1);
}

#[test]
pub fn test_rust_version0_read_at_boundary() {
    let mut machine = create_rust_machine("read_at_boundary64".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(
        result.err(),
        Some(Error::MemOutOfBound(0x400000, OutOfBoundKind::Memory))
    );
}

#[test]
pub fn test_rust_version0_write_at_boundary() {
    let mut machine = create_rust_machine("write_at_boundary64".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_rust_version1_argv_null() {
    let mut machine = create_rust_machine("argv_null_test".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_rust_version1_sp_alignment() {
    let mut machine = create_rust_machine("sp_alignment_test".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_rust_version1_jalr_bug() {
    let mut machine = create_rust_machine("jalr_bug".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_rust_version1_jalr_bug_noc() {
    let mut machine = create_rust_machine("jalr_bug_noc".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_rust_version1_read_at_boundary() {
    let mut machine = create_rust_machine("read_at_boundary64".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_rust_version1_write_at_boundary() {
    let mut machine = create_rust_machine("write_at_boundary64".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_version0_argv_null() {
    let mut machine = create_asm_machine("argv_null_test".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[test]
pub fn test_asm_version0_sp_alignment() {
    let mut machine = create_asm_machine("sp_alignment_test".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[test]
pub fn test_asm_version0_jalr_bug() {
    let mut machine = create_asm_machine("jalr_bug".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), -1);
}

#[test]
pub fn test_asm_version0_jalr_bug_noc() {
    let mut machine = create_asm_machine("jalr_bug_noc".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), -1);
}

#[test]
pub fn test_asm_version0_read_at_boundary() {
    let mut machine = create_asm_machine("read_at_boundary64".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(
        result.err(),
        Some(Error::MemOutOfBound(0x400000, OutOfBoundKind::Memory))
    );
}

#[test]
pub fn test_asm_version0_write_at_boundary() {
    let mut machine = create_asm_machine("write_at_boundary64".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_version1_argv_null() {
    let mut machine = create_asm_machine("argv_null_test".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_version1_sp_alignment() {
    let mut machine = create_asm_machine("sp_alignment_test".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_version1_jalr_bug() {
    let mut machine = create_asm_machine("jalr_bug".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_version1_jalr_bug_noc() {
    let mut machine = create_asm_machine("jalr_bug_noc".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_version1_read_at_boundary() {
    let mut machine = create_asm_machine("read_at_boundary64".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_version1_write_at_boundary() {
    let mut machine = create_asm_machine("write_at_boundary64".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_rust_version0_unaligned64() {
    let program = "unaligned64";
    let buffer = fs::read(format!("tests/programs/{}", program))
        .unwrap()
        .into();
    let core_machine = DefaultCoreMachine::<u64, Mem>::new(ISA_IMC, VERSION0, u64::MAX);
    let mut machine =
        DefaultMachineBuilder::<DefaultCoreMachine<u64, Mem>>::new(core_machine).build();
    let result = machine.load_program(&buffer, &vec![program.into()]);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::MemWriteOnExecutablePage(16)));
}

#[test]
pub fn test_rust_version1_unaligned64() {
    let mut machine = create_rust_machine("unaligned64".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_version0_unaligned64() {
    let program = "unaligned64";
    let buffer = fs::read(format!("tests/programs/{}", program))
        .unwrap()
        .into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::MAX);
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    let result = machine.load_program(&buffer, &vec![program.into()]);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::MemWriteOnExecutablePage(16)));
}

#[test]
pub fn test_asm_version1_unaligned64() {
    let mut machine = create_asm_machine("unaligned64".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_version0_writable_page() {
    let mut machine = create_asm_machine("writable_page".to_string(), VERSION0);
    // 0x12000 is the address of the variable "buffer", which can be found from the dump file.
    let page_index = 0x12000 / RISCV_PAGESIZE as u64;
    let flag = machine.machine.memory_mut().fetch_flag(page_index).unwrap();
    assert_eq!(flag, FLAG_DIRTY | FLAG_FREEZED);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_version1_writable_page() {
    let mut machine = create_asm_machine("writable_page".to_string(), VERSION1);
    let page_index = 0x12000 / RISCV_PAGESIZE as u64;
    let flag = machine.machine.memory_mut().fetch_flag(page_index).unwrap();
    assert_eq!(flag, FLAG_DIRTY);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_version0_goblin_overflow_elf() {
    // This test case only guarantee that the process of loading elf will not crash.
    let machine = create_asm_machine("goblin_overflow_elf".to_string(), VERSION0);
    assert_eq!(machine.machine.version(), VERSION0);
}

#[test]
pub fn test_asm_version1_goblin_overflow_elf() {
    // This test case only guarantee that the process of loading elf will not crash.
    let machine = create_asm_machine("goblin_overflow_elf".to_string(), VERSION1);
    assert_eq!(machine.machine.version(), VERSION1);
}

#[test]
pub fn test_asm_version0_cadd_hints() {
    let mut machine = create_rust_machine("cadd_hints".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        Error::InvalidInstruction {
            pc: 65656,
            instruction: 36906
        }
    );
}

#[test]
pub fn test_asm_version1_cadd_hints() {
    let mut machine = create_rust_machine("cadd_hints".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_version1_asm_trace_bug() {
    let buffer = fs::read("tests/programs/asm_trace_bug").unwrap().into();

    let mut machine = {
        let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_A | ISA_B | ISA_MOP, VERSION1, 2000);
        let machine = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
            .instruction_cycle_func(Box::new(constant_cycles))
            .build();
        AsmMachine::new(machine)
    };
    machine.load_program(&buffer, &[]).unwrap();
    let result = machine.run();

    assert_eq!(result, Err(Error::CyclesExceeded));
}

#[test]
pub fn test_asm_version2_asm_trace_bug() {
    let buffer = fs::read("tests/programs/asm_trace_bug").unwrap().into();

    let mut machine = {
        let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_A | ISA_B | ISA_MOP, VERSION2, 2000);
        let machine = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
            .instruction_cycle_func(Box::new(constant_cycles))
            .build();
        AsmMachine::new(machine)
    };
    machine.load_program(&buffer, &[]).unwrap();
    let result = machine.run();

    assert_eq!(
        result,
        Err(Error::MemOutOfBound(21474836484, OutOfBoundKind::Memory))
    );
}

#[test]
pub fn test_trace_version1_asm_trace_bug() {
    let buffer = fs::read("tests/programs/asm_trace_bug").unwrap().into();

    let mut machine = {
        let core_machine = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
            ISA_IMC | ISA_A | ISA_B | ISA_MOP,
            VERSION1,
            2000,
        );
        TraceMachine::new(
            DefaultMachineBuilder::new(core_machine)
                .instruction_cycle_func(Box::new(constant_cycles))
                .build(),
        )
    };
    machine.load_program(&buffer, &[]).unwrap();
    let result = machine.run();

    assert_eq!(result, Err(Error::CyclesExceeded));
}

#[test]
pub fn test_trace_version2_asm_trace_bug() {
    let buffer = fs::read("tests/programs/asm_trace_bug").unwrap().into();

    let mut machine = {
        let core_machine = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
            ISA_IMC | ISA_A | ISA_B | ISA_MOP,
            VERSION2,
            2000,
        );
        TraceMachine::new(
            DefaultMachineBuilder::new(core_machine)
                .instruction_cycle_func(Box::new(constant_cycles))
                .build(),
        )
    };
    machine.load_program(&buffer, &[]).unwrap();
    let result = machine.run();

    assert_eq!(
        result,
        Err(Error::MemOutOfBound(21474836484, OutOfBoundKind::Memory))
    );
}
