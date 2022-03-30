use std::fs;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

#[cfg(has_asm)]
use ckb_vm::machine::asm::AsmCoreMachine;
use ckb_vm::{
    machine::VERSION0,
    registers::{A0, A1, A2, A3, A4, A5, A7},
    run, CoreMachine, Debugger, DefaultCoreMachine, DefaultMachineBuilder, Error, FlatMemory,
    Memory, Register, SparseMemory, SupportMachine, Syscalls, WXorXMemory, ISA_IMC,
};
use ckb_vm_definitions::RISCV_PAGESIZE;

#[test]
pub fn test_andi() {
    let buffer = fs::read("tests/programs/andi").unwrap().into();
    let result = run::<u32, SparseMemory<u32>>(&buffer, &vec!["andi".into()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_nop() {
    let buffer = fs::read("tests/programs/nop").unwrap().into();
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
    let buffer = fs::read("tests/programs/syscall64").unwrap().into();
    let core_machine =
        DefaultCoreMachine::<u64, SparseMemory<u64>>::new(ISA_IMC, VERSION0, u64::max_value());
    let mut machine = DefaultMachineBuilder::new(core_machine)
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
    let buffer = fs::read("tests/programs/ebreak64").unwrap().into();
    let value = Arc::new(AtomicU8::new(0));
    let core_machine =
        DefaultCoreMachine::<u64, SparseMemory<u64>>::new(ISA_IMC, VERSION0, u64::max_value());
    let mut machine = DefaultMachineBuilder::new(core_machine)
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
    let buffer = fs::read("tests/programs/trace64").unwrap().into();
    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["trace64".into()]);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::MemWriteOnExecutablePage));
}

#[test]
pub fn test_jump0() {
    let buffer = fs::read("tests/programs/jump0_64").unwrap().into();
    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["jump0_64".into()]);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::MemWriteOnExecutablePage));
}

#[test]
pub fn test_misaligned_jump64() {
    let buffer = fs::read("tests/programs/misaligned_jump64").unwrap().into();
    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["misaligned_jump64".into()]);
    assert!(result.is_ok());
}

#[test]
pub fn test_mulw64() {
    let buffer = fs::read("tests/programs/mulw64").unwrap().into();
    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["mulw64".into()]);
    assert!(result.is_ok());
}

#[test]
pub fn test_invalid_file_offset64() {
    let buffer = fs::read("tests/programs/invalid_file_offset64")
        .unwrap()
        .into();
    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["invalid_file_offset64".into()]);
    assert_eq!(result.err(), Some(Error::ElfSegmentAddrOrSizeError));
}

#[test]
#[cfg_attr(all(miri, feature = "miri-ci"), ignore)]
pub fn test_op_rvc_srli_crash_32() {
    let buffer = fs::read("tests/programs/op_rvc_srli_crash_32")
        .unwrap()
        .into();
    let result = run::<u32, SparseMemory<u32>>(&buffer, &vec!["op_rvc_srli_crash_32".into()]);
    assert_eq!(result.err(), Some(Error::MemWriteOnExecutablePage));
}

#[test]
#[cfg_attr(all(miri, feature = "miri-ci"), ignore)]
pub fn test_op_rvc_srai_crash_32() {
    let buffer = fs::read("tests/programs/op_rvc_srai_crash_32")
        .unwrap()
        .into();
    let result = run::<u32, SparseMemory<u32>>(&buffer, &vec!["op_rvc_srai_crash_32".into()]);
    assert!(result.is_ok());
}

#[test]
#[cfg_attr(all(miri, feature = "miri-ci"), ignore)]
pub fn test_op_rvc_slli_crash_32() {
    let buffer = fs::read("tests/programs/op_rvc_slli_crash_32")
        .unwrap()
        .into();
    let result = run::<u32, SparseMemory<u32>>(&buffer, &vec!["op_rvc_slli_crash_32".into()]);
    assert!(result.is_ok());
}

#[test]
pub fn test_load_elf_crash_64() {
    let buffer = fs::read("tests/programs/load_elf_crash_64").unwrap().into();
    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["load_elf_crash_64".into()]);
    assert_eq!(result.err(), Some(Error::MemWriteOnExecutablePage));
}

#[test]
pub fn test_wxorx_crash_64() {
    let buffer = fs::read("tests/programs/wxorx_crash_64").unwrap().into();
    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["wxorx_crash_64".into()]);
    assert_eq!(result.err(), Some(Error::MemOutOfBound));
}

#[test]
pub fn test_flat_crash_64() {
    let buffer = fs::read("tests/programs/flat_crash_64").unwrap().into();
    let core_machine =
        DefaultCoreMachine::<u64, FlatMemory<u64>>::new(ISA_IMC, VERSION0, u64::max_value());
    let mut machine = DefaultMachineBuilder::new(core_machine).build();
    let result = machine.load_program(&buffer, &vec!["flat_crash_64".into()]);
    assert_eq!(result.err(), Some(Error::MemOutOfBound));
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
    let buffer = fs::read("tests/programs/ckbforks").unwrap();
    let ckbforks_exists_v0 = || -> bool {
        let elf = goblin_v023::elf::Elf::parse(&buffer).unwrap();
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
        let elf = goblin_v040::elf::Elf::parse(&buffer).unwrap();
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
    let buffer = fs::read("tests/programs/rvc_pageend").unwrap().into();
    let core_machine =
        DefaultCoreMachine::<u64, SparseMemory<u64>>::new(ISA_IMC, VERSION0, u64::max_value());
    let mut machine = DefaultMachineBuilder::new(core_machine).build();
    machine
        .load_program(&buffer, &vec!["rvc_end".into()])
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
pub fn test_outofcycles_in_syscall() {
    let buffer = fs::read("tests/programs/syscall64").unwrap().into();
    let core_machine = DefaultCoreMachine::<u64, SparseMemory<u64>>::new(ISA_IMC, VERSION0, 20);
    let mut machine = DefaultMachineBuilder::new(core_machine)
        .instruction_cycle_func(Box::new(|_, _, _, _| 1))
        .syscall(Box::new(OutOfCyclesSyscall {}))
        .build();
    machine
        .load_program(&buffer, &vec!["syscall".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::CyclesExceeded);
    assert_eq!(machine.cycles(), 108);
    assert_eq!(machine.registers()[A0], 39);
}
