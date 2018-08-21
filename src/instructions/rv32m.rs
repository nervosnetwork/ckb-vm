use super::super::machine::Machine;
use super::super::memory::Memory;
use super::super::Error;
use super::utils::{opcode, funct3, funct7, rd, rs1, rs2, update_register};
use super::{Instruction as GenericInstruction, Instruction::RV32M};

#[derive(Debug)]
pub enum Instruction {
    // R-type
    DIV { rd: usize, rs1: usize, rs2: usize },
}

impl Instruction {
    pub fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<(), Error> {
        match self {
            Instruction::DIV { rd, rs1, rs2 } => {
                let rs1_value: i32 = machine.registers[*rs1] as i32;
                let rs2_value: i32 = machine.registers[*rs2] as i32;
                let value = if rs2_value == 0 {
                    // This is documented in RISC-V spec, when divided by
                    // 0, RISC-V machine would return -1 in DIV instead of
                    // trapping.
                    -1
                } else {
                    let (result, _ ) = rs1_value.overflowing_div(rs2_value);
                    result
                };
                update_register(machine, *rd, value as u32);
            }
        }
        machine.pc += 4;
        Ok(())
    }
}

pub fn factory(instruction_bits: u32) -> Option<GenericInstruction> {
    if funct7(instruction_bits) != 0x1 || opcode(instruction_bits) != 0x33 {
        return None;
    }
    match funct3(instruction_bits) {
        0x4 => Some(RV32M(Instruction::DIV {
            rd: rd(instruction_bits),
            rs1: rs1(instruction_bits),
            rs2: rs2(instruction_bits),
        })),
        _ => None,
    }
}
