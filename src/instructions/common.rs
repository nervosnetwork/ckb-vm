use super::super::machine::Machine;
use super::super::memory::Memory;
use super::super::RISCV_MAX_MEMORY;
use super::register::Register;
use super::utils::update_register;
use super::{Error, RegisterIndex, SImmediate, UImmediate};

#[cfg(feature = "probes")]
use crate::probe::probe_jump;

// Other instruction set functions common with RVC

// ======================
// #  ALU instructions  #
// ======================
pub fn add<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = &machine.registers()[rs1 as usize];
    let rs2_value = &machine.registers()[rs2 as usize];
    let value = rs1_value.overflowing_add(rs2_value);
    update_register(machine, rd, value);
}

pub fn addw<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = &machine.registers()[rs1 as usize];
    let rs2_value = &machine.registers()[rs2 as usize];
    let value = rs1_value.overflowing_add(rs2_value);
    update_register(machine, rd, value.sign_extend(&Mac::REG::from_u8(32)));
}

pub fn sub<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = &machine.registers()[rs1 as usize];
    let rs2_value = &machine.registers()[rs2 as usize];
    let value = rs1_value.overflowing_sub(rs2_value);
    update_register(machine, rd, value);
}

pub fn subw<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = &machine.registers()[rs1 as usize];
    let rs2_value = &machine.registers()[rs2 as usize];
    let value = rs1_value.overflowing_sub(rs2_value);
    update_register(machine, rd, value.sign_extend(&Mac::REG::from_u8(32)));
}

pub fn addi<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: SImmediate,
) {
    let value = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    update_register(machine, rd, value);
}

pub fn addiw<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: SImmediate,
) {
    let value = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    update_register(machine, rd, value.sign_extend(&Mac::REG::from_u8(32)));
}

// =======================
// #  LOAD instructions  #
// =======================
fn check_load_boundary<R: Register>(version0: bool, address: &R, bytes: u64) -> Result<(), Error> {
    if version0 {
        let address = address.to_u64();
        let end = address.checked_add(bytes).ok_or(Error::MemOutOfBound)?;
        if end == RISCV_MAX_MEMORY as u64 {
            return Err(Error::MemOutOfBound);
        }
    }
    Ok(())
}

pub fn lb<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: SImmediate,
    version0: bool,
) -> Result<(), Error> {
    let address = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    check_load_boundary(version0, &address, 1)?;
    let value = machine.memory_mut().load8(&address)?;
    // sign-extened
    update_register(machine, rd, value.sign_extend(&Mac::REG::from_u8(8)));
    Ok(())
}

pub fn lh<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: SImmediate,
    version0: bool,
) -> Result<(), Error> {
    let address = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    check_load_boundary(version0, &address, 2)?;
    let value = machine.memory_mut().load16(&address)?;
    // sign-extened
    update_register(machine, rd, value.sign_extend(&Mac::REG::from_u8(16)));
    Ok(())
}

pub fn lw<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: SImmediate,
    version0: bool,
) -> Result<(), Error> {
    let address = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    check_load_boundary(version0, &address, 4)?;
    let value = machine.memory_mut().load32(&address)?;
    update_register(machine, rd, value.sign_extend(&Mac::REG::from_u8(32)));
    Ok(())
}

pub fn ld<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: SImmediate,
    version0: bool,
) -> Result<(), Error> {
    let address = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    check_load_boundary(version0, &address, 8)?;
    let value = machine.memory_mut().load64(&address)?;
    update_register(machine, rd, value.sign_extend(&Mac::REG::from_u8(64)));
    Ok(())
}

pub fn lbu<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: SImmediate,
    version0: bool,
) -> Result<(), Error> {
    let address = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    check_load_boundary(version0, &address, 1)?;
    let value = machine.memory_mut().load8(&address)?;
    update_register(machine, rd, value);
    Ok(())
}

pub fn lhu<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: SImmediate,
    version0: bool,
) -> Result<(), Error> {
    let address = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    check_load_boundary(version0, &address, 2)?;
    let value = machine.memory_mut().load16(&address)?;
    update_register(machine, rd, value);
    Ok(())
}

pub fn lwu<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: SImmediate,
    version0: bool,
) -> Result<(), Error> {
    let address = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    check_load_boundary(version0, &address, 4)?;
    let value = machine.memory_mut().load32(&address)?;
    update_register(machine, rd, value);
    Ok(())
}

// ========================
// #  STORE instructions  #
// ========================
pub fn sb<Mac: Machine>(
    machine: &mut Mac,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
    imm: SImmediate,
) -> Result<(), Error> {
    let address = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    let value = machine.registers()[rs2 as usize].clone();
    machine.memory_mut().store8(&address, &value)?;
    Ok(())
}

pub fn sh<Mac: Machine>(
    machine: &mut Mac,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
    imm: SImmediate,
) -> Result<(), Error> {
    let address = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    let value = machine.registers()[rs2 as usize].clone();
    machine.memory_mut().store16(&address, &value)?;
    Ok(())
}

pub fn sw<Mac: Machine>(
    machine: &mut Mac,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
    imm: SImmediate,
) -> Result<(), Error> {
    let address = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    let value = machine.registers()[rs2 as usize].clone();
    machine.memory_mut().store32(&address, &value)?;
    Ok(())
}

pub fn sd<Mac: Machine>(
    machine: &mut Mac,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
    imm: SImmediate,
) -> Result<(), Error> {
    let address = machine.registers()[rs1 as usize].overflowing_add(&Mac::REG::from_i32(imm));
    let value = machine.registers()[rs2 as usize].clone();
    machine.memory_mut().store64(&address, &value)?;
    Ok(())
}

// =========================
// #  BIT-OP instructions  #
// =========================
pub fn and<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = machine.registers()[rs1 as usize].clone();
    let rs2_value = machine.registers()[rs2 as usize].clone();
    let value = rs1_value & rs2_value;
    update_register(machine, rd, value);
}

pub fn xor<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = machine.registers()[rs1 as usize].clone();
    let rs2_value = machine.registers()[rs2 as usize].clone();
    let value = rs1_value ^ rs2_value;
    update_register(machine, rd, value);
}

pub fn or<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    rs2: RegisterIndex,
) {
    let rs1_value = machine.registers()[rs1 as usize].clone();
    let rs2_value = machine.registers()[rs2 as usize].clone();
    let value = rs1_value | rs2_value;
    update_register(machine, rd, value);
}

pub fn andi<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: SImmediate,
) {
    let value = machine.registers()[rs1 as usize].clone() & Mac::REG::from_i32(imm);
    update_register(machine, rd, value);
}

pub fn xori<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: SImmediate,
) {
    let value = machine.registers()[rs1 as usize].clone() ^ Mac::REG::from_i32(imm);
    update_register(machine, rd, value);
}

pub fn ori<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    imm: SImmediate,
) {
    let value = machine.registers()[rs1 as usize].clone() | Mac::REG::from_i32(imm);
    update_register(machine, rd, value);
}

pub fn slli<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = machine.registers()[rs1 as usize].clone() << Mac::REG::from_u32(shamt);
    update_register(machine, rd, value);
}

pub fn srli<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = machine.registers()[rs1 as usize].clone() >> Mac::REG::from_u32(shamt);
    update_register(machine, rd, value);
}

pub fn srai<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = machine.registers()[rs1 as usize].signed_shr(&Mac::REG::from_u32(shamt));
    update_register(machine, rd, value);
}

pub fn slliw<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = machine.registers()[rs1 as usize].clone() << Mac::REG::from_u32(shamt);
    update_register(machine, rd, value.sign_extend(&Mac::REG::from_u8(32)));
}

pub fn srliw<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = machine.registers()[rs1 as usize].zero_extend(&Mac::REG::from_u8(32))
        >> Mac::REG::from_u32(shamt);
    update_register(machine, rd, value.sign_extend(&Mac::REG::from_u8(32)));
}

pub fn sraiw<Mac: Machine>(
    machine: &mut Mac,
    rd: RegisterIndex,
    rs1: RegisterIndex,
    shamt: UImmediate,
) {
    let value = machine.registers()[rs1 as usize]
        .sign_extend(&Mac::REG::from_u8(32))
        .signed_shr(&Mac::REG::from_u32(shamt));
    update_register(machine, rd, value.sign_extend(&Mac::REG::from_u8(32)));
}

// =======================
// #  JUMP instructions  #
// =======================
pub fn jal<Mac: Machine>(machine: &mut Mac, rd: RegisterIndex, imm: SImmediate, xbytes: u8) {
    let link = machine.pc().overflowing_add(&Mac::REG::from_u8(xbytes));
    update_register(machine, rd, link.clone());
    let next_pc = machine.pc().overflowing_add(&Mac::REG::from_i32(imm));
    #[cfg(feature = "probes")]
    probe_jump(machine, link.clone(), next_pc.clone());
    machine.update_pc(next_pc);
}
