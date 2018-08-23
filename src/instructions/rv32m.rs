use super::super::machine::Machine;
use super::super::memory::Memory;
use super::super::Error;
use super::utils::{opcode, funct3, funct7, rd, rs1, rs2, update_register};
use super::{
    Instruction as GenericInstruction,
    Instruction::RV32M,
    NextPC,
    Execute,
};

#[derive(Debug)]
pub enum RtypeInstruction {
    MUL,
    MULH,
    MULHSU,
    MULHU,
    DIV,
    DIVU,
    REM,
    REMU,
}

pub type Rtype = super::Rtype<RtypeInstruction>;

#[derive(Debug)]
pub struct Instruction(Rtype);

impl Execute for Rtype {
    fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<Option<NextPC>, Error> {
        match &self.inst {
            RtypeInstruction::MUL => {
                let rs1_value = machine.registers[self.rs1];
                let rs2_value = machine.registers[self.rs2];
                let (value, _) = rs1_value.overflowing_mul(rs2_value);
                update_register(machine, self.rd, value);
            },
            RtypeInstruction::MULH => {},
            RtypeInstruction::MULHSU => {},
            RtypeInstruction::MULHU => {},
            RtypeInstruction::DIV => {
                let rs1_value: i32 = machine.registers[self.rs1] as i32;
                let rs2_value: i32 = machine.registers[self.rs2] as i32;
                let value = if rs2_value == 0 {
                    // This is documented in RISC-V spec, when divided by
                    // 0, RISC-V machine would return -1 in DIV instead of
                    // trapping.
                    -1
                } else {
                    rs1_value.overflowing_div(rs2_value).0
                };
                update_register(machine, self.rd, value as u32);
            },
            RtypeInstruction::DIVU => {},
            RtypeInstruction::REM => {},
            RtypeInstruction::REMU => {},
        }
        Ok(None)
    }
}

impl Instruction {
    pub fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<(), Error> {
        let next_pc = self.0.execute(machine)?;
        machine.pc = next_pc.unwrap_or(machine.pc + 4);
        Ok(())
    }
}

pub fn factory(instruction_bits: u32) -> Option<GenericInstruction> {
    if opcode(instruction_bits) != 0b_0110011 || funct7(instruction_bits) != 0b_0000001 {
        None
    } else {
        let inst_opt = match funct3(instruction_bits) {
            0b_000 => Some(RtypeInstruction::MUL),
            0b_001 => Some(RtypeInstruction::MULH),
            0b_010 => Some(RtypeInstruction::MULHSU),
            0b_011 => Some(RtypeInstruction::MULHU),
            0b_100 => Some(RtypeInstruction::DIV),
            0b_101 => Some(RtypeInstruction::DIVU),
            0b_110 => Some(RtypeInstruction::REM),
            0b_111 => Some(RtypeInstruction::REMU),
            _ => None,
        };
        inst_opt.map(|inst| RV32M(Instruction(Rtype {
            rd: rd(instruction_bits),
            rs1: rs1(instruction_bits),
            rs2: rs2(instruction_bits),
            inst,
        })))
    }
}
