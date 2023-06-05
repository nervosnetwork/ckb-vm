#![cfg(has_asm)]
use ckb_vm::cost_model::constant_cycles;
use ckb_vm::machine::asm::{AsmCoreMachine, AsmMachine};
use ckb_vm::machine::VERSION2;
use ckb_vm::{DefaultMachineBuilder, Error, SupportMachine, ISA_A, ISA_B, ISA_IMC};
use std::fs;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
pub mod machine_build;

#[test]
pub fn test_asm_suspend() {
    let expect_cycles = {
        let buffer = fs::read("tests/programs/suspend_resume").unwrap().into();
        let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_A | ISA_B, VERSION2, u64::max_value());
        let core = DefaultMachineBuilder::new(asm_core)
            .instruction_cycle_func(Box::new(constant_cycles))
            .build();
        let mut machine = AsmMachine::new(core);
        machine.load_program(&buffer, &vec!["main".into()]).unwrap();
        machine.run().unwrap();
        machine.machine.cycles()
    };

    let buffer = fs::read("tests/programs/suspend_resume").unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_A | ISA_B, VERSION2, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core)
        .instruction_cycle_func(Box::new(constant_cycles))
        .build();
    let mut machine = AsmMachine::new(core);
    machine.load_program(&buffer, &vec!["main".into()]).unwrap();

    let branch_suspend_cnt = Arc::new(AtomicU32::new(0));
    let branch_suspend_cnt_jh = branch_suspend_cnt.clone();

    let signal = machine.suspend.clone();
    let jh = std::thread::spawn(move || loop {
        let result = machine.run();
        if result == Err(Error::Suspend) {
            branch_suspend_cnt_jh.fetch_add(1, Ordering::SeqCst);
            continue;
        } else {
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 0);
            assert_eq!(machine.machine.cycles(), expect_cycles);
            break;
        }
    });
    for _ in 0..10 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        signal.store(true, Ordering::SeqCst);
    }
    jh.join().unwrap();
    assert_eq!(branch_suspend_cnt.load(Ordering::SeqCst), 10);
}
