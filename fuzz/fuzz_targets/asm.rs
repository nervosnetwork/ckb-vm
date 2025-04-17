#![no_main]
use ckb_vm::cost_model::constant_cycles;
use ckb_vm::machine::asm::{AsmCoreMachine, AsmDefaultMachineBuilder, AsmMachine};
use ckb_vm::machine::trace::TraceMachine;
use ckb_vm::machine::{
    DefaultCoreMachine, DefaultMachineRunner, RustDefaultMachineBuilder, SupportMachine, VERSION2,
};
use ckb_vm::memory::sparse::SparseMemory;
use ckb_vm::memory::wxorx::WXorXMemory;
use ckb_vm::{Bytes, Error, ISA_A, ISA_B, ISA_IMC, ISA_MOP};
use libfuzzer_sys::fuzz_target;

fn run_asm(data: &[u8]) -> Result<(i8, u64), Error> {
    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_A | ISA_B | ISA_MOP, VERSION2, 200_000);
    let core = AsmDefaultMachineBuilder::new(asm_core)
        .instruction_cycle_func(Box::new(constant_cycles))
        .build();
    let mut machine = AsmMachine::new(core);
    let program = Bytes::copy_from_slice(data);
    machine.load_program(&program, vec![].into_iter())?;
    let exit_code = machine.run()?;
    let cycles = machine.machine.cycles();
    Ok((exit_code, cycles))
}

fn run_int(data: &[u8]) -> Result<(i8, u64), Error> {
    let machine_core = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
        ISA_IMC | ISA_A | ISA_B | ISA_MOP,
        VERSION2,
        200_000,
    );
    let mut machine = TraceMachine::new(
        RustDefaultMachineBuilder::new(machine_core)
            .instruction_cycle_func(Box::new(constant_cycles))
            .build(),
    );
    let program = Bytes::copy_from_slice(data);
    machine.load_program(&program, vec![].into_iter())?;
    let exit_code = machine.run()?;
    let cycles = machine.machine.cycles();
    Ok((exit_code, cycles))
}

fuzz_target!(|data: &[u8]| {
    let r0 = run_asm(data);
    let r1 = run_asm(data);
    let r2 = run_asm(data);
    let r3 = run_int(data);
    assert_eq!(r0, r1);
    assert_eq!(r1, r2);
    if r2.is_ok() {
        assert_eq!(r2.unwrap(), r3.unwrap());
    } else {
        assert!(r3.is_err())
    }
});
