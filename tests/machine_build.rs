#[cfg(has_asm)]
use ckb_vm::machine::asm::{AsmCoreMachine, AsmMachine, AsmWrapMachine};
use ckb_vm::{
    machine::{trace::TraceMachine, DefaultCoreMachine, VERSION0, VERSION1},
    Bytes, DefaultMachineBuilder, SparseMemory, WXorXMemory, ISA_B, ISA_IMC, ISA_MOP,
};

#[cfg(has_asm)]
pub fn asm_v0_imc<'a>(path: &str) -> AsmMachine<'a> {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let asm_wrap = AsmWrapMachine::new(asm_core, false);
    let core = DefaultMachineBuilder::new(asm_wrap)
        .instruction_cycle_func(Box::new(|_| 1))
        .build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec![Bytes::from("main")])
        .unwrap();
    machine
}

#[cfg(has_asm)]
pub fn aot_v0_imc<'a>(path: &str) -> AsmMachine<'a> {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let asm_wrap = AsmWrapMachine::new(asm_core, true);
    let core = DefaultMachineBuilder::new(asm_wrap)
        .instruction_cycle_func(Box::new(|_| 1))
        .build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec![Bytes::from("main")])
        .unwrap();
    machine
}

pub fn int_v0_imc(
    path: &str,
) -> TraceMachine<DefaultCoreMachine<u64, WXorXMemory<SparseMemory<u64>>>> {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let core_machine = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
        ISA_IMC,
        VERSION0,
        u64::max_value(),
    );
    let mut machine = TraceMachine::new(
        DefaultMachineBuilder::new(core_machine)
            .instruction_cycle_func(Box::new(|_| 1))
            .build(),
    );
    machine
        .load_program(&buffer, &vec![Bytes::from("main")])
        .unwrap();
    machine
}

pub fn int_v0_imc_32(
    path: &str,
) -> TraceMachine<DefaultCoreMachine<u32, WXorXMemory<SparseMemory<u32>>>> {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let core_machine = DefaultCoreMachine::<u32, WXorXMemory<SparseMemory<u32>>>::new(
        ISA_IMC,
        VERSION0,
        u64::max_value(),
    );
    let mut machine = TraceMachine::new(
        DefaultMachineBuilder::new(core_machine)
            .instruction_cycle_func(Box::new(|_| 1))
            .build(),
    );
    machine
        .load_program(&buffer, &vec![Bytes::from("main")])
        .unwrap();
    machine
}

#[cfg(has_asm)]
pub fn asm_v1_imcb<'a>(path: &str) -> AsmMachine<'a> {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_B, VERSION1, u64::max_value());
    let asm_wrap = AsmWrapMachine::new(asm_core, false);
    let core = DefaultMachineBuilder::new(asm_wrap)
        .instruction_cycle_func(Box::new(|_| 1))
        .build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec![Bytes::from("main")])
        .unwrap();
    machine
}

#[cfg(has_asm)]
pub fn aot_v1_imcb<'a>(path: &str) -> AsmMachine<'a> {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_B, VERSION1, u64::max_value());
    let asm_wrap = AsmWrapMachine::new(asm_core, true);
    let core = DefaultMachineBuilder::new(asm_wrap)
        .instruction_cycle_func(Box::new(|_| 1))
        .build();
    let mut machine = AsmMachine::new(core);
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
            .instruction_cycle_func(Box::new(|_| 1))
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
    let asm_wrap = AsmWrapMachine::new(asm_core, false);
    let core = DefaultMachineBuilder::new(asm_wrap)
        .instruction_cycle_func(Box::new(|_| 1))
        .build();
    let mut machine = AsmMachine::new(core);
    let mut argv = vec![Bytes::from("main")];
    argv.extend_from_slice(&args);
    machine.load_program(&buffer, &argv).unwrap();
    machine
}

#[cfg(has_asm)]
pub fn aot_v1_mop<'a>(path: &str, args: Vec<Bytes>) -> AsmMachine<'a> {
    let buffer: Bytes = std::fs::read(path).unwrap().into();

    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_B | ISA_MOP, VERSION1, u64::max_value());
    let asm_wrap = AsmWrapMachine::new(asm_core, true);
    let core = DefaultMachineBuilder::new(asm_wrap)
        .instruction_cycle_func(Box::new(|_| 1))
        .build();
    let mut argv = vec![Bytes::from("main")];
    argv.extend_from_slice(&args);
    let mut machine = AsmMachine::new(core);
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
            .instruction_cycle_func(Box::new(|_| 1))
            .build(),
    );
    let mut argv = vec![Bytes::from("main")];
    argv.extend_from_slice(&args);
    machine.load_program(&buffer, &argv).unwrap();
    machine
}
