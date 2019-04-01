use crate::instructions::{extract_opcode, Instruction, InstructionOp};

pub fn is_jitable_instruction(i: Instruction) -> bool {
    match extract_opcode(i) {
        Ok(InstructionOp::ECALL) => false,
        Ok(InstructionOp::EBREAK) => false,
        _ => true,
    }
}
