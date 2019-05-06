use super::register::Register;
use super::utils::{funct3, funct7, opcode, rd, rs1, rs2};
use super::{Instruction, Rtype};
use ckb_vm_definitions::instructions as insts;

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
        Rtype::new(
            inst,
            rd(instruction_bits),
            rs1(instruction_bits),
            rs2(instruction_bits),
        )
        .0
    })
}
