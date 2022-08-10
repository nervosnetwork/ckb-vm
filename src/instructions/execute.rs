use super::{
    super::{machine::Machine, Error},
    common, extract_opcode, instruction_length,
    utils::update_register,
    Instruction, Itype, R4type, Register, Rtype, Stype, Utype, VItype, VVtype, VXtype,
};
use crate::instructions::v_alu as alu;
use crate::instructions::v_execute_macros::*;
use crate::memory::Memory;
use ckb_vm_definitions::{instructions as insts, registers::RA, VLEN};
pub use eint::{Eint, E1024, E128, E16, E2048, E256, E32, E512, E64, E8};

pub fn handle_unloaded<Mac: Machine>(_: &mut Mac, inst: Instruction) -> Result<(), Error> {
    Err(Error::InvalidOp(extract_opcode(inst)))
}

pub fn handle_add<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1() as usize];
    let rs2_value = &machine.registers()[i.rs2() as usize];
    let value = rs1_value.overflowing_add(rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_addi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let value =
        machine.registers()[i.rs1() as usize].overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_addiw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let value =
        machine.registers()[i.rs1() as usize].overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
    update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
    Ok(())
}

pub fn handle_addw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1() as usize];
    let rs2_value = &machine.registers()[i.rs2() as usize];
    let value = rs1_value.overflowing_add(rs2_value);
    update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
    Ok(())
}

pub fn handle_and<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = machine.registers()[i.rs1() as usize].clone();
    let rs2_value = machine.registers()[i.rs2() as usize].clone();
    let value = rs1_value & rs2_value;
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_andi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let value = machine.registers()[i.rs1() as usize].clone() & Mac::REG::from_i32(i.immediate_s());
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_div<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.overflowing_div_signed(&rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_divu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.overflowing_div(&rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_divuw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let rs1_value = rs1_value.zero_extend(&Mac::REG::from_u8(32));
    let rs2_value = rs2_value.zero_extend(&Mac::REG::from_u8(32));
    let value = rs1_value.overflowing_div(&rs2_value);
    update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
    Ok(())
}

pub fn handle_divw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let rs1_value = rs1_value.sign_extend(&Mac::REG::from_u8(32));
    let rs2_value = rs2_value.sign_extend(&Mac::REG::from_u8(32));
    let value = rs1_value.overflowing_div_signed(&rs2_value);
    update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
    Ok(())
}

pub fn handle_fence<Mac: Machine>(_: &mut Mac, _: Instruction) -> Result<(), Error> {
    Ok(())
}

pub fn handle_fencei<Mac: Machine>(_: &mut Mac, _: Instruction) -> Result<(), Error> {
    Ok(())
}

pub fn handle_lb<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let address =
        machine.registers()[i.rs1() as usize].overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
    common::check_load_boundary(machine.version() == 0, &address, 1)?;
    let value = machine.memory_mut().load8(&address)?;
    // sign-extened
    update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(8)));
    Ok(())
}

pub fn handle_lbu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let address =
        machine.registers()[i.rs1() as usize].overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
    common::check_load_boundary(machine.version() == 0, &address, 1)?;
    let value = machine.memory_mut().load8(&address)?;
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_ld<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let address =
        machine.registers()[i.rs1() as usize].overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
    common::check_load_boundary(machine.version() == 0, &address, 8)?;
    let value = machine.memory_mut().load64(&address)?;
    update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(64)));
    Ok(())
}

pub fn handle_lh<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let address =
        machine.registers()[i.rs1() as usize].overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
    common::check_load_boundary(machine.version() == 0, &address, 2)?;
    let value = machine.memory_mut().load16(&address)?;
    // sign-extened
    update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(16)));
    Ok(())
}

pub fn handle_lhu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let address =
        machine.registers()[i.rs1() as usize].overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
    common::check_load_boundary(machine.version() == 0, &address, 2)?;
    let value = machine.memory_mut().load16(&address)?;
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_lui<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Utype(inst);
    update_register(machine, i.rd(), Mac::REG::from_i32(i.immediate_s()));
    Ok(())
}

pub fn handle_lw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let address =
        machine.registers()[i.rs1() as usize].overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
    common::check_load_boundary(machine.version() == 0, &address, 4)?;
    let value = machine.memory_mut().load32(&address)?;
    update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
    Ok(())
}

pub fn handle_lwu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let address =
        machine.registers()[i.rs1() as usize].overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
    common::check_load_boundary(machine.version() == 0, &address, 4)?;
    let value = machine.memory_mut().load32(&address)?;
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_mul<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.overflowing_mul(&rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_mulh<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.overflowing_mul_high_signed(&rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_mulhsu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.overflowing_mul_high_signed_unsigned(&rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_mulhu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.overflowing_mul_high_unsigned(&rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_mulw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value
        .zero_extend(&Mac::REG::from_u8(32))
        .overflowing_mul(&rs2_value.zero_extend(&Mac::REG::from_u8(32)));
    update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
    Ok(())
}

pub fn handle_or<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = machine.registers()[i.rs1() as usize].clone();
    let rs2_value = machine.registers()[i.rs2() as usize].clone();
    let value = rs1_value | rs2_value;
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_ori<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let value = machine.registers()[i.rs1() as usize].clone() | Mac::REG::from_i32(i.immediate_s());
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_rem<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.overflowing_rem_signed(&rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_remu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.overflowing_rem(&rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_remuw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let rs1_value = rs1_value.zero_extend(&Mac::REG::from_u8(32));
    let rs2_value = rs2_value.zero_extend(&Mac::REG::from_u8(32));
    let value = rs1_value.overflowing_rem(&rs2_value);
    update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
    Ok(())
}

pub fn handle_remw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let rs1_value = rs1_value.sign_extend(&Mac::REG::from_u8(32));
    let rs2_value = rs2_value.sign_extend(&Mac::REG::from_u8(32));
    let value = rs1_value.overflowing_rem_signed(&rs2_value);
    update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
    Ok(())
}

pub fn handle_sb<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Stype(inst);
    let address =
        machine.registers()[i.rs1() as usize].overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
    let value = machine.registers()[i.rs2() as usize].clone();
    machine.memory_mut().store8(&address, &value)?;
    Ok(())
}

pub fn handle_sd<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Stype(inst);
    let address =
        machine.registers()[i.rs1() as usize].overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
    let value = machine.registers()[i.rs2() as usize].clone();
    machine.memory_mut().store64(&address, &value)?;
    Ok(())
}

pub fn handle_sh<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Stype(inst);
    let address =
        machine.registers()[i.rs1() as usize].overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
    let value = machine.registers()[i.rs2() as usize].clone();
    machine.memory_mut().store16(&address, &value)?;
    Ok(())
}

pub fn handle_sll<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let shift_value =
        machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
    let value = machine.registers()[i.rs1()].clone() << shift_value;
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_slli<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let value =
        machine.registers()[i.rs1() as usize].clone() << Mac::REG::from_u32(i.immediate_u());
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_slliw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let value =
        machine.registers()[i.rs1() as usize].clone() << Mac::REG::from_u32(i.immediate_u());
    update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
    Ok(())
}

pub fn handle_sllw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let shift_value = machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(0x1F);
    let value = machine.registers()[i.rs1()].clone() << shift_value;
    update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
    Ok(())
}

pub fn handle_slt<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.lt_s(&rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_slti<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let imm_value = Mac::REG::from_i32(i.immediate_s());
    let value = rs1_value.lt_s(&imm_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_sltiu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let imm_value = Mac::REG::from_i32(i.immediate_s());
    let value = rs1_value.lt(&imm_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_sltu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.lt(&rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_sra<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let shift_value =
        machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
    let value = machine.registers()[i.rs1()].signed_shr(&shift_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_srai<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let value =
        machine.registers()[i.rs1() as usize].signed_shr(&Mac::REG::from_u32(i.immediate_u()));
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_sraiw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let value = machine.registers()[i.rs1() as usize]
        .sign_extend(&Mac::REG::from_u8(32))
        .signed_shr(&Mac::REG::from_u32(i.immediate_u()));
    update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
    Ok(())
}

pub fn handle_sraw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let shift_value = machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(0x1F);
    let value = machine.registers()[i.rs1()]
        .sign_extend(&Mac::REG::from_u8(32))
        .signed_shr(&shift_value);
    update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
    Ok(())
}

pub fn handle_srl<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let shift_value =
        machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
    let value = machine.registers()[i.rs1()].clone() >> shift_value;
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_srli<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let value =
        machine.registers()[i.rs1() as usize].clone() >> Mac::REG::from_u32(i.immediate_u());
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_srliw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let value = machine.registers()[i.rs1() as usize].zero_extend(&Mac::REG::from_u8(32))
        >> Mac::REG::from_u32(i.immediate_u());
    update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
    Ok(())
}

pub fn handle_srlw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let shift_value = machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(0x1F);
    let value = machine.registers()[i.rs1()].zero_extend(&Mac::REG::from_u8(32)) >> shift_value;
    update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
    Ok(())
}

pub fn handle_sub<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1() as usize];
    let rs2_value = &machine.registers()[i.rs2() as usize];
    let value = rs1_value.overflowing_sub(rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_subw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1() as usize];
    let rs2_value = &machine.registers()[i.rs2() as usize];
    let value = rs1_value.overflowing_sub(rs2_value);
    update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
    Ok(())
}

pub fn handle_sw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Stype(inst);
    let address =
        machine.registers()[i.rs1() as usize].overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
    let value = machine.registers()[i.rs2() as usize].clone();
    machine.memory_mut().store32(&address, &value)?;
    Ok(())
}

pub fn handle_xor<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = machine.registers()[i.rs1() as usize].clone();
    let rs2_value = machine.registers()[i.rs2() as usize].clone();
    let value = rs1_value ^ rs2_value;
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_xori<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let value = machine.registers()[i.rs1() as usize].clone() ^ Mac::REG::from_i32(i.immediate_s());
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_adduw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let rs1_u = rs1_value.zero_extend(&Mac::REG::from_u8(32));
    let value = rs2_value.overflowing_add(&rs1_u);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_andn<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.clone() & !rs2_value.clone();
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_bclr<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
    let value = rs1_value.clone() & !(Mac::REG::one() << shamt);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_bclri<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &Mac::REG::from_u32(i.immediate_u());
    let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
    let value = rs1_value.clone() & !(Mac::REG::one() << shamt);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_bext<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
    let value = Mac::REG::one() & (rs1_value.clone() >> shamt);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_bexti<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &Mac::REG::from_u32(i.immediate_u());
    let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
    let value = Mac::REG::one() & (rs1_value.clone() >> shamt);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_binv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
    let value = rs1_value.clone() ^ (Mac::REG::one() << shamt);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_binvi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &Mac::REG::from_u32(i.immediate_u());
    let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
    let value = rs1_value.clone() ^ (Mac::REG::one() << shamt);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_bset<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
    let value = rs1_value.clone() | (Mac::REG::one() << shamt);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_bseti<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &Mac::REG::from_u32(i.immediate_u());
    let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
    let value = rs1_value.clone() | (Mac::REG::one() << shamt);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_clmul<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.clmul(rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_clmulh<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.clmulh(rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_clmulr<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.clmulr(rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_clz<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let value = rs1_value.clz();
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_clzw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let value = rs1_value
        .zero_extend(&Mac::REG::from_u8(32))
        .clz()
        .overflowing_sub(&Mac::REG::from_u8(32));
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_cpop<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let value = rs1_value.cpop();
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_cpopw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let value = rs1_value.zero_extend(&Mac::REG::from_u8(32)).cpop();
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_ctz<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let value = rs1_value.ctz();
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_ctzw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let value = (rs1_value.clone() | Mac::REG::from_u64(0xffff_ffff_0000_0000)).ctz();
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_max<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.ge_s(&rs2_value).cond(&rs1_value, &rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_maxu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.ge(&rs2_value).cond(&rs1_value, &rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_min<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.lt_s(&rs2_value).cond(&rs1_value, &rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_minu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.lt(&rs2_value).cond(&rs1_value, &rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_orcb<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let value = rs1_value.orcb();
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_orn<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.clone() | !rs2_value.clone();
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_rev8<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let value = rs1_value.rev8();
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_rol<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
    let value = rs1_value.rol(&shamt);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_rolw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let shamt = rs2_value.clone() & Mac::REG::from_u8(31);
    let twins = rs1_value
        .zero_extend(&Mac::REG::from_u8(32))
        .overflowing_mul(&Mac::REG::from_u64(0x_0000_0001_0000_0001));
    let value = twins.rol(&shamt).sign_extend(&Mac::REG::from_u8(32));
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_ror<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
    let value = rs1_value.ror(&shamt);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_rori<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &Mac::REG::from_u32(i.immediate_u());
    let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
    let value = rs1_value.ror(&shamt);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_roriw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &Mac::REG::from_u32(i.immediate_u());
    let shamt = rs2_value.clone() & Mac::REG::from_u8(31);
    let twins = rs1_value
        .zero_extend(&Mac::REG::from_u8(32))
        .overflowing_mul(&Mac::REG::from_u64(0x_0000_0001_0000_0001));
    let value = twins.ror(&shamt).sign_extend(&Mac::REG::from_u8(32));
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_rorw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let shamt = rs2_value.clone() & Mac::REG::from_u8(31);
    let twins = rs1_value
        .zero_extend(&Mac::REG::from_u8(32))
        .overflowing_mul(&Mac::REG::from_u64(0x_0000_0001_0000_0001));
    let value = twins.ror(&shamt).sign_extend(&Mac::REG::from_u8(32));
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_sextb<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let shift = &Mac::REG::from_u8(Mac::REG::BITS - 8);
    let value = rs1_value.signed_shl(shift).signed_shr(shift);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_sexth<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let shift = &Mac::REG::from_u8(Mac::REG::BITS - 16);
    let value = rs1_value.signed_shl(shift).signed_shr(shift);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_sh1add<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = (rs1_value.clone() << Mac::REG::from_u32(1)).overflowing_add(rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_sh1adduw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let rs1_z = rs1_value.clone().zero_extend(&Mac::REG::from_u8(32));
    let value = (rs1_z << Mac::REG::from_u32(1)).overflowing_add(rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_sh2add<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = (rs1_value.clone() << Mac::REG::from_u32(2)).overflowing_add(rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_sh2adduw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let rs1_z = rs1_value.clone().zero_extend(&Mac::REG::from_u8(32));
    let value = (rs1_z << Mac::REG::from_u32(2)).overflowing_add(rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_sh3add<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = (rs1_value.clone() << Mac::REG::from_u32(3)).overflowing_add(rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_sh3adduw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let rs1_z = rs1_value.clone().zero_extend(&Mac::REG::from_u8(32));
    let value = (rs1_z << Mac::REG::from_u32(3)).overflowing_add(rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_slliuw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = Mac::REG::from_u32(i.immediate_u());
    let rs1_u = rs1_value.clone().zero_extend(&Mac::REG::from_u8(32));
    let shamt = rs2_value & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
    let value = rs1_u << shamt;
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_xnor<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.clone() ^ !rs2_value.clone();
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_zexth<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let value = rs1_value.zero_extend(&Mac::REG::from_u8(16));
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_wide_mul<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = R4type(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value_h = rs1_value.overflowing_mul_high_signed(&rs2_value);
    let value_l = rs1_value.overflowing_mul(&rs2_value);
    update_register(machine, i.rd(), value_h);
    update_register(machine, i.rs3(), value_l);
    Ok(())
}

pub fn handle_wide_mulu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = R4type(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value_h = rs1_value.overflowing_mul_high_unsigned(&rs2_value);
    let value_l = rs1_value.overflowing_mul(&rs2_value);
    update_register(machine, i.rd(), value_h);
    update_register(machine, i.rs3(), value_l);
    Ok(())
}

pub fn handle_wide_mulsu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = R4type(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value_h = rs1_value.overflowing_mul_high_signed_unsigned(&rs2_value);
    let value_l = rs1_value.overflowing_mul(&rs2_value);
    update_register(machine, i.rd(), value_h);
    update_register(machine, i.rs3(), value_l);
    Ok(())
}

pub fn handle_wide_div<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = R4type(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value_h = rs1_value.overflowing_div_signed(&rs2_value);
    let value_l = rs1_value.overflowing_rem_signed(&rs2_value);
    update_register(machine, i.rd(), value_h);
    update_register(machine, i.rs3(), value_l);
    Ok(())
}

pub fn handle_wide_divu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = R4type(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value_h = rs1_value.overflowing_div(&rs2_value);
    let value_l = rs1_value.overflowing_rem(&rs2_value);
    update_register(machine, i.rd(), value_h);
    update_register(machine, i.rs3(), value_l);
    Ok(())
}

pub fn handle_ld_sign_extended_32_constant<Mac: Machine>(
    machine: &mut Mac,
    inst: Instruction,
) -> Result<(), Error> {
    let i = Utype(inst);
    update_register(machine, i.rd(), Mac::REG::from_i32(i.immediate_s()));
    Ok(())
}

pub fn handle_adc<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rd_value = &machine.registers()[i.rd()];
    let rs1_value = &machine.registers()[i.rs1()];
    let r = rd_value.overflowing_add(&rs1_value);
    update_register(machine, i.rd(), r);
    let rd_value = &machine.registers()[i.rd()];
    let rs1_value = &machine.registers()[i.rs1()];
    let r = rd_value.lt(&rs1_value);
    update_register(machine, i.rs1(), r);
    let rd_value = &machine.registers()[i.rd()];
    let rs2_value = &machine.registers()[i.rs2()];
    let r = rd_value.overflowing_add(&rs2_value);
    update_register(machine, i.rd(), r);
    let rd_value = &machine.registers()[i.rd()];
    let rs2_value = &machine.registers()[i.rs2()];
    let r = rd_value.lt(&rs2_value);
    update_register(machine, i.rs2(), r);
    let rs1_value = machine.registers()[i.rs1()].clone();
    let rs2_value = machine.registers()[i.rs2()].clone();
    let r = rs1_value | rs2_value;
    update_register(machine, i.rs1(), r);
    Ok(())
}

pub fn handle_sbb<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = R4type(inst);
    let rd_value = &machine.registers()[i.rd()];
    let rs1_value = &machine.registers()[i.rs1()];
    let r = rd_value.overflowing_sub(&rs1_value);
    update_register(machine, i.rs1(), r);
    let rd_value = &machine.registers()[i.rd()];
    let rs1_value = &machine.registers()[i.rs1()];
    let r = rd_value.lt(&rs1_value);
    update_register(machine, i.rs3(), r);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let r = rs1_value.overflowing_sub(&rs2_value);
    update_register(machine, i.rd(), r);
    let rd_value = &machine.registers()[i.rd()];
    let rs1_value = &machine.registers()[i.rs1()];
    let r = rs1_value.lt(&rd_value);
    update_register(machine, i.rs2(), r);
    let rs2_value = machine.registers()[i.rs2()].clone();
    let rs3_value = machine.registers()[i.rs3()].clone();
    let r = rs2_value | rs3_value;
    update_register(machine, i.rs1(), r);
    Ok(())
}

pub fn handle_custom_load_imm<Mac: Machine>(
    machine: &mut Mac,
    inst: Instruction,
) -> Result<(), Error> {
    let i = Utype(inst);
    let value = Mac::REG::from_i32(i.immediate_s());
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_auipc<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Utype(inst);
    let value = machine
        .pc()
        .overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_beq<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Stype(inst);
    let pc = machine.pc();
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let condition = rs1_value.eq(&rs2_value);
    let new_pc = condition.cond(
        &Mac::REG::from_i32(i.immediate_s()).overflowing_add(&pc),
        &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(&pc),
    );
    machine.update_pc(new_pc);
    Ok(())
}

pub fn handle_bge<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Stype(inst);
    let pc = machine.pc();
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let condition = rs1_value.ge_s(&rs2_value);
    let new_pc = condition.cond(
        &Mac::REG::from_i32(i.immediate_s()).overflowing_add(&pc),
        &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(&pc),
    );
    machine.update_pc(new_pc);
    Ok(())
}

pub fn handle_bgeu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Stype(inst);
    let pc = machine.pc();
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let condition = rs1_value.ge(&rs2_value);
    let new_pc = condition.cond(
        &Mac::REG::from_i32(i.immediate_s()).overflowing_add(&pc),
        &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(&pc),
    );
    machine.update_pc(new_pc);
    Ok(())
}

pub fn handle_blt<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Stype(inst);
    let pc = machine.pc();
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let condition = rs1_value.lt_s(&rs2_value);
    let new_pc = condition.cond(
        &Mac::REG::from_i32(i.immediate_s()).overflowing_add(&pc),
        &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(&pc),
    );
    machine.update_pc(new_pc);
    Ok(())
}

pub fn handle_bltu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Stype(inst);
    let pc = machine.pc();
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let condition = rs1_value.lt(&rs2_value);
    let new_pc = condition.cond(
        &Mac::REG::from_i32(i.immediate_s()).overflowing_add(&pc),
        &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(&pc),
    );
    machine.update_pc(new_pc);
    Ok(())
}

pub fn handle_bne<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Stype(inst);
    let pc = machine.pc();
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let condition = rs1_value.ne(&rs2_value);
    let new_pc = condition.cond(
        &Mac::REG::from_i32(i.immediate_s()).overflowing_add(&pc),
        &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(&pc),
    );
    machine.update_pc(new_pc);
    Ok(())
}

pub fn handle_ebreak<Mac: Machine>(machine: &mut Mac, _: Instruction) -> Result<(), Error> {
    machine.ebreak()?;
    Ok(())
}

pub fn handle_ecall<Mac: Machine>(machine: &mut Mac, _: Instruction) -> Result<(), Error> {
    // The semantic of ECALL is determined by the hardware, which
    // is not part of the spec, hence here the implementation is
    // deferred to the machine. This way custom ECALLs might be
    // provided for different environments.
    machine.ecall()?;
    Ok(())
}

pub fn handle_jal<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Utype(inst);
    let link = machine
        .pc()
        .overflowing_add(&Mac::REG::from_u8(instruction_length(inst)));
    update_register(machine, i.rd(), link);
    machine.update_pc(
        machine
            .pc()
            .overflowing_add(&Mac::REG::from_i32(i.immediate_s())),
    );
    Ok(())
}

pub fn handle_jalr<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    let size = instruction_length(inst);
    let link = machine.pc().overflowing_add(&Mac::REG::from_u8(size));
    if machine.version() >= 1 {
        let mut next_pc =
            machine.registers()[i.rs1()].overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
        next_pc = next_pc & (!Mac::REG::one());
        update_register(machine, i.rd(), link);
        machine.update_pc(next_pc);
    } else {
        update_register(machine, i.rd(), link);
        let mut next_pc =
            machine.registers()[i.rs1()].overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
        next_pc = next_pc & (!Mac::REG::one());
        machine.update_pc(next_pc);
    }
    Ok(())
}

pub fn handle_far_jump_rel<Mac: Machine>(
    machine: &mut Mac,
    inst: Instruction,
) -> Result<(), Error> {
    let i = Utype(inst);
    let size = instruction_length(inst);
    let link = machine.pc().overflowing_add(&Mac::REG::from_u8(size));
    let next_pc = machine
        .pc()
        .overflowing_add(&Mac::REG::from_i32(i.immediate_s()))
        & (!Mac::REG::one());
    update_register(machine, RA, link);
    machine.update_pc(next_pc);
    Ok(())
}

pub fn handle_far_jump_abs<Mac: Machine>(
    machine: &mut Mac,
    inst: Instruction,
) -> Result<(), Error> {
    let i = Utype(inst);
    let size = instruction_length(inst);
    let link = machine.pc().overflowing_add(&Mac::REG::from_u8(size));
    let next_pc = Mac::REG::from_i32(i.immediate_s()) & (!Mac::REG::one());
    update_register(machine, RA, link);
    machine.update_pc(next_pc);
    Ok(())
}

pub fn handle_custom_trace_end<Mac: Machine>(_: &mut Mac, inst: Instruction) -> Result<(), Error> {
    Err(Error::InvalidOp(extract_opcode(inst)))
}

pub fn handle_vsetvli<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::set_vl(
        machine,
        i.rd(),
        i.rs1(),
        machine.registers()[i.rs1()].to_u64(),
        i.immediate_u() as u64,
    )?;
    Ok(())
}

pub fn handle_vsetivli<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::set_vl(machine, i.rd(), 33, i.rs1() as u64, i.immediate_u() as u64)?;
    Ok(())
}

pub fn handle_vsetvl<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    common::set_vl(
        machine,
        i.rd(),
        i.rs1(),
        machine.registers()[i.rs1()].to_u64(),
        machine.registers()[i.rs2()].to_u64(),
    )?;
    Ok(())
}

pub fn handle_vlm_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld!(inst, machine, (machine.vl() + 7) / 8, 0, 1, 0);
    Ok(())
}

pub fn handle_vle8_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld!(inst, machine, machine.vl(), 0, 1, 1);
    Ok(())
}

pub fn handle_vle16_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld!(inst, machine, machine.vl(), 0, 2, 1);
    Ok(())
}

pub fn handle_vle32_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld!(inst, machine, machine.vl(), 0, 4, 1);
    Ok(())
}

pub fn handle_vle64_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld!(inst, machine, machine.vl(), 0, 8, 1);
    Ok(())
}

pub fn handle_vle128_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld!(inst, machine, machine.vl(), 0, 16, 1);
    Ok(())
}

pub fn handle_vle256_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld!(inst, machine, machine.vl(), 0, 32, 1);
    Ok(())
}

pub fn handle_vle512_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld!(inst, machine, machine.vl(), 0, 64, 1);
    Ok(())
}

pub fn handle_vle1024_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld!(inst, machine, machine.vl(), 0, 128, 1);
    Ok(())
}

pub fn handle_vsm_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd!(inst, machine, (machine.vl() + 7) / 8, 0, 1, 0);
    Ok(())
}

pub fn handle_vse8_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd!(inst, machine, machine.vl(), 0, 1, 1);
    Ok(())
}

pub fn handle_vse16_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd!(inst, machine, machine.vl(), 0, 2, 1);
    Ok(())
}

pub fn handle_vse32_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd!(inst, machine, machine.vl(), 0, 4, 1);
    Ok(())
}

pub fn handle_vse64_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd!(inst, machine, machine.vl(), 0, 8, 1);
    Ok(())
}

pub fn handle_vse128_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd!(inst, machine, machine.vl(), 0, 16, 1);
    Ok(())
}

pub fn handle_vse256_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd!(inst, machine, machine.vl(), 0, 32, 1);
    Ok(())
}

pub fn handle_vse512_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd!(inst, machine, machine.vl(), 0, 64, 1);
    Ok(())
}

pub fn handle_vse1024_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd!(inst, machine, machine.vl(), 0, 128, 1);
    Ok(())
}

pub fn handle_vlse8_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld!(inst, machine, machine.vl(), 1, 1, 1);
    Ok(())
}

pub fn handle_vlse16_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld!(inst, machine, machine.vl(), 1, 2, 1);
    Ok(())
}

pub fn handle_vlse32_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld!(inst, machine, machine.vl(), 1, 4, 1);
    Ok(())
}

pub fn handle_vlse64_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld!(inst, machine, machine.vl(), 1, 8, 1);
    Ok(())
}

pub fn handle_vlse128_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld!(inst, machine, machine.vl(), 1, 16, 1);
    Ok(())
}

pub fn handle_vlse256_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld!(inst, machine, machine.vl(), 1, 32, 1);
    Ok(())
}

pub fn handle_vlse512_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld!(inst, machine, machine.vl(), 1, 64, 1);
    Ok(())
}

pub fn handle_vlse1024_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld!(inst, machine, machine.vl(), 1, 128, 1);
    Ok(())
}

pub fn handle_vsse8_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd!(inst, machine, machine.vl(), 1, 1, 1);
    Ok(())
}

pub fn handle_vsse16_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd!(inst, machine, machine.vl(), 1, 2, 1);
    Ok(())
}

pub fn handle_vsse32_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd!(inst, machine, machine.vl(), 1, 4, 1);
    Ok(())
}

pub fn handle_vsse64_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd!(inst, machine, machine.vl(), 1, 8, 1);
    Ok(())
}

pub fn handle_vsse128_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd!(inst, machine, machine.vl(), 1, 16, 1);
    Ok(())
}

pub fn handle_vsse256_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd!(inst, machine, machine.vl(), 1, 32, 1);
    Ok(())
}

pub fn handle_vsse512_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd!(inst, machine, machine.vl(), 1, 64, 1);
    Ok(())
}

pub fn handle_vsse1024_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd!(inst, machine, machine.vl(), 1, 128, 1);
    Ok(())
}

pub fn handle_vluxei8_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_index!(inst, machine, 8);
    Ok(())
}

pub fn handle_vluxei16_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_index!(inst, machine, 16);
    Ok(())
}

pub fn handle_vluxei32_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_index!(inst, machine, 32);
    Ok(())
}

pub fn handle_vluxei64_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_index!(inst, machine, 64);
    Ok(())
}

pub fn handle_vloxei8_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_index!(inst, machine, 8);
    Ok(())
}

pub fn handle_vloxei16_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_index!(inst, machine, 16);
    Ok(())
}

pub fn handle_vloxei32_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_index!(inst, machine, 32);
    Ok(())
}

pub fn handle_vloxei64_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_index!(inst, machine, 64);
    Ok(())
}

pub fn handle_vsuxei8_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd_index!(inst, machine, 8);
    Ok(())
}

pub fn handle_vsuxei16_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd_index!(inst, machine, 16);
    Ok(())
}

pub fn handle_vsuxei32_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd_index!(inst, machine, 32);
    Ok(())
}

pub fn handle_vsuxei64_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd_index!(inst, machine, 64);
    Ok(())
}

pub fn handle_vsoxei8_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd_index!(inst, machine, 8);
    Ok(())
}

pub fn handle_vsoxei16_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd_index!(inst, machine, 16);
    Ok(())
}

pub fn handle_vsoxei32_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd_index!(inst, machine, 32);
    Ok(())
}

pub fn handle_vsoxei64_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd_index!(inst, machine, 64);
    Ok(())
}

pub fn handle_vl1re8_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_whole!(inst, machine, VLEN as u64 / 8);
    Ok(())
}

pub fn handle_vl1re16_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_whole!(inst, machine, VLEN as u64 / 8);
    Ok(())
}

pub fn handle_vl1re32_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_whole!(inst, machine, VLEN as u64 / 8);
    Ok(())
}

pub fn handle_vl1re64_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_whole!(inst, machine, VLEN as u64 / 8);
    Ok(())
}

pub fn handle_vl2re8_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_whole!(inst, machine, VLEN as u64 / 4);
    Ok(())
}

pub fn handle_vl2re16_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_whole!(inst, machine, VLEN as u64 / 4);
    Ok(())
}

pub fn handle_vl2re32_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_whole!(inst, machine, VLEN as u64 / 4);
    Ok(())
}

pub fn handle_vl2re64_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_whole!(inst, machine, VLEN as u64 / 4);
    Ok(())
}

pub fn handle_vl4re8_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_whole!(inst, machine, VLEN as u64 / 2);
    Ok(())
}

pub fn handle_vl4re16_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_whole!(inst, machine, VLEN as u64 / 2);
    Ok(())
}

pub fn handle_vl4re32_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_whole!(inst, machine, VLEN as u64 / 2);
    Ok(())
}

pub fn handle_vl4re64_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_whole!(inst, machine, VLEN as u64 / 2);
    Ok(())
}

pub fn handle_vl8re8_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_whole!(inst, machine, VLEN as u64);
    Ok(())
}

pub fn handle_vl8re16_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_whole!(inst, machine, VLEN as u64);
    Ok(())
}

pub fn handle_vl8re32_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_whole!(inst, machine, VLEN as u64);
    Ok(())
}

pub fn handle_vl8re64_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld_whole!(inst, machine, VLEN as u64);
    Ok(())
}

pub fn handle_vs1r_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd_whole!(inst, machine, VLEN as u64 / 8);
    Ok(())
}

pub fn handle_vs2r_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd_whole!(inst, machine, VLEN as u64 / 4);
    Ok(())
}

pub fn handle_vs4r_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd_whole!(inst, machine, VLEN as u64 / 2);
    Ok(())
}

pub fn handle_vs8r_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd_whole!(inst, machine, VLEN as u64);
    Ok(())
}

pub fn handle_vadd_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_s!(inst, machine, Eint::wrapping_add);
    Ok(())
}

pub fn handle_vadd_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_s!(inst, machine, Eint::wrapping_add);
    Ok(())
}

pub fn handle_vadd_vi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vi_loop_s!(inst, machine, Eint::wrapping_add);
    Ok(())
}

pub fn handle_vsub_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_s!(inst, machine, Eint::wrapping_sub);
    Ok(())
}

pub fn handle_vsub_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_s!(inst, machine, Eint::wrapping_sub);
    Ok(())
}

pub fn handle_vrsub_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_s!(inst, machine, alu::rsub);
    Ok(())
}

pub fn handle_vrsub_vi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vi_loop_s!(inst, machine, alu::rsub);
    Ok(())
}

pub fn handle_vw_addu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vv_loop_u!(inst, machine, Eint::widening_add_u);
    Ok(())
}

pub fn handle_vw_addu_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vx_loop_u!(inst, machine, Eint::widening_add_u);
    Ok(())
}

pub fn handle_vw_subu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vv_loop_u!(inst, machine, Eint::widening_sub_u);
    Ok(())
}

pub fn handle_vw_subu_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vx_loop_u!(inst, machine, Eint::widening_sub_u);
    Ok(())
}

pub fn handle_vwadd_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vv_loop_s!(inst, machine, Eint::widening_add_s);
    Ok(())
}

pub fn handle_vwadd_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vx_loop_s!(inst, machine, Eint::widening_add_s);
    Ok(())
}

pub fn handle_vwsub_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vv_loop_s!(inst, machine, Eint::widening_sub_s);
    Ok(())
}

pub fn handle_vwsub_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vx_loop_s!(inst, machine, Eint::widening_sub_s);
    Ok(())
}

pub fn handle_vwaddu_wv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_wv_loop_u!(inst, machine, Eint::wrapping_add);
    Ok(())
}

pub fn handle_vwaddu_wx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_wx_loop_u!(inst, machine, Eint::wrapping_add);
    Ok(())
}

pub fn handle_vwsubu_wv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_wv_loop_u!(inst, machine, Eint::wrapping_sub);
    Ok(())
}

pub fn handle_vwsubu_wx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_wx_loop_u!(inst, machine, Eint::wrapping_sub);
    Ok(())
}

pub fn handle_vwadd_wv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_wv_loop_s!(inst, machine, Eint::wrapping_add);
    Ok(())
}

pub fn handle_vwadd_wx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_wx_loop_s!(inst, machine, Eint::wrapping_add);
    Ok(())
}

pub fn handle_vwsub_wv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_wv_loop_s!(inst, machine, Eint::wrapping_sub);
    Ok(())
}

pub fn handle_vwsub_wx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_wx_loop_s!(inst, machine, Eint::wrapping_sub);
    Ok(())
}

pub fn handle_vzext_vf2<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_ext_u!(inst, machine, 2);
    Ok(())
}

pub fn handle_vzext_vf4<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_ext_u!(inst, machine, 4);
    Ok(())
}

pub fn handle_vzext_vf8<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_ext_u!(inst, machine, 8);
    Ok(())
}

pub fn handle_vsext_vf2<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_ext_s!(inst, machine, 2);
    Ok(())
}

pub fn handle_vsext_vf4<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_ext_s!(inst, machine, 4);
    Ok(())
}

pub fn handle_vsext_vf8<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_ext_s!(inst, machine, 8);
    Ok(())
}

pub fn handle_vadc_vvm<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vvm_loop_s!(inst, machine, alu::adc);
    Ok(())
}

pub fn handle_vadc_vxm<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vxm_loop_s!(inst, machine, alu::adc);
    Ok(())
}

pub fn handle_vadc_vim<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vim_loop_s!(inst, machine, alu::adc);
    Ok(())
}

pub fn handle_vmadc_vvm<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vvm_loop_s!(inst, machine, alu::madcm);
    Ok(())
}

pub fn handle_vmadc_vxm<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vxm_loop_s!(inst, machine, alu::madcm);
    Ok(())
}

pub fn handle_vmadc_vim<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vim_loop_s!(inst, machine, alu::madcm);
    Ok(())
}

pub fn handle_vmadc_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vv_loop_s!(inst, machine, alu::madc);
    Ok(())
}

pub fn handle_vmadc_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vx_loop_s!(inst, machine, alu::madc);
    Ok(())
}

pub fn handle_vmadc_vi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vi_loop_s!(inst, machine, alu::madc);
    Ok(())
}

pub fn handle_vsbc_vvm<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vvm_loop_s!(inst, machine, alu::sbc);
    Ok(())
}

pub fn handle_vsbc_vxm<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vxm_loop_s!(inst, machine, alu::sbc);
    Ok(())
}

pub fn handle_vmsbc_vvm<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vvm_loop_s!(inst, machine, alu::msbcm);
    Ok(())
}

pub fn handle_vmsbc_vxm<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vxm_loop_s!(inst, machine, alu::msbcm);
    Ok(())
}

pub fn handle_vmsbc_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vv_loop_s!(inst, machine, alu::msbc);
    Ok(())
}

pub fn handle_vmsbc_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vx_loop_s!(inst, machine, alu::msbc);
    Ok(())
}

pub fn handle_vand_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_s!(inst, machine, alu::and);
    Ok(())
}

pub fn handle_vand_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_s!(inst, machine, alu::and);
    Ok(())
}

pub fn handle_vand_vi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vi_loop_s!(inst, machine, alu::and);
    Ok(())
}

pub fn handle_vor_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_s!(inst, machine, alu::or);
    Ok(())
}

pub fn handle_vor_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_s!(inst, machine, alu::or);
    Ok(())
}

pub fn handle_vor_vi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vi_loop_s!(inst, machine, alu::or);
    Ok(())
}

pub fn handle_vxor_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_s!(inst, machine, alu::xor);
    Ok(())
}

pub fn handle_vxor_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_s!(inst, machine, alu::xor);
    Ok(())
}

pub fn handle_vxor_vi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vi_loop_s!(inst, machine, alu::xor);
    Ok(())
}

pub fn handle_vsll_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_s!(inst, machine, alu::sll);
    Ok(())
}

pub fn handle_vsll_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_s!(inst, machine, alu::sll);
    Ok(())
}

pub fn handle_vsll_vi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vi_loop_s!(inst, machine, alu::sll);
    Ok(())
}

pub fn handle_vsrl_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_s!(inst, machine, alu::srl);
    Ok(())
}

pub fn handle_vsrl_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_s!(inst, machine, alu::srl);
    Ok(())
}

pub fn handle_vsrl_vi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vi_loop_s!(inst, machine, alu::srl);
    Ok(())
}

pub fn handle_vsra_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_s!(inst, machine, alu::sra);
    Ok(())
}

pub fn handle_vsra_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_s!(inst, machine, alu::sra);
    Ok(())
}

pub fn handle_vsra_vi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vi_loop_s!(inst, machine, alu::sra);
    Ok(())
}

pub fn handle_vnsrl_wv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_wv_loop_u!(inst, machine, alu::srl);
    Ok(())
}

pub fn handle_vnsrl_wx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_wx_loop_u!(inst, machine, alu::srl);
    Ok(())
}

pub fn handle_vnsrl_wi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_wi_loop_u!(inst, machine, alu::srl);
    Ok(())
}

pub fn handle_vnsra_wv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_wv_loop_u!(inst, machine, alu::sra);
    Ok(())
}

pub fn handle_vnsra_wx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_wx_loop_u!(inst, machine, alu::sra);
    Ok(())
}

pub fn handle_vnsra_wi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_wi_loop_u!(inst, machine, alu::sra);
    Ok(())
}

pub fn handle_vmseq_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vv_loop_s!(inst, machine, alu::seq);
    Ok(())
}

pub fn handle_vmseq_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vx_loop_s!(inst, machine, alu::seq);
    Ok(())
}

pub fn handle_vmseq_vi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vi_loop_s!(inst, machine, alu::seq);
    Ok(())
}

pub fn handle_vmsne_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vv_loop_s!(inst, machine, alu::sne);
    Ok(())
}

pub fn handle_vmsne_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vx_loop_s!(inst, machine, alu::sne);
    Ok(())
}

pub fn handle_vmsne_vi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vi_loop_s!(inst, machine, alu::sne);
    Ok(())
}

pub fn handle_vmsltu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vv_loop_s!(inst, machine, alu::sltu);
    Ok(())
}

pub fn handle_vmsltu_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vx_loop_s!(inst, machine, alu::sltu);
    Ok(())
}

pub fn handle_vmslt_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vv_loop_s!(inst, machine, alu::slt);
    Ok(())
}

pub fn handle_vmslt_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vx_loop_s!(inst, machine, alu::slt);
    Ok(())
}

pub fn handle_vmsleu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vv_loop_s!(inst, machine, alu::sleu);
    Ok(())
}

pub fn handle_vmsleu_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vx_loop_s!(inst, machine, alu::sleu);
    Ok(())
}

pub fn handle_vmsleu_vi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vi_loop_s!(inst, machine, alu::sleu);
    Ok(())
}

pub fn handle_vmsle_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vv_loop_s!(inst, machine, alu::sle);
    Ok(())
}

pub fn handle_vmsle_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vx_loop_s!(inst, machine, alu::sle);
    Ok(())
}

pub fn handle_vmsle_vi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vi_loop_s!(inst, machine, alu::sle);
    Ok(())
}

pub fn handle_vmsgtu_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vx_loop_s!(inst, machine, alu::sgtu);
    Ok(())
}

pub fn handle_vmsgtu_vi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vi_loop_s!(inst, machine, alu::sgtu);
    Ok(())
}

pub fn handle_vmsgt_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vx_loop_s!(inst, machine, alu::sgt);
    Ok(())
}

pub fn handle_vmsgt_vi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vi_loop_s!(inst, machine, alu::sgt);
    Ok(())
}

pub fn handle_vmaxu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_u!(inst, machine, alu::maxu);
    Ok(())
}

pub fn handle_vmaxu_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_u!(inst, machine, alu::maxu);
    Ok(())
}

pub fn handle_vmax_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_u!(inst, machine, alu::max);
    Ok(())
}

pub fn handle_vmax_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_u!(inst, machine, alu::max);
    Ok(())
}

pub fn handle_vminu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_u!(inst, machine, alu::minu);
    Ok(())
}

pub fn handle_vminu_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_u!(inst, machine, alu::minu);
    Ok(())
}

pub fn handle_vmin_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_u!(inst, machine, alu::min);
    Ok(())
}

pub fn handle_vmin_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_u!(inst, machine, alu::min);
    Ok(())
}

pub fn handle_vmul_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_s!(inst, machine, Eint::wrapping_mul);
    Ok(())
}

pub fn handle_vmul_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_s!(inst, machine, Eint::wrapping_mul);
    Ok(())
}

pub fn handle_vmulh_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_s!(inst, machine, alu::mulh);
    Ok(())
}

pub fn handle_vmulh_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_s!(inst, machine, alu::mulh);
    Ok(())
}

pub fn handle_vmulhu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_s!(inst, machine, alu::mulhu);
    Ok(())
}

pub fn handle_vmulhu_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_s!(inst, machine, alu::mulhu);
    Ok(())
}

pub fn handle_vmulhsu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_s!(inst, machine, alu::mulhsu);
    Ok(())
}

pub fn handle_vmulhsu_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_s!(inst, machine, alu::mulhsu);
    Ok(())
}

pub fn handle_vdivu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_u!(inst, machine, Eint::wrapping_div_u);
    Ok(())
}

pub fn handle_vdivu_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_u!(inst, machine, Eint::wrapping_div_u);
    Ok(())
}

pub fn handle_vdiv_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_s!(inst, machine, Eint::wrapping_div_s);
    Ok(())
}

pub fn handle_vdiv_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_s!(inst, machine, Eint::wrapping_div_s);
    Ok(())
}

pub fn handle_vremu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_u!(inst, machine, Eint::wrapping_rem_u);
    Ok(())
}

pub fn handle_vremu_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_u!(inst, machine, Eint::wrapping_rem_u);
    Ok(())
}

pub fn handle_vrem_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_s!(inst, machine, Eint::wrapping_rem_s);
    Ok(())
}

pub fn handle_vrem_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_s!(inst, machine, Eint::wrapping_rem_s);
    Ok(())
}

pub fn handle_vwmulu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vv_loop_u!(inst, machine, Eint::widening_mul_u);
    Ok(())
}

pub fn handle_vwmulu_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vx_loop_u!(inst, machine, Eint::widening_mul_u);
    Ok(())
}

pub fn handle_vwmulsu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vv_loop_u!(inst, machine, Eint::widening_mul_su);
    Ok(())
}

pub fn handle_vwmulsu_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vx_loop_u!(inst, machine, Eint::widening_mul_su);
    Ok(())
}

pub fn handle_vwmul_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vv_loop_s!(inst, machine, Eint::widening_mul_s);
    Ok(())
}

pub fn handle_vwmul_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vx_loop_s!(inst, machine, Eint::widening_mul_s);
    Ok(())
}

pub fn handle_vmacc_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_destructive_s!(inst, machine, alu::macc);
    Ok(())
}

pub fn handle_vmacc_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_destructive_s!(inst, machine, alu::macc);
    Ok(())
}

pub fn handle_vnmsac_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_destructive_s!(inst, machine, alu::nmsac);
    Ok(())
}

pub fn handle_vnmsac_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_destructive_s!(inst, machine, alu::nmsac);
    Ok(())
}

pub fn handle_vmadd_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_destructive_s!(inst, machine, alu::madd);
    Ok(())
}

pub fn handle_vmadd_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_destructive_s!(inst, machine, alu::madd);
    Ok(())
}

pub fn handle_vnmsub_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_destructive_s!(inst, machine, alu::nmsub);
    Ok(())
}

pub fn handle_vnmsub_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_destructive_s!(inst, machine, alu::nmsub);
    Ok(())
}

pub fn handle_vwmaccu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vv_loop_destructive_s!(inst, machine, alu::wmaccu);
    Ok(())
}

pub fn handle_vwmaccu_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vx_loop_destructive_s!(inst, machine, alu::wmaccu);
    Ok(())
}

pub fn handle_vwmacc_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vv_loop_destructive_s!(inst, machine, alu::wmacc);
    Ok(())
}

pub fn handle_vwmacc_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vx_loop_destructive_s!(inst, machine, alu::wmacc);
    Ok(())
}

pub fn handle_vwmaccsu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vv_loop_destructive_s!(inst, machine, alu::wmaccsu);
    Ok(())
}

pub fn handle_vwmaccsu_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vx_loop_destructive_s!(inst, machine, alu::wmaccsu);
    Ok(())
}

pub fn handle_vwmaccus_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vx_loop_destructive_u!(inst, machine, alu::wmaccus);
    Ok(())
}

pub fn handle_vmerge_vvm<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vvm_loop_s!(inst, machine, alu::merge);
    Ok(())
}

pub fn handle_vmerge_vxm<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vxm_loop_s!(inst, machine, alu::merge);
    Ok(())
}

pub fn handle_vmerge_vim<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vim_loop_s!(inst, machine, alu::merge);
    Ok(())
}

pub fn handle_vmv_v_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_s!(inst, machine, alu::mv);
    Ok(())
}

pub fn handle_vmv_v_x<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_s!(inst, machine, alu::mv);
    Ok(())
}

pub fn handle_vmv_v_i<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vi_loop_s!(inst, machine, alu::mv);
    Ok(())
}

pub fn handle_vsaddu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_u!(inst, machine, alu::saddu);
    Ok(())
}

pub fn handle_vsaddu_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_u!(inst, machine, alu::saddu);
    Ok(())
}

pub fn handle_vsaddu_vi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vi_loop_u!(inst, machine, alu::saddu);
    Ok(())
}

pub fn handle_vsadd_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_s!(inst, machine, alu::sadd);
    Ok(())
}

pub fn handle_vsadd_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_s!(inst, machine, alu::sadd);
    Ok(())
}

pub fn handle_vsadd_vi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vi_loop_s!(inst, machine, alu::sadd);
    Ok(())
}

pub fn handle_vssubu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_u!(inst, machine, alu::ssubu);
    Ok(())
}

pub fn handle_vssubu_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_u!(inst, machine, alu::ssubu);
    Ok(())
}

pub fn handle_vssub_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_s!(inst, machine, alu::ssub);
    Ok(())
}

pub fn handle_vssub_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_s!(inst, machine, alu::ssub);
    Ok(())
}

pub fn handle_vaadd_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_s!(inst, machine, Eint::average_add_s);
    Ok(())
}

pub fn handle_vaadd_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_s!(inst, machine, Eint::average_add_s);
    Ok(())
}

pub fn handle_vaaddu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_u!(inst, machine, Eint::average_add_u);
    Ok(())
}

pub fn handle_vaaddu_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_u!(inst, machine, Eint::average_add_u);
    Ok(())
}

pub fn handle_vasub_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_s!(inst, machine, Eint::average_sub_s);
    Ok(())
}

pub fn handle_vasub_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_s!(inst, machine, Eint::average_sub_s);
    Ok(())
}

pub fn handle_vasubu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_u!(inst, machine, Eint::average_sub_u);
    Ok(())
}

pub fn handle_vasubu_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_u!(inst, machine, Eint::average_sub_u);
    Ok(())
}

pub fn handle_vsmul_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_s!(inst, machine, alu::smul);
    Ok(())
}

pub fn handle_vsmul_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_s!(inst, machine, alu::smul);
    Ok(())
}

pub fn handle_vssrl_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_u!(inst, machine, alu::srl);
    Ok(())
}

pub fn handle_vssrl_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_u!(inst, machine, alu::srl);
    Ok(())
}

pub fn handle_vssrl_vi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vi_loop_u!(inst, machine, alu::srl);
    Ok(())
}

pub fn handle_vssra_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_u!(inst, machine, alu::sra);
    Ok(())
}

pub fn handle_vssra_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_u!(inst, machine, alu::sra);
    Ok(())
}

pub fn handle_vssra_vi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vi_loop_u!(inst, machine, alu::sra);
    Ok(())
}

pub fn handle_vnclipu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_wv_loop_u!(inst, machine, alu::vnclipu);
    Ok(())
}

pub fn handle_vnclipu_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_wx_loop_u!(inst, machine, alu::vnclipu);
    Ok(())
}

pub fn handle_vnclipu_vi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_wi_loop_u!(inst, machine, alu::vnclipu);
    Ok(())
}

pub fn handle_vnclip_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_wv_loop_u!(inst, machine, alu::vnclip);
    Ok(())
}

pub fn handle_vnclip_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_wx_loop_u!(inst, machine, alu::vnclip);
    Ok(())
}

pub fn handle_vnclip_vi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_wi_loop_u!(inst, machine, alu::vnclip);
    Ok(())
}

pub fn handle_vredsum_vs<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vs_loop_s!(inst, machine, Eint::wrapping_add);
    Ok(())
}

pub fn handle_vredand_vs<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vs_loop_s!(inst, machine, alu::and);
    Ok(())
}

pub fn handle_vredor_vs<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vs_loop_s!(inst, machine, alu::or);
    Ok(())
}

pub fn handle_vredxor_vs<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vs_loop_s!(inst, machine, alu::xor);
    Ok(())
}

pub fn handle_vredminu_vs<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vs_loop_s!(inst, machine, alu::minu);
    Ok(())
}

pub fn handle_vredmin_vs<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vs_loop_s!(inst, machine, alu::min);
    Ok(())
}

pub fn handle_vredmaxu_vs<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vs_loop_s!(inst, machine, alu::maxu);
    Ok(())
}

pub fn handle_vredmax_vs<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vs_loop_s!(inst, machine, alu::max);
    Ok(())
}

pub fn handle_vwredsumu_vs<Mac: Machine>(
    machine: &mut Mac,
    inst: Instruction,
) -> Result<(), Error> {
    w_vs_loop_u!(inst, machine, Eint::wrapping_add);
    Ok(())
}

pub fn handle_vwredsum_vs<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vs_loop_s!(inst, machine, Eint::wrapping_add);
    Ok(())
}

pub fn handle_vmand_mm<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_mm_loop!(inst, machine, |b: bool, a: bool| b & a);
    Ok(())
}

pub fn handle_vmnand_mm<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_mm_loop!(inst, machine, |b: bool, a: bool| !(b & a));
    Ok(())
}

pub fn handle_vmandnot_mm<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_mm_loop!(inst, machine, |b: bool, a: bool| b & !a);
    Ok(())
}

pub fn handle_vmxor_mm<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_mm_loop!(inst, machine, |b: bool, a: bool| b ^ a);
    Ok(())
}

pub fn handle_vmor_mm<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_mm_loop!(inst, machine, |b: bool, a: bool| b | a);
    Ok(())
}

pub fn handle_vmnor_mm<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_mm_loop!(inst, machine, |b: bool, a: bool| !(b | a));
    Ok(())
}

pub fn handle_vmornot_mm<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_mm_loop!(inst, machine, |b: bool, a: bool| b | !a);
    Ok(())
}

pub fn handle_vmxnor_mm<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_mm_loop!(inst, machine, |b: bool, a: bool| !(b ^ a));
    Ok(())
}

pub fn handle_vcpop_m<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    x_m_loop!(inst, machine, alu::cpop);
    Ok(())
}

pub fn handle_vfirst_m<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    x_m_loop!(inst, machine, alu::first);
    Ok(())
}

pub fn handle_vsbf_m<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_m_loop!(inst, machine, alu::sbf);
    Ok(())
}

pub fn handle_vsif_m<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_m_loop!(inst, machine, alu::sif);
    Ok(())
}

pub fn handle_vsof_m<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_m_loop!(inst, machine, alu::sof);
    Ok(())
}

pub fn handle_viota_m<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    require_vill!(machine);
    let sew = machine.vsew();
    let lmul = machine.vlmul();
    let i = VVtype(inst);
    require_align!(i.vd() as u64, lmul as u64);
    require_noover!(i.vd() as u64, lmul as u64, i.vs2() as u64, 1);
    require_vm!(i);
    let mut iota: u32 = 0;
    for j in 0..machine.vl() as usize {
        if i.vm() == 0 && !machine.get_bit(0, j) {
            continue;
        }
        match sew {
            8 => E8::from(iota as u8).put(machine.element_mut(i.vd(), sew, j)),
            16 => E16::from(iota as u16).put(machine.element_mut(i.vd(), sew, j)),
            32 => E32::from(iota as u16).put(machine.element_mut(i.vd(), sew, j)),
            64 => E64::from(iota as u16).put(machine.element_mut(i.vd(), sew, j)),
            128 => E128::from(iota as u16).put(machine.element_mut(i.vd(), sew, j)),
            256 => E256::from(iota as u16).put(machine.element_mut(i.vd(), sew, j)),
            512 => E512::from(iota as u16).put(machine.element_mut(i.vd(), sew, j)),
            1024 => E1024::from(iota as u16).put(machine.element_mut(i.vd(), sew, j)),
            _ => unreachable!(),
        }
        if machine.get_bit(i.vs2(), j) {
            iota += 1;
        }
    }
    Ok(())
}

pub fn handle_vid_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    require_vill!(machine);
    let sew = machine.vsew();
    let lmul = machine.vlmul();
    let i = VVtype(inst);
    require_align!(i.vd() as u64, lmul as u64);
    require_vm!(i);
    for j in 0..machine.vl() as usize {
        if i.vm() == 0 && !machine.get_bit(0, j) {
            continue;
        }
        match sew {
            8 => E8::from(j as u8).put(machine.element_mut(i.vd(), sew, j)),
            16 => E16::from(j as u16).put(machine.element_mut(i.vd(), sew, j)),
            32 => E32::from(j as u16).put(machine.element_mut(i.vd(), sew, j)),
            64 => E64::from(j as u16).put(machine.element_mut(i.vd(), sew, j)),
            128 => E128::from(j as u16).put(machine.element_mut(i.vd(), sew, j)),
            256 => E256::from(j as u16).put(machine.element_mut(i.vd(), sew, j)),
            512 => E512::from(j as u16).put(machine.element_mut(i.vd(), sew, j)),
            1024 => E1024::from(j as u16).put(machine.element_mut(i.vd(), sew, j)),
            _ => unreachable!(),
        }
    }
    Ok(())
}

pub fn handle_vmv_x_s<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    require_vill!(machine);
    let sew = machine.vsew();
    let i = VVtype(inst);
    let r = match sew {
        8 => E8::get(machine.element_ref(i.vs2(), sew, 0)).0 as i8 as i64 as u64,
        16 => E16::get(machine.element_ref(i.vs2(), sew, 0)).0 as i16 as i64 as u64,
        32 => E32::get(machine.element_ref(i.vs2(), sew, 0)).0 as i32 as i64 as u64,
        64 => E64::get(machine.element_ref(i.vs2(), sew, 0)).u64(),
        128 => E128::get(machine.element_ref(i.vs2(), sew, 0)).u64(),
        256 => E256::get(machine.element_ref(i.vs2(), sew, 0)).u64(),
        512 => E512::get(machine.element_ref(i.vs2(), sew, 0)).u64(),
        1024 => E1024::get(machine.element_ref(i.vs2(), sew, 0)).u64(),
        _ => unreachable!(),
    };
    update_register(machine, i.vd(), Mac::REG::from_u64(r));
    Ok(())
}

pub fn handle_vmv_s_x<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    require_vill!(machine);
    let sew = machine.vsew();
    let i = VVtype(inst);
    if machine.vl() != 0 {
        match sew {
            8 => {
                let e = E8::from(machine.registers()[i.vs1()].to_u64());
                e.put(machine.element_mut(i.vd(), sew, 0));
            }
            16 => {
                let e = E16::from(machine.registers()[i.vs1()].to_u64());
                e.put(machine.element_mut(i.vd(), sew, 0));
            }
            32 => {
                let e = E32::from(machine.registers()[i.vs1()].to_u64());
                e.put(machine.element_mut(i.vd(), sew, 0));
            }
            64 => {
                let e = E64::from(machine.registers()[i.vs1()].to_u64());
                e.put(machine.element_mut(i.vd(), sew, 0));
            }
            128 => {
                let e = E128::from(machine.registers()[i.vs1()].to_i64());
                e.put(machine.element_mut(i.vd(), sew, 0));
            }
            256 => {
                let e = E256::from(machine.registers()[i.vs1()].to_i64());
                e.put(machine.element_mut(i.vd(), sew, 0));
            }
            512 => {
                let e = E512::from(machine.registers()[i.vs1()].to_i64());
                e.put(machine.element_mut(i.vd(), sew, 0));
            }
            1024 => {
                let e = E1024::from(machine.registers()[i.vs1()].to_i64());
                e.put(machine.element_mut(i.vd(), sew, 0));
            }
            _ => unreachable!(),
        };
    }
    Ok(())
}

pub fn handle_vslideup_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    require_vill!(machine);
    let sew = machine.vsew();
    let lmul = machine.vlmul();
    let i = VXtype(inst);
    require_align!(i.vd() as u64, lmul as u64);
    require_align!(i.vs2() as u64, lmul as u64);
    require_noover!(i.vd() as u64, 1, i.vs2() as u64, 1);
    require_vm!(i);
    let offset = machine.registers()[i.rs1()].to_u64();
    for j in offset..machine.vl() {
        if i.vm() == 0 && !machine.get_bit(0, j as usize) {
            continue;
        }
        let data = machine
            .element_ref(i.vs2(), sew, (j - offset) as usize)
            .to_vec();
        machine
            .element_mut(i.vd(), sew, j as usize)
            .copy_from_slice(&data);
    }
    Ok(())
}

pub fn handle_vslideup_vi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    require_vill!(machine);
    let sew = machine.vsew();
    let lmul = machine.vlmul();
    let i = VItype(inst);
    require_align!(i.vd() as u64, lmul as u64);
    require_align!(i.vs2() as u64, lmul as u64);
    require_noover!(i.vd() as u64, 1, i.vs2() as u64, 1);
    require_vm!(i);
    let offset = i.immediate_u() as u64;
    for j in offset..machine.vl() {
        if i.vm() == 0 && !machine.get_bit(0, j as usize) {
            continue;
        }
        let data = machine
            .element_ref(i.vs2(), sew, (j - offset) as usize)
            .to_vec();
        machine
            .element_mut(i.vd(), sew, j as usize)
            .copy_from_slice(&data);
    }
    Ok(())
}

pub fn handle_vslidedown_vx<Mac: Machine>(
    machine: &mut Mac,
    inst: Instruction,
) -> Result<(), Error> {
    require_vill!(machine);
    let sew = machine.vsew();
    let lmul = machine.vlmul();
    let i = VXtype(inst);
    require_align!(i.vd() as u64, lmul as u64);
    require_align!(i.vs2() as u64, lmul as u64);
    require_vm!(i);
    let offset = machine.registers()[i.rs1()].to_u64();
    for j in 0..machine.vl() {
        if i.vm() == 0 && !machine.get_bit(0, j as usize) {
            continue;
        }
        let (l, overflow) = offset.overflowing_add(j);
        let data = if !overflow && l < machine.vlmax() {
            machine.element_ref(i.vs2(), sew, l as usize).to_vec()
        } else {
            vec![0; sew as usize >> 3]
        };
        machine
            .element_mut(i.vd(), sew, j as usize)
            .copy_from_slice(&data);
    }
    Ok(())
}

pub fn handle_vslidedown_vi<Mac: Machine>(
    machine: &mut Mac,
    inst: Instruction,
) -> Result<(), Error> {
    require_vill!(machine);
    let sew = machine.vsew();
    let lmul = machine.vlmul();
    let i = VItype(inst);
    require_align!(i.vd() as u64, lmul as u64);
    require_align!(i.vs2() as u64, lmul as u64);
    require_vm!(i);
    let offset = i.immediate_u() as u64;
    for j in 0..machine.vl() {
        if i.vm() == 0 && !machine.get_bit(0, j as usize) {
            continue;
        }
        let data = if (j + offset) < machine.vlmax() {
            machine
                .element_ref(i.vs2(), sew, (j + offset) as usize)
                .to_vec()
        } else {
            vec![0; sew as usize >> 3]
        };
        machine
            .element_mut(i.vd(), sew, j as usize)
            .copy_from_slice(&data);
    }
    Ok(())
}

pub fn handle_vslide1up_vx<Mac: Machine>(
    machine: &mut Mac,
    inst: Instruction,
) -> Result<(), Error> {
    require_vill!(machine);
    let sew = machine.vsew();
    let lmul = machine.vlmul();
    let i = VXtype(inst);
    require_align!(i.vd() as u64, lmul as u64);
    require_align!(i.vs2() as u64, lmul as u64);
    require_noover!(i.vd() as u64, 1, i.vs2() as u64, 1);
    require_vm!(i);
    if machine.vl() != 0 {
        for j in 1..machine.vl() {
            if i.vm() == 0 && !machine.get_bit(0, j as usize) {
                continue;
            }
            let data = machine.element_ref(i.vs2(), sew, (j - 1) as usize).to_vec();
            machine
                .element_mut(i.vd(), sew, j as usize)
                .copy_from_slice(&data);
        }
        if i.vm() != 0 || machine.get_bit(0, 0) {
            match sew {
                8 => {
                    let vd0 = E8::from(machine.registers()[i.rs1()].to_u64());
                    vd0.put(machine.element_mut(i.vd(), sew, 0));
                }
                16 => {
                    let vd0 = E16::from(machine.registers()[i.rs1()].to_u64());
                    vd0.put(machine.element_mut(i.vd(), sew, 0));
                }
                32 => {
                    let vd0 = E32::from(machine.registers()[i.rs1()].to_u64());
                    vd0.put(machine.element_mut(i.vd(), sew, 0));
                }
                64 => {
                    let vd0 = E64::from(machine.registers()[i.rs1()].to_u64());
                    vd0.put(machine.element_mut(i.vd(), sew, 0));
                }
                128 => {
                    let vd0 = E128::from(machine.registers()[i.rs1()].to_i64());
                    vd0.put(machine.element_mut(i.vd(), sew, 0));
                }
                256 => {
                    let vd0 = E256::from(machine.registers()[i.rs1()].to_i64());
                    vd0.put(machine.element_mut(i.vd(), sew, 0));
                }
                512 => {
                    let vd0 = E512::from(machine.registers()[i.rs1()].to_i64());
                    vd0.put(machine.element_mut(i.vd(), sew, 0));
                }
                1024 => {
                    let vd0 = E1024::from(machine.registers()[i.rs1()].to_i64());
                    vd0.put(machine.element_mut(i.vd(), sew, 0));
                }
                _ => unreachable!(),
            }
        }
    }
    Ok(())
}

pub fn handle_vslide1down_vx<Mac: Machine>(
    machine: &mut Mac,
    inst: Instruction,
) -> Result<(), Error> {
    require_vill!(machine);
    let sew = machine.vsew();
    let lmul = machine.vlmul();
    let i = VXtype(inst);
    require_align!(i.vd() as u64, lmul as u64);
    require_align!(i.vs2() as u64, lmul as u64);
    require_vm!(i);
    if machine.vl() != 0 {
        for j in 0..machine.vl() - 1 {
            if i.vm() == 0 && !machine.get_bit(0, j as usize) {
                continue;
            }
            let data = machine.element_ref(i.vs2(), sew, j as usize + 1).to_vec();
            machine
                .element_mut(i.vd(), sew, j as usize)
                .copy_from_slice(&data);
        }
        if i.vm() != 0 || machine.get_bit(0, machine.vl() as usize - 1) {
            match sew {
                8 => {
                    let vd0 = E8::from(machine.registers()[i.rs1()].to_u64());
                    vd0.put(machine.element_mut(i.vd(), sew, (machine.vl() - 1) as usize));
                }
                16 => {
                    let vd0 = E16::from(machine.registers()[i.rs1()].to_u64());
                    vd0.put(machine.element_mut(i.vd(), sew, (machine.vl() - 1) as usize));
                }
                32 => {
                    let vd0 = E32::from(machine.registers()[i.rs1()].to_u64());
                    vd0.put(machine.element_mut(i.vd(), sew, (machine.vl() - 1) as usize));
                }
                64 => {
                    let vd0 = E64::from(machine.registers()[i.rs1()].to_u64());
                    vd0.put(machine.element_mut(i.vd(), sew, (machine.vl() - 1) as usize));
                }
                128 => {
                    let vd0 = E128::from(machine.registers()[i.rs1()].to_i64());
                    vd0.put(machine.element_mut(i.vd(), sew, (machine.vl() - 1) as usize));
                }
                256 => {
                    let vd0 = E256::from(machine.registers()[i.rs1()].to_i64());
                    vd0.put(machine.element_mut(i.vd(), sew, (machine.vl() - 1) as usize));
                }
                512 => {
                    let vd0 = E512::from(machine.registers()[i.rs1()].to_i64());
                    vd0.put(machine.element_mut(i.vd(), sew, (machine.vl() - 1) as usize));
                }
                1024 => {
                    let vd0 = E1024::from(machine.registers()[i.rs1()].to_i64());
                    vd0.put(machine.element_mut(i.vd(), sew, (machine.vl() - 1) as usize));
                }
                _ => unreachable!(),
            }
        }
    }
    Ok(())
}

pub fn handle_vrgather_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    require_vill!(machine);
    let sew = machine.vsew();
    let lmul = machine.vlmul();
    let i = VVtype(inst);
    require_align!(i.vd() as u64, lmul as u64);
    require_align!(i.vs1() as u64, lmul as u64);
    require_align!(i.vs2() as u64, lmul as u64);
    require!(i.vd() != i.vs1(), String::from("require: vd != vs1"));
    require!(i.vd() != i.vs2(), String::from("require: vd != vs2"));
    require_vm!(i);
    for j in 0..machine.vl() as usize {
        if i.vm() == 0 && !machine.get_bit(0, j) {
            continue;
        }
        let index = {
            let mut data = machine.element_ref(i.vs1(), sew, j).to_vec();
            data.resize(128, 0);
            E1024::get(&data)
        };
        let data = if index < E1024::from(machine.vlmax()) {
            machine
                .element_ref(i.vs2(), sew, index.u64() as usize)
                .to_vec()
        } else {
            vec![0; sew as usize >> 3]
        };
        machine.element_mut(i.vd(), sew, j).copy_from_slice(&data);
    }
    Ok(())
}

pub fn handle_vrgather_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    require_vill!(machine);
    let sew = machine.vsew();
    let lmul = machine.vlmul();
    let i = VXtype(inst);
    require_align!(i.vd() as u64, lmul as u64);
    require_align!(i.vs2() as u64, lmul as u64);
    require!(i.vd() != i.vs2(), String::from("require: vd != vs2"));
    require_vm!(i);
    for j in 0..machine.vl() as usize {
        if i.vm() == 0 && !machine.get_bit(0, j) {
            continue;
        }
        let index = machine.registers()[i.rs1()].to_u64();
        let data = if index < machine.vlmax() {
            machine.element_ref(i.vs2(), sew, index as usize).to_vec()
        } else {
            vec![0; sew as usize >> 3]
        };
        machine.element_mut(i.vd(), sew, j).copy_from_slice(&data);
    }
    Ok(())
}

pub fn handle_vrgather_vi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    require_vill!(machine);
    let sew = machine.vsew();
    let lmul = machine.vlmul();
    let i = VItype(inst);
    require_align!(i.vd() as u64, lmul as u64);
    require_align!(i.vs2() as u64, lmul as u64);
    require!(i.vd() != i.vs2(), String::from("require: vd != vs2"));
    require_vm!(i);
    for j in 0..machine.vl() as usize {
        if i.vm() == 0 && !machine.get_bit(0, j) {
            continue;
        }
        let index = i.immediate_u() as u64;
        let data = if index < machine.vlmax() {
            machine.element_ref(i.vs2(), sew, index as usize).to_vec()
        } else {
            vec![0; sew as usize >> 3]
        };
        machine.element_mut(i.vd(), sew, j).copy_from_slice(&data);
    }
    Ok(())
}

pub fn handle_vrghtherei16_vv<Mac: Machine>(
    machine: &mut Mac,
    inst: Instruction,
) -> Result<(), Error> {
    require_vill!(machine);
    let sew = machine.vsew();
    let lmul = machine.vlmul();
    let i = VVtype(inst);
    let emul = 16.0 / sew as f64 * lmul;
    require_emul!(emul);
    require_align!(i.vd() as u64, lmul as u64);
    require_align!(i.vs1() as u64, emul as u64);
    require_align!(i.vs2() as u64, lmul as u64);
    require_noover!(i.vd() as u64, lmul as u64, i.vs1() as u64, emul as u64);
    require!(i.vd() != i.vs2(), String::from("require: vd != vs2"));
    require_vm!(i);
    for j in 0..machine.vl() as usize {
        if i.vm() == 0 && !machine.get_bit(0, j) {
            continue;
        }
        let index = E16::get(&machine.element_ref(i.vs1(), 16, j)).u64();
        let data = if index < machine.vlmax() {
            machine.element_ref(i.vs2(), sew, index as usize).to_vec()
        } else {
            vec![0; sew as usize >> 3]
        };
        machine.element_mut(i.vd(), sew, j).copy_from_slice(&data);
    }
    Ok(())
}

pub fn handle_vcompress_vm<Mac: Machine>(
    machine: &mut Mac,
    inst: Instruction,
) -> Result<(), Error> {
    require_vill!(machine);
    let sew = machine.vsew();
    let lmul = machine.vlmul();
    let i = VVtype(inst);
    require_align!(i.vd() as u64, lmul as u64);
    require_align!(i.vs2() as u64, lmul as u64);
    require!(i.vd() != i.vs2(), String::from("require: vd != vs2"));
    require_noover!(i.vd() as u64, lmul as u64, i.vs1() as u64, 1);
    let mut k = 0;
    for j in 0..machine.vl() as usize {
        if machine.get_bit(i.vs1(), j) {
            let data = machine.element_ref(i.vs2(), sew, j).to_vec();
            machine.element_mut(i.vd(), sew, k).copy_from_slice(&data);
            k += 1;
        }
    }
    Ok(())
}

pub fn handle_vmv1r_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    vmv_r!(inst, machine, 1);
    Ok(())
}

pub fn handle_vmv2r_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    vmv_r!(inst, machine, 2);
    Ok(())
}

pub fn handle_vmv4r_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    vmv_r!(inst, machine, 4);
    Ok(())
}

pub fn handle_vmv8r_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    vmv_r!(inst, machine, 8);
    Ok(())
}

// ------------------------------------------------------------------------------------------------

pub type HandleFunction<Mac> = fn(&mut Mac, Instruction) -> Result<(), Error>;

#[rustfmt::skip]
pub fn generate_handle_function_list<Mac: Machine>() -> [Option<HandleFunction<Mac>>; 65536] {
    let mut handle_function_list = [None; 65536];
    handle_function_list[insts::OP_UNLOADED as usize] = Some(handle_unloaded::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_ADD as usize] = Some(handle_add::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_ADDI as usize] = Some(handle_addi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_ADDIW as usize] = Some(handle_addiw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_ADDW as usize] = Some(handle_addw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_AND as usize] = Some(handle_and::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_ANDI as usize] = Some(handle_andi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_DIV as usize] = Some(handle_div::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_DIVU as usize] = Some(handle_divu::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_DIVUW as usize] = Some(handle_divuw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_DIVW as usize] = Some(handle_divw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_FENCE as usize] = Some(handle_fence::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_FENCEI as usize] = Some(handle_fencei::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_LB as usize] = Some(handle_lb::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_LBU as usize] = Some(handle_lbu::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_LD as usize] = Some(handle_ld::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_LH as usize] = Some(handle_lh::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_LHU as usize] = Some(handle_lhu::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_LUI as usize] = Some(handle_lui::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_LW as usize] = Some(handle_lw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_LWU as usize] = Some(handle_lwu::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_MUL as usize] = Some(handle_mul::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_MULH as usize] = Some(handle_mulh::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_MULHSU as usize] = Some(handle_mulhsu::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_MULHU as usize] = Some(handle_mulhu::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_MULW as usize] = Some(handle_mulw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_OR as usize] = Some(handle_or::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_ORI as usize] = Some(handle_ori::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_REM as usize] = Some(handle_rem::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_REMU as usize] = Some(handle_remu::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_REMUW as usize] = Some(handle_remuw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_REMW as usize] = Some(handle_remw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SB as usize] = Some(handle_sb::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SD as usize] = Some(handle_sd::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SH as usize] = Some(handle_sh::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SLL as usize] = Some(handle_sll::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SLLI as usize] = Some(handle_slli::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SLLIW as usize] = Some(handle_slliw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SLLW as usize] = Some(handle_sllw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SLT as usize] = Some(handle_slt::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SLTI as usize] = Some(handle_slti::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SLTIU as usize] = Some(handle_sltiu::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SLTU as usize] = Some(handle_sltu::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SRA as usize] = Some(handle_sra::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SRAI as usize] = Some(handle_srai::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SRAIW as usize] = Some(handle_sraiw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SRAW as usize] = Some(handle_sraw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SRL as usize] = Some(handle_srl::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SRLI as usize] = Some(handle_srli::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SRLIW as usize] = Some(handle_srliw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SRLW as usize] = Some(handle_srlw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SUB as usize] = Some(handle_sub::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SUBW as usize] = Some(handle_subw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SW as usize] = Some(handle_sw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_XOR as usize] = Some(handle_xor::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_XORI as usize] = Some(handle_xori::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_ADDUW as usize] = Some(handle_adduw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_ANDN as usize] = Some(handle_andn::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_BCLR as usize] = Some(handle_bclr::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_BCLRI as usize] = Some(handle_bclri::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_BEXT as usize] = Some(handle_bext::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_BEXTI as usize] = Some(handle_bexti::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_BINV as usize] = Some(handle_binv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_BINVI as usize] = Some(handle_binvi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_BSET as usize] = Some(handle_bset::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_BSETI as usize] = Some(handle_bseti::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_CLMUL as usize] = Some(handle_clmul::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_CLMULH as usize] = Some(handle_clmulh::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_CLMULR as usize] = Some(handle_clmulr::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_CLZ as usize] = Some(handle_clz::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_CLZW as usize] = Some(handle_clzw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_CPOP as usize] = Some(handle_cpop::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_CPOPW as usize] = Some(handle_cpopw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_CTZ as usize] = Some(handle_ctz::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_CTZW as usize] = Some(handle_ctzw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_MAX as usize] = Some(handle_max::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_MAXU as usize] = Some(handle_maxu::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_MIN as usize] = Some(handle_min::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_MINU as usize] = Some(handle_minu::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_ORCB as usize] = Some(handle_orcb::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_ORN as usize] = Some(handle_orn::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_REV8 as usize] = Some(handle_rev8::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_ROL as usize] = Some(handle_rol::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_ROLW as usize] = Some(handle_rolw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_ROR as usize] = Some(handle_ror::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_RORI as usize] = Some(handle_rori::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_RORIW as usize] = Some(handle_roriw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_RORW as usize] = Some(handle_rorw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SEXTB as usize] = Some(handle_sextb::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SEXTH as usize] = Some(handle_sexth::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SH1ADD as usize] = Some(handle_sh1add::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SH1ADDUW as usize] = Some(handle_sh1adduw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SH2ADD as usize] = Some(handle_sh2add::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SH2ADDUW as usize] = Some(handle_sh2adduw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SH3ADD as usize] = Some(handle_sh3add::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SH3ADDUW as usize] = Some(handle_sh3adduw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SLLIUW as usize] = Some(handle_slliuw::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_XNOR as usize] = Some(handle_xnor::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_ZEXTH as usize] = Some(handle_zexth::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_WIDE_MUL as usize] = Some(handle_wide_mul::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_WIDE_MULU as usize] = Some(handle_wide_mulu::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_WIDE_MULSU as usize] = Some(handle_wide_mulsu::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_WIDE_DIV as usize] = Some(handle_wide_div::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_WIDE_DIVU as usize] = Some(handle_wide_divu::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_LD_SIGN_EXTENDED_32_CONSTANT as usize] = Some(handle_ld_sign_extended_32_constant::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_ADC as usize] = Some(handle_adc::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_SBB as usize] = Some(handle_sbb::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_CUSTOM_LOAD_IMM as usize] = Some(handle_custom_load_imm::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_AUIPC as usize] = Some(handle_auipc::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_BEQ as usize] = Some(handle_beq::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_BGE as usize] = Some(handle_bge::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_BGEU as usize] = Some(handle_bgeu::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_BLT as usize] = Some(handle_blt::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_BLTU as usize] = Some(handle_bltu::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_BNE as usize] = Some(handle_bne::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_EBREAK as usize] = Some(handle_ebreak::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_ECALL as usize] = Some(handle_ecall::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_JAL as usize] = Some(handle_jal::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_JALR as usize] = Some(handle_jalr::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_FAR_JUMP_REL as usize] = Some(handle_far_jump_rel::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_FAR_JUMP_ABS as usize] = Some(handle_far_jump_abs::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_CUSTOM_TRACE_END as usize] = Some(handle_custom_trace_end::<Mac> as HandleFunction::<Mac>);

    // rvv
    handle_function_list[insts::OP_VSETVLI as usize] = Some(handle_vsetvli::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSETIVLI as usize] = Some(handle_vsetivli::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSETVL as usize] = Some(handle_vsetvl::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLM_V as usize] = Some(handle_vlm_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLE8_V as usize] = Some(handle_vle8_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLE16_V as usize] = Some(handle_vle16_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLE32_V as usize] = Some(handle_vle32_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLE64_V as usize] = Some(handle_vle64_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLE128_V as usize] = Some(handle_vle128_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLE256_V as usize] = Some(handle_vle256_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLE512_V as usize] = Some(handle_vle512_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLE1024_V as usize] = Some(handle_vle1024_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSM_V as usize] = Some(handle_vsm_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSE8_V as usize] = Some(handle_vse8_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSE16_V as usize] = Some(handle_vse16_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSE32_V as usize] = Some(handle_vse32_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSE64_V as usize] = Some(handle_vse64_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSE128_V as usize] = Some(handle_vse128_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSE256_V as usize] = Some(handle_vse256_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSE512_V as usize] = Some(handle_vse512_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSE1024_V as usize] = Some(handle_vse1024_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLSE8_V as usize] = Some(handle_vlse8_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLSE16_V as usize] = Some(handle_vlse16_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLSE32_V as usize] = Some(handle_vlse32_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLSE64_V as usize] = Some(handle_vlse64_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLSE128_V as usize] = Some(handle_vlse128_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLSE256_V as usize] = Some(handle_vlse256_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLSE512_V as usize] = Some(handle_vlse512_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLSE1024_V as usize] = Some(handle_vlse1024_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSSE8_V as usize] = Some(handle_vsse8_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSSE16_V as usize] = Some(handle_vsse16_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSSE32_V as usize] = Some(handle_vsse32_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSSE64_V as usize] = Some(handle_vsse64_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSSE128_V as usize] = Some(handle_vsse128_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSSE256_V as usize] = Some(handle_vsse256_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSSE512_V as usize] = Some(handle_vsse512_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSSE1024_V as usize] = Some(handle_vsse1024_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLUXEI8_V as usize] = Some(handle_vluxei8_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLUXEI16_V as usize] = Some(handle_vluxei16_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLUXEI32_V as usize] = Some(handle_vluxei32_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLUXEI64_V as usize] = Some(handle_vluxei64_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLOXEI8_V as usize] = Some(handle_vloxei8_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLOXEI16_V as usize] = Some(handle_vloxei16_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLOXEI32_V as usize] = Some(handle_vloxei32_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VLOXEI64_V as usize] = Some(handle_vloxei64_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSUXEI8_V as usize] = Some(handle_vsuxei8_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSUXEI16_V as usize] = Some(handle_vsuxei16_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSUXEI32_V as usize] = Some(handle_vsuxei32_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSUXEI64_V as usize] = Some(handle_vsuxei64_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSOXEI8_V as usize] = Some(handle_vsoxei8_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSOXEI16_V as usize] = Some(handle_vsoxei16_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSOXEI32_V as usize] = Some(handle_vsoxei32_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSOXEI64_V as usize] = Some(handle_vsoxei64_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VL1RE8_V as usize] = Some(handle_vl1re8_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VL1RE16_V as usize] = Some(handle_vl1re16_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VL1RE32_V as usize] = Some(handle_vl1re32_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VL1RE64_V as usize] = Some(handle_vl1re64_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VL2RE8_V as usize] = Some(handle_vl2re8_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VL2RE16_V as usize] = Some(handle_vl2re16_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VL2RE32_V as usize] = Some(handle_vl2re32_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VL2RE64_V as usize] = Some(handle_vl2re64_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VL4RE8_V as usize] = Some(handle_vl4re8_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VL4RE16_V as usize] = Some(handle_vl4re16_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VL4RE32_V as usize] = Some(handle_vl4re32_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VL4RE64_V as usize] = Some(handle_vl4re64_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VL8RE8_V as usize] = Some(handle_vl8re8_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VL8RE16_V as usize] = Some(handle_vl8re16_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VL8RE32_V as usize] = Some(handle_vl8re32_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VL8RE64_V as usize] = Some(handle_vl8re64_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VS1R_V as usize] = Some(handle_vs1r_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VS2R_V as usize] = Some(handle_vs2r_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VS4R_V as usize] = Some(handle_vs4r_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VS8R_V as usize] = Some(handle_vs8r_v::<Mac> as HandleFunction::<Mac>);

    handle_function_list[insts::OP_VADD_VV as usize] = Some(handle_vadd_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VADD_VX as usize] = Some(handle_vadd_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VADD_VI as usize] = Some(handle_vadd_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSUB_VV as usize] = Some(handle_vsub_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSUB_VX as usize] = Some(handle_vsub_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VRSUB_VX as usize] = Some(handle_vrsub_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VRSUB_VI as usize] = Some(handle_vrsub_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWADDU_VV as usize] = Some(handle_vw_addu_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWADDU_VX as usize] = Some(handle_vw_addu_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWSUBU_VV as usize] = Some(handle_vw_subu_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWSUBU_VX as usize] = Some(handle_vw_subu_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWADD_VV as usize] = Some(handle_vwadd_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWADD_VX as usize] = Some(handle_vwadd_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWSUB_VV as usize] = Some(handle_vwsub_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWSUB_VX as usize] = Some(handle_vwsub_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWADDU_WV as usize] = Some(handle_vwaddu_wv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWADDU_WX as usize] = Some(handle_vwaddu_wx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWSUBU_WV as usize] = Some(handle_vwsubu_wv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWSUBU_WX as usize] = Some(handle_vwsubu_wx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWADD_WV as usize] = Some(handle_vwadd_wv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWADD_WX as usize] = Some(handle_vwadd_wx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWSUB_WV as usize] = Some(handle_vwsub_wv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWSUB_WX as usize] = Some(handle_vwsub_wx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VZEXT_VF2 as usize] = Some(handle_vzext_vf2::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VZEXT_VF4 as usize] = Some(handle_vzext_vf4::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VZEXT_VF8 as usize] = Some(handle_vzext_vf8::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSEXT_VF2 as usize] = Some(handle_vsext_vf2::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSEXT_VF4 as usize] = Some(handle_vsext_vf4::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSEXT_VF8 as usize] = Some(handle_vsext_vf8::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VADC_VVM as usize] = Some(handle_vadc_vvm::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VADC_VXM as usize] = Some(handle_vadc_vxm::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VADC_VIM as usize] = Some(handle_vadc_vim::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMADC_VVM as usize] = Some(handle_vmadc_vvm::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMADC_VXM as usize] = Some(handle_vmadc_vxm::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMADC_VIM as usize] = Some(handle_vmadc_vim::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMADC_VV as usize] = Some(handle_vmadc_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMADC_VX as usize] = Some(handle_vmadc_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMADC_VI as usize] = Some(handle_vmadc_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSBC_VVM as usize] = Some(handle_vsbc_vvm::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSBC_VXM as usize] = Some(handle_vsbc_vxm::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSBC_VVM as usize] = Some(handle_vmsbc_vvm::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSBC_VXM as usize] = Some(handle_vmsbc_vxm::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSBC_VV as usize] = Some(handle_vmsbc_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSBC_VX as usize] = Some(handle_vmsbc_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VAND_VV as usize] = Some(handle_vand_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VAND_VX as usize] = Some(handle_vand_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VAND_VI as usize] = Some(handle_vand_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VOR_VV as usize] = Some(handle_vor_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VOR_VX as usize] = Some(handle_vor_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VOR_VI as usize] = Some(handle_vor_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VXOR_VV as usize] = Some(handle_vxor_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VXOR_VX as usize] = Some(handle_vxor_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VXOR_VI as usize] = Some(handle_vxor_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSLL_VV as usize] = Some(handle_vsll_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSLL_VX as usize] = Some(handle_vsll_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSLL_VI as usize] = Some(handle_vsll_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSRL_VV as usize] = Some(handle_vsrl_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSRL_VX as usize] = Some(handle_vsrl_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSRL_VI as usize] = Some(handle_vsrl_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSRA_VV as usize] = Some(handle_vsra_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSRA_VX as usize] = Some(handle_vsra_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSRA_VI as usize] = Some(handle_vsra_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VNSRL_WV as usize] = Some(handle_vnsrl_wv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VNSRL_WX as usize] = Some(handle_vnsrl_wx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VNSRL_WI as usize] = Some(handle_vnsrl_wi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VNSRA_WV as usize] = Some(handle_vnsra_wv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VNSRA_WX as usize] = Some(handle_vnsra_wx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VNSRA_WI as usize] = Some(handle_vnsra_wi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSEQ_VV as usize] = Some(handle_vmseq_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSEQ_VX as usize] = Some(handle_vmseq_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSEQ_VI as usize] = Some(handle_vmseq_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSNE_VV as usize] = Some(handle_vmsne_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSNE_VX as usize] = Some(handle_vmsne_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSNE_VI as usize] = Some(handle_vmsne_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSLTU_VV as usize] = Some(handle_vmsltu_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSLTU_VX as usize] = Some(handle_vmsltu_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSLT_VV as usize] = Some(handle_vmslt_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSLT_VX as usize] = Some(handle_vmslt_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSLEU_VV as usize] = Some(handle_vmsleu_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSLEU_VX as usize] = Some(handle_vmsleu_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSLEU_VI as usize] = Some(handle_vmsleu_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSLE_VV as usize] = Some(handle_vmsle_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSLE_VX as usize] = Some(handle_vmsle_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSLE_VI as usize] = Some(handle_vmsle_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSGTU_VX as usize] = Some(handle_vmsgtu_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSGTU_VI as usize] = Some(handle_vmsgtu_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSGT_VX as usize] = Some(handle_vmsgt_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSGT_VI as usize] = Some(handle_vmsgt_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMAXU_VV as usize] = Some(handle_vmaxu_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMAXU_VX as usize] = Some(handle_vmaxu_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMAX_VV as usize] = Some(handle_vmax_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMAX_VX as usize] = Some(handle_vmax_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMINU_VV as usize] = Some(handle_vminu_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMINU_VX as usize] = Some(handle_vminu_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMIN_VV as usize] = Some(handle_vmin_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMIN_VX as usize] = Some(handle_vmin_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMUL_VV as usize] = Some(handle_vmul_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMUL_VX as usize] = Some(handle_vmul_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMULH_VV as usize] = Some(handle_vmulh_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMULH_VX as usize] = Some(handle_vmulh_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMULHU_VV as usize] = Some(handle_vmulhu_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMULHU_VX as usize] = Some(handle_vmulhu_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMULHSU_VV as usize] = Some(handle_vmulhsu_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMULHSU_VX as usize] = Some(handle_vmulhsu_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VDIVU_VV as usize] = Some(handle_vdivu_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VDIVU_VX as usize] = Some(handle_vdivu_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VDIV_VV as usize] = Some(handle_vdiv_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VDIV_VX as usize] = Some(handle_vdiv_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VREMU_VV as usize] = Some(handle_vremu_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VREMU_VX as usize] = Some(handle_vremu_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VREM_VV as usize] = Some(handle_vrem_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VREM_VX as usize] = Some(handle_vrem_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWMULU_VV as usize] = Some(handle_vwmulu_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWMULU_VX as usize] = Some(handle_vwmulu_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWMULSU_VV as usize] = Some(handle_vwmulsu_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWMULSU_VX as usize] = Some(handle_vwmulsu_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWMUL_VV as usize] = Some(handle_vwmul_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWMUL_VX as usize] = Some(handle_vwmul_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMACC_VV as usize] = Some(handle_vmacc_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMACC_VX as usize] = Some(handle_vmacc_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VNMSAC_VV as usize] = Some(handle_vnmsac_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VNMSAC_VX as usize] = Some(handle_vnmsac_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMADD_VV as usize] = Some(handle_vmadd_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMADD_VX as usize] = Some(handle_vmadd_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VNMSUB_VV as usize] = Some(handle_vnmsub_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VNMSUB_VX as usize] = Some(handle_vnmsub_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWMACCU_VV as usize] = Some(handle_vwmaccu_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWMACCU_VX as usize] = Some(handle_vwmaccu_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWMACC_VV as usize] = Some(handle_vwmacc_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWMACC_VX as usize] = Some(handle_vwmacc_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWMACCSU_VV as usize] = Some(handle_vwmaccsu_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWMACCSU_VX as usize] = Some(handle_vwmaccsu_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWMACCUS_VX as usize] = Some(handle_vwmaccus_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMERGE_VVM as usize] = Some(handle_vmerge_vvm::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMERGE_VXM as usize] = Some(handle_vmerge_vxm::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMERGE_VIM as usize] = Some(handle_vmerge_vim::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMV_V_V as usize] = Some(handle_vmv_v_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMV_V_X as usize] = Some(handle_vmv_v_x::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMV_V_I as usize] = Some(handle_vmv_v_i::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSADDU_VV as usize] = Some(handle_vsaddu_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSADDU_VX as usize] = Some(handle_vsaddu_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSADDU_VI as usize] = Some(handle_vsaddu_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSADD_VV as usize] = Some(handle_vsadd_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSADD_VX as usize] = Some(handle_vsadd_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSADD_VI as usize] = Some(handle_vsadd_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSSUBU_VV as usize] = Some(handle_vssubu_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSSUBU_VX as usize] = Some(handle_vssubu_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSSUB_VV as usize] = Some(handle_vssub_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSSUB_VX as usize] = Some(handle_vssub_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VAADD_VV as usize] = Some(handle_vaadd_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VAADD_VX as usize] = Some(handle_vaadd_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VAADDU_VV as usize] = Some(handle_vaaddu_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VAADDU_VX as usize] = Some(handle_vaaddu_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VASUB_VV as usize] = Some(handle_vasub_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VASUB_VX as usize] = Some(handle_vasub_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VASUBU_VV as usize] = Some(handle_vasubu_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VASUBU_VX as usize] = Some(handle_vasubu_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSMUL_VV as usize] = Some(handle_vsmul_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSMUL_VX as usize] = Some(handle_vsmul_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSSRL_VV as usize] = Some(handle_vssrl_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSSRL_VX as usize] = Some(handle_vssrl_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSSRL_VI as usize] = Some(handle_vssrl_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSSRA_VV as usize] = Some(handle_vssra_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSSRA_VX as usize] = Some(handle_vssra_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSSRA_VI as usize] = Some(handle_vssra_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VNCLIPU_WV as usize] = Some(handle_vnclipu_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VNCLIPU_WX as usize] = Some(handle_vnclipu_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VNCLIPU_WI as usize] = Some(handle_vnclipu_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VNCLIP_WV as usize] = Some(handle_vnclip_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VNCLIP_WX as usize] = Some(handle_vnclip_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VNCLIP_WI as usize] = Some(handle_vnclip_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VREDSUM_VS as usize] = Some(handle_vredsum_vs::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VREDAND_VS as usize] = Some(handle_vredand_vs::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VREDOR_VS as usize] = Some(handle_vredor_vs::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VREDXOR_VS as usize] = Some(handle_vredxor_vs::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VREDMINU_VS as usize] = Some(handle_vredminu_vs::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VREDMIN_VS as usize] = Some(handle_vredmin_vs::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VREDMAXU_VS as usize] = Some(handle_vredmaxu_vs::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VREDMAX_VS as usize] = Some(handle_vredmax_vs::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWREDSUMU_VS as usize] = Some(handle_vwredsumu_vs::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VWREDSUM_VS as usize] = Some(handle_vwredsum_vs::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMAND_MM as usize] = Some(handle_vmand_mm::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMNAND_MM as usize] = Some(handle_vmnand_mm::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMANDNOT_MM as usize] = Some(handle_vmandnot_mm::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMXOR_MM as usize] = Some(handle_vmxor_mm::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMOR_MM as usize] = Some(handle_vmor_mm::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMNOR_MM as usize] = Some(handle_vmnor_mm::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMORNOT_MM as usize] = Some(handle_vmornot_mm::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMXNOR_MM as usize] = Some(handle_vmxnor_mm::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VCPOP_M as usize] = Some(handle_vcpop_m::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VFIRST_M as usize] = Some(handle_vfirst_m::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSBF_M as usize] = Some(handle_vsbf_m::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSIF_M as usize] = Some(handle_vsif_m::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMSOF_M as usize] = Some(handle_vsof_m::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VIOTA_M as usize] = Some(handle_viota_m::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VID_V as usize] = Some(handle_vid_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMV_X_S as usize] = Some(handle_vmv_x_s::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMV_S_X as usize] = Some(handle_vmv_s_x::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSLIDEUP_VX as usize] = Some(handle_vslideup_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSLIDEUP_VI as usize] = Some(handle_vslideup_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSLIDEDOWN_VX as usize] = Some(handle_vslidedown_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSLIDEDOWN_VI as usize] = Some(handle_vslidedown_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSLIDE1UP_VX as usize] = Some(handle_vslide1up_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VSLIDE1DOWN_VX as usize] = Some(handle_vslide1down_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VRGATHER_VV as usize] = Some(handle_vrgather_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VRGATHER_VX as usize] = Some(handle_vrgather_vx::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VRGATHER_VI as usize] = Some(handle_vrgather_vi::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VRGATHEREI16_VV as usize] = Some(handle_vrghtherei16_vv::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VCOMPRESS_VM as usize] = Some(handle_vcompress_vm::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMV1R_V as usize] = Some(handle_vmv1r_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMV2R_V as usize] = Some(handle_vmv2r_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMV4R_V as usize] = Some(handle_vmv4r_v::<Mac> as HandleFunction::<Mac>);
    handle_function_list[insts::OP_VMV8R_V as usize] = Some(handle_vmv8r_v::<Mac> as HandleFunction::<Mac>);

    return handle_function_list;
}

pub fn execute_instruction<Mac: Machine>(
    machine: &mut Mac,
    handle_function_list: &[Option<HandleFunction<Mac>>],
    inst: Instruction,
) -> Result<(), Error> {
    let op = extract_opcode(inst);
    if let Some(f) = handle_function_list[op as usize] {
        f(machine, inst)?;
        return Ok(());
    }
    Ok(())
}

pub fn execute<Mac: Machine>(
    machine: &mut Mac,
    handle_function_list: &[Option<HandleFunction<Mac>>],
    inst: Instruction,
) -> Result<(), Error> {
    let instruction_size = instruction_length(inst);
    let next_pc = machine
        .pc()
        .overflowing_add(&Mac::REG::from_u8(instruction_size));
    machine.update_pc(next_pc);
    let r = execute_instruction(machine, handle_function_list, inst);
    machine.commit_pc();
    r
}
