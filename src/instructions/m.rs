use super::super::machine::Machine;
use super::super::Error;
use super::register::Register;
use super::utils::{funct3, funct7, opcode, rd, rs1, rs2, update_register};
use super::{Execute, Instruction as GenericInstruction, Instruction::M};

#[derive(Debug, Clone)]
pub enum RtypeInstruction {
    MUL,
    MULW,
    MULH,
    MULHSU,
    MULHU,
    DIV,
    DIVW,
    DIVU,
    DIVUW,
    REM,
    REMW,
    REMU,
    REMUW,
}

type Rtype = super::Rtype<RtypeInstruction>;

#[derive(Debug, Clone)]
pub struct Instruction(pub Rtype);

impl Execute for Rtype {
    fn execute<Mac: Machine>(&self, machine: &mut Mac) -> Result<Option<Mac::REG>, Error> {
        let rs1_value = &machine.registers()[self.rs1 as usize];
        let rs2_value = &machine.registers()[self.rs2 as usize];
        match &self.inst {
            RtypeInstruction::MUL => {
                let value = rs1_value.overflowing_mul(&rs2_value);
                update_register(machine, self.rd, value);
            }
            RtypeInstruction::MULW => {
                let value = rs1_value
                    .zero_extend(&Mac::REG::from_usize(32))
                    .overflowing_mul(&rs2_value.zero_extend(&Mac::REG::from_usize(32)));
                update_register(
                    machine,
                    self.rd,
                    value.sign_extend(&Mac::REG::from_usize(32)),
                );
            }
            RtypeInstruction::MULH => {
                let value = rs1_value.overflowing_mul_high_signed(&rs2_value);
                update_register(machine, self.rd, value);
            }
            RtypeInstruction::MULHSU => {
                let value = rs1_value.overflowing_mul_high_signed_unsigned(&rs2_value);
                update_register(machine, self.rd, value);
            }
            RtypeInstruction::MULHU => {
                let value = rs1_value.overflowing_mul_high_unsigned(&rs2_value);
                update_register(machine, self.rd, value);
            }
            RtypeInstruction::DIV => {
                let value = rs1_value.overflowing_div_signed(&rs2_value);
                update_register(machine, self.rd, value);
            }
            RtypeInstruction::DIVW => {
                let rs1_value = rs1_value.sign_extend(&Mac::REG::from_usize(32));
                let rs2_value = rs2_value.sign_extend(&Mac::REG::from_usize(32));
                let value = rs1_value.overflowing_div_signed(&rs2_value);
                update_register(
                    machine,
                    self.rd,
                    value.sign_extend(&Mac::REG::from_usize(32)),
                );
            }
            RtypeInstruction::DIVU => {
                let value = rs1_value.overflowing_div(&rs2_value);
                update_register(machine, self.rd, value);
            }
            RtypeInstruction::DIVUW => {
                let rs1_value = rs1_value.zero_extend(&Mac::REG::from_usize(32));
                let rs2_value = rs2_value.zero_extend(&Mac::REG::from_usize(32));
                let value = rs1_value.overflowing_div(&rs2_value);
                update_register(
                    machine,
                    self.rd,
                    value.sign_extend(&Mac::REG::from_usize(32)),
                );
            }
            RtypeInstruction::REM => {
                let value = rs1_value.overflowing_rem_signed(&rs2_value);
                update_register(machine, self.rd, value);
            }
            RtypeInstruction::REMW => {
                let rs1_value = rs1_value.sign_extend(&Mac::REG::from_usize(32));
                let rs2_value = rs2_value.sign_extend(&Mac::REG::from_usize(32));
                let value = rs1_value.overflowing_rem_signed(&rs2_value);
                update_register(
                    machine,
                    self.rd,
                    value.sign_extend(&Mac::REG::from_usize(32)),
                );
            }
            RtypeInstruction::REMU => {
                let value = rs1_value.overflowing_rem(&rs2_value);
                update_register(machine, self.rd, value);
            }
            RtypeInstruction::REMUW => {
                let rs1_value = rs1_value.zero_extend(&Mac::REG::from_usize(32));
                let rs2_value = rs2_value.zero_extend(&Mac::REG::from_usize(32));
                let value = rs1_value.overflowing_rem(&rs2_value);
                update_register(
                    machine,
                    self.rd,
                    value.sign_extend(&Mac::REG::from_usize(32)),
                );
            }
        }
        Ok(None)
    }
}

impl Instruction {
    pub fn execute<Mac: Machine>(&self, machine: &mut Mac) -> Result<(), Error> {
        let next_pc = self.0.execute(machine)?;
        let default_next_pc = machine.pc().overflowing_add(&Mac::REG::from_usize(4));
        machine.set_pc(next_pc.unwrap_or(default_next_pc));
        Ok(())
    }
}

pub fn factory<R: Register>(instruction_bits: u32) -> Option<GenericInstruction> {
    let bit_length = R::BITS;
    if bit_length != 32 && bit_length != 64 {
        return None;
    }
    let rv64 = bit_length == 64;
    if funct7(instruction_bits) != 0b_0000001 {
        return None;
    }
    let inst_opt = match opcode(instruction_bits) {
        0b_0110011 => match funct3(instruction_bits) {
            0b_000 => Some(RtypeInstruction::MUL),
            0b_001 => Some(RtypeInstruction::MULH),
            0b_010 => Some(RtypeInstruction::MULHSU),
            0b_011 => Some(RtypeInstruction::MULHU),
            0b_100 => Some(RtypeInstruction::DIV),
            0b_101 => Some(RtypeInstruction::DIVU),
            0b_110 => Some(RtypeInstruction::REM),
            0b_111 => Some(RtypeInstruction::REMU),
            _ => None,
        },
        0b_0111011 if rv64 => match funct3(instruction_bits) {
            0b_000 => Some(RtypeInstruction::MULW),
            0b_100 => Some(RtypeInstruction::DIVW),
            0b_101 => Some(RtypeInstruction::DIVUW),
            0b_110 => Some(RtypeInstruction::REMW),
            0b_111 => Some(RtypeInstruction::REMUW),
            _ => None,
        },
        _ => None,
    };
    inst_opt.map(|inst| {
        M(Instruction(Rtype {
            rd: rd(instruction_bits),
            rs1: rs1(instruction_bits),
            rs2: rs2(instruction_bits),
            inst,
        }))
    })
}
