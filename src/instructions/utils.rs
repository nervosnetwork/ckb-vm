use super::super::machine::Machine;
use crate::RISCV_GENERAL_REGISTER_NUMBER;

// Inspired from https://github.com/riscv/riscv-isa-sim/blob/master/riscv/decode.h#L105-L106
#[inline(always)]
pub fn x(instruction_bits: u32, lower: usize, length: usize, shifts: usize) -> u32 {
    ((instruction_bits >> lower) & ((1 << length) - 1)) << shifts
}

#[inline(always)]
pub fn xs(instruction_bits: u32, lower: usize, length: usize, shifts: usize) -> u32 {
    ((instruction_bits as i32) << (32 - lower - length) >> (32 - length) << shifts) as u32
}

#[inline(always)]
pub fn opcode(instruction_bits: u32) -> u32 {
    x(instruction_bits, 0, 7, 0)
}

#[inline(always)]
pub fn funct3(instruction_bits: u32) -> u32 {
    x(instruction_bits, 12, 3, 0)
}

#[inline(always)]
pub fn funct7(instruction_bits: u32) -> u32 {
    x(instruction_bits, 25, 7, 0)
}

#[inline(always)]
pub fn rd(instruction_bits: u32) -> usize {
    x(instruction_bits, 7, 5, 0) as usize
}

#[inline(always)]
pub fn rs1(instruction_bits: u32) -> usize {
    x(instruction_bits, 15, 5, 0) as usize
}

#[inline(always)]
pub fn rs2(instruction_bits: u32) -> usize {
    x(instruction_bits, 20, 5, 0) as usize
}

#[inline(always)]
pub fn rs3(instruction_bits: u32) -> usize {
    x(instruction_bits, 27, 5, 0) as usize
}

#[inline(always)]
pub fn btype_immediate(instruction_bits: u32) -> i32 {
    (x(instruction_bits, 8, 4, 1)
        | x(instruction_bits, 25, 6, 5)
        | x(instruction_bits, 7, 1, 11)
        | xs(instruction_bits, 31, 1, 12)) as i32
}

#[inline(always)]
pub fn jtype_immediate(instruction_bits: u32) -> i32 {
    (x(instruction_bits, 21, 10, 1)
        | x(instruction_bits, 20, 1, 11)
        | x(instruction_bits, 12, 8, 12)
        | xs(instruction_bits, 31, 1, 20)) as i32
}

#[inline(always)]
pub fn itype_immediate(instruction_bits: u32) -> i32 {
    xs(instruction_bits, 20, 12, 0) as i32
}

#[inline(always)]
pub fn stype_immediate(instruction_bits: u32) -> i32 {
    (x(instruction_bits, 7, 5, 0) | xs(instruction_bits, 25, 7, 5)) as i32
}

#[inline(always)]
pub fn utype_immediate(instruction_bits: u32) -> i32 {
    xs(instruction_bits, 12, 20, 12) as i32
}

pub fn update_register<M: Machine>(machine: &mut M, register_index: usize, value: M::REG) {
    let register_index = register_index % RISCV_GENERAL_REGISTER_NUMBER;
    // In RISC-V, x0 is a special zero register with the following properties:
    //
    // * All writes to this register are silently ignored
    // * All reads from this register will respond with 0
    //
    // The goal here is to maintain a place where we can read zeros to allow for
    // compact encoding. Hence we are ignoring all writes to x0 register here.
    if register_index > 0 {
        machine.set_register(register_index, value);
    }
}
