use bytes::Bytes;
#[cfg(has_aot)]
use ckb_vm::machine::aot::AotCompilingMachine;
#[cfg(has_asm)]
use ckb_vm::machine::asm::{AsmCoreMachine, AsmGlueMachine, AsmMachine};
use ckb_vm::machine::{DefaultCoreMachine, DefaultMachineBuilder, VERSION1};
use ckb_vm::{
    registers::A7, Error, Register, SparseMemory, SupportMachine, Syscalls, TraceMachine,
    WXorXMemory, DEFAULT_STACK_SIZE, ISA_IMC, ISA_MOP, RISCV_MAX_MEMORY,
};

#[allow(dead_code)]
mod machine_build;

pub struct CustomSyscall {}

impl<Mac: SupportMachine> Syscalls<Mac> for CustomSyscall {
    fn initialize(&mut self, _: &mut Mac) -> Result<(), Error> {
        Ok(())
    }

    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, Error> {
        let code = &machine.registers()[A7];
        if code.to_i32() != 1111 {
            return Ok(false);
        }
        let cycles = machine.cycles();
        machine.reset(machine.max_cycles());
        machine.set_cycles(cycles);
        let code_data = std::fs::read("tests/programs/reset_callee").unwrap();
        let code = Bytes::from(code_data);
        machine.load_elf(&code, true).unwrap();
        machine.initialize_stack(
            &[],
            (RISCV_MAX_MEMORY - DEFAULT_STACK_SIZE) as u64,
            DEFAULT_STACK_SIZE as u64,
        )?;
        Ok(true)
    }
}

#[test]
fn test_reset_int() {
    let code_data = std::fs::read("tests/programs/reset_caller").unwrap();
    let code = Bytes::from(code_data);

    let core_machine = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
        ISA_IMC | ISA_MOP,
        VERSION1,
        u64::max_value(),
    );
    let mut machine = DefaultMachineBuilder::new(core_machine)
        .instruction_cycle_func(Box::new(machine_build::instruction_cycle_func))
        .syscall(Box::new(CustomSyscall {}))
        .build();
    machine.load_program(&code, &vec![]).unwrap();
    let result = machine.run();
    let cycles = machine.cycles();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
    assert_eq!(cycles, 775);
}

#[test]
fn test_reset_int_with_trace() {
    let code_data = std::fs::read("tests/programs/reset_caller").unwrap();
    let code = Bytes::from(code_data);

    let core_machine = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
        ISA_IMC | ISA_MOP,
        VERSION1,
        u64::max_value(),
    );
    let mut machine = TraceMachine::new(
        DefaultMachineBuilder::new(core_machine)
            .instruction_cycle_func(Box::new(machine_build::instruction_cycle_func))
            .syscall(Box::new(CustomSyscall {}))
            .build(),
    );
    machine.load_program(&code, &vec![]).unwrap();
    let result = machine.run();
    let cycles = machine.machine.cycles();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
    assert_eq!(cycles, 775);
}

#[test]
#[cfg(has_asm)]
fn test_reset_asm() {
    let code_data = std::fs::read("tests/programs/reset_caller").unwrap();
    let code = Bytes::from(code_data);

    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_MOP, VERSION1, u64::max_value());
    let asm_glue = AsmGlueMachine::new(asm_core);
    let core = DefaultMachineBuilder::new(asm_glue)
        .instruction_cycle_func(Box::new(machine_build::instruction_cycle_func))
        .syscall(Box::new(CustomSyscall {}))
        .build();
    let mut machine = AsmMachine::new(core, None);
    machine.load_program(&code, &vec![]).unwrap();

    let result = machine.run();
    let cycles = machine.machine.cycles();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
    assert_eq!(cycles, 775);
}

#[test]
#[cfg(has_aot)]
pub fn test_reset_aot() {
    let code_data = std::fs::read("tests/programs/reset_caller").unwrap();
    let code = Bytes::from(code_data);

    let mut aot_machine = AotCompilingMachine::load(
        &code,
        Some(Box::new(machine_build::instruction_cycle_func)),
        ISA_IMC | ISA_MOP,
        VERSION1,
    )
    .unwrap();
    let code = aot_machine.compile().unwrap();

    let buffer: Bytes = std::fs::read("tests/programs/reset_caller").unwrap().into();

    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_MOP, VERSION1, u64::max_value());
    let asm_glue = AsmGlueMachine::new(asm_core);
    let core = DefaultMachineBuilder::new(asm_glue)
        .instruction_cycle_func(Box::new(machine_build::instruction_cycle_func))
        .syscall(Box::new(CustomSyscall {}))
        .build();
    let mut machine = AsmMachine::new(core, Some(std::sync::Arc::new(code)));
    machine.load_program(&buffer, &vec![]).unwrap();

    let result = machine.run();
    let cycles = machine.machine.cycles();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
    assert_eq!(cycles, 775);
}
