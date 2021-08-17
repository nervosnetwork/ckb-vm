#![no_main]
use arbitrary::Arbitrary;
use ckb_vm::machine::{DefaultCoreMachine, DefaultMachineBuilder, SupportMachine, VERSION1};
use ckb_vm::registers::A7;
use ckb_vm::{Bytes, Error};
use ckb_vm::{
    Register, SparseMemory, Syscalls, WXorXMemory, DEFAULT_STACK_SIZE, ISA_IMC, ISA_MOP,
    RISCV_MAX_MEMORY,
};
use libfuzzer_sys::fuzz_target;
use std::fs;

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
        let code_data = fs::read("tests/programs/reset_callee").unwrap();
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

#[derive(Arbitrary, Debug)]
pub struct FuzzData {
    argv: Vec<String>,
}

fuzz_target!(|data: &[u8]| {
    let code_data = fs::read("tests/programs/reset_caller").unwrap();
    let code = Bytes::from(code_data);

    let core_machine = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
        ISA_IMC | ISA_MOP,
        VERSION1,
        u64::max_value(),
    );
    let mut machine = DefaultMachineBuilder::new(core_machine)
        .instruction_cycle_func(Box::new(|_| 1))
        .syscall(Box::new(CustomSyscall {}))
        .build();
    machine.load_program(&code, &vec![]).unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
});
