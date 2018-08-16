use super::super::machine::Machine;
use super::super::memory::Memory;
use super::super::Error;
use super::utils::update_register;
use super::{Instruction as GenericInstruction, Instruction::C};

#[inline(always)]
// This function extract bits [15:13] and bits [1:0], then connect them
// into a 5 bit opcode to decode RVC instructions
fn extract_opcode(instruction_bits: u32) -> u32 {
    (instruction_bits & 0x3) | ((instruction_bits >> 11) & 0x1C)
}

// This function extract bits [12:10] and bits [6:7], then connect them
// into a 5 bit opcode to decode RVC ALU instructions
fn extract_alu_opcode(instruction_bits: u32) -> u32 {
    ((instruction_bits >> 5) & 0x3) | ((instruction_bits >> 8) & 0x1C)
}

// This function extract 3 bits from least_bit to form a register number,
// here since we are only using 3 bits, we can only reference the most popular
// used registers x8 - x15. In other words, a number of 0 extracted here means
// x8, 1 means x9, etc.
fn extract_compact_register_number(instruction_bits: u32, least_bit: usize) -> usize {
    ((instruction_bits >> least_bit) & 0x7) as usize + 8
}

#[derive(Debug)]
pub enum Instruction {
    SUB { rd: usize, rs2: usize },
}

impl Instruction {
    pub fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<(), Error> {
        match self {
            Instruction::SUB { rd, rs2 } => {
                let (value, _) = machine.registers[*rd].overflowing_sub(machine.registers[*rs2]);
                machine.pc += 2;
                update_register(machine, *rd, value);
            }
        }
        Ok(())
    }
}

pub fn factory(instruction_bits: u32) -> Option<GenericInstruction> {
    match extract_opcode(instruction_bits) {
        0x11 => match extract_alu_opcode(instruction_bits) {
            0xc => Some(C(Instruction::SUB {
                rd: extract_compact_register_number(instruction_bits, 7),
                rs2: extract_compact_register_number(instruction_bits, 2),
            })),
            _ => None,
        },
        _ => None,
    }
}
