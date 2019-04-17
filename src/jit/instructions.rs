use crate::instructions::{extract_opcode, Instruction};
use ckb_vm_definitions::instructions::{OP_EBREAK, OP_ECALL};

pub fn is_jitable_instruction(i: Instruction) -> bool {
    match extract_opcode(i) {
        OP_ECALL => false,
        OP_EBREAK => false,
        _ => true,
    }
}
