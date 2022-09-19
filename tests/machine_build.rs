#[cfg(has_asm)]
use ckb_vm::machine::asm::{AsmCoreMachine, AsmGlueMachine, AsmMachine};
#[cfg(has_aot)]
use ckb_vm::machine::{aot::AotCompilingMachine, asm::AotCode};

use bytes::Bytes;
use ckb_vm::machine::{trace::TraceMachine, DefaultCoreMachine, VERSION1};
use ckb_vm::registers::{A0, A7};
use ckb_vm::{
    DefaultMachineBuilder, Instruction, Memory, Register, SparseMemory, SupportMachine, Syscalls,
    WXorXMemory, ISA_B, ISA_IMC, ISA_MOP, ISA_V,
};
use std::io::Write;

pub struct DebugSyscall {}

impl<Mac: SupportMachine> Syscalls<Mac> for DebugSyscall {
    fn initialize(&mut self, _machine: &mut Mac) -> Result<(), ckb_vm::error::Error> {
        Ok(())
    }

    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, ckb_vm::error::Error> {
        let code = &machine.registers()[A7];
        if code.to_i32() != 2177 {
            return Ok(false);
        }

        let mut addr = machine.registers()[A0].to_u64();
        let mut buffer = Vec::new();

        loop {
            let byte = machine
                .memory_mut()
                .load8(&Mac::REG::from_u64(addr))?
                .to_u8();
            if byte == 0 {
                break;
            }
            buffer.push(byte);
            addr += 1;
        }

        std::io::stdout().write(&buffer)?;
        if buffer.last().copied() != Some('\n' as u8) {
            std::io::stdout().write(&['\n' as u8])?;
        }

        Ok(true)
    }
}

pub fn instruction_cycle_func(_: Instruction, _: u64, _: u64) -> u64 {
    1
}

#[cfg(has_asm)]
pub fn asm_v1_imcb(path: &str) -> AsmMachine {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_B, VERSION1, u64::max_value());
    let asm_glue = AsmGlueMachine::new(asm_core);
    let core = DefaultMachineBuilder::new(asm_glue)
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
    let mut aot_machine = AotCompilingMachine::load(
        &buffer,
        Some(Box::new(instruction_cycle_func)),
        ISA_IMC | ISA_B,
        VERSION1,
    )
    .unwrap();
    aot_machine.compile().unwrap()
}

#[cfg(has_aot)]
pub fn aot_v1_imcb(path: &str, code: AotCode) -> AsmMachine {
    let buffer: Bytes = std::fs::read(path).unwrap().into();

    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_B, VERSION1, u64::max_value());
    let asm_glue = AsmGlueMachine::new(asm_core);
    let core = DefaultMachineBuilder::new(asm_glue)
        .instruction_cycle_func(Box::new(instruction_cycle_func))
        .build();
    let mut machine = AsmMachine::new(core, Some(std::sync::Arc::new(code)));
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
pub fn asm_v1_mop(path: &str, args: Vec<Bytes>) -> AsmMachine {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_B | ISA_MOP, VERSION1, u64::max_value());
    let asm_glue = AsmGlueMachine::new(asm_core);
    let core = DefaultMachineBuilder::new(asm_glue)
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
    let mut aot_machine = AotCompilingMachine::load(
        &buffer,
        Some(Box::new(instruction_cycle_func)),
        ISA_IMC | ISA_B | ISA_MOP,
        VERSION1,
    )
    .unwrap();
    aot_machine.compile().unwrap()
}

#[cfg(has_aot)]
pub fn aot_v1_mop(path: &str, args: Vec<Bytes>, code: AotCode) -> AsmMachine {
    let buffer: Bytes = std::fs::read(path).unwrap().into();

    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_B | ISA_MOP, VERSION1, u64::max_value());
    let asm_glue = AsmGlueMachine::new(asm_core);
    let core = DefaultMachineBuilder::new(asm_glue)
        .instruction_cycle_func(Box::new(instruction_cycle_func))
        .build();
    let mut argv = vec![Bytes::from("main")];
    argv.extend_from_slice(&args);
    let mut machine = AsmMachine::new(core, Some(std::sync::Arc::new(code)));
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
pub fn asm_v1_imcv(path: &str) -> AsmMachine {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_V, VERSION1, u64::max_value());
    let asm_glue = AsmGlueMachine::new(asm_core);
    let core = DefaultMachineBuilder::new(asm_glue)
        .instruction_cycle_func(Box::new(instruction_cycle_func))
        .syscall(Box::new(DebugSyscall {}))
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
    let mut aot_machine = AotCompilingMachine::load(
        &buffer,
        Some(Box::new(instruction_cycle_func)),
        ISA_IMC | ISA_V,
        VERSION1,
    )
    .unwrap();
    aot_machine.compile().unwrap()
}

#[cfg(has_aot)]
pub fn aot_v1_imcv(path: &str, code: AotCode) -> AsmMachine {
    let buffer: Bytes = std::fs::read(path).unwrap().into();

    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_V, VERSION1, u64::max_value());
    let asm_glue = AsmGlueMachine::new(asm_core);
    let core = DefaultMachineBuilder::new(asm_glue)
        .instruction_cycle_func(Box::new(instruction_cycle_func))
        .syscall(Box::new(DebugSyscall {}))
        .build();
    let mut machine = AsmMachine::new(core, Some(std::sync::Arc::new(code)));
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
            .syscall(Box::new(DebugSyscall {}))
            .build(),
    );
    machine
        .load_program(&buffer, &vec![Bytes::from("main")])
        .unwrap();
    machine
}
