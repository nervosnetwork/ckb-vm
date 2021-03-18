#![no_main]
use bytes::Bytes;
use ckb_vm::machine::asm::{AsmCoreMachine, AsmMachine};
use ckb_vm::machine::{DefaultMachineBuilder, VERSION0};
use ckb_vm::ISA_IMC;
use libfuzzer_sys::fuzz_target;

fn run(data: &[u8]) {
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, 200_000);
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
        .instruction_cycle_func(Box::new(|_| 1))
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
