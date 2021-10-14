#[cfg(has_asm)]
use ckb_vm::machine::asm::{AsmCoreMachine, AsmMachine};
#[cfg(has_aot)]
use ckb_vm::machine::{aot::AotCompilingMachine, asm::AotCode};

use ckb_vm::machine::{trace::TraceMachine, DefaultCoreMachine, VERSION1};
use ckb_vm::{DefaultMachineBuilder, ISA_B, ISA_IMC, ISA_MOP, ISA_V};
use ckb_vm::{Instruction, SparseMemory, WXorXMemory};

use bytes::Bytes;

pub fn instruction_cycle_func(_: Instruction) -> u64 {
    1
}

#[cfg(has_asm)]
pub fn asm_v1_imcb<'a>(path: &str) -> AsmMachine<'a> {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_B, VERSION1, u64::max_value());
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
        .instruction_cycle_func(Box::new(instruction_cycle_func))
        .build();
    let mut machine = AsmMachine::new(core, None);
    machine
        .load_program(&buffer, &vec![Bytes::from("main")])
        .unwrap();
    machine
}

#[cfg(has_aot)]
pub fn aot_v1_imcb_code(path: &str) -> AotCode {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let mut aot_machine =
        AotCompilingMachine::load(&buffer, None, ISA_IMC | ISA_B, VERSION1).unwrap();
    aot_machine.compile().unwrap()
}

#[cfg(has_aot)]
pub fn aot_v1_imcb<'a>(path: &str, code: &'a AotCode) -> AsmMachine<'a> {
    let buffer: Bytes = std::fs::read(path).unwrap().into();

    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_B, VERSION1, u64::max_value());
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
        .instruction_cycle_func(Box::new(instruction_cycle_func))
        .build();
    let mut machine = AsmMachine::new(core, Some(code));
    machine
        .load_program(&buffer, &vec![Bytes::from("main")])
        .unwrap();
    machine
}

pub fn int_v1_imcb(
    path: &str,
) -> TraceMachine<DefaultCoreMachine<u64, WXorXMemory<SparseMemory<u64>>>> {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let core_machine = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
        ISA_IMC | ISA_B,
        VERSION1,
        u64::max_value(),
    );
    let mut machine = TraceMachine::new(
        DefaultMachineBuilder::new(core_machine)
            .instruction_cycle_func(Box::new(instruction_cycle_func))
            .build(),
    );
    machine
        .load_program(&buffer, &vec![Bytes::from("main")])
        .unwrap();
    machine
}

#[cfg(has_asm)]
pub fn asm_v1_mop<'a>(path: &str, args: Vec<Bytes>) -> AsmMachine<'a> {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_B | ISA_MOP, VERSION1, u64::max_value());
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
        .instruction_cycle_func(Box::new(instruction_cycle_func))
        .build();
    let mut machine = AsmMachine::new(core, None);
    let mut argv = vec![Bytes::from("main")];
    argv.extend_from_slice(&args);
    machine.load_program(&buffer, &argv).unwrap();
    machine
}

#[cfg(has_aot)]
pub fn aot_v1_mop_code(path: &str) -> AotCode {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let mut aot_machine =
        AotCompilingMachine::load(&buffer, None, ISA_IMC | ISA_B | ISA_MOP, VERSION1).unwrap();
    aot_machine.compile().unwrap()
}

#[cfg(has_aot)]
pub fn aot_v1_mop<'a>(path: &str, args: Vec<Bytes>, code: &'a AotCode) -> AsmMachine<'a> {
    let buffer: Bytes = std::fs::read(path).unwrap().into();

    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_B | ISA_MOP, VERSION1, u64::max_value());
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
        .instruction_cycle_func(Box::new(instruction_cycle_func))
        .build();
    let mut argv = vec![Bytes::from("main")];
    argv.extend_from_slice(&args);
    let mut machine = AsmMachine::new(core, Some(code));
    machine.load_program(&buffer, &argv).unwrap();
    machine
}

pub fn int_v1_mop(
    path: &str,
    args: Vec<Bytes>,
) -> TraceMachine<DefaultCoreMachine<u64, WXorXMemory<SparseMemory<u64>>>> {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let core_machine = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
        ISA_IMC | ISA_B | ISA_MOP,
        VERSION1,
        u64::max_value(),
    );
    let mut machine = TraceMachine::new(
        DefaultMachineBuilder::new(core_machine)
            .instruction_cycle_func(Box::new(instruction_cycle_func))
            .build(),
    );
    let mut argv = vec![Bytes::from("main")];
    argv.extend_from_slice(&args);
    machine.load_program(&buffer, &argv).unwrap();
    machine
}

#[cfg(has_asm)]
pub fn asm_v1_imcv<'a>(path: &str) -> AsmMachine<'a> {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_V, VERSION1, u64::max_value());
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
        .instruction_cycle_func(Box::new(instruction_cycle_func))
        .build();
    let mut machine = AsmMachine::new(core, None);
    machine
        .load_program(&buffer, &vec![Bytes::from("main")])
        .unwrap();
    machine
}

#[cfg(has_aot)]
pub fn aot_v1_imcv_code(path: &str) -> AotCode {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let mut aot_machine =
        AotCompilingMachine::load(&buffer, None, ISA_IMC | ISA_V, VERSION1).unwrap();
    aot_machine.compile().unwrap()
}

#[cfg(has_aot)]
pub fn aot_v1_imcv<'a>(path: &str, code: &'a AotCode) -> AsmMachine<'a> {
    let buffer: Bytes = std::fs::read(path).unwrap().into();

    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_V, VERSION1, u64::max_value());
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
        .instruction_cycle_func(Box::new(instruction_cycle_func))
        .build();
    let mut machine = AsmMachine::new(core, Some(code));
    machine
        .load_program(&buffer, &vec![Bytes::from("main")])
        .unwrap();
    machine
}

pub fn int_v1_imcv(
    path: &str,
) -> TraceMachine<DefaultCoreMachine<u64, WXorXMemory<SparseMemory<u64>>>> {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let core_machine = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
        ISA_IMC | ISA_V,
        VERSION1,
        u64::max_value(),
    );
    let mut machine = TraceMachine::new(
        DefaultMachineBuilder::new(core_machine)
            .instruction_cycle_func(Box::new(instruction_cycle_func))
            .build(),
    );
    machine
        .load_program(&buffer, &vec![Bytes::from("main")])
        .unwrap();
    machine
}
