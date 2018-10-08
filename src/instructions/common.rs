use super::super::machine::Machine;
use super::super::memory::Memory;
use super::register::Register;
use super::utils::update_register;
use super::{Error, Immediate, RegisterIndex, UImmediate};

// Other instruction set functions common with RVC

// ======================
// #  ALU instructions  #
// ======================
pub fn add<Mac: Machine<R, M>, R: Register, M: Memory>(
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

pub fn addw<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = machine.registers()[rs1];
    let rs2_value = machine.registers()[rs2];
    let (value, _) = rs1_value.overflowing_add(rs2_value);
    update_register(machine, rd, value.sign_extend(32));
}

pub fn sub<Mac: Machine<R, M>, R: Register, M: Memory>(
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

pub fn subw<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = machine.registers()[rs1];
    let rs2_value = machine.registers()[rs2];
    let (value, _) = rs1_value.overflowing_sub(rs2_value);
    update_register(machine, rd, value.sign_extend(32));
}

pub fn addi<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
) {
    let (value, _) = machine.registers()[rs1].overflowing_add(R::from_i32(imm));
    update_register(machine, rd, value);
}

pub fn addiw<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
) {
    let (value, _) = machine.registers()[rs1].overflowing_add(R::from_i32(imm));
    update_register(machine, rd, value.sign_extend(32));
}

// =======================
// #  LOAD instructions  #
// =======================
pub fn lb<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers()[rs1].overflowing_add(R::from_i32(imm));
    let value = machine.memory_mut().load8(address.to_usize())?;
    // sign-extened
    update_register(machine, rd, R::from_i8(value as i8));
    Ok(())
}

pub fn lh<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers()[rs1].overflowing_add(R::from_i32(imm));
    let value = machine.memory_mut().load16(address.to_usize())?;
    // sign-extened
    update_register(machine, rd, R::from_i16(value as i16));
    Ok(())
}

pub fn lw<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers()[rs1].overflowing_add(R::from_i32(imm));
    let value = machine.memory_mut().load32(address.to_usize())?;
    update_register(machine, rd, R::from_i32(value as i32));
    Ok(())
}

pub fn ld<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers()[rs1].overflowing_add(R::from_i32(imm));
    let value = machine.memory_mut().load64(address.to_usize())?;
    update_register(machine, rd, R::from_i64(value as i64));
    Ok(())
}

pub fn lbu<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers()[rs1].overflowing_add(R::from_i32(imm));
    let value = machine.memory_mut().load8(address.to_usize())?;
    update_register(machine, rd, R::from_u8(value));
    Ok(())
}

pub fn lhu<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers()[rs1].overflowing_add(R::from_i32(imm));
    let value = machine.memory_mut().load16(address.to_usize())?;
    update_register(machine, rd, R::from_u16(value));
    Ok(())
}

pub fn lwu<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers()[rs1].overflowing_add(R::from_i32(imm));
    let value = machine.memory_mut().load32(address.to_usize())?;
    update_register(machine, rd, R::from_u32(value));
    Ok(())
}

// ========================
// #  STORE instructions  #
// ========================
pub fn sb<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
    imm: Immediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers()[rs1].overflowing_add(R::from_i32(imm));
    let value = machine.registers()[rs2].to_u8();
    machine.memory_mut().store8(address.to_usize(), value)?;
    Ok(())
}

pub fn sh<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
    imm: Immediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers()[rs1].overflowing_add(R::from_i32(imm));
    let value = machine.registers()[rs2].to_u16();
    machine.memory_mut().store16(address.to_usize(), value)?;
    Ok(())
}

pub fn sw<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
    imm: Immediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers()[rs1].overflowing_add(R::from_i32(imm));
    let value = machine.registers()[rs2].to_u32();
    machine.memory_mut().store32(address.to_usize(), value)?;
    Ok(())
}

pub fn sd<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
    imm: Immediate,
) -> Result<(), Error> {
    let (address, _) = machine.registers()[rs1].overflowing_add(R::from_i32(imm));
    let value = machine.registers()[rs2].to_u64();
    machine.memory_mut().store64(address.to_usize(), value)?;
    Ok(())
}

// =========================
// #  BIT-OP instructions  #
// =========================
pub fn and<Mac: Machine<R, M>, R: Register, M: Memory>(
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

pub fn xor<Mac: Machine<R, M>, R: Register, M: Memory>(
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

pub fn or<Mac: Machine<R, M>, R: Register, M: Memory>(
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

pub fn andi<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
) {
    let value = machine.registers()[rs1] & R::from_i32(imm);
    update_register(machine, rd, value);
}

pub fn xori<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
) {
    let value = machine.registers()[rs1] ^ R::from_i32(imm);
    update_register(machine, rd, value);
}

pub fn ori<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
) {
    let value = machine.registers()[rs1] | R::from_i32(imm);
    update_register(machine, rd, value);
}

pub fn slli<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = machine.registers()[rs1] << (shamt as usize);
    update_register(machine, rd, value);
}

pub fn srli<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = machine.registers()[rs1] >> (shamt as usize);
    update_register(machine, rd, value);
}

pub fn srai<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = machine.registers()[rs1].signed_shr(shamt as usize);
    update_register(machine, rd, value);
}

pub fn slliw<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = machine.registers()[rs1] << (shamt as usize);
    update_register(machine, rd, value.sign_extend(32));
}

pub fn srliw<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = machine.registers()[rs1].zero_extend(32) >> (shamt as usize);
    update_register(machine, rd, value.sign_extend(32));
}

pub fn sraiw<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = machine.registers()[rs1]
        .sign_extend(32)
        .signed_shr(shamt as usize);
    update_register(machine, rd, value.sign_extend(32));
}

// =======================
// #  JUMP instructions  #
// =======================
pub fn jal<Mac: Machine<R, M>, R: Register, M: Memory>(
    machine: &mut Mac,
    rd: RegisterIndex,
    imm: Immediate,
    xbytes: usize,
) -> Option<R> {
    let (link, _) = machine.pc().overflowing_add(R::from_usize(xbytes));
    update_register(machine, rd, link);
    Some(machine.pc().overflowing_add(R::from_i32(imm)).0)
}
