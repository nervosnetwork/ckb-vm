use super::super::machine::Machine;
use super::super::Error;
use super::utils::{extract_opcode, extract_rd, extract_utype_immediate, update_register};
use super::{Instruction as GenericInstruction, Instruction::RV32I};

#[derive(Debug)]
pub enum Instruction {
    AUIPC { rd: usize, imm: u32 },
}

impl Instruction {
    pub fn execute(&self, machine: &mut Machine) -> Result<(), Error> {
        match self {
            Instruction::AUIPC { rd, imm } => {
                let value = machine.pc + imm;
                machine.pc += 4;
                update_register(machine, *rd, value);
            }
        }
        Ok(())
    }
}

pub fn factory(instruction_bits: u32) -> Option<GenericInstruction> {
    match extract_opcode(instruction_bits) {
        0x17 => Some(RV32I(Instruction::AUIPC{
            rd: extract_rd(instruction_bits),
            imm: extract_utype_immediate(instruction_bits),
        })),
        _ => None,
    }
}
