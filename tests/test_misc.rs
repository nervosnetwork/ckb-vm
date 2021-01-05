extern crate ckb_vm;

use bytes::Bytes;
use ckb_vm::{
    parse_elf,
    registers::{A0, A1, A2, A3, A4, A5, A7},
    run, Debugger, DefaultCoreMachine, DefaultMachine, DefaultMachineBuilder, Error, FlatMemory,
    Register, SparseMemory, SupportMachine, Syscalls,
};
use std::fs::File;
use std::io::Read;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

#[test]
pub fn test_andi() {
    let mut file = File::open("tests/programs/andi").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u32, SparseMemory<u32>>(&buffer, &vec!["andi".into()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_nop() {
    let mut file = File::open("tests/programs/nop").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u32, SparseMemory<u32>>(&buffer, &vec!["nop".into()]);
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
pub fn test_custom_syscall() {
    let mut file = File::open("tests/programs/syscall64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let mut machine =
        DefaultMachineBuilder::<DefaultCoreMachine<u64, SparseMemory<u64>>>::default()
            .syscall(Box::new(CustomSyscall {}))
            .build();
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
pub fn test_ebreak() {
    let mut file = File::open("tests/programs/ebreak64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let value = Arc::new(AtomicU8::new(0));
    let mut machine =
        DefaultMachineBuilder::<DefaultCoreMachine<u64, SparseMemory<u64>>>::default()
            .debugger(Box::new(CustomDebugger {
                value: Arc::clone(&value),
            }))
            .build();
    machine
        .load_program(&buffer, &vec!["ebreak".into()])
        .unwrap();
    assert_eq!(value.load(Ordering::Relaxed), 1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(value.load(Ordering::Relaxed), 2);
}

#[test]
pub fn test_trace() {
    let mut file = File::open("tests/programs/trace64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["trace64".into()]);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::InvalidPermission));
}

#[test]
pub fn test_jump0() {
    let mut file = File::open("tests/programs/jump0_64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["jump0_64".into()]);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::InvalidPermission));
}

#[test]
pub fn test_misaligned_jump64() {
    let mut file = File::open("tests/programs/misaligned_jump64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["misaligned_jump64".into()]);
    assert!(result.is_ok());
}

#[test]
pub fn test_mulw64() {
    let mut file = File::open("tests/programs/mulw64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["mulw64".into()]);
    assert!(result.is_ok());
}

#[test]
pub fn test_invalid_file_offset64() {
    let mut file = File::open("tests/programs/invalid_file_offset64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["invalid_file_offset64".into()]);
    assert_eq!(result.err(), Some(Error::OutOfBound));
}

#[test]
pub fn test_op_rvc_srli_crash_32() {
    let mut file = File::open("tests/programs/op_rvc_srli_crash_32").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u32, SparseMemory<u32>>(&buffer, &vec!["op_rvc_srli_crash_32".into()]);
    assert_eq!(result.err(), Some(Error::InvalidPermission));
}

#[test]
pub fn test_op_rvc_srai_crash_32() {
    let mut file = File::open("tests/programs/op_rvc_srai_crash_32").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u32, SparseMemory<u32>>(&buffer, &vec!["op_rvc_srai_crash_32".into()]);
    assert!(result.is_ok());
}

#[test]
pub fn test_op_rvc_slli_crash_32() {
    let mut file = File::open("tests/programs/op_rvc_slli_crash_32").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u32, SparseMemory<u32>>(&buffer, &vec!["op_rvc_slli_crash_32".into()]);
    assert!(result.is_ok());
}

#[test]
pub fn test_load_elf_crash_64() {
    let mut file = File::open("tests/programs/load_elf_crash_64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["load_elf_crash_64".into()]);
    assert_eq!(result.err(), Some(Error::InvalidPermission));
}

#[test]
pub fn test_wxorx_crash_64() {
    let mut file = File::open("tests/programs/wxorx_crash_64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["wxorx_crash_64".into()]);
    assert_eq!(result.err(), Some(Error::OutOfBound));
}

#[test]
pub fn test_flat_crash_64() {
    let mut file = File::open("tests/programs/flat_crash_64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let mut machine = DefaultMachine::<DefaultCoreMachine<u64, FlatMemory<u64>>>::default();
    let result = machine.load_program(&buffer, &vec!["flat_crash_64".into()]);
    assert_eq!(result.err(), Some(Error::OutOfBound));
}

pub fn test_contains_ckbforks_section() {
    let mut file = File::open("tests/programs/ckbforks").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let elf = parse_elf(&buffer).unwrap();

    let ckbforks_exists = || -> bool {
        for section_header in &elf.section_headers {
            if let Some(Ok(r)) = elf.shdr_strtab.get(section_header.sh_name) {
                if r == ".ckb.forks" {
                    return true;
                }
            }
        }
        return false;
    }();
    assert_eq!(ckbforks_exists, true);

    let mut machine =
        DefaultMachineBuilder::<DefaultCoreMachine<u64, SparseMemory<u64>>>::default().build();
    machine
        .load_program_elf(&buffer, &vec!["ebreak".into()], &elf)
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}
