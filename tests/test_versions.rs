#![cfg(has_asm)]

use ckb_vm::{
    machine::{
        asm::{AsmCoreMachine, AsmMachine, AsmWrapMachine},
        VERSION0, VERSION1,
    },
    memory::FLAG_FREEZED,
    CoreMachine, DefaultCoreMachine, DefaultMachine, DefaultMachineBuilder, Error, Memory,
    SparseMemory, WXorXMemory, ISA_IMC, RISCV_PAGESIZE,
};

type Mem = WXorXMemory<SparseMemory<u64>>;

fn create_rust_machine<'a>(
    program: String,
    version: u32,
) -> DefaultMachine<'a, DefaultCoreMachine<u64, Mem>> {
    let path = format!("tests/programs/{}", program);
    let code = std::fs::read(path).unwrap().into();
    let core_machine = DefaultCoreMachine::<u64, Mem>::new(ISA_IMC, version, u64::max_value());
    let mut machine =
        DefaultMachineBuilder::<DefaultCoreMachine<u64, Mem>>::new(core_machine).build();
    machine.load_program(&code, &vec![program.into()]).unwrap();
    machine
}

fn create_asm_machine<'a>(program: String, version: u32) -> AsmMachine<'a> {
    let path = format!("tests/programs/{}", program);
    let code = std::fs::read(path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, version, u64::max_value());
    let asm_wrap = AsmWrapMachine::new(asm_core, false);
    let core = DefaultMachineBuilder::new(asm_wrap).build();
    let mut machine = AsmMachine::new(core);
    machine.load_program(&code, &vec![program.into()]).unwrap();
    machine
}

fn create_aot_machine<'a>(program: String, version: u32) -> AsmMachine<'a> {
    let path = format!("tests/programs/{}", program);
    let code = std::fs::read(path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, version, u64::max_value());
    let asm_wrap = AsmWrapMachine::new(asm_core, true);
    let core = DefaultMachineBuilder::new(asm_wrap).build();
    let mut machine = AsmMachine::new(core);
    machine.load_program(&code, &vec![program.into()]).unwrap();
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
    assert_eq!(result.err(), Some(Error::OutOfBound));
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
    assert_eq!(result.err(), Some(Error::OutOfBound));
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
pub fn test_aot_version0_argv_null() {
    let mut machine = create_aot_machine("argv_null_test".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[test]
pub fn test_aot_version0_sp_alignment() {
    let mut machine = create_aot_machine("sp_alignment_test".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[test]
pub fn test_aot_version0_jalr_bug() {
    let mut machine = create_aot_machine("jalr_bug".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), -1);
}

#[test]
pub fn test_aot_version0_jalr_bug_noc() {
    let mut machine = create_aot_machine("jalr_bug_noc".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), -1);
}

#[test]
pub fn test_aot_version0_read_at_boundary() {
    let mut machine = create_aot_machine("read_at_boundary64".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::OutOfBound));
}

#[test]
pub fn test_aot_version0_write_at_boundary() {
    let mut machine = create_aot_machine("write_at_boundary64".to_string(), VERSION0);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_aot_version1_argv_null() {
    let mut machine = create_aot_machine("argv_null_test".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_aot_version1_sp_alignment() {
    let mut machine = create_aot_machine("sp_alignment_test".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_aot_version1_jalr_bug() {
    let mut machine = create_aot_machine("jalr_bug".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_aot_version1_jalr_bug_noc() {
    let mut machine = create_aot_machine("jalr_bug_noc".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_aot_version1_read_at_boundary() {
    let mut machine = create_aot_machine("read_at_boundary64".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_aot_version1_write_at_boundary() {
    let mut machine = create_aot_machine("write_at_boundary64".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_rust_version0_unaligned64() {
    let path = "tests/programs/unaligned64";
    let code = std::fs::read(path).unwrap().into();
    let core_machine = DefaultCoreMachine::<u64, Mem>::new(ISA_IMC, VERSION0, u64::max_value());
    let mut machine =
        DefaultMachineBuilder::<DefaultCoreMachine<u64, Mem>>::new(core_machine).build();
    let result = machine.load_program(&code, &vec!["unaligned64".into()]);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::InvalidPermission));
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
    let path = "tests/programs/unaligned64";
    let code = std::fs::read(path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let asm_wrap = AsmWrapMachine::new(asm_core, false);
    let core = DefaultMachineBuilder::new(asm_wrap).build();
    let mut machine = AsmMachine::new(core);
    let result = machine.load_program(&code, &vec!["unaligned64".into()]);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::InvalidPermission));
}

#[test]
pub fn test_asm_version1_unaligned64() {
    let mut machine = create_asm_machine("unaligned64".to_string(), VERSION1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_aot_version0_unaligned64() {
    let path = "tests/programs/unaligned64";
    let code = std::fs::read(path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let asm_wrap = AsmWrapMachine::new(asm_core, true);
    let core = DefaultMachineBuilder::new(asm_wrap).build();
    let mut machine = AsmMachine::new(core);
    let result = machine.load_program(&code, &vec!["unaligned64".into()]);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::InvalidPermission));
}

#[test]
pub fn test_aot_version1_unaligned64() {
    let mut machine = create_aot_machine("unaligned64".to_string(), VERSION1);
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
    assert_eq!(flag, FLAG_FREEZED);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_version1_writable_page() {
    let mut machine = create_asm_machine("writable_page".to_string(), VERSION1);
    let page_index = 0x12000 / RISCV_PAGESIZE as u64;
    let flag = machine.machine.memory_mut().fetch_flag(page_index).unwrap();
    assert_eq!(flag, 0);
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
