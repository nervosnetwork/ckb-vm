#![no_main]
use ckb_vm::cost_model::constant_cycles;
use ckb_vm::machine::asm::{AsmCoreMachine, AsmMachine};
use ckb_vm::machine::{DefaultMachineBuilder, VERSION2};
use ckb_vm::{Bytes, ISA_A, ISA_B, ISA_IMC, ISA_MOP};
use libfuzzer_sys::fuzz_target;

fn run(data: &[u8]) {
    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_A | ISA_B | ISA_MOP, VERSION2, 200_000);
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
        .instruction_cycle_func(Box::new(constant_cycles))
        .build();
    let mut machine = AsmMachine::new(core);
    let program = Bytes::copy_from_slice(data);
    if let Ok(_) = machine.load_program(&program, &[]) {
        let _ = machine.run();
    }
}

fuzz_target!(|data: &[u8]| {
    run(data);
});
