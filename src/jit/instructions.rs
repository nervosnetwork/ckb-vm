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

pub fn is_basic_block_end_instruction(i: &Instruction) -> bool {
    match i {
        Instruction::I(i) => match i {
            i::Instruction::I(i) => match i.inst() {
                i::ItypeInstruction::JALR => true,
                _ => false,
            },
            i::Instruction::B(_) => true,
            i::Instruction::Env(_) => true,
            i::Instruction::JAL { .. } => true,
            _ => false,
        },
        Instruction::RVC(i) => match i {
            rvc::Instruction::BEQZ { .. } => true,
            rvc::Instruction::BNEZ { .. } => true,
            rvc::Instruction::JAL { .. } => true,
            rvc::Instruction::J { .. } => true,
            rvc::Instruction::JR { .. } => true,
            rvc::Instruction::JALR { .. } => true,
            rvc::Instruction::EBREAK => true,
            _ => false,
        },
        Instruction::M(_) => false,
    }
}

pub fn instruction_length(i: &Instruction) -> usize {
    match i {
        Instruction::RVC(_) => 2,
        _ => 4,
    }
}
