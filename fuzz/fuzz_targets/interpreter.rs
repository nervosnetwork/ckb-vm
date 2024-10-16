#![no_main]
use ckb_vm::cost_model::constant_cycles;
use ckb_vm::machine::trace::TraceMachine;
use ckb_vm::machine::{DefaultCoreMachine, DefaultMachineBuilder, SupportMachine, VERSION2};
use ckb_vm::memory::sparse::SparseMemory;
use ckb_vm::memory::wxorx::WXorXMemory;
use ckb_vm::{Bytes, Error, ISA_A, ISA_B, ISA_IMC, ISA_MOP};
use libfuzzer_sys::fuzz_target;

fn run(data: &[u8]) -> Result<(i8, u64), Error> {
    let machine_memory = WXorXMemory::new(SparseMemory::<u64>::default());
    let machine_core = DefaultCoreMachine::new_with_memory(
        ISA_IMC | ISA_A | ISA_B | ISA_MOP,
        VERSION2,
        200_000,
        machine_memory,
    );
    let mut machine = TraceMachine::new(
        DefaultMachineBuilder::new(machine_core)
            .instruction_cycle_func(Box::new(constant_cycles))
            .build(),
    );
    let program = Bytes::copy_from_slice(data);
    machine.load_program(&program, &[])?;
    let exit_code = machine.run()?;
    let cycles = machine.machine.cycles();
    Ok((exit_code, cycles))
}

fuzz_target!(|data: &[u8]| {
    let r0 = run(data);
    let r1 = run(data);
    let r2 = run(data);
    assert_eq!(r0, r1);
    assert_eq!(r1, r2);
});
