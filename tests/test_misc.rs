pub mod machine_build;

use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

#[cfg(has_asm)]
use ckb_vm::machine::asm::AsmCoreMachine;
use ckb_vm::{
    machine::{trace::TraceMachine, VERSION0},
    registers::{A0, A1, A2, A3, A4, A5, A7},
    CoreMachine, Debugger, DefaultCoreMachine, DefaultMachineBuilder, Error, FlatMemory, Memory,
    Register, SparseMemory, SupportMachine, Syscalls, WXorXMemory, ISA_IMC,
};
use ckb_vm_definitions::RISCV_PAGESIZE;

#[test]
pub fn test_andi() {
    let mut machine = machine_build::int_v0_imc_32("tests/programs/andi");
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_nop() {
    let mut machine = machine_build::int_v0_imc_32("tests/programs/nop");
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
pub fn test_custom_syscall() {
    let path = "tests/programs/syscall64";
    let code = std::fs::read(path).unwrap().into();
    let core_machine = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
        ISA_IMC,
        VERSION0,
        u64::max_value(),
    );
    let mut machine = DefaultMachineBuilder::new(core_machine)
        .syscall(Box::new(CustomSyscall {}))
        .build();
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
pub fn test_ebreak() {
    let path = "tests/programs/ebreak64";
    let code = std::fs::read(path).unwrap().into();
    let value = Arc::new(AtomicU8::new(0));
    let core_machine = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
        ISA_IMC,
        VERSION0,
        u64::max_value(),
    );
    let mut machine = DefaultMachineBuilder::new(core_machine)
        .debugger(Box::new(CustomDebugger {
            value: Arc::clone(&value),
        }))
        .build();
    machine
        .load_program(&code, &vec!["ebreak64".into()])
        .unwrap();
    assert_eq!(value.load(Ordering::Relaxed), 1);
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(value.load(Ordering::Relaxed), 2);
}

#[test]
pub fn test_trace() {
    let mut machine = machine_build::int_v0_imc("tests/programs/trace64");
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::InvalidPermission));
}

#[test]
pub fn test_jump0() {
    let mut machine = machine_build::int_v0_imc("tests/programs/jump0_64");
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::InvalidPermission));
}

#[test]
pub fn test_misaligned_jump64() {
    let mut machine = machine_build::int_v0_imc("tests/programs/misaligned_jump64");
    let result = machine.run();
    assert!(result.is_ok());
}

#[test]
pub fn test_mulw64() {
    let mut machine = machine_build::int_v0_imc("tests/programs/mulw64");
    let result = machine.run();
    assert!(result.is_ok());
}

#[test]
pub fn test_invalid_file_offset64() {
    let path = "tests/programs/invalid_file_offset64";
    let code = std::fs::read(path).unwrap().into();
    let core_machine = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
        ISA_IMC,
        VERSION0,
        u64::max_value(),
    );
    let mut machine = TraceMachine::new(DefaultMachineBuilder::new(core_machine).build());
    let result = machine.load_program(&code, &vec!["invalid_file_offset64".into()]);
    assert_eq!(result.err(), Some(Error::OutOfBound));
}

#[test]
#[cfg_attr(all(miri, feature = "miri-ci"), ignore)]
pub fn test_op_rvc_srli_crash_32() {
    let mut machine = machine_build::int_v0_imc_32("tests/programs/op_rvc_srli_crash_32");
    let result = machine.run();
    assert_eq!(result.err(), Some(Error::InvalidPermission));
}

#[test]
#[cfg_attr(all(miri, feature = "miri-ci"), ignore)]
pub fn test_op_rvc_srai_crash_32() {
    let mut machine = machine_build::int_v0_imc_32("tests/programs/op_rvc_srai_crash_32");
    let result = machine.run();
    assert!(result.is_ok());
}

#[test]
#[cfg_attr(all(miri, feature = "miri-ci"), ignore)]
pub fn test_op_rvc_slli_crash_32() {
    let mut machine = machine_build::int_v0_imc_32("tests/programs/op_rvc_slli_crash_32");
    let result = machine.run();
    assert!(result.is_ok());
}

#[test]
pub fn test_load_elf_crash_64() {
    let mut machine = machine_build::int_v0_imc("tests/programs/load_elf_crash_64");
    let result = machine.run();
    assert_eq!(result.err(), Some(Error::InvalidPermission));
}

#[test]
pub fn test_wxorx_crash_64() {
    let mut machine = machine_build::int_v0_imc("tests/programs/wxorx_crash_64");
    let result = machine.run();
    assert_eq!(result.err(), Some(Error::OutOfBound));
}

#[test]
pub fn test_flat_crash_64() {
    let path = "tests/programs/flat_crash_64";
    let code = std::fs::read(path).unwrap().into();
    let core_machine = DefaultCoreMachine::<u64, WXorXMemory<FlatMemory<u64>>>::new(
        ISA_IMC,
        VERSION0,
        u64::max_value(),
    );
    let mut machine = TraceMachine::new(DefaultMachineBuilder::new(core_machine).build());
    let result = machine.load_program(&code, &vec!["flat_crash_64".into()]);
    assert_eq!(result.err(), Some(Error::OutOfBound));
}

#[test]
pub fn test_memory_store_empty_bytes() {
    assert_memory_store_empty_bytes(&mut FlatMemory::<u64>::default());
    assert_memory_store_empty_bytes(&mut SparseMemory::<u64>::default());
    assert_memory_store_empty_bytes(&mut WXorXMemory::<FlatMemory<u64>>::default());
    #[cfg(has_asm)]
    assert_memory_store_empty_bytes(&mut AsmCoreMachine::new(ISA_IMC, VERSION0, 200_000));
}

fn assert_memory_store_empty_bytes<M: Memory>(memory: &mut M) {
    assert!(memory.store_byte(0, 0, 42).is_ok());
    assert!(memory.store_bytes(0, &[]).is_ok());
}

pub fn test_contains_ckbforks_section() {
    let path = "tests/programs/ckbforks";
    let code = std::fs::read(path).unwrap();
    let ckbforks_exists_v0 = || -> bool {
        let elf = goblin_v023::elf::Elf::parse(&code).unwrap();
        for section_header in &elf.section_headers {
            if let Some(Ok(r)) = elf.shdr_strtab.get(section_header.sh_name) {
                if r == ".ckb.forks" {
                    return true;
                }
            }
        }
        return false;
    }();
    let ckbforks_exists_v1 = || -> bool {
        let elf = goblin_v040::elf::Elf::parse(&code).unwrap();
        for section_header in &elf.section_headers {
            if let Some(Ok(r)) = elf.shdr_strtab.get(section_header.sh_name) {
                if r == ".ckb.forks" {
                    return true;
                }
            }
        }
        return false;
    }();
    assert_eq!(ckbforks_exists_v0, true);
    assert_eq!(ckbforks_exists_v1, true);
}

#[test]
pub fn test_rvc_pageend() {
    // The last instruction of a executable memory page is an RVC instruction.
    let path = "tests/programs/rvc_pageend";
    let code = std::fs::read(path).unwrap().into();
    let core_machine = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
        ISA_IMC,
        VERSION0,
        u64::max_value(),
    );
    let mut machine = DefaultMachineBuilder::new(core_machine).build();
    machine
        .load_program(&code, &vec!["rvc_end".into()])
        .unwrap();

    let anchor_pc: u64 = 69630;
    // Ensure that anchor_pc is in the end of the page
    assert_eq!(anchor_pc as usize % RISCV_PAGESIZE, RISCV_PAGESIZE - 2);
    let memory = machine.memory_mut();
    // Ensure that the data segment is located at anchor_pc + 2
    let data0 = memory.load16(&(anchor_pc + 2)).unwrap().to_u32();
    assert_eq!(data0, 4);
    let data1 = memory.load16(&(anchor_pc + 6)).unwrap().to_u32();
    assert_eq!(data1, 2);
    // Ensure that the anchor instruction is "c.jr a0"
    let anchor_inst = memory.load16(&anchor_pc).unwrap().to_u16();
    assert_eq!(anchor_inst, 0x8502);

    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}
