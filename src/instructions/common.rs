use super::super::machine::Machine;
use super::super::memory::Memory;
use super::utils::{update_register};
use super::{RegisterIndex, UImmediate, Error};


// Other instruction set functions common with RVC

// ======================
// #  ALU instructions  #
// ======================
pub fn add<M: Memory>(
    machine: &mut Machine<M>,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = machine.registers[rs1];
    let rs2_value = machine.registers[rs2];
    let (value, _) = rs1_value.overflowing_add(rs2_value);
    update_register(machine, rd, value);
}

pub fn sub<M: Memory>(
    machine: &mut Machine<M>,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = machine.registers[rs1];
    let rs2_value = machine.registers[rs2];
    let (value, _) = rs1_value.overflowing_sub(rs2_value);
    update_register(machine, rd, value);
}

pub fn addi<M: Memory>(
    machine: &mut Machine<M>,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: UImmediate,
) {
    let (value, _) = machine.registers[rs1].overflowing_add(imm);
    update_register(machine, rd, value);
}

// =======================
// #  LOAD instructions  #
// =======================
pub fn lb<M: Memory>(
    machine: &mut Machine<M>,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: UImmediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers[rs1].overflowing_add(imm);
    let value = machine.memory.load8(address as usize)?;
    // sign-extened
    update_register(machine, rd, (value as i8) as u32);
    Ok(())
}

pub fn lh<M: Memory>(
    machine: &mut Machine<M>,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: UImmediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers[rs1].overflowing_add(imm);
    let value = machine.memory.load16(address as usize)?;
    // sign-extened
    update_register(machine, rd, (value as i16) as u32);
    Ok(())
}

pub fn lw<M: Memory>(
    machine: &mut Machine<M>,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: UImmediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers[rs1].overflowing_add(imm);
    let value = machine.memory.load32(address as usize)?;
    update_register(machine, rd, value);
    Ok(())
}

pub fn lbu<M: Memory>(
    machine: &mut Machine<M>,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: UImmediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers[rs1].overflowing_add(imm);
    let value = machine.memory.load8(address as usize)?;
    update_register(machine, rd, value as u32);
    Ok(())
}

pub fn lhu<M: Memory>(
    machine: &mut Machine<M>,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: UImmediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers[rs1].overflowing_add(imm);
    let value = machine.memory.load16(address as usize)?;
    update_register(machine, rd, value as u32);
    Ok(())
}

// ========================
// #  STORE instructions  #
// ========================
pub fn sb<M: Memory>(
    machine: &mut Machine<M>,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
    imm: UImmediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers[rs1].overflowing_add(imm);
    let value = machine.registers[rs2] as u8;
    machine.memory.store8(address as usize, value)?;
    Ok(())
}

pub fn sh<M: Memory>(
    machine: &mut Machine<M>,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
    imm: UImmediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers[rs1].overflowing_add(imm);
    let value = machine.registers[rs2] as u16;
    machine.memory.store16(address as usize, value)?;
    Ok(())
}

pub fn sw<M: Memory>(
    machine: &mut Machine<M>,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
    imm: UImmediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers[rs1].overflowing_add(imm);
    let value = machine.registers[rs2];
    machine.memory.store32(address as usize, value)?;
    Ok(())
}

// =========================
// #  BIT-OP instructions  #
// =========================
pub fn and<M: Memory>(
    machine: &mut Machine<M>,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = machine.registers[rs1];
    let rs2_value = machine.registers[rs2];
    let value = rs1_value & rs2_value;
    update_register(machine, rd, value);
}

pub fn xor<M: Memory>(
    machine: &mut Machine<M>,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = machine.registers[rs1];
    let rs2_value = machine.registers[rs2];
    let value = rs1_value ^ rs2_value;
    update_register(machine, rd, value);
}

pub fn or<M: Memory>(
    machine: &mut Machine<M>,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = machine.registers[rs1];
    let rs2_value = machine.registers[rs2];
    let value = rs1_value | rs2_value;
    update_register(machine, rd, value);
}

pub fn andi<M: Memory>(
    machine: &mut Machine<M>,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: UImmediate,
) {
    let value = machine.registers[rs1] & imm;
    update_register(machine, rd, value);
}

pub fn xori<M: Memory>(
    machine: &mut Machine<M>,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: UImmediate,
) {
    let value = machine.registers[rs1] ^ imm;
    update_register(machine, rd, value);
}

pub fn ori<M: Memory>(
    machine: &mut Machine<M>,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: UImmediate,
) {
    let value = machine.registers[rs1] | imm;
    update_register(machine, rd, value);
}

pub fn slli<M: Memory>(
    machine: &mut Machine<M>,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = machine.registers[rs1] << shamt;
    update_register(machine, rd, value);
}

pub fn srli<M: Memory>(
    machine: &mut Machine<M>,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = machine.registers[rs1] >> shamt;
    update_register(machine, rd, value);
}

pub fn srai<M: Memory>(
    machine: &mut Machine<M>,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = (machine.registers[rs1] as i32) >> shamt;
    update_register(machine, rd, value as u32);
}
