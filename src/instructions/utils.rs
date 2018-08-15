use super::super::machine::Machine;
use RISCV_GENERAL_REGISTER_NUMBER;

#[inline(always)]
pub fn extract_opcode(instruction: u32) -> u32 {
    instruction & 0x7F
}

#[inline(always)]
pub fn extract_rd(instruction: u32) -> usize {
    ((instruction >> 7) & 0x1F) as usize
}

#[inline(always)]
pub fn extract_utype_immediate(instruction: u32) -> u32 {
    instruction & 0xFFFF_F000
}

pub fn update_register(machine: &mut Machine, register_index: usize, value: u32) {
    let register_index = register_index % RISCV_GENERAL_REGISTER_NUMBER;
    if register_index > 0 {
        machine.registers[register_index] = value;
    }
}
