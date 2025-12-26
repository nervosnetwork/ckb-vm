use super::utils::{funct3, funct7, opcode, rd, rs1, rs2};
use super::{Instruction, Register, set_instruction_length_2, set_instruction_length_4};
use crate::elf::CFI;
use crate::instructions::i::nop;
use crate::instructions::utils::lpad_4byte_aligned_mark;
use crate::instructions::{Itype, Rtype, Utype, extract_opcode, instruction_length};
use ckb_vm_definitions::instructions as insts;

pub fn may_be_operation(rd: usize) -> Instruction {
    Itype::new_u(insts::OP_ADDI, rd, 0, 0).0
}

pub fn factory<R: Register>(
    pc: u64,
    instruction_bits: u32,
    version: u32,
    cfi: CFI,
) -> Option<Instruction> {
    let inst = factory_bare::<R>(pc, instruction_bits, version, cfi);
    if let Some(i) = inst {
        let opcode = extract_opcode(i);
        let length = instruction_length(i);
        let rd = rd(instruction_bits);
        match opcode {
            insts::OP_LPAD => {
                if !cfi.allow_lpad() {
                    return Some(set_instruction_length_4(nop()));
                }
                return Some(i);
            }
            insts::OP_SSPUSH | insts::OP_SSPOPCHK | insts::OP_SSRDP => {
                if !cfi.ss {
                    let mop = may_be_operation(rd);
                    match length {
                        2 => return Some(set_instruction_length_2(mop)),
                        4 => return Some(set_instruction_length_4(mop)),
                        _ => return None, // Should not happen.
                    }
                }
                return Some(i);
            }
            insts::OP_SSAMOSWAP_W | insts::OP_SSAMOSWAP_D => {
                if !cfi.ss {
                    return None;
                }
                return Some(i);
            }
            _ => unreachable!(), // Should not happen.
        }
    }
    None
}

pub fn factory_bare<R: Register>(
    pc: u64,
    instruction_bits: u32,
    _: u32,
    _: CFI,
) -> Option<Instruction> {
    match opcode(instruction_bits) {
        0b_0010111 => {
            if rd(instruction_bits) == 0 {
                let inst = Utype::new(insts::OP_LPAD, 0, instruction_bits & 0xFFFFF000).0;
                let inst = set_instruction_length_4(inst);
                if pc % 4 == 0 {
                    return Some(lpad_4byte_aligned_mark(inst));
                }
                return Some(inst);
            }
        }
        0b_1110011 => {
            if instruction_bits == 0b_1100111_00001_00000_100_00000_1110011 {
                let inst = Rtype::new(insts::OP_SSPUSH, 0, 0, 1).0;
                let inst = set_instruction_length_4(inst);
                return Some(inst);
            }
            if instruction_bits == 0b_1100111_00101_00000_100_00000_1110011 {
                let inst = Rtype::new(insts::OP_SSPUSH, 0, 0, 5).0;
                let inst = set_instruction_length_4(inst);
                return Some(inst);
            }
            if instruction_bits == 0b_110011011100_00001_100_00000_1110011 {
                let inst = Itype::new_u(insts::OP_SSPOPCHK, 0, 1, 0).0;
                let inst = set_instruction_length_4(inst);
                return Some(inst);
            }
            if instruction_bits == 0b_110011011100_00101_100_00000_1110011 {
                let inst = Itype::new_u(insts::OP_SSPOPCHK, 0, 5, 0).0;
                let inst = set_instruction_length_4(inst);
                return Some(inst);
            }
            if instruction_bits & 0xFFFF_F000 == 0b_110011011100_00000_100_00000_0000000 {
                let rd = rd(instruction_bits);
                if rd == 0 {
                    return None;
                }
                let inst = Itype::new_u(insts::OP_SSRDP, rd, 0, 0).0;
                let inst = set_instruction_length_4(inst);
                return Some(inst);
            }
        }
        0b_0101111 => {
            let f7 = funct7(instruction_bits);
            let f5 = f7 >> 2;
            if f5 != 0b_01001 {
                return None;
            }
            let f3 = funct3(instruction_bits);
            if f3 == 0b_010 {
                let inst = Rtype::new(
                    insts::OP_SSAMOSWAP_W,
                    rd(instruction_bits),
                    rs1(instruction_bits),
                    rs2(instruction_bits),
                )
                .0;
                let inst = set_instruction_length_4(inst);
                return Some(inst);
            }
            // RV64 only.
            if f3 == 0b_011 && R::BITS == 64 {
                let inst = Rtype::new(
                    insts::OP_SSAMOSWAP_D,
                    rd(instruction_bits),
                    rs1(instruction_bits),
                    rs2(instruction_bits),
                )
                .0;
                let inst = set_instruction_length_4(inst);
                return Some(inst);
            }
            return None;
        }
        _ => {
            if instruction_bits == 0b_011_0_0_000_1_00000_01 {
                let inst = Rtype::new(insts::OP_SSPUSH, 0, 0, 1).0;
                let inst = set_instruction_length_2(inst);
                return Some(inst);
            }
            if instruction_bits == 0b_011_0_0_010_1_00000_01 {
                let inst = Itype::new_u(insts::OP_SSPOPCHK, 0, 5, 0).0;
                let inst = set_instruction_length_2(inst);
                return Some(inst);
            }
        }
    }
    None
}
