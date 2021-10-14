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
    let r = execute_v_instruction(machine, inst)?;
    if r == false {
        return Err(Error::InvalidOp(extract_opcode(inst)));
    }
    Ok(())
}

pub fn execute_v_instruction<Mac: Machine>(
    machine: &mut Mac,
    inst: Instruction,
) -> Result<bool, Error> {
    let op = extract_opcode(inst);
    match op {
        insts::OP_VSETVLI => {
            let i = Itype(inst);
            common::set_vl(
                machine,
                i.rd(),
                i.rs1(),
                machine.registers()[i.rs1()].to_u64(),
                i.immediate_u() as u64,
            )?;
        }
        insts::OP_VSETIVLI => {
            let i = Itype(inst);
            common::set_vl(machine, i.rd(), 33, i.rs1() as u64, i.immediate_u() as u64)?;
        }
        insts::OP_VSETVL => {
            let i = Rtype(inst);
            common::set_vl(
                machine,
                i.rd(),
                i.rs1(),
                machine.registers()[i.rs1()].to_u64(),
                machine.registers()[i.rs2()].to_u64(),
            )?;
        }
        insts::OP_VLM_V => {
            ld!(inst, machine, (machine.vl() + 7) / 8, 0, 1, 0);
        }
        insts::OP_VLE8_V => {
            ld!(inst, machine, machine.vl(), 0, 1, 1);
        }
        insts::OP_VLE16_V => {
            ld!(inst, machine, machine.vl(), 0, 2, 1);
        }
        insts::OP_VLE32_V => {
            ld!(inst, machine, machine.vl(), 0, 4, 1);
        }
        insts::OP_VLE64_V => {
            ld!(inst, machine, machine.vl(), 0, 8, 1);
        }
        insts::OP_VLE128_V => {
            ld!(inst, machine, machine.vl(), 0, 16, 1);
        }
        insts::OP_VLE256_V => {
            ld!(inst, machine, machine.vl(), 0, 32, 1);
        }
        insts::OP_VLE512_V => {
            ld!(inst, machine, machine.vl(), 0, 64, 1);
        }
        insts::OP_VLE1024_V => {
            ld!(inst, machine, machine.vl(), 0, 128, 1);
        }
        insts::OP_VSM_V => {
            sd!(inst, machine, (machine.vl() + 7) / 8, 0, 1, 0);
        }
        insts::OP_VSE8_V => {
            sd!(inst, machine, machine.vl(), 0, 1, 1);
        }
        insts::OP_VSE16_V => {
            sd!(inst, machine, machine.vl(), 0, 2, 1);
        }
        insts::OP_VSE32_V => {
            sd!(inst, machine, machine.vl(), 0, 4, 1);
        }
        insts::OP_VSE64_V => {
            sd!(inst, machine, machine.vl(), 0, 8, 1);
        }
        insts::OP_VSE128_V => {
            sd!(inst, machine, machine.vl(), 0, 16, 1);
        }
        insts::OP_VSE256_V => {
            sd!(inst, machine, machine.vl(), 0, 32, 1);
        }
        insts::OP_VSE512_V => {
            sd!(inst, machine, machine.vl(), 0, 64, 1);
        }
        insts::OP_VSE1024_V => {
            sd!(inst, machine, machine.vl(), 0, 128, 1);
        }
        insts::OP_VLSE8_V => {
            ld!(inst, machine, machine.vl(), 1, 1, 1);
        }
        insts::OP_VLSE16_V => {
            ld!(inst, machine, machine.vl(), 1, 2, 1);
        }
        insts::OP_VLSE32_V => {
            ld!(inst, machine, machine.vl(), 1, 4, 1);
        }
        insts::OP_VLSE64_V => {
            ld!(inst, machine, machine.vl(), 1, 8, 1);
        }
        insts::OP_VLSE128_V => {
            ld!(inst, machine, machine.vl(), 1, 16, 1);
        }
        insts::OP_VLSE256_V => {
            ld!(inst, machine, machine.vl(), 1, 32, 1);
        }
        insts::OP_VLSE512_V => {
            ld!(inst, machine, machine.vl(), 1, 64, 1);
        }
        insts::OP_VLSE1024_V => {
            ld!(inst, machine, machine.vl(), 1, 128, 1);
        }
        insts::OP_VSSE8_V => {
            sd!(inst, machine, machine.vl(), 1, 1, 1);
        }
        insts::OP_VSSE16_V => {
            sd!(inst, machine, machine.vl(), 1, 2, 1);
        }
        insts::OP_VSSE32_V => {
            sd!(inst, machine, machine.vl(), 1, 4, 1);
        }
        insts::OP_VSSE64_V => {
            sd!(inst, machine, machine.vl(), 1, 8, 1);
        }
        insts::OP_VSSE128_V => {
            sd!(inst, machine, machine.vl(), 1, 16, 1);
        }
        insts::OP_VSSE256_V => {
            sd!(inst, machine, machine.vl(), 1, 32, 1);
        }
        insts::OP_VSSE512_V => {
            sd!(inst, machine, machine.vl(), 1, 64, 1);
        }
        insts::OP_VSSE1024_V => {
            sd!(inst, machine, machine.vl(), 1, 128, 1);
        }
        insts::OP_VLUXEI8_V => {
            ld_index!(inst, machine, 8);
        }
        insts::OP_VLUXEI16_V => {
            ld_index!(inst, machine, 16);
        }
        insts::OP_VLUXEI32_V => {
            ld_index!(inst, machine, 32);
        }
        insts::OP_VLUXEI64_V => {
            ld_index!(inst, machine, 64);
        }
        insts::OP_VLOXEI8_V => {
            ld_index!(inst, machine, 8);
        }
        insts::OP_VLOXEI16_V => {
            ld_index!(inst, machine, 16);
        }
        insts::OP_VLOXEI32_V => {
            ld_index!(inst, machine, 32);
        }
        insts::OP_VLOXEI64_V => {
            ld_index!(inst, machine, 64);
        }
        insts::OP_VSUXEI8_V => {
            sd_index!(inst, machine, 8);
        }
        insts::OP_VSUXEI16_V => {
            sd_index!(inst, machine, 16);
        }
        insts::OP_VSUXEI32_V => {
            sd_index!(inst, machine, 32);
        }
        insts::OP_VSUXEI64_V => {
            sd_index!(inst, machine, 64);
        }
        insts::OP_VSOXEI8_V => {
            sd_index!(inst, machine, 8);
        }
        insts::OP_VSOXEI16_V => {
            sd_index!(inst, machine, 16);
        }
        insts::OP_VSOXEI32_V => {
            sd_index!(inst, machine, 32);
        }
        insts::OP_VSOXEI64_V => {
            sd_index!(inst, machine, 64);
        }
        insts::OP_VL1RE8_V => {
            ld_whole!(inst, machine, VLEN as u64 / 8);
        }
        insts::OP_VL1RE16_V => {
            ld_whole!(inst, machine, VLEN as u64 / 8);
        }
        insts::OP_VL1RE32_V => {
            ld_whole!(inst, machine, VLEN as u64 / 8);
        }
        insts::OP_VL1RE64_V => {
            ld_whole!(inst, machine, VLEN as u64 / 8);
        }
        insts::OP_VL2RE8_V => {
            ld_whole!(inst, machine, VLEN as u64 / 4);
        }
        insts::OP_VL2RE16_V => {
            ld_whole!(inst, machine, VLEN as u64 / 4);
        }
        insts::OP_VL2RE32_V => {
            ld_whole!(inst, machine, VLEN as u64 / 4);
        }
        insts::OP_VL2RE64_V => {
            ld_whole!(inst, machine, VLEN as u64 / 4);
        }
        insts::OP_VL4RE8_V => {
            ld_whole!(inst, machine, VLEN as u64 / 2);
        }
        insts::OP_VL4RE16_V => {
            ld_whole!(inst, machine, VLEN as u64 / 2);
        }
        insts::OP_VL4RE32_V => {
            ld_whole!(inst, machine, VLEN as u64 / 2);
        }
        insts::OP_VL4RE64_V => {
            ld_whole!(inst, machine, VLEN as u64 / 2);
        }
        insts::OP_VL8RE8_V => {
            ld_whole!(inst, machine, VLEN as u64);
        }
        insts::OP_VL8RE16_V => {
            ld_whole!(inst, machine, VLEN as u64);
        }
        insts::OP_VL8RE32_V => {
            ld_whole!(inst, machine, VLEN as u64);
        }
        insts::OP_VL8RE64_V => {
            ld_whole!(inst, machine, VLEN as u64);
        }
        insts::OP_VS1R_V => {
            sd_whole!(inst, machine, VLEN as u64 / 8);
        }
        insts::OP_VS2R_V => {
            sd_whole!(inst, machine, VLEN as u64 / 4);
        }
        insts::OP_VS4R_V => {
            sd_whole!(inst, machine, VLEN as u64 / 2);
        }
        insts::OP_VS8R_V => {
            sd_whole!(inst, machine, VLEN as u64 / 1);
        }
        insts::OP_VADD_VV => {
            v_vv_loop_s!(inst, machine, Eint::wrapping_add);
        }
        insts::OP_VADD_VX => {
            v_vx_loop_s!(inst, machine, Eint::wrapping_add);
        }
        insts::OP_VADD_VI => {
            v_vi_loop_s!(inst, machine, Eint::wrapping_add);
        }
        insts::OP_VSUB_VV => {
            v_vv_loop_s!(inst, machine, Eint::wrapping_sub);
        }
        insts::OP_VSUB_VX => {
            v_vx_loop_s!(inst, machine, Eint::wrapping_sub);
        }
        insts::OP_VRSUB_VX => {
            v_vx_loop_s!(inst, machine, alu::rsub);
        }
        insts::OP_VRSUB_VI => {
            v_vi_loop_s!(inst, machine, alu::rsub);
        }
        insts::OP_VWADDU_VV => {
            w_vv_loop_u!(inst, machine, Eint::widening_add_u);
        }
        insts::OP_VWADDU_VX => {
            w_vx_loop_u!(inst, machine, Eint::widening_add_u);
        }
        insts::OP_VWSUBU_VV => {
            w_vv_loop_u!(inst, machine, Eint::widening_sub_u);
        }
        insts::OP_VWSUBU_VX => {
            w_vx_loop_u!(inst, machine, Eint::widening_sub_u);
        }
        insts::OP_VWADD_VV => {
            w_vv_loop_s!(inst, machine, Eint::widening_add_s);
        }
        insts::OP_VWADD_VX => {
            w_vx_loop_s!(inst, machine, Eint::widening_add_s);
        }
        insts::OP_VWSUB_VV => {
            w_vv_loop_s!(inst, machine, Eint::widening_sub_s);
        }
        insts::OP_VWSUB_VX => {
            w_vx_loop_s!(inst, machine, Eint::widening_sub_s);
        }
        insts::OP_VWADDU_WV => {
            w_wv_loop_u!(inst, machine, Eint::wrapping_add);
        }
        insts::OP_VWADDU_WX => {
            w_wx_loop_u!(inst, machine, Eint::wrapping_add);
        }
        insts::OP_VWSUBU_WV => {
            w_wv_loop_u!(inst, machine, Eint::wrapping_sub);
        }
        insts::OP_VWSUBU_WX => {
            w_wx_loop_u!(inst, machine, Eint::wrapping_sub);
        }
        insts::OP_VWADD_WV => {
            w_wv_loop_s!(inst, machine, Eint::wrapping_add);
        }
        insts::OP_VWADD_WX => {
            w_wx_loop_s!(inst, machine, Eint::wrapping_add);
        }
        insts::OP_VWSUB_WV => {
            w_wv_loop_s!(inst, machine, Eint::wrapping_sub);
        }
        insts::OP_VWSUB_WX => {
            w_wx_loop_s!(inst, machine, Eint::wrapping_sub);
        }
        insts::OP_VZEXT_VF2 => {
            v_vv_loop_ext_u!(inst, machine, 2);
        }
        insts::OP_VZEXT_VF4 => {
            v_vv_loop_ext_u!(inst, machine, 4);
        }
        insts::OP_VZEXT_VF8 => {
            v_vv_loop_ext_u!(inst, machine, 8);
        }
        insts::OP_VSEXT_VF2 => {
            v_vv_loop_ext_s!(inst, machine, 2);
        }
        insts::OP_VSEXT_VF4 => {
            v_vv_loop_ext_s!(inst, machine, 4);
        }
        insts::OP_VSEXT_VF8 => {
            v_vv_loop_ext_s!(inst, machine, 8);
        }
        insts::OP_VADC_VVM => {
            v_vvm_loop_s!(inst, machine, alu::adc);
        }
        insts::OP_VADC_VXM => {
            v_vxm_loop_s!(inst, machine, alu::adc);
        }
        insts::OP_VADC_VIM => {
            v_vim_loop_s!(inst, machine, alu::adc);
        }
        insts::OP_VMADC_VVM => {
            m_vvm_loop_s!(inst, machine, alu::madcm);
        }
        insts::OP_VMADC_VXM => {
            m_vxm_loop_s!(inst, machine, alu::madcm);
        }
        insts::OP_VMADC_VIM => {
            m_vim_loop_s!(inst, machine, alu::madcm);
        }
        insts::OP_VMADC_VV => {
            m_vv_loop_s!(inst, machine, alu::madc);
        }
        insts::OP_VMADC_VX => {
            m_vx_loop_s!(inst, machine, alu::madc);
        }
        insts::OP_VMADC_VI => {
            m_vi_loop_s!(inst, machine, alu::madc);
        }
        insts::OP_VSBC_VVM => {
            v_vvm_loop_s!(inst, machine, alu::sbc);
        }
        insts::OP_VSBC_VXM => {
            v_vxm_loop_s!(inst, machine, alu::sbc);
        }
        insts::OP_VMSBC_VVM => {
            m_vvm_loop_s!(inst, machine, alu::msbcm);
        }
        insts::OP_VMSBC_VXM => {
            m_vxm_loop_s!(inst, machine, alu::msbcm);
        }
        insts::OP_VMSBC_VV => {
            m_vv_loop_s!(inst, machine, alu::msbc);
        }
        insts::OP_VMSBC_VX => {
            m_vx_loop_s!(inst, machine, alu::msbc);
        }
        insts::OP_VAND_VV => {
            v_vv_loop_s!(inst, machine, alu::and);
        }
        insts::OP_VAND_VX => {
            v_vx_loop_s!(inst, machine, alu::and);
        }
        insts::OP_VAND_VI => {
            v_vi_loop_s!(inst, machine, alu::and);
        }
        insts::OP_VOR_VV => {
            v_vv_loop_s!(inst, machine, alu::or);
        }
        insts::OP_VOR_VX => {
            v_vx_loop_s!(inst, machine, alu::or);
        }
        insts::OP_VOR_VI => {
            v_vi_loop_s!(inst, machine, alu::or);
        }
        insts::OP_VXOR_VV => {
            v_vv_loop_s!(inst, machine, alu::xor);
        }
        insts::OP_VXOR_VX => {
            v_vx_loop_s!(inst, machine, alu::xor);
        }
        insts::OP_VXOR_VI => {
            v_vi_loop_s!(inst, machine, alu::xor);
        }
        insts::OP_VSLL_VV => {
            v_vv_loop_u!(inst, machine, alu::sll);
        }
        insts::OP_VSLL_VX => {
            v_vx_loop_u!(inst, machine, alu::sll);
        }
        insts::OP_VSLL_VI => {
            v_vi_loop_u!(inst, machine, alu::sll);
        }
        insts::OP_VSRL_VV => {
            v_vv_loop_u!(inst, machine, alu::srl);
        }
        insts::OP_VSRL_VX => {
            v_vx_loop_u!(inst, machine, alu::srl);
        }
        insts::OP_VSRL_VI => {
            v_vi_loop_u!(inst, machine, alu::srl);
        }
        insts::OP_VSRA_VV => {
            v_vv_loop_u!(inst, machine, alu::sra);
        }
        insts::OP_VSRA_VX => {
            v_vx_loop_u!(inst, machine, alu::sra);
        }
        insts::OP_VSRA_VI => {
            v_vi_loop_u!(inst, machine, alu::sra);
        }
        insts::OP_VNSRL_WV => {
            v_wv_loop_u!(inst, machine, alu::srl);
        }
        insts::OP_VNSRL_WX => {
            v_wx_loop_u!(inst, machine, alu::srl);
        }
        insts::OP_VNSRL_WI => {
            v_wi_loop_u!(inst, machine, alu::srl);
        }
        insts::OP_VNSRA_WV => {
            v_wv_loop_u!(inst, machine, alu::sra);
        }
        insts::OP_VNSRA_WX => {
            v_wx_loop_u!(inst, machine, alu::sra);
        }
        insts::OP_VNSRA_WI => {
            v_wi_loop_u!(inst, machine, alu::sra);
        }
        insts::OP_VMSEQ_VV => {
            m_vv_loop_s!(inst, machine, alu::seq);
        }
        insts::OP_VMSEQ_VX => {
            m_vx_loop_s!(inst, machine, alu::seq);
        }
        insts::OP_VMSEQ_VI => {
            m_vi_loop_s!(inst, machine, alu::seq);
        }
        insts::OP_VMSNE_VV => {
            m_vv_loop_s!(inst, machine, alu::sne);
        }
        insts::OP_VMSNE_VX => {
            m_vx_loop_s!(inst, machine, alu::sne);
        }
        insts::OP_VMSNE_VI => {
            m_vi_loop_s!(inst, machine, alu::sne);
        }
        insts::OP_VMSLTU_VV => {
            m_vv_loop_s!(inst, machine, alu::sltu);
        }
        insts::OP_VMSLTU_VX => {
            m_vx_loop_s!(inst, machine, alu::sltu);
        }
        insts::OP_VMSLT_VV => {
            m_vv_loop_s!(inst, machine, alu::slt);
        }
        insts::OP_VMSLT_VX => {
            m_vx_loop_s!(inst, machine, alu::slt);
        }
        insts::OP_VMSLEU_VV => {
            m_vv_loop_s!(inst, machine, alu::sleu);
        }
        insts::OP_VMSLEU_VX => {
            m_vx_loop_s!(inst, machine, alu::sleu);
        }
        insts::OP_VMSLEU_VI => {
            m_vi_loop_s!(inst, machine, alu::sleu);
        }
        insts::OP_VMSLE_VV => {
            m_vv_loop_s!(inst, machine, alu::sle);
        }
        insts::OP_VMSLE_VX => {
            m_vx_loop_s!(inst, machine, alu::sle);
        }
        insts::OP_VMSLE_VI => {
            m_vi_loop_s!(inst, machine, alu::sle);
        }
        insts::OP_VMSGTU_VX => {
            m_vx_loop_s!(inst, machine, alu::sgtu);
        }
        insts::OP_VMSGTU_VI => {
            m_vi_loop_s!(inst, machine, alu::sgtu);
        }
        insts::OP_VMSGT_VX => {
            m_vx_loop_s!(inst, machine, alu::sgt);
        }
        insts::OP_VMSGT_VI => {
            m_vi_loop_s!(inst, machine, alu::sgt);
        }
        insts::OP_VMAXU_VV => {
            v_vv_loop_u!(inst, machine, alu::maxu);
        }
        insts::OP_VMAXU_VX => {
            v_vx_loop_u!(inst, machine, alu::maxu);
        }
        insts::OP_VMAX_VV => {
            v_vv_loop_s!(inst, machine, alu::max);
        }
        insts::OP_VMAX_VX => {
            v_vx_loop_s!(inst, machine, alu::max);
        }
        insts::OP_VMINU_VV => {
            v_vv_loop_u!(inst, machine, alu::minu);
        }
        insts::OP_VMINU_VX => {
            v_vx_loop_u!(inst, machine, alu::minu);
        }
        insts::OP_VMIN_VV => {
            v_vv_loop_s!(inst, machine, alu::min);
        }
        insts::OP_VMIN_VX => {
            v_vx_loop_s!(inst, machine, alu::min);
        }
        insts::OP_VMUL_VV => {
            v_vv_loop_s!(inst, machine, Eint::wrapping_mul);
        }
        insts::OP_VMUL_VX => {
            v_vx_loop_s!(inst, machine, Eint::wrapping_mul);
        }
        insts::OP_VMULH_VV => {
            v_vv_loop_s!(inst, machine, alu::mulh);
        }
        insts::OP_VMULH_VX => {
            v_vx_loop_s!(inst, machine, alu::mulh);
        }
        insts::OP_VMULHU_VV => {
            v_vv_loop_u!(inst, machine, alu::mulhu);
        }
        insts::OP_VMULHU_VX => {
            v_vx_loop_u!(inst, machine, alu::mulhu);
        }
        insts::OP_VMULHSU_VV => {
            v_vv_loop_u!(inst, machine, alu::mulhsu);
        }
        insts::OP_VMULHSU_VX => {
            v_vx_loop_u!(inst, machine, alu::mulhsu);
        }
        insts::OP_VDIVU_VV => {
            v_vv_loop_u!(inst, machine, Eint::wrapping_div_u);
        }
        insts::OP_VDIVU_VX => {
            v_vx_loop_u!(inst, machine, Eint::wrapping_div_u);
        }
        insts::OP_VDIV_VV => {
            v_vv_loop_s!(inst, machine, Eint::wrapping_div_s);
        }
        insts::OP_VDIV_VX => {
            v_vx_loop_s!(inst, machine, Eint::wrapping_div_s);
        }
        insts::OP_VREMU_VV => {
            v_vv_loop_u!(inst, machine, Eint::wrapping_rem_u);
        }
        insts::OP_VREMU_VX => {
            v_vx_loop_u!(inst, machine, Eint::wrapping_rem_u);
        }
        insts::OP_VREM_VV => {
            v_vv_loop_s!(inst, machine, Eint::wrapping_rem_s);
        }
        insts::OP_VREM_VX => {
            v_vx_loop_s!(inst, machine, Eint::wrapping_rem_s);
        }
        insts::OP_VWMULU_VV => {
            w_vv_loop_u!(inst, machine, Eint::widening_mul_u);
        }
        insts::OP_VWMULU_VX => {
            w_vx_loop_u!(inst, machine, Eint::widening_mul_u);
        }
        insts::OP_VWMULSU_VV => {
            w_vv_loop_u!(inst, machine, Eint::widening_mul_su);
        }
        insts::OP_VWMULSU_VX => {
            w_vx_loop_u!(inst, machine, Eint::widening_mul_su);
        }
        insts::OP_VWMUL_VV => {
            w_vv_loop_s!(inst, machine, Eint::widening_mul_s);
        }
        insts::OP_VWMUL_VX => {
            w_vx_loop_s!(inst, machine, Eint::widening_mul_s);
        }
        insts::OP_VMACC_VV => {
            v_vv_loop_destructive_s!(inst, machine, alu::macc);
        }
        insts::OP_VMACC_VX => {
            v_vx_loop_destructive_s!(inst, machine, alu::macc);
        }
        insts::OP_VNMSAC_VV => {
            v_vv_loop_destructive_s!(inst, machine, alu::nmsac);
        }
        insts::OP_VNMSAC_VX => {
            v_vx_loop_destructive_s!(inst, machine, alu::nmsac);
        }
        insts::OP_VMADD_VV => {
            v_vv_loop_destructive_s!(inst, machine, alu::madd);
        }
        insts::OP_VMADD_VX => {
            v_vx_loop_destructive_s!(inst, machine, alu::madd);
        }
        insts::OP_VNMSUB_VV => {
            v_vv_loop_destructive_s!(inst, machine, alu::nmsub);
        }
        insts::OP_VNMSUB_VX => {
            v_vx_loop_destructive_s!(inst, machine, alu::nmsub);
        }
        insts::OP_VWMACCU_VV => {
            w_vv_loop_destructive_s!(inst, machine, alu::wmaccu);
        }
        insts::OP_VWMACCU_VX => {
            w_vx_loop_destructive_u!(inst, machine, alu::wmaccu);
        }
        insts::OP_VWMACC_VV => {
            w_vv_loop_destructive_s!(inst, machine, alu::wmacc);
        }
        insts::OP_VWMACC_VX => {
            w_vx_loop_destructive_s!(inst, machine, alu::wmacc);
        }
        insts::OP_VWMACCSU_VV => {
            w_vv_loop_destructive_s!(inst, machine, alu::wmaccsu);
        }
        insts::OP_VWMACCSU_VX => {
            w_vx_loop_destructive_s!(inst, machine, alu::wmaccsu);
        }
        insts::OP_VWMACCUS_VX => {
            w_vx_loop_destructive_u!(inst, machine, alu::wmaccus);
        }
        insts::OP_VMERGE_VVM => {
            v_vvm_loop_s!(inst, machine, alu::merge);
        }
        insts::OP_VMERGE_VXM => {
            v_vxm_loop_s!(inst, machine, alu::merge);
        }
        insts::OP_VMERGE_VIM => {
            v_vim_loop_s!(inst, machine, alu::merge);
        }
        insts::OP_VMV_V_V => {
            v_vv_loop_s!(inst, machine, alu::mv);
        }
        insts::OP_VMV_V_X => {
            v_vx_loop_s!(inst, machine, alu::mv);
        }
        insts::OP_VMV_V_I => {
            v_vi_loop_s!(inst, machine, alu::mv);
        }
        insts::OP_VSADDU_VV => {
            v_vv_loop_u!(inst, machine, alu::saddu);
        }
        insts::OP_VSADDU_VX => {
            v_vx_loop_u!(inst, machine, alu::saddu);
        }
        insts::OP_VSADDU_VI => {
            v_vi_loop_s!(inst, machine, alu::saddu);
        }
        insts::OP_VSADD_VV => {
            v_vv_loop_s!(inst, machine, alu::sadd);
        }
        insts::OP_VSADD_VX => {
            v_vx_loop_s!(inst, machine, alu::sadd);
        }
        insts::OP_VSADD_VI => {
            v_vi_loop_s!(inst, machine, alu::sadd);
        }
        insts::OP_VSSUBU_VV => {
            v_vv_loop_u!(inst, machine, alu::ssubu);
        }
        insts::OP_VSSUBU_VX => {
            v_vx_loop_u!(inst, machine, alu::ssubu);
        }
        insts::OP_VSSUB_VV => {
            v_vv_loop_s!(inst, machine, alu::ssub);
        }
        insts::OP_VSSUB_VX => {
            v_vx_loop_s!(inst, machine, alu::ssub);
        }
        insts::OP_VAADD_VV => {
            v_vv_loop_s!(inst, machine, Eint::average_add_s);
        }
        insts::OP_VAADD_VX => {
            v_vx_loop_s!(inst, machine, Eint::average_add_s);
        }
        insts::OP_VAADDU_VV => {
            v_vv_loop_u!(inst, machine, Eint::average_add_u);
        }
        insts::OP_VAADDU_VX => {
            v_vx_loop_u!(inst, machine, Eint::average_add_u);
        }
        insts::OP_VASUB_VV => {
            v_vv_loop_s!(inst, machine, Eint::average_sub_s);
        }
        insts::OP_VASUB_VX => {
            v_vx_loop_s!(inst, machine, Eint::average_sub_s);
        }
        insts::OP_VASUBU_VV => {
            v_vv_loop_u!(inst, machine, Eint::average_sub_u);
        }
        insts::OP_VASUBU_VX => {
            v_vx_loop_u!(inst, machine, Eint::average_sub_u);
        }
        insts::OP_VSMUL_VV => {
            v_vv_loop_s!(inst, machine, alu::smul);
        }
        insts::OP_VSMUL_VX => {
            v_vx_loop_s!(inst, machine, alu::smul);
        }
        insts::OP_VSSRL_VV => {
            v_vv_loop_u!(inst, machine, alu::srl);
        }
        insts::OP_VSSRL_VX => {
            v_vx_loop_u!(inst, machine, alu::srl);
        }
        insts::OP_VSSRL_VI => {
            v_vi_loop_u!(inst, machine, alu::srl);
        }
        insts::OP_VSSRA_VV => {
            v_vv_loop_u!(inst, machine, alu::sra);
        }
        insts::OP_VSSRA_VX => {
            v_vx_loop_u!(inst, machine, alu::sra);
        }
        insts::OP_VSSRA_VI => {
            v_vi_loop_u!(inst, machine, alu::sra);
        }
        insts::OP_VNCLIPU_WV => {
            v_wv_loop_u!(inst, machine, alu::vnclipu);
        }
        insts::OP_VNCLIPU_WX => {
            v_wx_loop_u!(inst, machine, alu::vnclipu);
        }
        insts::OP_VNCLIPU_WI => {
            v_wi_loop_u!(inst, machine, alu::vnclipu);
        }
        insts::OP_VNCLIP_WV => {
            v_wv_loop_u!(inst, machine, alu::vnclip);
        }
        insts::OP_VNCLIP_WX => {
            v_wx_loop_u!(inst, machine, alu::vnclip);
        }
        insts::OP_VNCLIP_WI => {
            v_wi_loop_u!(inst, machine, alu::vnclip);
        }
        insts::OP_VREDSUM_VS => {
            v_vs_loop_s!(inst, machine, Eint::wrapping_add);
        }
        insts::OP_VREDAND_VS => {
            v_vs_loop_s!(inst, machine, alu::and);
        }
        insts::OP_VREDOR_VS => {
            v_vs_loop_s!(inst, machine, alu::or);
        }
        insts::OP_VREDXOR_VS => {
            v_vs_loop_s!(inst, machine, alu::xor);
        }
        insts::OP_VREDMINU_VS => {
            v_vs_loop_s!(inst, machine, alu::minu);
        }
        insts::OP_VREDMIN_VS => {
            v_vs_loop_s!(inst, machine, alu::min);
        }
        insts::OP_VREDMAXU_VS => {
            v_vs_loop_s!(inst, machine, alu::maxu);
        }
        insts::OP_VREDMAX_VS => {
            v_vs_loop_s!(inst, machine, alu::max);
        }
        insts::OP_VWREDSUMU_VS => {
            w_vs_loop_u!(inst, machine, Eint::wrapping_add);
        }
        insts::OP_VWREDSUM_VS => {
            w_vs_loop_s!(inst, machine, Eint::wrapping_add);
        }
        insts::OP_VMAND_MM => {
            m_mm_loop!(inst, machine, |b: bool, a: bool| b & a);
        }
        insts::OP_VMNAND_MM => {
            m_mm_loop!(inst, machine, |b: bool, a: bool| !(b & a));
        }
        insts::OP_VMANDNOT_MM => {
            m_mm_loop!(inst, machine, |b: bool, a: bool| b & !a);
        }
        insts::OP_VMXOR_MM => {
            m_mm_loop!(inst, machine, |b: bool, a: bool| b ^ a);
        }
        insts::OP_VMOR_MM => {
            m_mm_loop!(inst, machine, |b: bool, a: bool| b | a);
        }
        insts::OP_VMNOR_MM => {
            m_mm_loop!(inst, machine, |b: bool, a: bool| !(b | a));
        }
        insts::OP_VMORNOT_MM => {
            m_mm_loop!(inst, machine, |b: bool, a: bool| b | !a);
        }
        insts::OP_VMXNOR_MM => {
            m_mm_loop!(inst, machine, |b: bool, a: bool| !(b ^ a));
        }
        insts::OP_VCPOP_M => {
            x_m_loop!(inst, machine, alu::cpop);
        }
        insts::OP_VFIRST_M => {
            x_m_loop!(inst, machine, alu::first);
        }
        insts::OP_VMSBF_M => {
            m_m_loop!(inst, machine, alu::sbf);
        }
        insts::OP_VMSIF_M => {
            m_m_loop!(inst, machine, alu::sif);
        }
        insts::OP_VMSOF_M => {
            m_m_loop!(inst, machine, alu::sof);
        }
        insts::OP_VIOTA_M => {
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
        }
        insts::OP_VID_V => {
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
        }
        insts::OP_VMV_X_S => {
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
        }
        insts::OP_VMV_S_X => {
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
        }
        insts::OP_VSLIDEUP_VX => {
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
        }
        insts::OP_VSLIDEUP_VI => {
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
        }
        insts::OP_VSLIDEDOWN_VX => {
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
        }
        insts::OP_VSLIDEDOWN_VI => {
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
        }
        insts::OP_VSLIDE1UP_VX => {
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
        }
        insts::OP_VSLIDE1DOWN_VX => {
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
        }
        insts::OP_VRGATHER_VV => {
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
        }
        insts::OP_VRGATHER_VX => {
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
        }
        insts::OP_VRGATHER_VI => {
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
        }
        insts::OP_VRGATHEREI16_VV => {
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
        }
        insts::OP_VCOMPRESS_VM => {
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
        }
        insts::OP_VMV1R_V => {
            vmv_r!(inst, machine, 1);
        }
        insts::OP_VMV2R_V => {
            vmv_r!(inst, machine, 2);
        }
        insts::OP_VMV4R_V => {
            vmv_r!(inst, machine, 4);
        }
        insts::OP_VMV8R_V => {
            vmv_r!(inst, machine, 8);
        }
        _ => return Ok(false),
    };
    Ok(true)
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

pub fn handle_vle512_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    ld!(inst, machine, machine.vl(), 0, 64, 1);
    Ok(())
}

pub fn handle_vwmulu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vv_loop_u!(inst, machine, Eint::widening_mul_u);
    Ok(())
}

pub fn handle_vnsrl_wx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_wx_loop_u!(inst, machine, alu::srl);
    Ok(())
}

pub fn handle_vsll_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_u!(inst, machine, alu::sll);
    Ok(())
}

pub fn handle_vsrl_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vx_loop_u!(inst, machine, alu::srl);
    Ok(())
}

pub fn handle_vwmaccu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vv_loop_destructive_s!(inst, machine, alu::wmaccu);
    Ok(())
}

pub fn handle_vmsleu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_vv_loop_s!(inst, machine, alu::sleu);
    Ok(())
}

pub fn handle_vsub_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_vv_loop_s!(inst, machine, Eint::wrapping_sub);
    Ok(())
}

pub fn handle_vse512_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    sd!(inst, machine, machine.vl(), 0, 64, 1);
    Ok(())
}
