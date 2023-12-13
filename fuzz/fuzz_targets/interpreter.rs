#![no_main]
use ckb_vm::cost_model::constant_cycles;
use ckb_vm::machine::{DefaultCoreMachine, DefaultMachineBuilder, VERSION2};
use ckb_vm::memory::sparse::SparseMemory;
use ckb_vm::memory::wxorx::WXorXMemory;
use ckb_vm::{Bytes, ISA_A, ISA_B, ISA_IMC, ISA_MOP};
use libfuzzer_sys::fuzz_target;

fn run(data: &[u8]) {
    let machine_memory = WXorXMemory::new(SparseMemory::<u64>::default());
    let machine_core = DefaultCoreMachine::new_with_memory(
        ISA_IMC | ISA_A | ISA_B | ISA_MOP,
        VERSION2,
        200_000,
        machine_memory,
    );
    let mut machine = DefaultMachineBuilder::new(machine_core)
        .instruction_cycle_func(Box::new(constant_cycles))
        .build();
    let program = Bytes::copy_from_slice(data);
    if let Ok(_) = machine.load_program(&program, &[]) {
        let _ = machine.run();
    }
}

fuzz_target!(|data: &[u8]| {
    run(data);
});
