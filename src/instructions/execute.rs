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

pub fn vcheck_vlm_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    vcheck_ld!(inst, machine, (machine.vl() + 7) / 8, 0, 1, 0);
    Ok(())
}

pub fn comply_vlm_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    comply_ld!(inst, machine, (machine.vl() + 7) / 8, 0, 1, 0);
    Ok(())
}

pub fn handle_vlm_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    vcheck_vlm_v(machine, inst)?;
    comply_vlm_v(machine, inst)?;
    Ok(())
}

pub fn vcheck_vle8_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    vcheck_ld!(inst, machine, machine.vl(), 0, 1, 1);
    Ok(())
}

pub fn comply_vle8_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    comply_ld!(inst, machine, machine.vl(), 0, 1, 1);
    Ok(())
}

pub fn handle_vle8_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    vcheck_vle8_v(machine, inst)?;
    comply_vle8_v(machine, inst)?;
    Ok(())
}

pub fn vcheck_vle16_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    vcheck_ld!(inst, machine, machine.vl(), 0, 2, 1);
    Ok(())
}

pub fn comply_vle16_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    comply_ld!(inst, machine, machine.vl(), 0, 2, 1);
    Ok(())
}

pub fn handle_vle16_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    vcheck_vle16_v(machine, inst)?;
    comply_vle16_v(machine, inst)?;
    Ok(())
}

pub fn vcheck_vle32_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    vcheck_ld!(inst, machine, machine.vl(), 0, 4, 1);
    Ok(())
}

pub fn comply_vle32_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    comply_ld!(inst, machine, machine.vl(), 0, 4, 1);
    Ok(())
}

pub fn handle_vle32_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    vcheck_vle32_v(machine, inst)?;
    comply_vle32_v(machine, inst)?;
    Ok(())
}

pub fn vcheck_vle64_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    vcheck_ld!(inst, machine, machine.vl(), 0, 8, 1);
    Ok(())
}

pub fn comply_vle64_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    comply_ld!(inst, machine, machine.vl(), 0, 8, 1);
    Ok(())
}

pub fn handle_vle64_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    vcheck_vle64_v(machine, inst)?;
    comply_vle64_v(machine, inst)?;
    Ok(())
}

pub fn vcheck_vle128_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    vcheck_ld!(inst, machine, machine.vl(), 0, 16, 1);
    Ok(())
}

pub fn comply_vle128_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    comply_ld!(inst, machine, machine.vl(), 0, 16, 1);
    Ok(())
}

pub fn handle_vle128_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    vcheck_vle128_v(machine, inst)?;
    comply_vle128_v(machine, inst)?;
    Ok(())
}

pub fn vcheck_vle256_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    vcheck_ld!(inst, machine, machine.vl(), 0, 32, 1);
    Ok(())
}

pub fn comply_vle256_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    comply_ld!(inst, machine, machine.vl(), 0, 32, 1);
    Ok(())
}

pub fn handle_vle256_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    vcheck_vle256_v(machine, inst)?;
    comply_vle256_v(machine, inst)?;
    Ok(())
}

pub fn vcheck_vle512_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    vcheck_ld!(inst, machine, machine.vl(), 0, 64, 1);
    Ok(())
}

pub fn comply_vle512_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    comply_ld!(inst, machine, machine.vl(), 0, 64, 1);
    Ok(())
}

pub fn handle_vle512_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    vcheck_vle512_v(machine, inst)?;
    comply_vle512_v(machine, inst)?;
    Ok(())
}

pub fn vcheck_vle1024_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    vcheck_ld!(inst, machine, machine.vl(), 0, 128, 1);
    Ok(())
}

pub fn comply_vle1024_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    comply_ld!(inst, machine, machine.vl(), 0, 128, 1);
    Ok(())
}

pub fn handle_vle1024_v<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    vcheck_vle1024_v(machine, inst)?;
    comply_vle1024_v(machine, inst)?;
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

pub fn handle_vwaddu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vv_loop_u!(inst, machine, Eint::widening_add_u);
    Ok(())
}

pub fn handle_vwaddu_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vx_loop_u!(inst, machine, Eint::widening_add_u);
    Ok(())
}

pub fn handle_vwsubu_vv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    w_vv_loop_u!(inst, machine, Eint::widening_sub_u);
    Ok(())
}

pub fn handle_vwsubu_vx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
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

pub fn handle_vnclipu_wv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_wv_loop_u!(inst, machine, alu::vnclipu);
    Ok(())
}

pub fn handle_vnclipu_wx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_wx_loop_u!(inst, machine, alu::vnclipu);
    Ok(())
}

pub fn handle_vnclipu_wi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_wi_loop_u!(inst, machine, alu::vnclipu);
    Ok(())
}

pub fn handle_vnclip_wv<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_wv_loop_u!(inst, machine, alu::vnclip);
    Ok(())
}

pub fn handle_vnclip_wx<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_wx_loop_u!(inst, machine, alu::vnclip);
    Ok(())
}

pub fn handle_vnclip_wi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    v_wi_loop_u!(inst, machine, alu::vnclip);
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
    v_vi_loop_s!(inst, machine, alu::saddu);
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

pub fn handle_vmsbf_m<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_m_loop!(inst, machine, alu::sbf);
    Ok(())
}

pub fn handle_vmsif_m<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    m_m_loop!(inst, machine, alu::sif);
    Ok(())
}

pub fn handle_vmsof_m<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
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

pub fn handle_vrgatherei16_vv<Mac: Machine>(
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

pub type HandleFunction<Mac> = fn(&mut Mac, Instruction) -> Result<(), Error>;

pub fn vcheck_skip<Mac: Machine>(_: &mut Mac, _: Instruction) -> Result<(), Error> {
    Ok(())
}

pub fn generate_vcheck_function_list<Mac: Machine>(
) -> [HandleFunction<Mac>; insts::MAXIMUM_LEVEL2_OPCODE as usize + 1] {
    [
        vcheck_skip, // handle_unloaded::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_add::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_addi::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_addiw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_addw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_and::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_andi::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_div::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_divu::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_divuw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_divw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_fence::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_fencei::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_lb::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_lbu::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_ld::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_lh::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_lhu::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_lui::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_lw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_lwu::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_mul::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_mulh::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_mulhsu::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_mulhu::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_mulw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_or::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_ori::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_rem::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_remu::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_remuw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_remw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_sb::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_sd::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_sh::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_sll::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_slli::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_slliw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_sllw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_slt::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_slti::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_sltiu::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_sltu::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_sra::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_srai::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_sraiw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_sraw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_srl::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_srli::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_srliw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_srlw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_sub::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_subw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_sw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_xor::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_xori::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_adduw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_andn::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_bclr::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_bclri::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_bext::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_bexti::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_binv::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_binvi::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_bset::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_bseti::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_clmul::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_clmulh::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_clmulr::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_clz::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_clzw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_cpop::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_cpopw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_ctz::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_ctzw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_max::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_maxu::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_min::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_minu::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_orcb::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_orn::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_rev8::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_rol::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_rolw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_ror::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_rori::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_roriw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_rorw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_sextb::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_sexth::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_sh1add::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_sh1adduw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_sh2add::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_sh2adduw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_sh3add::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_sh3adduw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_slliuw::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_xnor::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_zexth::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_wide_mul::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_wide_mulu::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_wide_mulsu::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_wide_div::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_wide_divu::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_ld_sign_extended_32_constant::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_adc::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_sbb::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_custom_load_imm::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_auipc::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_beq::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_bge::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_bgeu::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_blt::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_bltu::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_bne::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_ebreak::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_ecall::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_jal::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_jalr::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_far_jump_rel::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_far_jump_abs::<Mac> as HandleFunction<Mac>,
        vcheck_skip, // handle_custom_trace_end::<Mac> as HandleFunction<Mac>,
        handle_vsetvli::<Mac> as HandleFunction<Mac>,
        handle_vsetivli::<Mac> as HandleFunction<Mac>,
        handle_vsetvl::<Mac> as HandleFunction<Mac>,
        vcheck_vlm_v::<Mac> as HandleFunction<Mac>,
        vcheck_vle8_v::<Mac> as HandleFunction<Mac>,
        vcheck_vle16_v::<Mac> as HandleFunction<Mac>,
        vcheck_vle32_v::<Mac> as HandleFunction<Mac>,
        vcheck_vle64_v::<Mac> as HandleFunction<Mac>,
        vcheck_vle128_v::<Mac> as HandleFunction<Mac>,
        vcheck_vle256_v::<Mac> as HandleFunction<Mac>,
        vcheck_vle512_v::<Mac> as HandleFunction<Mac>,
        vcheck_vle1024_v::<Mac> as HandleFunction<Mac>,
        handle_vsm_v::<Mac> as HandleFunction<Mac>,
        handle_vse8_v::<Mac> as HandleFunction<Mac>,
        handle_vse16_v::<Mac> as HandleFunction<Mac>,
        handle_vse32_v::<Mac> as HandleFunction<Mac>,
        handle_vse64_v::<Mac> as HandleFunction<Mac>,
        handle_vse128_v::<Mac> as HandleFunction<Mac>,
        handle_vse256_v::<Mac> as HandleFunction<Mac>,
        handle_vse512_v::<Mac> as HandleFunction<Mac>,
        handle_vse1024_v::<Mac> as HandleFunction<Mac>,
        handle_vadd_vv::<Mac> as HandleFunction<Mac>,
        handle_vadd_vx::<Mac> as HandleFunction<Mac>,
        handle_vadd_vi::<Mac> as HandleFunction<Mac>,
        handle_vsub_vv::<Mac> as HandleFunction<Mac>,
        handle_vsub_vx::<Mac> as HandleFunction<Mac>,
        handle_vrsub_vx::<Mac> as HandleFunction<Mac>,
        handle_vrsub_vi::<Mac> as HandleFunction<Mac>,
        handle_vmul_vv::<Mac> as HandleFunction<Mac>,
        handle_vmul_vx::<Mac> as HandleFunction<Mac>,
        handle_vdiv_vv::<Mac> as HandleFunction<Mac>,
        handle_vdiv_vx::<Mac> as HandleFunction<Mac>,
        handle_vdivu_vv::<Mac> as HandleFunction<Mac>,
        handle_vdivu_vx::<Mac> as HandleFunction<Mac>,
        handle_vrem_vv::<Mac> as HandleFunction<Mac>,
        handle_vrem_vx::<Mac> as HandleFunction<Mac>,
        handle_vremu_vv::<Mac> as HandleFunction<Mac>,
        handle_vremu_vx::<Mac> as HandleFunction<Mac>,
        handle_vsll_vv::<Mac> as HandleFunction<Mac>,
        handle_vsll_vx::<Mac> as HandleFunction<Mac>,
        handle_vsll_vi::<Mac> as HandleFunction<Mac>,
        handle_vsrl_vv::<Mac> as HandleFunction<Mac>,
        handle_vsrl_vx::<Mac> as HandleFunction<Mac>,
        handle_vsrl_vi::<Mac> as HandleFunction<Mac>,
        handle_vsra_vv::<Mac> as HandleFunction<Mac>,
        handle_vsra_vx::<Mac> as HandleFunction<Mac>,
        handle_vsra_vi::<Mac> as HandleFunction<Mac>,
        handle_vmseq_vv::<Mac> as HandleFunction<Mac>,
        handle_vmseq_vx::<Mac> as HandleFunction<Mac>,
        handle_vmseq_vi::<Mac> as HandleFunction<Mac>,
        handle_vmsne_vv::<Mac> as HandleFunction<Mac>,
        handle_vmsne_vx::<Mac> as HandleFunction<Mac>,
        handle_vmsne_vi::<Mac> as HandleFunction<Mac>,
        handle_vmsltu_vv::<Mac> as HandleFunction<Mac>,
        handle_vmsltu_vx::<Mac> as HandleFunction<Mac>,
        handle_vmslt_vv::<Mac> as HandleFunction<Mac>,
        handle_vmslt_vx::<Mac> as HandleFunction<Mac>,
        handle_vmsleu_vv::<Mac> as HandleFunction<Mac>,
        handle_vmsleu_vx::<Mac> as HandleFunction<Mac>,
        handle_vmsleu_vi::<Mac> as HandleFunction<Mac>,
        handle_vmsle_vv::<Mac> as HandleFunction<Mac>,
        handle_vmsle_vx::<Mac> as HandleFunction<Mac>,
        handle_vmsle_vi::<Mac> as HandleFunction<Mac>,
        handle_vmsgtu_vx::<Mac> as HandleFunction<Mac>,
        handle_vmsgtu_vi::<Mac> as HandleFunction<Mac>,
        handle_vmsgt_vx::<Mac> as HandleFunction<Mac>,
        handle_vmsgt_vi::<Mac> as HandleFunction<Mac>,
        handle_vminu_vv::<Mac> as HandleFunction<Mac>,
        handle_vminu_vx::<Mac> as HandleFunction<Mac>,
        handle_vmin_vv::<Mac> as HandleFunction<Mac>,
        handle_vmin_vx::<Mac> as HandleFunction<Mac>,
        handle_vmaxu_vv::<Mac> as HandleFunction<Mac>,
        handle_vmaxu_vx::<Mac> as HandleFunction<Mac>,
        handle_vmax_vv::<Mac> as HandleFunction<Mac>,
        handle_vmax_vx::<Mac> as HandleFunction<Mac>,
        handle_vwaddu_vv::<Mac> as HandleFunction<Mac>,
        handle_vwaddu_vx::<Mac> as HandleFunction<Mac>,
        handle_vwsubu_vv::<Mac> as HandleFunction<Mac>,
        handle_vwsubu_vx::<Mac> as HandleFunction<Mac>,
        handle_vwadd_vv::<Mac> as HandleFunction<Mac>,
        handle_vwadd_vx::<Mac> as HandleFunction<Mac>,
        handle_vwsub_vv::<Mac> as HandleFunction<Mac>,
        handle_vwsub_vx::<Mac> as HandleFunction<Mac>,
        handle_vwaddu_wv::<Mac> as HandleFunction<Mac>,
        handle_vwaddu_wx::<Mac> as HandleFunction<Mac>,
        handle_vwsubu_wv::<Mac> as HandleFunction<Mac>,
        handle_vwsubu_wx::<Mac> as HandleFunction<Mac>,
        handle_vwadd_wv::<Mac> as HandleFunction<Mac>,
        handle_vwadd_wx::<Mac> as HandleFunction<Mac>,
        handle_vwsub_wv::<Mac> as HandleFunction<Mac>,
        handle_vwsub_wx::<Mac> as HandleFunction<Mac>,
        handle_vzext_vf8::<Mac> as HandleFunction<Mac>,
        handle_vsext_vf8::<Mac> as HandleFunction<Mac>,
        handle_vzext_vf4::<Mac> as HandleFunction<Mac>,
        handle_vsext_vf4::<Mac> as HandleFunction<Mac>,
        handle_vzext_vf2::<Mac> as HandleFunction<Mac>,
        handle_vsext_vf2::<Mac> as HandleFunction<Mac>,
        handle_vadc_vvm::<Mac> as HandleFunction<Mac>,
        handle_vadc_vxm::<Mac> as HandleFunction<Mac>,
        handle_vadc_vim::<Mac> as HandleFunction<Mac>,
        handle_vmadc_vvm::<Mac> as HandleFunction<Mac>,
        handle_vmadc_vxm::<Mac> as HandleFunction<Mac>,
        handle_vmadc_vim::<Mac> as HandleFunction<Mac>,
        handle_vmadc_vv::<Mac> as HandleFunction<Mac>,
        handle_vmadc_vx::<Mac> as HandleFunction<Mac>,
        handle_vmadc_vi::<Mac> as HandleFunction<Mac>,
        handle_vsbc_vvm::<Mac> as HandleFunction<Mac>,
        handle_vsbc_vxm::<Mac> as HandleFunction<Mac>,
        handle_vmsbc_vvm::<Mac> as HandleFunction<Mac>,
        handle_vmsbc_vxm::<Mac> as HandleFunction<Mac>,
        handle_vmsbc_vv::<Mac> as HandleFunction<Mac>,
        handle_vmsbc_vx::<Mac> as HandleFunction<Mac>,
        handle_vand_vv::<Mac> as HandleFunction<Mac>,
        handle_vand_vi::<Mac> as HandleFunction<Mac>,
        handle_vand_vx::<Mac> as HandleFunction<Mac>,
        handle_vor_vv::<Mac> as HandleFunction<Mac>,
        handle_vor_vx::<Mac> as HandleFunction<Mac>,
        handle_vor_vi::<Mac> as HandleFunction<Mac>,
        handle_vxor_vv::<Mac> as HandleFunction<Mac>,
        handle_vxor_vx::<Mac> as HandleFunction<Mac>,
        handle_vxor_vi::<Mac> as HandleFunction<Mac>,
        handle_vnsrl_wv::<Mac> as HandleFunction<Mac>,
        handle_vnsrl_wx::<Mac> as HandleFunction<Mac>,
        handle_vnsrl_wi::<Mac> as HandleFunction<Mac>,
        handle_vnsra_wv::<Mac> as HandleFunction<Mac>,
        handle_vnsra_wx::<Mac> as HandleFunction<Mac>,
        handle_vnsra_wi::<Mac> as HandleFunction<Mac>,
        handle_vmulh_vv::<Mac> as HandleFunction<Mac>,
        handle_vmulh_vx::<Mac> as HandleFunction<Mac>,
        handle_vmulhu_vv::<Mac> as HandleFunction<Mac>,
        handle_vmulhu_vx::<Mac> as HandleFunction<Mac>,
        handle_vmulhsu_vv::<Mac> as HandleFunction<Mac>,
        handle_vmulhsu_vx::<Mac> as HandleFunction<Mac>,
        handle_vwmulu_vv::<Mac> as HandleFunction<Mac>,
        handle_vwmulu_vx::<Mac> as HandleFunction<Mac>,
        handle_vwmulsu_vv::<Mac> as HandleFunction<Mac>,
        handle_vwmulsu_vx::<Mac> as HandleFunction<Mac>,
        handle_vwmul_vv::<Mac> as HandleFunction<Mac>,
        handle_vwmul_vx::<Mac> as HandleFunction<Mac>,
        handle_vmv_v_v::<Mac> as HandleFunction<Mac>,
        handle_vmv_v_x::<Mac> as HandleFunction<Mac>,
        handle_vmv_v_i::<Mac> as HandleFunction<Mac>,
        handle_vsaddu_vv::<Mac> as HandleFunction<Mac>,
        handle_vsaddu_vx::<Mac> as HandleFunction<Mac>,
        handle_vsaddu_vi::<Mac> as HandleFunction<Mac>,
        handle_vsadd_vv::<Mac> as HandleFunction<Mac>,
        handle_vsadd_vx::<Mac> as HandleFunction<Mac>,
        handle_vsadd_vi::<Mac> as HandleFunction<Mac>,
        handle_vssubu_vv::<Mac> as HandleFunction<Mac>,
        handle_vssubu_vx::<Mac> as HandleFunction<Mac>,
        handle_vssub_vv::<Mac> as HandleFunction<Mac>,
        handle_vssub_vx::<Mac> as HandleFunction<Mac>,
        handle_vaaddu_vv::<Mac> as HandleFunction<Mac>,
        handle_vaaddu_vx::<Mac> as HandleFunction<Mac>,
        handle_vaadd_vv::<Mac> as HandleFunction<Mac>,
        handle_vaadd_vx::<Mac> as HandleFunction<Mac>,
        handle_vasubu_vv::<Mac> as HandleFunction<Mac>,
        handle_vasubu_vx::<Mac> as HandleFunction<Mac>,
        handle_vasub_vv::<Mac> as HandleFunction<Mac>,
        handle_vasub_vx::<Mac> as HandleFunction<Mac>,
        handle_vmv1r_v::<Mac> as HandleFunction<Mac>,
        handle_vmv2r_v::<Mac> as HandleFunction<Mac>,
        handle_vmv4r_v::<Mac> as HandleFunction<Mac>,
        handle_vmv8r_v::<Mac> as HandleFunction<Mac>,
        handle_vfirst_m::<Mac> as HandleFunction<Mac>,
        handle_vmand_mm::<Mac> as HandleFunction<Mac>,
        handle_vmnand_mm::<Mac> as HandleFunction<Mac>,
        handle_vmandnot_mm::<Mac> as HandleFunction<Mac>,
        handle_vmxor_mm::<Mac> as HandleFunction<Mac>,
        handle_vmor_mm::<Mac> as HandleFunction<Mac>,
        handle_vmnor_mm::<Mac> as HandleFunction<Mac>,
        handle_vmornot_mm::<Mac> as HandleFunction<Mac>,
        handle_vmxnor_mm::<Mac> as HandleFunction<Mac>,
        handle_vlse8_v::<Mac> as HandleFunction<Mac>,
        handle_vlse16_v::<Mac> as HandleFunction<Mac>,
        handle_vlse32_v::<Mac> as HandleFunction<Mac>,
        handle_vlse64_v::<Mac> as HandleFunction<Mac>,
        handle_vlse128_v::<Mac> as HandleFunction<Mac>,
        handle_vlse256_v::<Mac> as HandleFunction<Mac>,
        handle_vlse512_v::<Mac> as HandleFunction<Mac>,
        handle_vlse1024_v::<Mac> as HandleFunction<Mac>,
        handle_vsse8_v::<Mac> as HandleFunction<Mac>,
        handle_vsse16_v::<Mac> as HandleFunction<Mac>,
        handle_vsse32_v::<Mac> as HandleFunction<Mac>,
        handle_vsse64_v::<Mac> as HandleFunction<Mac>,
        handle_vsse128_v::<Mac> as HandleFunction<Mac>,
        handle_vsse256_v::<Mac> as HandleFunction<Mac>,
        handle_vsse512_v::<Mac> as HandleFunction<Mac>,
        handle_vsse1024_v::<Mac> as HandleFunction<Mac>,
        handle_vluxei8_v::<Mac> as HandleFunction<Mac>,
        handle_vluxei16_v::<Mac> as HandleFunction<Mac>,
        handle_vluxei32_v::<Mac> as HandleFunction<Mac>,
        handle_vluxei64_v::<Mac> as HandleFunction<Mac>,
        handle_vloxei8_v::<Mac> as HandleFunction<Mac>,
        handle_vloxei16_v::<Mac> as HandleFunction<Mac>,
        handle_vloxei32_v::<Mac> as HandleFunction<Mac>,
        handle_vloxei64_v::<Mac> as HandleFunction<Mac>,
        handle_vsuxei8_v::<Mac> as HandleFunction<Mac>,
        handle_vsuxei16_v::<Mac> as HandleFunction<Mac>,
        handle_vsuxei32_v::<Mac> as HandleFunction<Mac>,
        handle_vsuxei64_v::<Mac> as HandleFunction<Mac>,
        handle_vsoxei8_v::<Mac> as HandleFunction<Mac>,
        handle_vsoxei16_v::<Mac> as HandleFunction<Mac>,
        handle_vsoxei32_v::<Mac> as HandleFunction<Mac>,
        handle_vsoxei64_v::<Mac> as HandleFunction<Mac>,
        handle_vl1re8_v::<Mac> as HandleFunction<Mac>,
        handle_vl1re16_v::<Mac> as HandleFunction<Mac>,
        handle_vl1re32_v::<Mac> as HandleFunction<Mac>,
        handle_vl1re64_v::<Mac> as HandleFunction<Mac>,
        handle_vl2re8_v::<Mac> as HandleFunction<Mac>,
        handle_vl2re16_v::<Mac> as HandleFunction<Mac>,
        handle_vl2re32_v::<Mac> as HandleFunction<Mac>,
        handle_vl2re64_v::<Mac> as HandleFunction<Mac>,
        handle_vl4re8_v::<Mac> as HandleFunction<Mac>,
        handle_vl4re16_v::<Mac> as HandleFunction<Mac>,
        handle_vl4re32_v::<Mac> as HandleFunction<Mac>,
        handle_vl4re64_v::<Mac> as HandleFunction<Mac>,
        handle_vl8re8_v::<Mac> as HandleFunction<Mac>,
        handle_vl8re16_v::<Mac> as HandleFunction<Mac>,
        handle_vl8re32_v::<Mac> as HandleFunction<Mac>,
        handle_vl8re64_v::<Mac> as HandleFunction<Mac>,
        handle_vs1r_v::<Mac> as HandleFunction<Mac>,
        handle_vs2r_v::<Mac> as HandleFunction<Mac>,
        handle_vs4r_v::<Mac> as HandleFunction<Mac>,
        handle_vs8r_v::<Mac> as HandleFunction<Mac>,
        handle_vmacc_vv::<Mac> as HandleFunction<Mac>,
        handle_vmacc_vx::<Mac> as HandleFunction<Mac>,
        handle_vnmsac_vv::<Mac> as HandleFunction<Mac>,
        handle_vnmsac_vx::<Mac> as HandleFunction<Mac>,
        handle_vmadd_vv::<Mac> as HandleFunction<Mac>,
        handle_vmadd_vx::<Mac> as HandleFunction<Mac>,
        handle_vnmsub_vv::<Mac> as HandleFunction<Mac>,
        handle_vnmsub_vx::<Mac> as HandleFunction<Mac>,
        handle_vssrl_vv::<Mac> as HandleFunction<Mac>,
        handle_vssrl_vx::<Mac> as HandleFunction<Mac>,
        handle_vssrl_vi::<Mac> as HandleFunction<Mac>,
        handle_vssra_vv::<Mac> as HandleFunction<Mac>,
        handle_vssra_vx::<Mac> as HandleFunction<Mac>,
        handle_vssra_vi::<Mac> as HandleFunction<Mac>,
        handle_vsmul_vv::<Mac> as HandleFunction<Mac>,
        handle_vsmul_vx::<Mac> as HandleFunction<Mac>,
        handle_vwmaccu_vv::<Mac> as HandleFunction<Mac>,
        handle_vwmaccu_vx::<Mac> as HandleFunction<Mac>,
        handle_vwmacc_vv::<Mac> as HandleFunction<Mac>,
        handle_vwmacc_vx::<Mac> as HandleFunction<Mac>,
        handle_vwmaccsu_vv::<Mac> as HandleFunction<Mac>,
        handle_vwmaccsu_vx::<Mac> as HandleFunction<Mac>,
        handle_vwmaccus_vx::<Mac> as HandleFunction<Mac>,
        handle_vmerge_vvm::<Mac> as HandleFunction<Mac>,
        handle_vmerge_vxm::<Mac> as HandleFunction<Mac>,
        handle_vmerge_vim::<Mac> as HandleFunction<Mac>,
        handle_vnclipu_wv::<Mac> as HandleFunction<Mac>,
        handle_vnclipu_wx::<Mac> as HandleFunction<Mac>,
        handle_vnclipu_wi::<Mac> as HandleFunction<Mac>,
        handle_vnclip_wv::<Mac> as HandleFunction<Mac>,
        handle_vnclip_wx::<Mac> as HandleFunction<Mac>,
        handle_vnclip_wi::<Mac> as HandleFunction<Mac>,
        handle_vredsum_vs::<Mac> as HandleFunction<Mac>,
        handle_vredand_vs::<Mac> as HandleFunction<Mac>,
        handle_vredor_vs::<Mac> as HandleFunction<Mac>,
        handle_vredxor_vs::<Mac> as HandleFunction<Mac>,
        handle_vredminu_vs::<Mac> as HandleFunction<Mac>,
        handle_vredmin_vs::<Mac> as HandleFunction<Mac>,
        handle_vredmaxu_vs::<Mac> as HandleFunction<Mac>,
        handle_vredmax_vs::<Mac> as HandleFunction<Mac>,
        handle_vwredsumu_vs::<Mac> as HandleFunction<Mac>,
        handle_vwredsum_vs::<Mac> as HandleFunction<Mac>,
        handle_vcpop_m::<Mac> as HandleFunction<Mac>,
        handle_vmsbf_m::<Mac> as HandleFunction<Mac>,
        handle_vmsof_m::<Mac> as HandleFunction<Mac>,
        handle_vmsif_m::<Mac> as HandleFunction<Mac>,
        handle_viota_m::<Mac> as HandleFunction<Mac>,
        handle_vid_v::<Mac> as HandleFunction<Mac>,
        handle_vmv_x_s::<Mac> as HandleFunction<Mac>,
        handle_vmv_s_x::<Mac> as HandleFunction<Mac>,
        handle_vcompress_vm::<Mac> as HandleFunction<Mac>,
        handle_vslide1up_vx::<Mac> as HandleFunction<Mac>,
        handle_vslideup_vx::<Mac> as HandleFunction<Mac>,
        handle_vslideup_vi::<Mac> as HandleFunction<Mac>,
        handle_vslide1down_vx::<Mac> as HandleFunction<Mac>,
        handle_vslidedown_vx::<Mac> as HandleFunction<Mac>,
        handle_vslidedown_vi::<Mac> as HandleFunction<Mac>,
        handle_vrgather_vx::<Mac> as HandleFunction<Mac>,
        handle_vrgather_vv::<Mac> as HandleFunction<Mac>,
        handle_vrgatherei16_vv::<Mac> as HandleFunction<Mac>,
        handle_vrgather_vi::<Mac> as HandleFunction<Mac>,
    ]
}

pub fn generate_comply_function_list<Mac: Machine>(
) -> [HandleFunction<Mac>; insts::MAXIMUM_LEVEL2_OPCODE as usize + 1] {
    [
        handle_unloaded::<Mac> as HandleFunction<Mac>,
        handle_add::<Mac> as HandleFunction<Mac>,
        handle_addi::<Mac> as HandleFunction<Mac>,
        handle_addiw::<Mac> as HandleFunction<Mac>,
        handle_addw::<Mac> as HandleFunction<Mac>,
        handle_and::<Mac> as HandleFunction<Mac>,
        handle_andi::<Mac> as HandleFunction<Mac>,
        handle_div::<Mac> as HandleFunction<Mac>,
        handle_divu::<Mac> as HandleFunction<Mac>,
        handle_divuw::<Mac> as HandleFunction<Mac>,
        handle_divw::<Mac> as HandleFunction<Mac>,
        handle_fence::<Mac> as HandleFunction<Mac>,
        handle_fencei::<Mac> as HandleFunction<Mac>,
        handle_lb::<Mac> as HandleFunction<Mac>,
        handle_lbu::<Mac> as HandleFunction<Mac>,
        handle_ld::<Mac> as HandleFunction<Mac>,
        handle_lh::<Mac> as HandleFunction<Mac>,
        handle_lhu::<Mac> as HandleFunction<Mac>,
        handle_lui::<Mac> as HandleFunction<Mac>,
        handle_lw::<Mac> as HandleFunction<Mac>,
        handle_lwu::<Mac> as HandleFunction<Mac>,
        handle_mul::<Mac> as HandleFunction<Mac>,
        handle_mulh::<Mac> as HandleFunction<Mac>,
        handle_mulhsu::<Mac> as HandleFunction<Mac>,
        handle_mulhu::<Mac> as HandleFunction<Mac>,
        handle_mulw::<Mac> as HandleFunction<Mac>,
        handle_or::<Mac> as HandleFunction<Mac>,
        handle_ori::<Mac> as HandleFunction<Mac>,
        handle_rem::<Mac> as HandleFunction<Mac>,
        handle_remu::<Mac> as HandleFunction<Mac>,
        handle_remuw::<Mac> as HandleFunction<Mac>,
        handle_remw::<Mac> as HandleFunction<Mac>,
        handle_sb::<Mac> as HandleFunction<Mac>,
        handle_sd::<Mac> as HandleFunction<Mac>,
        handle_sh::<Mac> as HandleFunction<Mac>,
        handle_sll::<Mac> as HandleFunction<Mac>,
        handle_slli::<Mac> as HandleFunction<Mac>,
        handle_slliw::<Mac> as HandleFunction<Mac>,
        handle_sllw::<Mac> as HandleFunction<Mac>,
        handle_slt::<Mac> as HandleFunction<Mac>,
        handle_slti::<Mac> as HandleFunction<Mac>,
        handle_sltiu::<Mac> as HandleFunction<Mac>,
        handle_sltu::<Mac> as HandleFunction<Mac>,
        handle_sra::<Mac> as HandleFunction<Mac>,
        handle_srai::<Mac> as HandleFunction<Mac>,
        handle_sraiw::<Mac> as HandleFunction<Mac>,
        handle_sraw::<Mac> as HandleFunction<Mac>,
        handle_srl::<Mac> as HandleFunction<Mac>,
        handle_srli::<Mac> as HandleFunction<Mac>,
        handle_srliw::<Mac> as HandleFunction<Mac>,
        handle_srlw::<Mac> as HandleFunction<Mac>,
        handle_sub::<Mac> as HandleFunction<Mac>,
        handle_subw::<Mac> as HandleFunction<Mac>,
        handle_sw::<Mac> as HandleFunction<Mac>,
        handle_xor::<Mac> as HandleFunction<Mac>,
        handle_xori::<Mac> as HandleFunction<Mac>,
        handle_adduw::<Mac> as HandleFunction<Mac>,
        handle_andn::<Mac> as HandleFunction<Mac>,
        handle_bclr::<Mac> as HandleFunction<Mac>,
        handle_bclri::<Mac> as HandleFunction<Mac>,
        handle_bext::<Mac> as HandleFunction<Mac>,
        handle_bexti::<Mac> as HandleFunction<Mac>,
        handle_binv::<Mac> as HandleFunction<Mac>,
        handle_binvi::<Mac> as HandleFunction<Mac>,
        handle_bset::<Mac> as HandleFunction<Mac>,
        handle_bseti::<Mac> as HandleFunction<Mac>,
        handle_clmul::<Mac> as HandleFunction<Mac>,
        handle_clmulh::<Mac> as HandleFunction<Mac>,
        handle_clmulr::<Mac> as HandleFunction<Mac>,
        handle_clz::<Mac> as HandleFunction<Mac>,
        handle_clzw::<Mac> as HandleFunction<Mac>,
        handle_cpop::<Mac> as HandleFunction<Mac>,
        handle_cpopw::<Mac> as HandleFunction<Mac>,
        handle_ctz::<Mac> as HandleFunction<Mac>,
        handle_ctzw::<Mac> as HandleFunction<Mac>,
        handle_max::<Mac> as HandleFunction<Mac>,
        handle_maxu::<Mac> as HandleFunction<Mac>,
        handle_min::<Mac> as HandleFunction<Mac>,
        handle_minu::<Mac> as HandleFunction<Mac>,
        handle_orcb::<Mac> as HandleFunction<Mac>,
        handle_orn::<Mac> as HandleFunction<Mac>,
        handle_rev8::<Mac> as HandleFunction<Mac>,
        handle_rol::<Mac> as HandleFunction<Mac>,
        handle_rolw::<Mac> as HandleFunction<Mac>,
        handle_ror::<Mac> as HandleFunction<Mac>,
        handle_rori::<Mac> as HandleFunction<Mac>,
        handle_roriw::<Mac> as HandleFunction<Mac>,
        handle_rorw::<Mac> as HandleFunction<Mac>,
        handle_sextb::<Mac> as HandleFunction<Mac>,
        handle_sexth::<Mac> as HandleFunction<Mac>,
        handle_sh1add::<Mac> as HandleFunction<Mac>,
        handle_sh1adduw::<Mac> as HandleFunction<Mac>,
        handle_sh2add::<Mac> as HandleFunction<Mac>,
        handle_sh2adduw::<Mac> as HandleFunction<Mac>,
        handle_sh3add::<Mac> as HandleFunction<Mac>,
        handle_sh3adduw::<Mac> as HandleFunction<Mac>,
        handle_slliuw::<Mac> as HandleFunction<Mac>,
        handle_xnor::<Mac> as HandleFunction<Mac>,
        handle_zexth::<Mac> as HandleFunction<Mac>,
        handle_wide_mul::<Mac> as HandleFunction<Mac>,
        handle_wide_mulu::<Mac> as HandleFunction<Mac>,
        handle_wide_mulsu::<Mac> as HandleFunction<Mac>,
        handle_wide_div::<Mac> as HandleFunction<Mac>,
        handle_wide_divu::<Mac> as HandleFunction<Mac>,
        handle_ld_sign_extended_32_constant::<Mac> as HandleFunction<Mac>,
        handle_adc::<Mac> as HandleFunction<Mac>,
        handle_sbb::<Mac> as HandleFunction<Mac>,
        handle_custom_load_imm::<Mac> as HandleFunction<Mac>,
        handle_auipc::<Mac> as HandleFunction<Mac>,
        handle_beq::<Mac> as HandleFunction<Mac>,
        handle_bge::<Mac> as HandleFunction<Mac>,
        handle_bgeu::<Mac> as HandleFunction<Mac>,
        handle_blt::<Mac> as HandleFunction<Mac>,
        handle_bltu::<Mac> as HandleFunction<Mac>,
        handle_bne::<Mac> as HandleFunction<Mac>,
        handle_ebreak::<Mac> as HandleFunction<Mac>,
        handle_ecall::<Mac> as HandleFunction<Mac>,
        handle_jal::<Mac> as HandleFunction<Mac>,
        handle_jalr::<Mac> as HandleFunction<Mac>,
        handle_far_jump_rel::<Mac> as HandleFunction<Mac>,
        handle_far_jump_abs::<Mac> as HandleFunction<Mac>,
        handle_custom_trace_end::<Mac> as HandleFunction<Mac>,
        handle_vsetvli::<Mac> as HandleFunction<Mac>,
        handle_vsetivli::<Mac> as HandleFunction<Mac>,
        handle_vsetvl::<Mac> as HandleFunction<Mac>,
        comply_vlm_v::<Mac> as HandleFunction<Mac>,
        comply_vle8_v::<Mac> as HandleFunction<Mac>,
        comply_vle16_v::<Mac> as HandleFunction<Mac>,
        comply_vle32_v::<Mac> as HandleFunction<Mac>,
        comply_vle64_v::<Mac> as HandleFunction<Mac>,
        comply_vle128_v::<Mac> as HandleFunction<Mac>,
        comply_vle256_v::<Mac> as HandleFunction<Mac>,
        comply_vle512_v::<Mac> as HandleFunction<Mac>,
        comply_vle1024_v::<Mac> as HandleFunction<Mac>,
        handle_vsm_v::<Mac> as HandleFunction<Mac>,
        handle_vse8_v::<Mac> as HandleFunction<Mac>,
        handle_vse16_v::<Mac> as HandleFunction<Mac>,
        handle_vse32_v::<Mac> as HandleFunction<Mac>,
        handle_vse64_v::<Mac> as HandleFunction<Mac>,
        handle_vse128_v::<Mac> as HandleFunction<Mac>,
        handle_vse256_v::<Mac> as HandleFunction<Mac>,
        handle_vse512_v::<Mac> as HandleFunction<Mac>,
        handle_vse1024_v::<Mac> as HandleFunction<Mac>,
        handle_vadd_vv::<Mac> as HandleFunction<Mac>,
        handle_vadd_vx::<Mac> as HandleFunction<Mac>,
        handle_vadd_vi::<Mac> as HandleFunction<Mac>,
        handle_vsub_vv::<Mac> as HandleFunction<Mac>,
        handle_vsub_vx::<Mac> as HandleFunction<Mac>,
        handle_vrsub_vx::<Mac> as HandleFunction<Mac>,
        handle_vrsub_vi::<Mac> as HandleFunction<Mac>,
        handle_vmul_vv::<Mac> as HandleFunction<Mac>,
        handle_vmul_vx::<Mac> as HandleFunction<Mac>,
        handle_vdiv_vv::<Mac> as HandleFunction<Mac>,
        handle_vdiv_vx::<Mac> as HandleFunction<Mac>,
        handle_vdivu_vv::<Mac> as HandleFunction<Mac>,
        handle_vdivu_vx::<Mac> as HandleFunction<Mac>,
        handle_vrem_vv::<Mac> as HandleFunction<Mac>,
        handle_vrem_vx::<Mac> as HandleFunction<Mac>,
        handle_vremu_vv::<Mac> as HandleFunction<Mac>,
        handle_vremu_vx::<Mac> as HandleFunction<Mac>,
        handle_vsll_vv::<Mac> as HandleFunction<Mac>,
        handle_vsll_vx::<Mac> as HandleFunction<Mac>,
        handle_vsll_vi::<Mac> as HandleFunction<Mac>,
        handle_vsrl_vv::<Mac> as HandleFunction<Mac>,
        handle_vsrl_vx::<Mac> as HandleFunction<Mac>,
        handle_vsrl_vi::<Mac> as HandleFunction<Mac>,
        handle_vsra_vv::<Mac> as HandleFunction<Mac>,
        handle_vsra_vx::<Mac> as HandleFunction<Mac>,
        handle_vsra_vi::<Mac> as HandleFunction<Mac>,
        handle_vmseq_vv::<Mac> as HandleFunction<Mac>,
        handle_vmseq_vx::<Mac> as HandleFunction<Mac>,
        handle_vmseq_vi::<Mac> as HandleFunction<Mac>,
        handle_vmsne_vv::<Mac> as HandleFunction<Mac>,
        handle_vmsne_vx::<Mac> as HandleFunction<Mac>,
        handle_vmsne_vi::<Mac> as HandleFunction<Mac>,
        handle_vmsltu_vv::<Mac> as HandleFunction<Mac>,
        handle_vmsltu_vx::<Mac> as HandleFunction<Mac>,
        handle_vmslt_vv::<Mac> as HandleFunction<Mac>,
        handle_vmslt_vx::<Mac> as HandleFunction<Mac>,
        handle_vmsleu_vv::<Mac> as HandleFunction<Mac>,
        handle_vmsleu_vx::<Mac> as HandleFunction<Mac>,
        handle_vmsleu_vi::<Mac> as HandleFunction<Mac>,
        handle_vmsle_vv::<Mac> as HandleFunction<Mac>,
        handle_vmsle_vx::<Mac> as HandleFunction<Mac>,
        handle_vmsle_vi::<Mac> as HandleFunction<Mac>,
        handle_vmsgtu_vx::<Mac> as HandleFunction<Mac>,
        handle_vmsgtu_vi::<Mac> as HandleFunction<Mac>,
        handle_vmsgt_vx::<Mac> as HandleFunction<Mac>,
        handle_vmsgt_vi::<Mac> as HandleFunction<Mac>,
        handle_vminu_vv::<Mac> as HandleFunction<Mac>,
        handle_vminu_vx::<Mac> as HandleFunction<Mac>,
        handle_vmin_vv::<Mac> as HandleFunction<Mac>,
        handle_vmin_vx::<Mac> as HandleFunction<Mac>,
        handle_vmaxu_vv::<Mac> as HandleFunction<Mac>,
        handle_vmaxu_vx::<Mac> as HandleFunction<Mac>,
        handle_vmax_vv::<Mac> as HandleFunction<Mac>,
        handle_vmax_vx::<Mac> as HandleFunction<Mac>,
        handle_vwaddu_vv::<Mac> as HandleFunction<Mac>,
        handle_vwaddu_vx::<Mac> as HandleFunction<Mac>,
        handle_vwsubu_vv::<Mac> as HandleFunction<Mac>,
        handle_vwsubu_vx::<Mac> as HandleFunction<Mac>,
        handle_vwadd_vv::<Mac> as HandleFunction<Mac>,
        handle_vwadd_vx::<Mac> as HandleFunction<Mac>,
        handle_vwsub_vv::<Mac> as HandleFunction<Mac>,
        handle_vwsub_vx::<Mac> as HandleFunction<Mac>,
        handle_vwaddu_wv::<Mac> as HandleFunction<Mac>,
        handle_vwaddu_wx::<Mac> as HandleFunction<Mac>,
        handle_vwsubu_wv::<Mac> as HandleFunction<Mac>,
        handle_vwsubu_wx::<Mac> as HandleFunction<Mac>,
        handle_vwadd_wv::<Mac> as HandleFunction<Mac>,
        handle_vwadd_wx::<Mac> as HandleFunction<Mac>,
        handle_vwsub_wv::<Mac> as HandleFunction<Mac>,
        handle_vwsub_wx::<Mac> as HandleFunction<Mac>,
        handle_vzext_vf8::<Mac> as HandleFunction<Mac>,
        handle_vsext_vf8::<Mac> as HandleFunction<Mac>,
        handle_vzext_vf4::<Mac> as HandleFunction<Mac>,
        handle_vsext_vf4::<Mac> as HandleFunction<Mac>,
        handle_vzext_vf2::<Mac> as HandleFunction<Mac>,
        handle_vsext_vf2::<Mac> as HandleFunction<Mac>,
        handle_vadc_vvm::<Mac> as HandleFunction<Mac>,
        handle_vadc_vxm::<Mac> as HandleFunction<Mac>,
        handle_vadc_vim::<Mac> as HandleFunction<Mac>,
        handle_vmadc_vvm::<Mac> as HandleFunction<Mac>,
        handle_vmadc_vxm::<Mac> as HandleFunction<Mac>,
        handle_vmadc_vim::<Mac> as HandleFunction<Mac>,
        handle_vmadc_vv::<Mac> as HandleFunction<Mac>,
        handle_vmadc_vx::<Mac> as HandleFunction<Mac>,
        handle_vmadc_vi::<Mac> as HandleFunction<Mac>,
        handle_vsbc_vvm::<Mac> as HandleFunction<Mac>,
        handle_vsbc_vxm::<Mac> as HandleFunction<Mac>,
        handle_vmsbc_vvm::<Mac> as HandleFunction<Mac>,
        handle_vmsbc_vxm::<Mac> as HandleFunction<Mac>,
        handle_vmsbc_vv::<Mac> as HandleFunction<Mac>,
        handle_vmsbc_vx::<Mac> as HandleFunction<Mac>,
        handle_vand_vv::<Mac> as HandleFunction<Mac>,
        handle_vand_vi::<Mac> as HandleFunction<Mac>,
        handle_vand_vx::<Mac> as HandleFunction<Mac>,
        handle_vor_vv::<Mac> as HandleFunction<Mac>,
        handle_vor_vx::<Mac> as HandleFunction<Mac>,
        handle_vor_vi::<Mac> as HandleFunction<Mac>,
        handle_vxor_vv::<Mac> as HandleFunction<Mac>,
        handle_vxor_vx::<Mac> as HandleFunction<Mac>,
        handle_vxor_vi::<Mac> as HandleFunction<Mac>,
        handle_vnsrl_wv::<Mac> as HandleFunction<Mac>,
        handle_vnsrl_wx::<Mac> as HandleFunction<Mac>,
        handle_vnsrl_wi::<Mac> as HandleFunction<Mac>,
        handle_vnsra_wv::<Mac> as HandleFunction<Mac>,
        handle_vnsra_wx::<Mac> as HandleFunction<Mac>,
        handle_vnsra_wi::<Mac> as HandleFunction<Mac>,
        handle_vmulh_vv::<Mac> as HandleFunction<Mac>,
        handle_vmulh_vx::<Mac> as HandleFunction<Mac>,
        handle_vmulhu_vv::<Mac> as HandleFunction<Mac>,
        handle_vmulhu_vx::<Mac> as HandleFunction<Mac>,
        handle_vmulhsu_vv::<Mac> as HandleFunction<Mac>,
        handle_vmulhsu_vx::<Mac> as HandleFunction<Mac>,
        handle_vwmulu_vv::<Mac> as HandleFunction<Mac>,
        handle_vwmulu_vx::<Mac> as HandleFunction<Mac>,
        handle_vwmulsu_vv::<Mac> as HandleFunction<Mac>,
        handle_vwmulsu_vx::<Mac> as HandleFunction<Mac>,
        handle_vwmul_vv::<Mac> as HandleFunction<Mac>,
        handle_vwmul_vx::<Mac> as HandleFunction<Mac>,
        handle_vmv_v_v::<Mac> as HandleFunction<Mac>,
        handle_vmv_v_x::<Mac> as HandleFunction<Mac>,
        handle_vmv_v_i::<Mac> as HandleFunction<Mac>,
        handle_vsaddu_vv::<Mac> as HandleFunction<Mac>,
        handle_vsaddu_vx::<Mac> as HandleFunction<Mac>,
        handle_vsaddu_vi::<Mac> as HandleFunction<Mac>,
        handle_vsadd_vv::<Mac> as HandleFunction<Mac>,
        handle_vsadd_vx::<Mac> as HandleFunction<Mac>,
        handle_vsadd_vi::<Mac> as HandleFunction<Mac>,
        handle_vssubu_vv::<Mac> as HandleFunction<Mac>,
        handle_vssubu_vx::<Mac> as HandleFunction<Mac>,
        handle_vssub_vv::<Mac> as HandleFunction<Mac>,
        handle_vssub_vx::<Mac> as HandleFunction<Mac>,
        handle_vaaddu_vv::<Mac> as HandleFunction<Mac>,
        handle_vaaddu_vx::<Mac> as HandleFunction<Mac>,
        handle_vaadd_vv::<Mac> as HandleFunction<Mac>,
        handle_vaadd_vx::<Mac> as HandleFunction<Mac>,
        handle_vasubu_vv::<Mac> as HandleFunction<Mac>,
        handle_vasubu_vx::<Mac> as HandleFunction<Mac>,
        handle_vasub_vv::<Mac> as HandleFunction<Mac>,
        handle_vasub_vx::<Mac> as HandleFunction<Mac>,
        handle_vmv1r_v::<Mac> as HandleFunction<Mac>,
        handle_vmv2r_v::<Mac> as HandleFunction<Mac>,
        handle_vmv4r_v::<Mac> as HandleFunction<Mac>,
        handle_vmv8r_v::<Mac> as HandleFunction<Mac>,
        handle_vfirst_m::<Mac> as HandleFunction<Mac>,
        handle_vmand_mm::<Mac> as HandleFunction<Mac>,
        handle_vmnand_mm::<Mac> as HandleFunction<Mac>,
        handle_vmandnot_mm::<Mac> as HandleFunction<Mac>,
        handle_vmxor_mm::<Mac> as HandleFunction<Mac>,
        handle_vmor_mm::<Mac> as HandleFunction<Mac>,
        handle_vmnor_mm::<Mac> as HandleFunction<Mac>,
        handle_vmornot_mm::<Mac> as HandleFunction<Mac>,
        handle_vmxnor_mm::<Mac> as HandleFunction<Mac>,
        handle_vlse8_v::<Mac> as HandleFunction<Mac>,
        handle_vlse16_v::<Mac> as HandleFunction<Mac>,
        handle_vlse32_v::<Mac> as HandleFunction<Mac>,
        handle_vlse64_v::<Mac> as HandleFunction<Mac>,
        handle_vlse128_v::<Mac> as HandleFunction<Mac>,
        handle_vlse256_v::<Mac> as HandleFunction<Mac>,
        handle_vlse512_v::<Mac> as HandleFunction<Mac>,
        handle_vlse1024_v::<Mac> as HandleFunction<Mac>,
        handle_vsse8_v::<Mac> as HandleFunction<Mac>,
        handle_vsse16_v::<Mac> as HandleFunction<Mac>,
        handle_vsse32_v::<Mac> as HandleFunction<Mac>,
        handle_vsse64_v::<Mac> as HandleFunction<Mac>,
        handle_vsse128_v::<Mac> as HandleFunction<Mac>,
        handle_vsse256_v::<Mac> as HandleFunction<Mac>,
        handle_vsse512_v::<Mac> as HandleFunction<Mac>,
        handle_vsse1024_v::<Mac> as HandleFunction<Mac>,
        handle_vluxei8_v::<Mac> as HandleFunction<Mac>,
        handle_vluxei16_v::<Mac> as HandleFunction<Mac>,
        handle_vluxei32_v::<Mac> as HandleFunction<Mac>,
        handle_vluxei64_v::<Mac> as HandleFunction<Mac>,
        handle_vloxei8_v::<Mac> as HandleFunction<Mac>,
        handle_vloxei16_v::<Mac> as HandleFunction<Mac>,
        handle_vloxei32_v::<Mac> as HandleFunction<Mac>,
        handle_vloxei64_v::<Mac> as HandleFunction<Mac>,
        handle_vsuxei8_v::<Mac> as HandleFunction<Mac>,
        handle_vsuxei16_v::<Mac> as HandleFunction<Mac>,
        handle_vsuxei32_v::<Mac> as HandleFunction<Mac>,
        handle_vsuxei64_v::<Mac> as HandleFunction<Mac>,
        handle_vsoxei8_v::<Mac> as HandleFunction<Mac>,
        handle_vsoxei16_v::<Mac> as HandleFunction<Mac>,
        handle_vsoxei32_v::<Mac> as HandleFunction<Mac>,
        handle_vsoxei64_v::<Mac> as HandleFunction<Mac>,
        handle_vl1re8_v::<Mac> as HandleFunction<Mac>,
        handle_vl1re16_v::<Mac> as HandleFunction<Mac>,
        handle_vl1re32_v::<Mac> as HandleFunction<Mac>,
        handle_vl1re64_v::<Mac> as HandleFunction<Mac>,
        handle_vl2re8_v::<Mac> as HandleFunction<Mac>,
        handle_vl2re16_v::<Mac> as HandleFunction<Mac>,
        handle_vl2re32_v::<Mac> as HandleFunction<Mac>,
        handle_vl2re64_v::<Mac> as HandleFunction<Mac>,
        handle_vl4re8_v::<Mac> as HandleFunction<Mac>,
        handle_vl4re16_v::<Mac> as HandleFunction<Mac>,
        handle_vl4re32_v::<Mac> as HandleFunction<Mac>,
        handle_vl4re64_v::<Mac> as HandleFunction<Mac>,
        handle_vl8re8_v::<Mac> as HandleFunction<Mac>,
        handle_vl8re16_v::<Mac> as HandleFunction<Mac>,
        handle_vl8re32_v::<Mac> as HandleFunction<Mac>,
        handle_vl8re64_v::<Mac> as HandleFunction<Mac>,
        handle_vs1r_v::<Mac> as HandleFunction<Mac>,
        handle_vs2r_v::<Mac> as HandleFunction<Mac>,
        handle_vs4r_v::<Mac> as HandleFunction<Mac>,
        handle_vs8r_v::<Mac> as HandleFunction<Mac>,
        handle_vmacc_vv::<Mac> as HandleFunction<Mac>,
        handle_vmacc_vx::<Mac> as HandleFunction<Mac>,
        handle_vnmsac_vv::<Mac> as HandleFunction<Mac>,
        handle_vnmsac_vx::<Mac> as HandleFunction<Mac>,
        handle_vmadd_vv::<Mac> as HandleFunction<Mac>,
        handle_vmadd_vx::<Mac> as HandleFunction<Mac>,
        handle_vnmsub_vv::<Mac> as HandleFunction<Mac>,
        handle_vnmsub_vx::<Mac> as HandleFunction<Mac>,
        handle_vssrl_vv::<Mac> as HandleFunction<Mac>,
        handle_vssrl_vx::<Mac> as HandleFunction<Mac>,
        handle_vssrl_vi::<Mac> as HandleFunction<Mac>,
        handle_vssra_vv::<Mac> as HandleFunction<Mac>,
        handle_vssra_vx::<Mac> as HandleFunction<Mac>,
        handle_vssra_vi::<Mac> as HandleFunction<Mac>,
        handle_vsmul_vv::<Mac> as HandleFunction<Mac>,
        handle_vsmul_vx::<Mac> as HandleFunction<Mac>,
        handle_vwmaccu_vv::<Mac> as HandleFunction<Mac>,
        handle_vwmaccu_vx::<Mac> as HandleFunction<Mac>,
        handle_vwmacc_vv::<Mac> as HandleFunction<Mac>,
        handle_vwmacc_vx::<Mac> as HandleFunction<Mac>,
        handle_vwmaccsu_vv::<Mac> as HandleFunction<Mac>,
        handle_vwmaccsu_vx::<Mac> as HandleFunction<Mac>,
        handle_vwmaccus_vx::<Mac> as HandleFunction<Mac>,
        handle_vmerge_vvm::<Mac> as HandleFunction<Mac>,
        handle_vmerge_vxm::<Mac> as HandleFunction<Mac>,
        handle_vmerge_vim::<Mac> as HandleFunction<Mac>,
        handle_vnclipu_wv::<Mac> as HandleFunction<Mac>,
        handle_vnclipu_wx::<Mac> as HandleFunction<Mac>,
        handle_vnclipu_wi::<Mac> as HandleFunction<Mac>,
        handle_vnclip_wv::<Mac> as HandleFunction<Mac>,
        handle_vnclip_wx::<Mac> as HandleFunction<Mac>,
        handle_vnclip_wi::<Mac> as HandleFunction<Mac>,
        handle_vredsum_vs::<Mac> as HandleFunction<Mac>,
        handle_vredand_vs::<Mac> as HandleFunction<Mac>,
        handle_vredor_vs::<Mac> as HandleFunction<Mac>,
        handle_vredxor_vs::<Mac> as HandleFunction<Mac>,
        handle_vredminu_vs::<Mac> as HandleFunction<Mac>,
        handle_vredmin_vs::<Mac> as HandleFunction<Mac>,
        handle_vredmaxu_vs::<Mac> as HandleFunction<Mac>,
        handle_vredmax_vs::<Mac> as HandleFunction<Mac>,
        handle_vwredsumu_vs::<Mac> as HandleFunction<Mac>,
        handle_vwredsum_vs::<Mac> as HandleFunction<Mac>,
        handle_vcpop_m::<Mac> as HandleFunction<Mac>,
        handle_vmsbf_m::<Mac> as HandleFunction<Mac>,
        handle_vmsof_m::<Mac> as HandleFunction<Mac>,
        handle_vmsif_m::<Mac> as HandleFunction<Mac>,
        handle_viota_m::<Mac> as HandleFunction<Mac>,
        handle_vid_v::<Mac> as HandleFunction<Mac>,
        handle_vmv_x_s::<Mac> as HandleFunction<Mac>,
        handle_vmv_s_x::<Mac> as HandleFunction<Mac>,
        handle_vcompress_vm::<Mac> as HandleFunction<Mac>,
        handle_vslide1up_vx::<Mac> as HandleFunction<Mac>,
        handle_vslideup_vx::<Mac> as HandleFunction<Mac>,
        handle_vslideup_vi::<Mac> as HandleFunction<Mac>,
        handle_vslide1down_vx::<Mac> as HandleFunction<Mac>,
        handle_vslidedown_vx::<Mac> as HandleFunction<Mac>,
        handle_vslidedown_vi::<Mac> as HandleFunction<Mac>,
        handle_vrgather_vx::<Mac> as HandleFunction<Mac>,
        handle_vrgather_vv::<Mac> as HandleFunction<Mac>,
        handle_vrgatherei16_vv::<Mac> as HandleFunction<Mac>,
        handle_vrgather_vi::<Mac> as HandleFunction<Mac>,
    ]
}

pub fn generate_handle_function_list<Mac: Machine>(
) -> [HandleFunction<Mac>; insts::MAXIMUM_LEVEL2_OPCODE as usize + 1] {
    [
        handle_unloaded::<Mac> as HandleFunction<Mac>,
        handle_add::<Mac> as HandleFunction<Mac>,
        handle_addi::<Mac> as HandleFunction<Mac>,
        handle_addiw::<Mac> as HandleFunction<Mac>,
        handle_addw::<Mac> as HandleFunction<Mac>,
        handle_and::<Mac> as HandleFunction<Mac>,
        handle_andi::<Mac> as HandleFunction<Mac>,
        handle_div::<Mac> as HandleFunction<Mac>,
        handle_divu::<Mac> as HandleFunction<Mac>,
        handle_divuw::<Mac> as HandleFunction<Mac>,
        handle_divw::<Mac> as HandleFunction<Mac>,
        handle_fence::<Mac> as HandleFunction<Mac>,
        handle_fencei::<Mac> as HandleFunction<Mac>,
        handle_lb::<Mac> as HandleFunction<Mac>,
        handle_lbu::<Mac> as HandleFunction<Mac>,
        handle_ld::<Mac> as HandleFunction<Mac>,
        handle_lh::<Mac> as HandleFunction<Mac>,
        handle_lhu::<Mac> as HandleFunction<Mac>,
        handle_lui::<Mac> as HandleFunction<Mac>,
        handle_lw::<Mac> as HandleFunction<Mac>,
        handle_lwu::<Mac> as HandleFunction<Mac>,
        handle_mul::<Mac> as HandleFunction<Mac>,
        handle_mulh::<Mac> as HandleFunction<Mac>,
        handle_mulhsu::<Mac> as HandleFunction<Mac>,
        handle_mulhu::<Mac> as HandleFunction<Mac>,
        handle_mulw::<Mac> as HandleFunction<Mac>,
        handle_or::<Mac> as HandleFunction<Mac>,
        handle_ori::<Mac> as HandleFunction<Mac>,
        handle_rem::<Mac> as HandleFunction<Mac>,
        handle_remu::<Mac> as HandleFunction<Mac>,
        handle_remuw::<Mac> as HandleFunction<Mac>,
        handle_remw::<Mac> as HandleFunction<Mac>,
        handle_sb::<Mac> as HandleFunction<Mac>,
        handle_sd::<Mac> as HandleFunction<Mac>,
        handle_sh::<Mac> as HandleFunction<Mac>,
        handle_sll::<Mac> as HandleFunction<Mac>,
        handle_slli::<Mac> as HandleFunction<Mac>,
        handle_slliw::<Mac> as HandleFunction<Mac>,
        handle_sllw::<Mac> as HandleFunction<Mac>,
        handle_slt::<Mac> as HandleFunction<Mac>,
        handle_slti::<Mac> as HandleFunction<Mac>,
        handle_sltiu::<Mac> as HandleFunction<Mac>,
        handle_sltu::<Mac> as HandleFunction<Mac>,
        handle_sra::<Mac> as HandleFunction<Mac>,
        handle_srai::<Mac> as HandleFunction<Mac>,
        handle_sraiw::<Mac> as HandleFunction<Mac>,
        handle_sraw::<Mac> as HandleFunction<Mac>,
        handle_srl::<Mac> as HandleFunction<Mac>,
        handle_srli::<Mac> as HandleFunction<Mac>,
        handle_srliw::<Mac> as HandleFunction<Mac>,
        handle_srlw::<Mac> as HandleFunction<Mac>,
        handle_sub::<Mac> as HandleFunction<Mac>,
        handle_subw::<Mac> as HandleFunction<Mac>,
        handle_sw::<Mac> as HandleFunction<Mac>,
        handle_xor::<Mac> as HandleFunction<Mac>,
        handle_xori::<Mac> as HandleFunction<Mac>,
        handle_adduw::<Mac> as HandleFunction<Mac>,
        handle_andn::<Mac> as HandleFunction<Mac>,
        handle_bclr::<Mac> as HandleFunction<Mac>,
        handle_bclri::<Mac> as HandleFunction<Mac>,
        handle_bext::<Mac> as HandleFunction<Mac>,
        handle_bexti::<Mac> as HandleFunction<Mac>,
        handle_binv::<Mac> as HandleFunction<Mac>,
        handle_binvi::<Mac> as HandleFunction<Mac>,
        handle_bset::<Mac> as HandleFunction<Mac>,
        handle_bseti::<Mac> as HandleFunction<Mac>,
        handle_clmul::<Mac> as HandleFunction<Mac>,
        handle_clmulh::<Mac> as HandleFunction<Mac>,
        handle_clmulr::<Mac> as HandleFunction<Mac>,
        handle_clz::<Mac> as HandleFunction<Mac>,
        handle_clzw::<Mac> as HandleFunction<Mac>,
        handle_cpop::<Mac> as HandleFunction<Mac>,
        handle_cpopw::<Mac> as HandleFunction<Mac>,
        handle_ctz::<Mac> as HandleFunction<Mac>,
        handle_ctzw::<Mac> as HandleFunction<Mac>,
        handle_max::<Mac> as HandleFunction<Mac>,
        handle_maxu::<Mac> as HandleFunction<Mac>,
        handle_min::<Mac> as HandleFunction<Mac>,
        handle_minu::<Mac> as HandleFunction<Mac>,
        handle_orcb::<Mac> as HandleFunction<Mac>,
        handle_orn::<Mac> as HandleFunction<Mac>,
        handle_rev8::<Mac> as HandleFunction<Mac>,
        handle_rol::<Mac> as HandleFunction<Mac>,
        handle_rolw::<Mac> as HandleFunction<Mac>,
        handle_ror::<Mac> as HandleFunction<Mac>,
        handle_rori::<Mac> as HandleFunction<Mac>,
        handle_roriw::<Mac> as HandleFunction<Mac>,
        handle_rorw::<Mac> as HandleFunction<Mac>,
        handle_sextb::<Mac> as HandleFunction<Mac>,
        handle_sexth::<Mac> as HandleFunction<Mac>,
        handle_sh1add::<Mac> as HandleFunction<Mac>,
        handle_sh1adduw::<Mac> as HandleFunction<Mac>,
        handle_sh2add::<Mac> as HandleFunction<Mac>,
        handle_sh2adduw::<Mac> as HandleFunction<Mac>,
        handle_sh3add::<Mac> as HandleFunction<Mac>,
        handle_sh3adduw::<Mac> as HandleFunction<Mac>,
        handle_slliuw::<Mac> as HandleFunction<Mac>,
        handle_xnor::<Mac> as HandleFunction<Mac>,
        handle_zexth::<Mac> as HandleFunction<Mac>,
        handle_wide_mul::<Mac> as HandleFunction<Mac>,
        handle_wide_mulu::<Mac> as HandleFunction<Mac>,
        handle_wide_mulsu::<Mac> as HandleFunction<Mac>,
        handle_wide_div::<Mac> as HandleFunction<Mac>,
        handle_wide_divu::<Mac> as HandleFunction<Mac>,
        handle_ld_sign_extended_32_constant::<Mac> as HandleFunction<Mac>,
        handle_adc::<Mac> as HandleFunction<Mac>,
        handle_sbb::<Mac> as HandleFunction<Mac>,
        handle_custom_load_imm::<Mac> as HandleFunction<Mac>,
        handle_auipc::<Mac> as HandleFunction<Mac>,
        handle_beq::<Mac> as HandleFunction<Mac>,
        handle_bge::<Mac> as HandleFunction<Mac>,
        handle_bgeu::<Mac> as HandleFunction<Mac>,
        handle_blt::<Mac> as HandleFunction<Mac>,
        handle_bltu::<Mac> as HandleFunction<Mac>,
        handle_bne::<Mac> as HandleFunction<Mac>,
        handle_ebreak::<Mac> as HandleFunction<Mac>,
        handle_ecall::<Mac> as HandleFunction<Mac>,
        handle_jal::<Mac> as HandleFunction<Mac>,
        handle_jalr::<Mac> as HandleFunction<Mac>,
        handle_far_jump_rel::<Mac> as HandleFunction<Mac>,
        handle_far_jump_abs::<Mac> as HandleFunction<Mac>,
        handle_custom_trace_end::<Mac> as HandleFunction<Mac>,
        handle_vsetvli::<Mac> as HandleFunction<Mac>,
        handle_vsetivli::<Mac> as HandleFunction<Mac>,
        handle_vsetvl::<Mac> as HandleFunction<Mac>,
        handle_vlm_v::<Mac> as HandleFunction<Mac>,
        handle_vle8_v::<Mac> as HandleFunction<Mac>,
        handle_vle16_v::<Mac> as HandleFunction<Mac>,
        handle_vle32_v::<Mac> as HandleFunction<Mac>,
        handle_vle64_v::<Mac> as HandleFunction<Mac>,
        handle_vle128_v::<Mac> as HandleFunction<Mac>,
        handle_vle256_v::<Mac> as HandleFunction<Mac>,
        handle_vle512_v::<Mac> as HandleFunction<Mac>,
        handle_vle1024_v::<Mac> as HandleFunction<Mac>,
        handle_vsm_v::<Mac> as HandleFunction<Mac>,
        handle_vse8_v::<Mac> as HandleFunction<Mac>,
        handle_vse16_v::<Mac> as HandleFunction<Mac>,
        handle_vse32_v::<Mac> as HandleFunction<Mac>,
        handle_vse64_v::<Mac> as HandleFunction<Mac>,
        handle_vse128_v::<Mac> as HandleFunction<Mac>,
        handle_vse256_v::<Mac> as HandleFunction<Mac>,
        handle_vse512_v::<Mac> as HandleFunction<Mac>,
        handle_vse1024_v::<Mac> as HandleFunction<Mac>,
        handle_vadd_vv::<Mac> as HandleFunction<Mac>,
        handle_vadd_vx::<Mac> as HandleFunction<Mac>,
        handle_vadd_vi::<Mac> as HandleFunction<Mac>,
        handle_vsub_vv::<Mac> as HandleFunction<Mac>,
        handle_vsub_vx::<Mac> as HandleFunction<Mac>,
        handle_vrsub_vx::<Mac> as HandleFunction<Mac>,
        handle_vrsub_vi::<Mac> as HandleFunction<Mac>,
        handle_vmul_vv::<Mac> as HandleFunction<Mac>,
        handle_vmul_vx::<Mac> as HandleFunction<Mac>,
        handle_vdiv_vv::<Mac> as HandleFunction<Mac>,
        handle_vdiv_vx::<Mac> as HandleFunction<Mac>,
        handle_vdivu_vv::<Mac> as HandleFunction<Mac>,
        handle_vdivu_vx::<Mac> as HandleFunction<Mac>,
        handle_vrem_vv::<Mac> as HandleFunction<Mac>,
        handle_vrem_vx::<Mac> as HandleFunction<Mac>,
        handle_vremu_vv::<Mac> as HandleFunction<Mac>,
        handle_vremu_vx::<Mac> as HandleFunction<Mac>,
        handle_vsll_vv::<Mac> as HandleFunction<Mac>,
        handle_vsll_vx::<Mac> as HandleFunction<Mac>,
        handle_vsll_vi::<Mac> as HandleFunction<Mac>,
        handle_vsrl_vv::<Mac> as HandleFunction<Mac>,
        handle_vsrl_vx::<Mac> as HandleFunction<Mac>,
        handle_vsrl_vi::<Mac> as HandleFunction<Mac>,
        handle_vsra_vv::<Mac> as HandleFunction<Mac>,
        handle_vsra_vx::<Mac> as HandleFunction<Mac>,
        handle_vsra_vi::<Mac> as HandleFunction<Mac>,
        handle_vmseq_vv::<Mac> as HandleFunction<Mac>,
        handle_vmseq_vx::<Mac> as HandleFunction<Mac>,
        handle_vmseq_vi::<Mac> as HandleFunction<Mac>,
        handle_vmsne_vv::<Mac> as HandleFunction<Mac>,
        handle_vmsne_vx::<Mac> as HandleFunction<Mac>,
        handle_vmsne_vi::<Mac> as HandleFunction<Mac>,
        handle_vmsltu_vv::<Mac> as HandleFunction<Mac>,
        handle_vmsltu_vx::<Mac> as HandleFunction<Mac>,
        handle_vmslt_vv::<Mac> as HandleFunction<Mac>,
        handle_vmslt_vx::<Mac> as HandleFunction<Mac>,
        handle_vmsleu_vv::<Mac> as HandleFunction<Mac>,
        handle_vmsleu_vx::<Mac> as HandleFunction<Mac>,
        handle_vmsleu_vi::<Mac> as HandleFunction<Mac>,
        handle_vmsle_vv::<Mac> as HandleFunction<Mac>,
        handle_vmsle_vx::<Mac> as HandleFunction<Mac>,
        handle_vmsle_vi::<Mac> as HandleFunction<Mac>,
        handle_vmsgtu_vx::<Mac> as HandleFunction<Mac>,
        handle_vmsgtu_vi::<Mac> as HandleFunction<Mac>,
        handle_vmsgt_vx::<Mac> as HandleFunction<Mac>,
        handle_vmsgt_vi::<Mac> as HandleFunction<Mac>,
        handle_vminu_vv::<Mac> as HandleFunction<Mac>,
        handle_vminu_vx::<Mac> as HandleFunction<Mac>,
        handle_vmin_vv::<Mac> as HandleFunction<Mac>,
        handle_vmin_vx::<Mac> as HandleFunction<Mac>,
        handle_vmaxu_vv::<Mac> as HandleFunction<Mac>,
        handle_vmaxu_vx::<Mac> as HandleFunction<Mac>,
        handle_vmax_vv::<Mac> as HandleFunction<Mac>,
        handle_vmax_vx::<Mac> as HandleFunction<Mac>,
        handle_vwaddu_vv::<Mac> as HandleFunction<Mac>,
        handle_vwaddu_vx::<Mac> as HandleFunction<Mac>,
        handle_vwsubu_vv::<Mac> as HandleFunction<Mac>,
        handle_vwsubu_vx::<Mac> as HandleFunction<Mac>,
        handle_vwadd_vv::<Mac> as HandleFunction<Mac>,
        handle_vwadd_vx::<Mac> as HandleFunction<Mac>,
        handle_vwsub_vv::<Mac> as HandleFunction<Mac>,
        handle_vwsub_vx::<Mac> as HandleFunction<Mac>,
        handle_vwaddu_wv::<Mac> as HandleFunction<Mac>,
        handle_vwaddu_wx::<Mac> as HandleFunction<Mac>,
        handle_vwsubu_wv::<Mac> as HandleFunction<Mac>,
        handle_vwsubu_wx::<Mac> as HandleFunction<Mac>,
        handle_vwadd_wv::<Mac> as HandleFunction<Mac>,
        handle_vwadd_wx::<Mac> as HandleFunction<Mac>,
        handle_vwsub_wv::<Mac> as HandleFunction<Mac>,
        handle_vwsub_wx::<Mac> as HandleFunction<Mac>,
        handle_vzext_vf8::<Mac> as HandleFunction<Mac>,
        handle_vsext_vf8::<Mac> as HandleFunction<Mac>,
        handle_vzext_vf4::<Mac> as HandleFunction<Mac>,
        handle_vsext_vf4::<Mac> as HandleFunction<Mac>,
        handle_vzext_vf2::<Mac> as HandleFunction<Mac>,
        handle_vsext_vf2::<Mac> as HandleFunction<Mac>,
        handle_vadc_vvm::<Mac> as HandleFunction<Mac>,
        handle_vadc_vxm::<Mac> as HandleFunction<Mac>,
        handle_vadc_vim::<Mac> as HandleFunction<Mac>,
        handle_vmadc_vvm::<Mac> as HandleFunction<Mac>,
        handle_vmadc_vxm::<Mac> as HandleFunction<Mac>,
        handle_vmadc_vim::<Mac> as HandleFunction<Mac>,
        handle_vmadc_vv::<Mac> as HandleFunction<Mac>,
        handle_vmadc_vx::<Mac> as HandleFunction<Mac>,
        handle_vmadc_vi::<Mac> as HandleFunction<Mac>,
        handle_vsbc_vvm::<Mac> as HandleFunction<Mac>,
        handle_vsbc_vxm::<Mac> as HandleFunction<Mac>,
        handle_vmsbc_vvm::<Mac> as HandleFunction<Mac>,
        handle_vmsbc_vxm::<Mac> as HandleFunction<Mac>,
        handle_vmsbc_vv::<Mac> as HandleFunction<Mac>,
        handle_vmsbc_vx::<Mac> as HandleFunction<Mac>,
        handle_vand_vv::<Mac> as HandleFunction<Mac>,
        handle_vand_vi::<Mac> as HandleFunction<Mac>,
        handle_vand_vx::<Mac> as HandleFunction<Mac>,
        handle_vor_vv::<Mac> as HandleFunction<Mac>,
        handle_vor_vx::<Mac> as HandleFunction<Mac>,
        handle_vor_vi::<Mac> as HandleFunction<Mac>,
        handle_vxor_vv::<Mac> as HandleFunction<Mac>,
        handle_vxor_vx::<Mac> as HandleFunction<Mac>,
        handle_vxor_vi::<Mac> as HandleFunction<Mac>,
        handle_vnsrl_wv::<Mac> as HandleFunction<Mac>,
        handle_vnsrl_wx::<Mac> as HandleFunction<Mac>,
        handle_vnsrl_wi::<Mac> as HandleFunction<Mac>,
        handle_vnsra_wv::<Mac> as HandleFunction<Mac>,
        handle_vnsra_wx::<Mac> as HandleFunction<Mac>,
        handle_vnsra_wi::<Mac> as HandleFunction<Mac>,
        handle_vmulh_vv::<Mac> as HandleFunction<Mac>,
        handle_vmulh_vx::<Mac> as HandleFunction<Mac>,
        handle_vmulhu_vv::<Mac> as HandleFunction<Mac>,
        handle_vmulhu_vx::<Mac> as HandleFunction<Mac>,
        handle_vmulhsu_vv::<Mac> as HandleFunction<Mac>,
        handle_vmulhsu_vx::<Mac> as HandleFunction<Mac>,
        handle_vwmulu_vv::<Mac> as HandleFunction<Mac>,
        handle_vwmulu_vx::<Mac> as HandleFunction<Mac>,
        handle_vwmulsu_vv::<Mac> as HandleFunction<Mac>,
        handle_vwmulsu_vx::<Mac> as HandleFunction<Mac>,
        handle_vwmul_vv::<Mac> as HandleFunction<Mac>,
        handle_vwmul_vx::<Mac> as HandleFunction<Mac>,
        handle_vmv_v_v::<Mac> as HandleFunction<Mac>,
        handle_vmv_v_x::<Mac> as HandleFunction<Mac>,
        handle_vmv_v_i::<Mac> as HandleFunction<Mac>,
        handle_vsaddu_vv::<Mac> as HandleFunction<Mac>,
        handle_vsaddu_vx::<Mac> as HandleFunction<Mac>,
        handle_vsaddu_vi::<Mac> as HandleFunction<Mac>,
        handle_vsadd_vv::<Mac> as HandleFunction<Mac>,
        handle_vsadd_vx::<Mac> as HandleFunction<Mac>,
        handle_vsadd_vi::<Mac> as HandleFunction<Mac>,
        handle_vssubu_vv::<Mac> as HandleFunction<Mac>,
        handle_vssubu_vx::<Mac> as HandleFunction<Mac>,
        handle_vssub_vv::<Mac> as HandleFunction<Mac>,
        handle_vssub_vx::<Mac> as HandleFunction<Mac>,
        handle_vaaddu_vv::<Mac> as HandleFunction<Mac>,
        handle_vaaddu_vx::<Mac> as HandleFunction<Mac>,
        handle_vaadd_vv::<Mac> as HandleFunction<Mac>,
        handle_vaadd_vx::<Mac> as HandleFunction<Mac>,
        handle_vasubu_vv::<Mac> as HandleFunction<Mac>,
        handle_vasubu_vx::<Mac> as HandleFunction<Mac>,
        handle_vasub_vv::<Mac> as HandleFunction<Mac>,
        handle_vasub_vx::<Mac> as HandleFunction<Mac>,
        handle_vmv1r_v::<Mac> as HandleFunction<Mac>,
        handle_vmv2r_v::<Mac> as HandleFunction<Mac>,
        handle_vmv4r_v::<Mac> as HandleFunction<Mac>,
        handle_vmv8r_v::<Mac> as HandleFunction<Mac>,
        handle_vfirst_m::<Mac> as HandleFunction<Mac>,
        handle_vmand_mm::<Mac> as HandleFunction<Mac>,
        handle_vmnand_mm::<Mac> as HandleFunction<Mac>,
        handle_vmandnot_mm::<Mac> as HandleFunction<Mac>,
        handle_vmxor_mm::<Mac> as HandleFunction<Mac>,
        handle_vmor_mm::<Mac> as HandleFunction<Mac>,
        handle_vmnor_mm::<Mac> as HandleFunction<Mac>,
        handle_vmornot_mm::<Mac> as HandleFunction<Mac>,
        handle_vmxnor_mm::<Mac> as HandleFunction<Mac>,
        handle_vlse8_v::<Mac> as HandleFunction<Mac>,
        handle_vlse16_v::<Mac> as HandleFunction<Mac>,
        handle_vlse32_v::<Mac> as HandleFunction<Mac>,
        handle_vlse64_v::<Mac> as HandleFunction<Mac>,
        handle_vlse128_v::<Mac> as HandleFunction<Mac>,
        handle_vlse256_v::<Mac> as HandleFunction<Mac>,
        handle_vlse512_v::<Mac> as HandleFunction<Mac>,
        handle_vlse1024_v::<Mac> as HandleFunction<Mac>,
        handle_vsse8_v::<Mac> as HandleFunction<Mac>,
        handle_vsse16_v::<Mac> as HandleFunction<Mac>,
        handle_vsse32_v::<Mac> as HandleFunction<Mac>,
        handle_vsse64_v::<Mac> as HandleFunction<Mac>,
        handle_vsse128_v::<Mac> as HandleFunction<Mac>,
        handle_vsse256_v::<Mac> as HandleFunction<Mac>,
        handle_vsse512_v::<Mac> as HandleFunction<Mac>,
        handle_vsse1024_v::<Mac> as HandleFunction<Mac>,
        handle_vluxei8_v::<Mac> as HandleFunction<Mac>,
        handle_vluxei16_v::<Mac> as HandleFunction<Mac>,
        handle_vluxei32_v::<Mac> as HandleFunction<Mac>,
        handle_vluxei64_v::<Mac> as HandleFunction<Mac>,
        handle_vloxei8_v::<Mac> as HandleFunction<Mac>,
        handle_vloxei16_v::<Mac> as HandleFunction<Mac>,
        handle_vloxei32_v::<Mac> as HandleFunction<Mac>,
        handle_vloxei64_v::<Mac> as HandleFunction<Mac>,
        handle_vsuxei8_v::<Mac> as HandleFunction<Mac>,
        handle_vsuxei16_v::<Mac> as HandleFunction<Mac>,
        handle_vsuxei32_v::<Mac> as HandleFunction<Mac>,
        handle_vsuxei64_v::<Mac> as HandleFunction<Mac>,
        handle_vsoxei8_v::<Mac> as HandleFunction<Mac>,
        handle_vsoxei16_v::<Mac> as HandleFunction<Mac>,
        handle_vsoxei32_v::<Mac> as HandleFunction<Mac>,
        handle_vsoxei64_v::<Mac> as HandleFunction<Mac>,
        handle_vl1re8_v::<Mac> as HandleFunction<Mac>,
        handle_vl1re16_v::<Mac> as HandleFunction<Mac>,
        handle_vl1re32_v::<Mac> as HandleFunction<Mac>,
        handle_vl1re64_v::<Mac> as HandleFunction<Mac>,
        handle_vl2re8_v::<Mac> as HandleFunction<Mac>,
        handle_vl2re16_v::<Mac> as HandleFunction<Mac>,
        handle_vl2re32_v::<Mac> as HandleFunction<Mac>,
        handle_vl2re64_v::<Mac> as HandleFunction<Mac>,
        handle_vl4re8_v::<Mac> as HandleFunction<Mac>,
        handle_vl4re16_v::<Mac> as HandleFunction<Mac>,
        handle_vl4re32_v::<Mac> as HandleFunction<Mac>,
        handle_vl4re64_v::<Mac> as HandleFunction<Mac>,
        handle_vl8re8_v::<Mac> as HandleFunction<Mac>,
        handle_vl8re16_v::<Mac> as HandleFunction<Mac>,
        handle_vl8re32_v::<Mac> as HandleFunction<Mac>,
        handle_vl8re64_v::<Mac> as HandleFunction<Mac>,
        handle_vs1r_v::<Mac> as HandleFunction<Mac>,
        handle_vs2r_v::<Mac> as HandleFunction<Mac>,
        handle_vs4r_v::<Mac> as HandleFunction<Mac>,
        handle_vs8r_v::<Mac> as HandleFunction<Mac>,
        handle_vmacc_vv::<Mac> as HandleFunction<Mac>,
        handle_vmacc_vx::<Mac> as HandleFunction<Mac>,
        handle_vnmsac_vv::<Mac> as HandleFunction<Mac>,
        handle_vnmsac_vx::<Mac> as HandleFunction<Mac>,
        handle_vmadd_vv::<Mac> as HandleFunction<Mac>,
        handle_vmadd_vx::<Mac> as HandleFunction<Mac>,
        handle_vnmsub_vv::<Mac> as HandleFunction<Mac>,
        handle_vnmsub_vx::<Mac> as HandleFunction<Mac>,
        handle_vssrl_vv::<Mac> as HandleFunction<Mac>,
        handle_vssrl_vx::<Mac> as HandleFunction<Mac>,
        handle_vssrl_vi::<Mac> as HandleFunction<Mac>,
        handle_vssra_vv::<Mac> as HandleFunction<Mac>,
        handle_vssra_vx::<Mac> as HandleFunction<Mac>,
        handle_vssra_vi::<Mac> as HandleFunction<Mac>,
        handle_vsmul_vv::<Mac> as HandleFunction<Mac>,
        handle_vsmul_vx::<Mac> as HandleFunction<Mac>,
        handle_vwmaccu_vv::<Mac> as HandleFunction<Mac>,
        handle_vwmaccu_vx::<Mac> as HandleFunction<Mac>,
        handle_vwmacc_vv::<Mac> as HandleFunction<Mac>,
        handle_vwmacc_vx::<Mac> as HandleFunction<Mac>,
        handle_vwmaccsu_vv::<Mac> as HandleFunction<Mac>,
        handle_vwmaccsu_vx::<Mac> as HandleFunction<Mac>,
        handle_vwmaccus_vx::<Mac> as HandleFunction<Mac>,
        handle_vmerge_vvm::<Mac> as HandleFunction<Mac>,
        handle_vmerge_vxm::<Mac> as HandleFunction<Mac>,
        handle_vmerge_vim::<Mac> as HandleFunction<Mac>,
        handle_vnclipu_wv::<Mac> as HandleFunction<Mac>,
        handle_vnclipu_wx::<Mac> as HandleFunction<Mac>,
        handle_vnclipu_wi::<Mac> as HandleFunction<Mac>,
        handle_vnclip_wv::<Mac> as HandleFunction<Mac>,
        handle_vnclip_wx::<Mac> as HandleFunction<Mac>,
        handle_vnclip_wi::<Mac> as HandleFunction<Mac>,
        handle_vredsum_vs::<Mac> as HandleFunction<Mac>,
        handle_vredand_vs::<Mac> as HandleFunction<Mac>,
        handle_vredor_vs::<Mac> as HandleFunction<Mac>,
        handle_vredxor_vs::<Mac> as HandleFunction<Mac>,
        handle_vredminu_vs::<Mac> as HandleFunction<Mac>,
        handle_vredmin_vs::<Mac> as HandleFunction<Mac>,
        handle_vredmaxu_vs::<Mac> as HandleFunction<Mac>,
        handle_vredmax_vs::<Mac> as HandleFunction<Mac>,
        handle_vwredsumu_vs::<Mac> as HandleFunction<Mac>,
        handle_vwredsum_vs::<Mac> as HandleFunction<Mac>,
        handle_vcpop_m::<Mac> as HandleFunction<Mac>,
        handle_vmsbf_m::<Mac> as HandleFunction<Mac>,
        handle_vmsof_m::<Mac> as HandleFunction<Mac>,
        handle_vmsif_m::<Mac> as HandleFunction<Mac>,
        handle_viota_m::<Mac> as HandleFunction<Mac>,
        handle_vid_v::<Mac> as HandleFunction<Mac>,
        handle_vmv_x_s::<Mac> as HandleFunction<Mac>,
        handle_vmv_s_x::<Mac> as HandleFunction<Mac>,
        handle_vcompress_vm::<Mac> as HandleFunction<Mac>,
        handle_vslide1up_vx::<Mac> as HandleFunction<Mac>,
        handle_vslideup_vx::<Mac> as HandleFunction<Mac>,
        handle_vslideup_vi::<Mac> as HandleFunction<Mac>,
        handle_vslide1down_vx::<Mac> as HandleFunction<Mac>,
        handle_vslidedown_vx::<Mac> as HandleFunction<Mac>,
        handle_vslidedown_vi::<Mac> as HandleFunction<Mac>,
        handle_vrgather_vx::<Mac> as HandleFunction<Mac>,
        handle_vrgather_vv::<Mac> as HandleFunction<Mac>,
        handle_vrgatherei16_vv::<Mac> as HandleFunction<Mac>,
        handle_vrgather_vi::<Mac> as HandleFunction<Mac>,
    ]
}

pub fn execute_instruction<Mac: Machine>(
    machine: &mut Mac,
    handle_function_list: &[HandleFunction<Mac>],
    inst: Instruction,
) -> Result<(), Error> {
    let op = extract_opcode(inst);
    handle_function_list[op as usize](machine, inst)
}

pub fn execute<Mac: Machine>(
    machine: &mut Mac,
    handle_function_list: &[HandleFunction<Mac>],
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
