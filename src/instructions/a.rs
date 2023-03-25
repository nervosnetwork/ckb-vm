use ckb_vm_definitions::instructions as insts;

use super::utils::{funct3, funct7, opcode, rd, rs1, rs2};
use super::{set_instruction_length_4, Instruction, Register, Rtype};

pub fn factory<R: Register>(instruction_bits: u32, _: u32) -> Option<Instruction> {
    let bit_length = R::BITS;
    if bit_length != 32 && bit_length != 64 {
        return None;
    }
    let rv64 = bit_length == 64;
    if opcode(instruction_bits) != 0b_0101111 {
        return None;
    }
    let f7 = funct7(instruction_bits);
    let f5 = f7 >> 2;
    let f3 = funct3(instruction_bits);
    let match_rv32 = || match (f3, f5) {
        (0b010, 0b00010) => {
            if rs2(instruction_bits) == 0 {
                Some(insts::OP_LR_W)
            } else {
                None
            }
        }
        (0b010, 0b00011) => Some(insts::OP_SC_W),
        (0b010, 0b00001) => Some(insts::OP_AMOSWAP_W),
        (0b010, 0b00000) => Some(insts::OP_AMOADD_W),
        (0b010, 0b00100) => Some(insts::OP_AMOXOR_W),
        (0b010, 0b01100) => Some(insts::OP_AMOAND_W),
        (0b010, 0b01000) => Some(insts::OP_AMOOR_W),
        (0b010, 0b10000) => Some(insts::OP_AMOMIN_W),
        (0b010, 0b10100) => Some(insts::OP_AMOMAX_W),
        (0b010, 0b11000) => Some(insts::OP_AMOMINU_W),
        (0b010, 0b11100) => Some(insts::OP_AMOMAXU_W),
        _ => None,
    };
    let match_rv64 = || match (f3, f5) {
        (0b011, 0b00010) => {
            if rs2(instruction_bits) == 0 {
                Some(insts::OP_LR_D)
            } else {
                None
            }
        }
        (0b011, 0b00011) => Some(insts::OP_SC_D),
        (0b011, 0b00001) => Some(insts::OP_AMOSWAP_D),
        (0b011, 0b00000) => Some(insts::OP_AMOADD_D),
        (0b011, 0b00100) => Some(insts::OP_AMOXOR_D),
        (0b011, 0b01100) => Some(insts::OP_AMOAND_D),
        (0b011, 0b01000) => Some(insts::OP_AMOOR_D),
        (0b011, 0b10000) => Some(insts::OP_AMOMIN_D),
        (0b011, 0b10100) => Some(insts::OP_AMOMAX_D),
        (0b011, 0b11000) => Some(insts::OP_AMOMINU_D),
        (0b011, 0b11100) => Some(insts::OP_AMOMAXU_D),
        _ => None,
    };
    let inst_opt = if rv64 {
        match_rv32().or_else(match_rv64)
    } else {
        match_rv32()
    };
    inst_opt
        .map(|inst| {
            Rtype::new(
                inst,
                rd(instruction_bits),
                rs1(instruction_bits),
                rs2(instruction_bits),
            )
            .0
        })
        .map(set_instruction_length_4)
}
