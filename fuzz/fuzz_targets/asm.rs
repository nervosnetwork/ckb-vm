#![no_main]
use bytes::Bytes;
use ckb_vm::instructions::cost_model::instruction_cycles;
use ckb_vm::machine::asm::{AsmCoreMachine, AsmGlueMachine, AsmMachine};
use ckb_vm::machine::{DefaultMachineBuilder, VERSION0};
use ckb_vm::ISA_IMC;
use libfuzzer_sys::fuzz_target;

fn run(data: &[u8]) {
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, 200_000);
    let asm_glue = AsmGlueMachine::new(asm_core);
    let core = DefaultMachineBuilder::new(asm_glue)
        .instruction_cycle_func(Box::new(instruction_cycles))
        .build();
    let mut machine = AsmMachine::new(core, None);
    let program = Bytes::copy_from_slice(data);
    if let Ok(_) = machine.load_program(&program, &[]) {
        let _ = machine.run();
    }
}

fuzz_target!(|data: &[u8]| {
    run(data);
});
