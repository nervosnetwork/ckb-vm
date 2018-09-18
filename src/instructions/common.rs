use super::super::machine::Machine;
use super::super::memory::Memory;
use super::utils::update_register;
use super::{Error, Immediate, NextPC, RegisterIndex, UImmediate};

// Other instruction set functions common with RVC

// ======================
// #  ALU instructions  #
// ======================
pub fn add<Mac: Machine<u32, M>, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = machine.registers()[rs1];
    let rs2_value = machine.registers()[rs2];
    let (value, _) = rs1_value.overflowing_add(rs2_value);
    update_register(machine, rd, value);
}

pub fn sub<Mac: Machine<u32, M>, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = machine.registers()[rs1];
    let rs2_value = machine.registers()[rs2];
    let (value, _) = rs1_value.overflowing_sub(rs2_value);
    update_register(machine, rd, value);
}

pub fn addi<Mac: Machine<u32, M>, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: UImmediate,
) {
    let (value, _) = machine.registers()[rs1].overflowing_add(imm);
    update_register(machine, rd, value);
}

// =======================
// #  LOAD instructions  #
// =======================
pub fn lb<Mac: Machine<u32, M>, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: UImmediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers()[rs1].overflowing_add(imm);
    let value = machine.memory_mut().load8(address as usize)?;
    // sign-extened
    update_register(machine, rd, (value as i8) as u32);
    Ok(())
}

pub fn lh<Mac: Machine<u32, M>, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: UImmediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers()[rs1].overflowing_add(imm);
    let value = machine.memory_mut().load16(address as usize)?;
    // sign-extened
    update_register(machine, rd, (value as i16) as u32);
    Ok(())
}

pub fn lw<Mac: Machine<u32, M>, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: UImmediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers()[rs1].overflowing_add(imm);
    let value = machine.memory_mut().load32(address as usize)?;
    update_register(machine, rd, value);
    Ok(())
}

pub fn lbu<Mac: Machine<u32, M>, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: UImmediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers()[rs1].overflowing_add(imm);
    let value = machine.memory_mut().load8(address as usize)?;
    update_register(machine, rd, u32::from(value));
    Ok(())
}

pub fn lhu<Mac: Machine<u32, M>, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: UImmediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers()[rs1].overflowing_add(imm);
    let value = machine.memory_mut().load16(address as usize)?;
    update_register(machine, rd, u32::from(value));
    Ok(())
}

// ========================
// #  STORE instructions  #
// ========================
pub fn sb<Mac: Machine<u32, M>, M: Memory>(
    machine: &mut Mac,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
    imm: UImmediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers()[rs1].overflowing_add(imm);
    let value = machine.registers()[rs2] as u8;
    machine.memory_mut().store8(address as usize, value)?;
    Ok(())
}

pub fn sh<Mac: Machine<u32, M>, M: Memory>(
    machine: &mut Mac,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
    imm: UImmediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers()[rs1].overflowing_add(imm);
    let value = machine.registers()[rs2] as u16;
    machine.memory_mut().store16(address as usize, value)?;
    Ok(())
}

pub fn sw<Mac: Machine<u32, M>, M: Memory>(
    machine: &mut Mac,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
    imm: UImmediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers()[rs1].overflowing_add(imm);
    let value = machine.registers()[rs2];
    machine.memory_mut().store32(address as usize, value)?;
    Ok(())
}

// =========================
// #  BIT-OP instructions  #
// =========================
pub fn and<Mac: Machine<u32, M>, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = machine.registers()[rs1];
    let rs2_value = machine.registers()[rs2];
    let value = rs1_value & rs2_value;
    update_register(machine, rd, value);
}

pub fn xor<Mac: Machine<u32, M>, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = machine.registers()[rs1];
    let rs2_value = machine.registers()[rs2];
    let value = rs1_value ^ rs2_value;
    update_register(machine, rd, value);
}

pub fn or<Mac: Machine<u32, M>, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = machine.registers()[rs1];
    let rs2_value = machine.registers()[rs2];
    let value = rs1_value | rs2_value;
    update_register(machine, rd, value);
}

pub fn andi<Mac: Machine<u32, M>, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: UImmediate,
) {
    let value = machine.registers()[rs1] & imm;
    update_register(machine, rd, value);
}

pub fn xori<Mac: Machine<u32, M>, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: UImmediate,
) {
    let value = machine.registers()[rs1] ^ imm;
    update_register(machine, rd, value);
}

pub fn ori<Mac: Machine<u32, M>, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: UImmediate,
) {
    let value = machine.registers()[rs1] | imm;
    update_register(machine, rd, value);
}

pub fn slli<Mac: Machine<u32, M>, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = machine.registers()[rs1] << shamt;
    update_register(machine, rd, value);
}

pub fn srli<Mac: Machine<u32, M>, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = machine.registers()[rs1] >> shamt;
    update_register(machine, rd, value);
}

pub fn srai<Mac: Machine<u32, M>, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = (machine.registers()[rs1] as i32) >> shamt;
    update_register(machine, rd, value as u32);
}

// =======================
// #  JUMP instructions  #
// =======================
pub fn jal<Mac: Machine<u32, M>, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    imm: Immediate,
    xbytes: u32,
) -> Option<NextPC> {
    let link = machine.pc() + xbytes;
    update_register(machine, rd, link);
    Some(machine.pc().overflowing_add(imm as u32).0)
}
