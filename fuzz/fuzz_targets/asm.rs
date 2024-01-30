#![no_main]
use ckb_vm::cost_model::constant_cycles;
use ckb_vm::machine::asm::{AsmCoreMachine, AsmMachine};
use ckb_vm::machine::{DefaultMachineBuilder, VERSION2};
use ckb_vm::{Bytes, Error, ISA_A, ISA_B, ISA_IMC, ISA_MOP};
use libfuzzer_sys::fuzz_target;

fn run(data: &[u8]) -> Result<i8, Error> {
    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_A | ISA_B | ISA_MOP, VERSION2, 200_000);
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
        .instruction_cycle_func(Box::new(constant_cycles))
        .build();
    let mut machine = AsmMachine::new(core);
    let program = Bytes::copy_from_slice(data);
    machine.load_program(&program, &[])?;
    machine.run()
}

fuzz_target!(|data: &[u8]| {
    let r0 = run(data);
    let r1 = run(data);
    let r2 = run(data);
    assert_eq!(r0, r1);
    assert_eq!(r1, r2);
});
