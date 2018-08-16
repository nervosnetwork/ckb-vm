use super::super::machine::Machine;
use super::super::memory::Memory;
use super::super::Error;
use super::utils::{extract_opcode, extract_funct3,
                   extract_rd, extract_rs1,
                   extract_utype_immediate, extract_itype_immediate,
                   update_register};
use super::{Instruction as GenericInstruction, Instruction::RV32I};

#[derive(Debug)]
pub enum Instruction {
    AUIPC { rd: usize, imm: i32 },
    ADDI { rd: usize, rs1: usize, imm: i32 },
}

impl Instruction {
    pub fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<(), Error> {
        match self {
            Instruction::AUIPC { rd, imm } => {
                let (value, _) = machine.pc.overflowing_add(*imm as u32);
                machine.pc += 4;
                update_register(machine, *rd, value);
            },
            Instruction::ADDI { rd, rs1, imm } => {
                let (value, _) = machine.registers[*rs1].overflowing_add(*imm as u32);
                update_register(machine, *rd, value);
                machine.pc += 4;
            }
        }
        Ok(())
    }
}

pub fn factory(instruction_bits: u32) -> Option<GenericInstruction> {
    match extract_opcode(instruction_bits) {
        0x17 => Some(RV32I(Instruction::AUIPC {
            rd: extract_rd(instruction_bits),
            imm: extract_utype_immediate(instruction_bits),
        })),
        0x13 => match extract_funct3(instruction_bits) {
            0x0 => Some(RV32I(Instruction::ADDI {
                rd: extract_rd(instruction_bits),
                rs1: extract_rs1(instruction_bits),
                imm: extract_itype_immediate(instruction_bits),
            })),
            _ => None,
        },
        _ => None,
    }
}
