use super::utils::{opcode, rd};
use super::{Instruction, Register, set_instruction_length_4};
use crate::instructions::Utype;
use ckb_vm_definitions::instructions as insts;

pub fn factory<R: Register>(instruction_bits: u32, _: u32) -> Option<Instruction> {
    let inst = match opcode(instruction_bits) {
        0b_0010111 => {
            if rd(instruction_bits) == 0 {
                Some(Utype::new(insts::OP_LPAD, 0, instruction_bits & 0xFFFFF000).0)
            } else {
                None
            }
        }
        _ => None,
    };
    inst.map(set_instruction_length_4)
}
