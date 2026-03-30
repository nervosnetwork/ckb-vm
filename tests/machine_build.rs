use bytes::Bytes;
use ckb_vm::cost_model::constant_cycles;
#[cfg(has_asm)]
use ckb_vm::machine::asm::{AsmCoreMachine, AsmDefaultMachineBuilder, AsmMachine};
use ckb_vm::machine::{DefaultCoreMachine, trace::TraceMachine};
use ckb_vm::registers::{A0, A7};
use ckb_vm::{
    DefaultMachineRunner, Error, Register, RustDefaultMachineBuilder, SparseMemory, SupportMachine,
    Syscalls, WXorXMemory,
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
    let core = AsmDefaultMachineBuilder::new(asm_core)
        .instruction_cycle_func(Box::new(constant_cycles))
        .syscall(Box::new(SleepSyscall {}))
        .build();
    let mut machine = AsmMachine::new(core);
    let mut argv = vec![];
    argv.extend(args.into_iter().map(Ok));
    machine.load_program(&buffer, argv.into_iter()).unwrap();
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
        RustDefaultMachineBuilder::new(core_machine)
            .instruction_cycle_func(Box::new(constant_cycles))
            .syscall(Box::new(SleepSyscall {}))
            .build(),
    );
    let mut argv = vec![];
    argv.extend(args.into_iter().map(Ok));
    machine.load_program(&buffer, argv.into_iter()).unwrap();
    machine
}
