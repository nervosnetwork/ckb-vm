use bytes::Bytes;
use ckb_vm::cost_model::constant_cycles;
#[cfg(has_asm)]
use ckb_vm::machine::asm::{AsmCoreMachine, AsmMachine};
use ckb_vm::machine::{trace::TraceMachine, DefaultCoreMachine};
use ckb_vm::registers::{A0, A7};
use ckb_vm::{
    DefaultMachineBuilder, Error, Register, SparseMemory, SupportMachine, Syscalls, WXorXMemory,
};

pub struct SleepSyscall {}

impl<Mac: SupportMachine> Syscalls<Mac> for SleepSyscall {
    fn initialize(&mut self, _machine: &mut Mac) -> Result<(), Error> {
        Ok(())
    }

    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, Error> {
        let code = &machine.registers()[A7];
        if code.to_i32() != 1000 {
            return Ok(false);
        }
        let duration = machine.registers()[A0].to_u64();
        std::thread::sleep(std::time::Duration::from_millis(duration));

        machine.set_register(A0, Mac::REG::from_u8(0));
        Ok(true)
    }
}

#[cfg(has_asm)]
pub fn asm(path: &str, args: Vec<Bytes>, version: u32, isa: u8) -> AsmMachine {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let asm_core = AsmCoreMachine::new(isa, version, u64::MAX);
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
        .instruction_cycle_func(Box::new(constant_cycles))
        .syscall(Box::new(SleepSyscall {}))
        .build();
    let mut machine = AsmMachine::new(core);
    let mut argv = vec![Bytes::from("main")];
    argv.extend_from_slice(&args);
    machine.load_program(&buffer, &argv).unwrap();
    machine
}

pub fn int(
    path: &str,
    args: Vec<Bytes>,
    version: u32,
    isa: u8,
) -> TraceMachine<DefaultCoreMachine<u64, WXorXMemory<SparseMemory<u64>>>> {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let core_machine =
        DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(isa, version, u64::MAX);
    let mut machine = TraceMachine::new(
        DefaultMachineBuilder::new(core_machine)
            .instruction_cycle_func(Box::new(constant_cycles))
            .syscall(Box::new(SleepSyscall {}))
            .build(),
    );
    let mut argv = vec![Bytes::from("main")];
    argv.extend_from_slice(&args);
    machine.load_program(&buffer, &argv).unwrap();
    machine
}
