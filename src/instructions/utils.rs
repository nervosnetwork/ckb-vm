use super::super::machine::Machine;
use super::super::memory::Memory;
use RISCV_GENERAL_REGISTER_NUMBER;

#[inline(always)]
pub fn extract_opcode(instruction: u32) -> u32 {
    instruction & 0x7F
}

#[inline(always)]
pub fn extract_funct3(instruction: u32) -> u32 {
    ((instruction >> 12) & 0x7)
}

#[inline(always)]
pub fn extract_rd(instruction: u32) -> usize {
    ((instruction >> 7) & 0x1F) as usize
}

#[inline(always)]
pub fn extract_rs1(instruction: u32) -> usize {
    ((instruction >> 15) & 0x1F) as usize
}

#[inline(always)]
pub fn extract_rs2(instruction: u32) -> usize {
    ((instruction >> 20) & 0x1F) as usize
}

#[inline(always)]
pub fn extract_itype_immediate(instruction: u32) -> i32 {
    ((instruction & 0xFFF0_0000) as i32) >> 20
}

#[inline(always)]
pub fn extract_utype_immediate(instruction: u32) -> i32 {
    (instruction & 0xFFFF_F000) as i32
}

pub fn update_register<M: Memory>(machine: &mut Machine<M>, register_index: usize, value: u32) {
    let register_index = register_index % RISCV_GENERAL_REGISTER_NUMBER;
    // In RISC-V, x0 is a special zero register with the following properties:
    //
    // * All writes to this register are silently ignored
    // * All reads from this register will respond with 0
    //
    // The goal here is to maintain a place where we can read zeros to allow for
    // compact encoding. Hence we are ignoring all writes to x0 register here.
    if register_index > 0 {
        machine.registers[register_index] = value;
    }
}
