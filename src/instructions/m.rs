use super::super::machine::Machine;
use super::super::Error;
use super::register::Register;
use super::utils::{funct3, funct7, opcode, rd, rs1, rs2, update_register};
use super::{Instruction, Rtype, MODULE_M};
use crate::instructions as insts;

pub fn execute<Mac: Machine>(i: Instruction, machine: &mut Mac) -> Result<(), Error> {
    let i = Rtype(i);
    let op = i.op();
    let rs1_value = &machine.registers()[i.rs1() as usize];
    let rs2_value = &machine.registers()[i.rs2() as usize];
    match op {
        insts::OP_MUL => {
            let value = rs1_value.overflowing_mul(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MULW => {
            let value = rs1_value
                .zero_extend(&Mac::REG::from_usize(32))
                .overflowing_mul(&rs2_value.zero_extend(&Mac::REG::from_usize(32)));
            update_register(
                machine,
                i.rd(),
                value.sign_extend(&Mac::REG::from_usize(32)),
            );
        }
        insts::OP_MULH => {
            let value = rs1_value.overflowing_mul_high_signed(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MULHSU => {
            let value = rs1_value.overflowing_mul_high_signed_unsigned(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MULHU => {
            let value = rs1_value.overflowing_mul_high_unsigned(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_DIV => {
            let value = rs1_value.overflowing_div_signed(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_DIVW => {
            let rs1_value = rs1_value.sign_extend(&Mac::REG::from_usize(32));
            let rs2_value = rs2_value.sign_extend(&Mac::REG::from_usize(32));
            let value = rs1_value.overflowing_div_signed(&rs2_value);
            update_register(
                machine,
                i.rd(),
                value.sign_extend(&Mac::REG::from_usize(32)),
            );
        }
        insts::OP_DIVU => {
            let value = rs1_value.overflowing_div(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_DIVUW => {
            let rs1_value = rs1_value.zero_extend(&Mac::REG::from_usize(32));
            let rs2_value = rs2_value.zero_extend(&Mac::REG::from_usize(32));
            let value = rs1_value.overflowing_div(&rs2_value);
            update_register(
                machine,
                i.rd(),
                value.sign_extend(&Mac::REG::from_usize(32)),
            );
        }
        insts::OP_REM => {
            let value = rs1_value.overflowing_rem_signed(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_REMW => {
            let rs1_value = rs1_value.sign_extend(&Mac::REG::from_usize(32));
            let rs2_value = rs2_value.sign_extend(&Mac::REG::from_usize(32));
            let value = rs1_value.overflowing_rem_signed(&rs2_value);
            update_register(
                machine,
                i.rd(),
                value.sign_extend(&Mac::REG::from_usize(32)),
            );
        }
        insts::OP_REMU => {
            let value = rs1_value.overflowing_rem(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_REMUW => {
            let rs1_value = rs1_value.zero_extend(&Mac::REG::from_usize(32));
            let rs2_value = rs2_value.zero_extend(&Mac::REG::from_usize(32));
            let value = rs1_value.overflowing_rem(&rs2_value);
            update_register(
                machine,
                i.rd(),
                value.sign_extend(&Mac::REG::from_usize(32)),
            );
        }
        _ => return Err(Error::InvalidOp(op as u8)),
    };
    let next_pc = machine.pc().overflowing_add(&Mac::REG::from_usize(4));
    machine.set_pc(next_pc);
    Ok(())
}

pub fn factory<R: Register>(instruction_bits: u32) -> Option<Instruction> {
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
            0b_000 => Some(insts::OP_MUL),
            0b_001 => Some(insts::OP_MULH),
            0b_010 => Some(insts::OP_MULHSU),
            0b_011 => Some(insts::OP_MULHU),
            0b_100 => Some(insts::OP_DIV),
            0b_101 => Some(insts::OP_DIVU),
            0b_110 => Some(insts::OP_REM),
            0b_111 => Some(insts::OP_REMU),
            _ => None,
        },
        0b_0111011 if rv64 => match funct3(instruction_bits) {
            0b_000 => Some(insts::OP_MULW),
            0b_100 => Some(insts::OP_DIVW),
            0b_101 => Some(insts::OP_DIVUW),
            0b_110 => Some(insts::OP_REMW),
            0b_111 => Some(insts::OP_REMUW),
            _ => None,
        },
        _ => None,
    };
    inst_opt.map(|inst| {
        Rtype::assemble(
            inst,
            rd(instruction_bits),
            rs1(instruction_bits),
            rs2(instruction_bits),
            MODULE_M,
        )
        .0
    })
}
