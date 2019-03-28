use crate::instructions::{i, rvc, Instruction};

pub fn is_jitable_instruction(i: &Instruction) -> bool {
    match i {
        Instruction::I(i) => match i {
            i::Instruction::Env(_) => false,
            _ => true,
        },
        Instruction::RVC(i) => match i {
            rvc::Instruction::EBREAK => false,
            _ => true,
        },
        Instruction::M(_) => true,
    }
}
