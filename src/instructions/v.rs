use ckb_vm_definitions::instructions as insts;

use super::{
    Instruction, Itype, Register, Rtype, set_instruction_length_4,
    utils::{funct3, opcode, rd, rs1, rs2},
};

const OP_V_OPCODE: u32 = 0b101_0111;
const FUNCT3_VSETVLI: u32 = 0b111;
const FUNCT3_VV: u32 = 0b000;

pub fn factory<R: Register>(instruction_bits: u32, _: u32) -> Option<Instruction> {
    if R::BITS != 64 {
        return None;
    }
    if opcode(instruction_bits) != OP_V_OPCODE {
        return None;
    }
    let inst = match funct3(instruction_bits) {
        FUNCT3_VSETVLI => decode_vsetvli(instruction_bits),
        FUNCT3_VV => decode_vadd_vv(instruction_bits),
        _ => None,
    }?;
    Some(set_instruction_length_4(inst))
}

fn decode_vsetvli(instruction_bits: u32) -> Option<Instruction> {
    if funct6(instruction_bits) != 0 || vm_bit(instruction_bits) != 0 {
        return None;
    }
    let zimm = vtype_zimm(instruction_bits);
    if !supports_vtype(zimm) {
        return None;
    }
    Some(
        Itype::new_u(
            insts::OP_VSETVLI,
            rd(instruction_bits),
            rs1(instruction_bits),
            zimm,
        )
        .0,
    )
}

fn decode_vadd_vv(instruction_bits: u32) -> Option<Instruction> {
    if funct6(instruction_bits) != 0 || vm_bit(instruction_bits) == 0 {
        return None;
    }
    Some(
        Rtype::new(
            insts::OP_VADD_VV,
            rd(instruction_bits),
            rs1(instruction_bits),
            rs2(instruction_bits),
        )
        .0,
    )
}

fn vtype_zimm(instruction_bits: u32) -> u32 {
    (instruction_bits >> 20) & 0x7ff
}

fn vm_bit(instruction_bits: u32) -> u32 {
    (instruction_bits >> 25) & 0x1
}

fn funct6(instruction_bits: u32) -> u32 {
    (instruction_bits >> 26) & 0x3f
}

fn supports_vtype(zimm: u32) -> bool {
    let vsew = zimm & 0x7;
    let vlmul = (zimm >> 3) & 0x7;
    vsew == 3 && vlmul == 0
}
