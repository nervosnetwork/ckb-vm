#![no_main]
use ckb_vm::cost_model::constant_cycles;
use ckb_vm::machine::asm::{AsmCoreMachine, AsmMachine};
use ckb_vm::machine::{DefaultMachineBuilder, VERSION2};
use ckb_vm::snapshot;
use ckb_vm::{Bytes, Error, SupportMachine, ISA_A, ISA_B, ISA_IMC, ISA_MOP};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut machine1 = {
        let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_A | ISA_B | ISA_MOP, VERSION2, 200_000);
        let machine = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
            .instruction_cycle_func(Box::new(constant_cycles))
            .build();
        AsmMachine::new(machine)
    };
    let program = Bytes::copy_from_slice(data);
    if machine1.load_program(&program, &[]).is_err() {
        return;
    };
    let result1 = machine1.run();
    if machine1.machine.cycles() < 4 {
        return;
    }

    let half_cycles = machine1.machine.cycles() / 2;
    let mut machine2 = {
        let asm_core =
            AsmCoreMachine::new(ISA_IMC | ISA_A | ISA_B | ISA_MOP, VERSION2, half_cycles);
        let machine = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
            .instruction_cycle_func(Box::new(constant_cycles))
            .build();
        AsmMachine::new(machine)
    };
    machine2.load_program(&program, &[]).unwrap();
    let result2 = machine2.run();
    assert_eq!(result2.unwrap_err(), Error::CyclesExceeded);
    let snap = snapshot::make_snapshot(&mut machine2.machine).unwrap();

    let mut machine3 = {
        let asm_core =
            AsmCoreMachine::new(ISA_IMC | ISA_A | ISA_B | ISA_MOP, VERSION2, half_cycles);
        let machine = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
            .instruction_cycle_func(Box::new(constant_cycles))
            .build();
        AsmMachine::new(machine)
    };
    snapshot::resume(&mut machine3.machine, &snap).unwrap();

    machine3.machine.set_cycles(machine2.machine.cycles());
    machine3.machine.set_max_cycles(200_000);
    let result3 = machine3.run();
    assert_eq!(result1, result3);
    assert_eq!(machine1.machine.cycles(), machine3.machine.cycles());
});
