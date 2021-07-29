#![cfg(has_asm)]

use ckb_vm::{
    decoder::build_decoder,
    instructions::extract_opcode,
    machine::{
        asm::{AsmCoreMachine, AsmMachine},
        CoreMachine, VERSION0, VERSION1,
    },
    memory::Memory,
    registers::{A0, A1, A2, A3, A4, A5, A7},
    Debugger, DefaultMachineBuilder, Error, Instruction, Register, SupportMachine, Syscalls, ISA_B,
    ISA_IMC, ISA_MOP,
};
use ckb_vm_definitions::instructions as insts;
use std::fs;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

#[test]
pub fn test_asm_pprof_abc() {
    let buffer = fs::read("/src/ckb-vm-pprof/res/abc").unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION1, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core)
        .instruction_cycle_func(Box::new(|_| 1))
        .build();
    let mut machine = AsmMachine::new(core, None);
    machine.load_program(&buffer, &vec!["abc".into()]).unwrap();
    let result = machine.run();
    machine
        .pprof_logger
        .unwrap()
        .tree_root
        .borrow_mut()
        .display_flamegraph("", &mut std::io::stdout());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_asm_pprof_bench_pairing() {
    let buffer = fs::read("/src/ckb-vm-pprof/res/bench_pairing")
        .unwrap()
        .into();
    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_B | ISA_MOP, VERSION1, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core)
        .instruction_cycle_func(Box::new(instruction_cycles))
        .build();
    let mut machine = AsmMachine::new(core, None);
    machine
        .load_program(&buffer, &vec!["bench_pairing".into()])
        .unwrap();
    let result = machine.run();
    machine
        .pprof_logger
        .unwrap()
        .tree_root
        .borrow_mut()
        .display_flamegraph("", &mut std::io::stdout());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

pub fn instruction_cycles(i: Instruction) -> u64 {
    match extract_opcode(i) {
        // IMC
        insts::OP_JALR => 3,
        insts::OP_LD => 2,
        insts::OP_LW => 3,
        insts::OP_LH => 3,
        insts::OP_LB => 3,
        insts::OP_LWU => 3,
        insts::OP_LHU => 3,
        insts::OP_LBU => 3,
        insts::OP_SB => 3,
        insts::OP_SH => 3,
        insts::OP_SW => 3,
        insts::OP_SD => 2,
        insts::OP_BEQ => 3,
        insts::OP_BGE => 3,
        insts::OP_BGEU => 3,
        insts::OP_BLT => 3,
        insts::OP_BLTU => 3,
        insts::OP_BNE => 3,
        insts::OP_EBREAK => 500,
        insts::OP_ECALL => 500,
        insts::OP_JAL => 3,
        insts::OP_MUL => 5,
        insts::OP_MULW => 5,
        insts::OP_MULH => 5,
        insts::OP_MULHU => 5,
        insts::OP_MULHSU => 5,
        insts::OP_DIV => 32,
        insts::OP_DIVW => 32,
        insts::OP_DIVU => 32,
        insts::OP_DIVUW => 32,
        insts::OP_REM => 32,
        insts::OP_REMW => 32,
        insts::OP_REMU => 32,
        insts::OP_REMUW => 32,
        // MOP
        insts::OP_WIDE_MUL => 5,
        insts::OP_WIDE_MULU => 5,
        insts::OP_WIDE_MULSU => 5,
        insts::OP_WIDE_DIV => 32,
        insts::OP_WIDE_DIVU => 32,
        insts::OP_FAR_JUMP_REL => 3,
        insts::OP_FAR_JUMP_ABS => 3,
        _ => 1,
    }
}
