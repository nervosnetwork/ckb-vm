#[cfg(has_asm)]
use ckb_vm::machine::aot::AotCompilingMachine;
#[cfg(has_asm)]
use ckb_vm::machine::asm::{AsmCoreMachine, AsmMachine};

use ckb_vm::machine::{trace::TraceMachine, DefaultCoreMachine, VERSION1};
use ckb_vm::{DefaultMachineBuilder, ISA_B, ISA_IMC};
use ckb_vm::{SparseMemory, WXorXMemory};

use bytes::Bytes;

#[cfg(has_asm)]
pub fn asm_v1_imcb(path: &str) {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_B, VERSION1, u64::max_value());
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core).build();
    let mut machine = AsmMachine::new(core, None);
    machine.load_program(&buffer, &["main".into()]).unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[cfg(has_asm)]
pub fn aot_v1_imcb(path: &str) {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let mut aot_machine =
        AotCompilingMachine::load(&buffer, None, ISA_IMC | ISA_B, VERSION1).unwrap();
    let code = aot_machine.compile().unwrap();
    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_B, VERSION1, u64::max_value());
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core).build();
    let mut machine = AsmMachine::new(core, Some(&code));
    machine.load_program(&buffer, &vec!["main".into()]).unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

pub fn int_v1_imcb(path: &str) {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let core_machine = DefaultCoreMachine::<u64, WXorXMemory<u64, SparseMemory<u64>>>::new(
        ISA_IMC | ISA_B,
        VERSION1,
        u64::max_value(),
    );
    let mut machine = TraceMachine::new(DefaultMachineBuilder::new(core_machine).build());
    machine.load_program(&buffer, &vec!["main".into()]).unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}
