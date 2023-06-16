use super::{
    super::{machine::Machine, Error},
    common, extract_opcode, instruction_length,
    utils::update_register,
    Instruction, InstructionOpcode, Itype, R4type, R5type, Register, Rtype, Stype, Utype,
};
use crate::memory::Memory;
use ckb_vm_definitions::{
    for_each_inst_array1, for_each_inst_match2,
    instructions::{self as insts, paste},
    registers::RA,
};

pub fn handle_sub<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    common::sub(machine, i.rd(), i.rs1(), i.rs2());
    Ok(())
}

pub fn handle_subw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    common::subw(machine, i.rd(), i.rs1(), i.rs2());
    Ok(())
}

pub fn handle_add<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    common::add(machine, i.rd(), i.rs1(), i.rs2());
    Ok(())
}

pub fn handle_addw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    common::addw(machine, i.rd(), i.rs1(), i.rs2());
    Ok(())
}

pub fn handle_xor<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    common::xor(machine, i.rd(), i.rs1(), i.rs2());
    Ok(())
}

pub fn handle_or<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    common::or(machine, i.rd(), i.rs1(), i.rs2());
    Ok(())
}

pub fn handle_and<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    common::and(machine, i.rd(), i.rs1(), i.rs2());
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

pub fn handle_sllw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let shift_value = machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(0x1F);
    let value = machine.registers()[i.rs1()].clone() << shift_value;
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

pub fn handle_srlw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let shift_value = machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(0x1F);
    let value = machine.registers()[i.rs1()].zero_extend(&Mac::REG::from_u8(32)) >> shift_value;
    update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
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

pub fn handle_sraw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let shift_value = machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(0x1F);
    let value = machine.registers()[i.rs1()]
        .sign_extend(&Mac::REG::from_u8(32))
        .signed_shr(&shift_value);
    update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
    Ok(())
}

pub fn handle_slt<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.lt_s(rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_sltu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.lt(rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_lb_version0<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::lb(machine, i.rd(), i.rs1(), i.immediate_s(), true)?;
    Ok(())
}

pub fn handle_lb_version1<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::lb(machine, i.rd(), i.rs1(), i.immediate_s(), false)?;
    Ok(())
}

pub fn handle_lh_version0<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::lh(machine, i.rd(), i.rs1(), i.immediate_s(), true)?;
    Ok(())
}

pub fn handle_lh_version1<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::lh(machine, i.rd(), i.rs1(), i.immediate_s(), false)?;
    Ok(())
}

pub fn handle_lw_version0<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::lw(machine, i.rd(), i.rs1(), i.immediate_s(), true)?;
    Ok(())
}

pub fn handle_lw_version1<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::lw(machine, i.rd(), i.rs1(), i.immediate_s(), false)?;
    Ok(())
}

pub fn handle_ld_version0<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::ld(machine, i.rd(), i.rs1(), i.immediate_s(), true)?;
    Ok(())
}

pub fn handle_ld_version1<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::ld(machine, i.rd(), i.rs1(), i.immediate_s(), false)?;
    Ok(())
}

pub fn handle_lbu_version0<Mac: Machine>(
    machine: &mut Mac,
    inst: Instruction,
) -> Result<(), Error> {
    let i = Itype(inst);
    common::lbu(machine, i.rd(), i.rs1(), i.immediate_s(), true)?;
    Ok(())
}

pub fn handle_lbu_version1<Mac: Machine>(
    machine: &mut Mac,
    inst: Instruction,
) -> Result<(), Error> {
    let i = Itype(inst);
    common::lbu(machine, i.rd(), i.rs1(), i.immediate_s(), false)?;
    Ok(())
}

pub fn handle_lhu_version0<Mac: Machine>(
    machine: &mut Mac,
    inst: Instruction,
) -> Result<(), Error> {
    let i = Itype(inst);
    common::lhu(machine, i.rd(), i.rs1(), i.immediate_s(), true)?;
    Ok(())
}

pub fn handle_lhu_version1<Mac: Machine>(
    machine: &mut Mac,
    inst: Instruction,
) -> Result<(), Error> {
    let i = Itype(inst);
    common::lhu(machine, i.rd(), i.rs1(), i.immediate_s(), false)?;
    Ok(())
}

pub fn handle_lwu_version0<Mac: Machine>(
    machine: &mut Mac,
    inst: Instruction,
) -> Result<(), Error> {
    let i = Itype(inst);
    common::lwu(machine, i.rd(), i.rs1(), i.immediate_s(), true)?;
    Ok(())
}

pub fn handle_lwu_version1<Mac: Machine>(
    machine: &mut Mac,
    inst: Instruction,
) -> Result<(), Error> {
    let i = Itype(inst);
    common::lwu(machine, i.rd(), i.rs1(), i.immediate_s(), false)?;
    Ok(())
}

pub fn handle_addi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::addi(machine, i.rd(), i.rs1(), i.immediate_s());
    Ok(())
}

pub fn handle_addiw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::addiw(machine, i.rd(), i.rs1(), i.immediate_s());
    Ok(())
}

pub fn handle_xori<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::xori(machine, i.rd(), i.rs1(), i.immediate_s());
    Ok(())
}

pub fn handle_lr_w<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let address = machine.registers()[i.rs1()].clone();
    let value = machine.memory_mut().load32(&address)?;
    update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
    machine.memory_mut().set_lr(&address);
    Ok(())
}

pub fn handle_sc_w<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let address = machine.registers()[i.rs1()].clone();
    let condition = address.eq(machine.memory().lr());
    let mem_value = condition.cond(
        &machine.registers()[i.rs2()].clone(),
        &machine.memory_mut().load32(&address)?,
    );
    let rd_value = condition.cond(&Mac::REG::from_u8(0), &Mac::REG::from_u8(1));
    machine.memory_mut().store32(&address, &mem_value)?;
    update_register(machine, i.rd(), rd_value);
    machine.memory_mut().set_lr(&Mac::REG::from_u64(u64::MAX));
    Ok(())
}

pub fn handle_amoswap_w<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = machine.registers()[i.rs1()].clone();
    let rs2_value = machine.registers()[i.rs2()].clone();
    let mem_value = machine.memory_mut().load32(&rs1_value)?;
    let mem_value = mem_value.sign_extend(&Mac::REG::from_u8(32));
    update_register(machine, i.rd(), mem_value);
    machine.memory_mut().store32(&rs1_value, &rs2_value)?;
    Ok(())
}

pub fn handle_amoadd_w<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = machine.registers()[i.rs1()].clone();
    let rs2_value = machine.registers()[i.rs2()].clone();
    let mem_value = machine.memory_mut().load32(&rs1_value)?;
    let mem_value = mem_value.sign_extend(&Mac::REG::from_u8(32));
    update_register(machine, i.rd(), mem_value.clone());
    let mem_value = rs2_value.overflowing_add(&mem_value);
    machine.memory_mut().store32(&rs1_value, &mem_value)?;
    Ok(())
}

pub fn handle_amoxor_w<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = machine.registers()[i.rs1()].clone();
    let rs2_value = machine.registers()[i.rs2()].clone();
    let mem_value = machine.memory_mut().load32(&rs1_value)?;
    let mem_value = mem_value.sign_extend(&Mac::REG::from_u8(32));
    update_register(machine, i.rd(), mem_value.clone());
    let mem_value = rs2_value ^ mem_value;
    machine.memory_mut().store32(&rs1_value, &mem_value)?;
    Ok(())
}

pub fn handle_amoand_w<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = machine.registers()[i.rs1()].clone();
    let rs2_value = machine.registers()[i.rs2()].clone();
    let mem_value = machine.memory_mut().load32(&rs1_value)?;
    let mem_value = mem_value.sign_extend(&Mac::REG::from_u8(32));
    update_register(machine, i.rd(), mem_value.clone());
    let mem_value = rs2_value & mem_value;
    machine.memory_mut().store32(&rs1_value, &mem_value)?;
    Ok(())
}

pub fn handle_amoor_w<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = machine.registers()[i.rs1()].clone();
    let rs2_value = machine.registers()[i.rs2()].clone();
    let mem_value = machine.memory_mut().load32(&rs1_value)?;
    let mem_value = mem_value.sign_extend(&Mac::REG::from_u8(32));
    update_register(machine, i.rd(), mem_value.clone());
    let mem_value = rs2_value | mem_value;
    machine.memory_mut().store32(&rs1_value, &mem_value)?;
    Ok(())
}

pub fn handle_amomin_w<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = machine.registers()[i.rs1()].clone();
    let rs2_value = machine.registers()[i.rs2()].sign_extend(&Mac::REG::from_u8(32));
    let mem_value = machine.memory_mut().load32(&rs1_value)?;
    let mem_value = mem_value.sign_extend(&Mac::REG::from_u8(32));
    update_register(machine, i.rd(), mem_value.clone());
    let mem_value = rs2_value.lt_s(&mem_value).cond(&rs2_value, &mem_value);
    machine.memory_mut().store32(&rs1_value, &mem_value)?;
    Ok(())
}

pub fn handle_amomax_w<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = machine.registers()[i.rs1()].clone();
    let rs2_value = machine.registers()[i.rs2()].sign_extend(&Mac::REG::from_u8(32));
    let mem_value = machine.memory_mut().load32(&rs1_value)?;
    let mem_value = mem_value.sign_extend(&Mac::REG::from_u8(32));
    update_register(machine, i.rd(), mem_value.clone());
    let mem_value = rs2_value.ge_s(&mem_value).cond(&rs2_value, &mem_value);
    machine.memory_mut().store32(&rs1_value, &mem_value)?;
    Ok(())
}

pub fn handle_amominu_w<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = machine.registers()[i.rs1()].clone();
    let rs2_value = machine.registers()[i.rs2()].zero_extend(&Mac::REG::from_u8(32));
    let mem_value = machine.memory_mut().load32(&rs1_value)?;
    let mem_value_sext = mem_value.sign_extend(&Mac::REG::from_u8(32));
    update_register(machine, i.rd(), mem_value_sext);
    let mem_value = rs2_value.lt(&mem_value).cond(&rs2_value, &mem_value);
    machine.memory_mut().store32(&rs1_value, &mem_value)?;
    Ok(())
}

pub fn handle_amomaxu_w<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = machine.registers()[i.rs1()].clone();
    let rs2_value = machine.registers()[i.rs2()].zero_extend(&Mac::REG::from_u8(32));
    let mem_value = machine.memory_mut().load32(&rs1_value)?;
    let mem_value_sext = mem_value.sign_extend(&Mac::REG::from_u8(32));
    update_register(machine, i.rd(), mem_value_sext);
    let mem_value = rs2_value.ge(&mem_value).cond(&rs2_value, &mem_value);
    machine.memory_mut().store32(&rs1_value, &mem_value)?;
    Ok(())
}

pub fn handle_lr_d<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let address = machine.registers()[i.rs1()].clone();
    let value = machine.memory_mut().load64(&address)?;
    update_register(machine, i.rd(), value);
    machine.memory_mut().set_lr(&address);
    Ok(())
}

pub fn handle_sc_d<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let address = machine.registers()[i.rs1()].clone();
    let condition = address.eq(machine.memory().lr());
    let mem_value = condition.cond(
        &machine.registers()[i.rs2()].clone(),
        &machine.memory_mut().load64(&address)?,
    );
    let rd_value = condition.cond(&Mac::REG::from_u8(0), &Mac::REG::from_u8(1));
    machine.memory_mut().store64(&address, &mem_value)?;
    update_register(machine, i.rd(), rd_value);
    machine.memory_mut().set_lr(&Mac::REG::from_u64(u64::MAX));
    Ok(())
}

pub fn handle_amoswap_d<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = machine.registers()[i.rs1()].clone();
    let rs2_value = machine.registers()[i.rs2()].clone();
    let mem_value = machine.memory_mut().load64(&rs1_value)?;
    update_register(machine, i.rd(), mem_value);
    machine.memory_mut().store64(&rs1_value, &rs2_value)?;
    Ok(())
}

pub fn handle_amoadd_d<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = machine.registers()[i.rs1()].clone();
    let rs2_value = machine.registers()[i.rs2()].clone();
    let mem_value = machine.memory_mut().load64(&rs1_value)?;
    update_register(machine, i.rd(), mem_value.clone());
    let mem_value = rs2_value.overflowing_add(&mem_value);
    machine.memory_mut().store64(&rs1_value, &mem_value)?;
    Ok(())
}

pub fn handle_amoxor_d<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = machine.registers()[i.rs1()].clone();
    let rs2_value = machine.registers()[i.rs2()].clone();
    let mem_value = machine.memory_mut().load64(&rs1_value)?;
    update_register(machine, i.rd(), mem_value.clone());
    let mem_value = rs2_value ^ mem_value;
    machine.memory_mut().store64(&rs1_value, &mem_value)?;
    Ok(())
}

pub fn handle_amoand_d<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = machine.registers()[i.rs1()].clone();
    let rs2_value = machine.registers()[i.rs2()].clone();
    let mem_value = machine.memory_mut().load64(&rs1_value)?;
    update_register(machine, i.rd(), mem_value.clone());
    let mem_value = rs2_value & mem_value;
    machine.memory_mut().store64(&rs1_value, &mem_value)?;
    Ok(())
}

pub fn handle_amoor_d<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = machine.registers()[i.rs1()].clone();
    let rs2_value = machine.registers()[i.rs2()].clone();
    let mem_value = machine.memory_mut().load64(&rs1_value)?;
    update_register(machine, i.rd(), mem_value.clone());
    let mem_value = rs2_value | mem_value;
    machine.memory_mut().store64(&rs1_value, &mem_value)?;
    Ok(())
}

pub fn handle_amomin_d<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = machine.registers()[i.rs1()].clone();
    let rs2_value = machine.registers()[i.rs2()].clone();
    let mem_value = machine.memory_mut().load64(&rs1_value)?;
    update_register(machine, i.rd(), mem_value.clone());
    let mem_value = rs2_value.lt_s(&mem_value).cond(&rs2_value, &mem_value);
    machine.memory_mut().store64(&rs1_value, &mem_value)?;
    Ok(())
}

pub fn handle_amomax_d<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = machine.registers()[i.rs1()].clone();
    let rs2_value = machine.registers()[i.rs2()].clone();
    let mem_value = machine.memory_mut().load64(&rs1_value)?;
    update_register(machine, i.rd(), mem_value.clone());
    let mem_value = rs2_value.ge_s(&mem_value).cond(&rs2_value, &mem_value);
    machine.memory_mut().store64(&rs1_value, &mem_value)?;
    Ok(())
}

pub fn handle_amominu_d<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = machine.registers()[i.rs1()].clone();
    let rs2_value = machine.registers()[i.rs2()].clone();
    let mem_value = machine.memory_mut().load64(&rs1_value)?;
    update_register(machine, i.rd(), mem_value.clone());
    let mem_value = rs2_value.lt(&mem_value).cond(&rs2_value, &mem_value);
    machine.memory_mut().store64(&rs1_value, &mem_value)?;
    Ok(())
}

pub fn handle_amomaxu_d<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = machine.registers()[i.rs1()].clone();
    let rs2_value = machine.registers()[i.rs2()].clone();
    let mem_value = machine.memory_mut().load64(&rs1_value)?;
    update_register(machine, i.rd(), mem_value.clone());
    let mem_value = rs2_value.ge(&mem_value).cond(&rs2_value, &mem_value);
    machine.memory_mut().store64(&rs1_value, &mem_value)?;
    Ok(())
}

pub fn handle_ori<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::ori(machine, i.rd(), i.rs1(), i.immediate_s());
    Ok(())
}

pub fn handle_andi<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::andi(machine, i.rd(), i.rs1(), i.immediate_s());
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

pub fn handle_jalr_version0<Mac: Machine>(
    machine: &mut Mac,
    inst: Instruction,
) -> Result<(), Error> {
    let i = Itype(inst);
    let size = instruction_length(inst);
    let link = machine.pc().overflowing_add(&Mac::REG::from_u8(size));
    update_register(machine, i.rd(), link);
    let mut next_pc =
        machine.registers()[i.rs1()].overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
    next_pc = next_pc & (!Mac::REG::one());
    machine.update_pc(next_pc);
    Ok(())
}

pub fn handle_jalr_version1<Mac: Machine>(
    machine: &mut Mac,
    inst: Instruction,
) -> Result<(), Error> {
    let i = Itype(inst);
    let size = instruction_length(inst);
    let link = machine.pc().overflowing_add(&Mac::REG::from_u8(size));
    let mut next_pc =
        machine.registers()[i.rs1()].overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
    next_pc = next_pc & (!Mac::REG::one());
    update_register(machine, i.rd(), link);
    machine.update_pc(next_pc);
    Ok(())
}

pub fn handle_slli<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::slli(machine, i.rd(), i.rs1(), i.immediate_u());
    Ok(())
}

pub fn handle_srli<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::srli(machine, i.rd(), i.rs1(), i.immediate_u());
    Ok(())
}

pub fn handle_srai<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::srai(machine, i.rd(), i.rs1(), i.immediate_u());
    Ok(())
}

pub fn handle_slliw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::slliw(machine, i.rd(), i.rs1(), i.immediate_u());
    Ok(())
}

pub fn handle_srliw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::srliw(machine, i.rd(), i.rs1(), i.immediate_u());
    Ok(())
}

pub fn handle_sraiw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Itype(inst);
    common::sraiw(machine, i.rd(), i.rs1(), i.immediate_u());
    Ok(())
}

pub fn handle_sb<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Stype(inst);
    common::sb(machine, i.rs1(), i.rs2(), i.immediate_s())?;
    Ok(())
}

pub fn handle_sh<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Stype(inst);
    common::sh(machine, i.rs1(), i.rs2(), i.immediate_s())?;
    Ok(())
}

pub fn handle_sw<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Stype(inst);
    common::sw(machine, i.rs1(), i.rs2(), i.immediate_s())?;
    Ok(())
}

pub fn handle_sd<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Stype(inst);
    common::sd(machine, i.rs1(), i.rs2(), i.immediate_s())?;
    Ok(())
}

pub fn handle_beq<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Stype(inst);
    let pc = machine.pc();
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let condition = rs1_value.eq(rs2_value);
    let new_pc = condition.cond(
        &Mac::REG::from_i32(i.immediate_s()).overflowing_add(pc),
        &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(pc),
    );
    machine.update_pc(new_pc);
    Ok(())
}

pub fn handle_bne<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Stype(inst);
    let pc = machine.pc();
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let condition = rs1_value.ne(rs2_value);
    let new_pc = condition.cond(
        &Mac::REG::from_i32(i.immediate_s()).overflowing_add(pc),
        &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(pc),
    );
    machine.update_pc(new_pc);
    Ok(())
}

pub fn handle_blt<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Stype(inst);
    let pc = machine.pc();
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let condition = rs1_value.lt_s(rs2_value);
    let new_pc = condition.cond(
        &Mac::REG::from_i32(i.immediate_s()).overflowing_add(pc),
        &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(pc),
    );
    machine.update_pc(new_pc);
    Ok(())
}

pub fn handle_bge<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Stype(inst);
    let pc = machine.pc();
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let condition = rs1_value.ge_s(rs2_value);
    let new_pc = condition.cond(
        &Mac::REG::from_i32(i.immediate_s()).overflowing_add(pc),
        &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(pc),
    );
    machine.update_pc(new_pc);
    Ok(())
}

pub fn handle_bltu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Stype(inst);
    let pc = machine.pc();
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let condition = rs1_value.lt(rs2_value);
    let new_pc = condition.cond(
        &Mac::REG::from_i32(i.immediate_s()).overflowing_add(pc),
        &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(pc),
    );
    machine.update_pc(new_pc);
    Ok(())
}

pub fn handle_bgeu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Stype(inst);
    let pc = machine.pc();
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let condition = rs1_value.ge(rs2_value);
    let new_pc = condition.cond(
        &Mac::REG::from_i32(i.immediate_s()).overflowing_add(pc),
        &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(pc),
    );
    machine.update_pc(new_pc);
    Ok(())
}

pub fn handle_lui<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Utype(inst);
    update_register(machine, i.rd(), Mac::REG::from_i32(i.immediate_s()));
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

pub fn handle_ecall<Mac: Machine>(machine: &mut Mac, _inst: Instruction) -> Result<(), Error> {
    // The semantic of ECALL is determined by the hardware, which
    // is not part of the spec, hence here the implementation is
    // deferred to the machine. This way custom ECALLs might be
    // provided for different environments.
    machine.ecall()?;
    Ok(())
}

pub fn handle_ebreak<Mac: Machine>(machine: &mut Mac, _inst: Instruction) -> Result<(), Error> {
    machine.ebreak()?;
    Ok(())
}

pub fn handle_fencei<Mac: Machine>(_machine: &mut Mac, _inst: Instruction) -> Result<(), Error> {
    Ok(())
}

pub fn handle_fence<Mac: Machine>(_machine: &mut Mac, _inst: Instruction) -> Result<(), Error> {
    Ok(())
}

pub fn handle_jal<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Utype(inst);
    common::jal(machine, i.rd(), i.immediate_s(), instruction_length(inst));
    Ok(())
}

pub fn handle_mul<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.overflowing_mul(rs2_value);
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

pub fn handle_mulh<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.overflowing_mul_high_signed(rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_mulhsu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.overflowing_mul_high_signed_unsigned(rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_mulhu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.overflowing_mul_high_unsigned(rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_div<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.overflowing_div_signed(rs2_value);
    update_register(machine, i.rd(), value);
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

pub fn handle_divu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.overflowing_div(rs2_value);
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

pub fn handle_rem<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.overflowing_rem_signed(rs2_value);
    update_register(machine, i.rd(), value);
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

pub fn handle_remu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.overflowing_rem(rs2_value);
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
    let value = rs1_value.ge_s(rs2_value).cond(rs1_value, rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_maxu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.ge(rs2_value).cond(rs1_value, rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_min<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.lt_s(rs2_value).cond(rs1_value, rs2_value);
    update_register(machine, i.rd(), value);
    Ok(())
}

pub fn handle_minu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value = rs1_value.lt(rs2_value).cond(rs1_value, rs2_value);
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
    let value_h = rs1_value.overflowing_mul_high_signed(rs2_value);
    let value_l = rs1_value.overflowing_mul(rs2_value);
    update_register(machine, i.rd(), value_h);
    update_register(machine, i.rs3(), value_l);
    Ok(())
}

pub fn handle_wide_mulu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = R4type(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value_h = rs1_value.overflowing_mul_high_unsigned(rs2_value);
    let value_l = rs1_value.overflowing_mul(rs2_value);
    update_register(machine, i.rd(), value_h);
    update_register(machine, i.rs3(), value_l);
    Ok(())
}

pub fn handle_wide_mulsu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = R4type(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value_h = rs1_value.overflowing_mul_high_signed_unsigned(rs2_value);
    let value_l = rs1_value.overflowing_mul(rs2_value);
    update_register(machine, i.rd(), value_h);
    update_register(machine, i.rs3(), value_l);
    Ok(())
}

pub fn handle_wide_div<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = R4type(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value_h = rs1_value.overflowing_div_signed(rs2_value);
    let value_l = rs1_value.overflowing_rem_signed(rs2_value);
    update_register(machine, i.rd(), value_h);
    update_register(machine, i.rs3(), value_l);
    Ok(())
}

pub fn handle_wide_divu<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = R4type(inst);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let value_h = rs1_value.overflowing_div(rs2_value);
    let value_l = rs1_value.overflowing_rem(rs2_value);
    update_register(machine, i.rd(), value_h);
    update_register(machine, i.rs3(), value_l);
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

pub fn handle_adc<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = Rtype(inst);
    let rd_value = &machine.registers()[i.rd()];
    let rs1_value = &machine.registers()[i.rs1()];
    let r = rd_value.overflowing_add(rs1_value);
    update_register(machine, i.rd(), r);
    let rd_value = &machine.registers()[i.rd()];
    let rs1_value = &machine.registers()[i.rs1()];
    let r = rd_value.lt(rs1_value);
    update_register(machine, i.rs1(), r);
    let rd_value = &machine.registers()[i.rd()];
    let rs2_value = &machine.registers()[i.rs2()];
    let r = rd_value.overflowing_add(rs2_value);
    update_register(machine, i.rd(), r);
    let rd_value = &machine.registers()[i.rd()];
    let rs2_value = &machine.registers()[i.rs2()];
    let r = rd_value.lt(rs2_value);
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
    let r = rd_value.overflowing_sub(rs1_value);
    update_register(machine, i.rs1(), r);
    let rd_value = &machine.registers()[i.rd()];
    let rs1_value = &machine.registers()[i.rs1()];
    let r = rd_value.lt(rs1_value);
    update_register(machine, i.rs3(), r);
    let rs1_value = &machine.registers()[i.rs1()];
    let rs2_value = &machine.registers()[i.rs2()];
    let r = rs1_value.overflowing_sub(rs2_value);
    update_register(machine, i.rd(), r);
    let rd_value = &machine.registers()[i.rd()];
    let rs1_value = &machine.registers()[i.rs1()];
    let r = rs1_value.lt(rd_value);
    update_register(machine, i.rs2(), r);
    let rs2_value = machine.registers()[i.rs2()].clone();
    let rs3_value = machine.registers()[i.rs3()].clone();
    let r = rs2_value | rs3_value;
    update_register(machine, i.rs1(), r);
    Ok(())
}

pub fn handle_adcs<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = R4type(inst);
    let rs1_value = machine.registers()[i.rs1()].clone();
    let rs2_value = machine.registers()[i.rs2()].clone();
    let r = rs1_value.overflowing_add(&rs2_value);
    update_register(machine, i.rd(), r.clone());
    let r = r.lt(&rs1_value);
    update_register(machine, i.rs3(), r);
    Ok(())
}

pub fn handle_sbbs<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = R4type(inst);
    let rs1_value = machine.registers()[i.rs1()].clone();
    let rs2_value = machine.registers()[i.rs2()].clone();
    let r = rs1_value.overflowing_sub(&rs2_value);
    update_register(machine, i.rd(), r);
    let r = rs1_value.lt(&rs2_value);
    update_register(machine, i.rs3(), r);
    Ok(())
}

pub fn handle_add3a<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = R5type(inst);
    {
        let rd_value = machine.registers()[i.rd()].clone();
        let rs1_value = machine.registers()[i.rs1()].clone();
        let r = rd_value.overflowing_add(&rs1_value);
        update_register(machine, i.rd(), r);
    }
    {
        let rd_value = &machine.registers()[i.rd()];
        let rs1_value = &machine.registers()[i.rs1()];
        let r = rd_value.lt(rs1_value);
        update_register(machine, i.rs2(), r);
    }
    {
        let rs2_value = &machine.registers()[i.rs2()];
        let rs4_value = &machine.registers()[i.rs4()];
        let r = rs2_value.overflowing_add(rs4_value);
        update_register(machine, i.rs3(), r);
    }
    Ok(())
}

pub fn handle_add3b<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = R5type(inst);
    {
        let rs1_value = machine.registers()[i.rs1()].clone();
        let rs2_value = machine.registers()[i.rs2()].clone();
        let r = rs1_value.overflowing_add(&rs2_value);
        update_register(machine, i.rd(), r);
    }
    {
        let rd_value = &machine.registers()[i.rd()];
        let rs1_value = &machine.registers()[i.rs1()];
        let r = rd_value.lt(rs1_value);
        update_register(machine, i.rs1(), r);
    }
    {
        let rs1_value = &machine.registers()[i.rs1()];
        let rs4_value = &machine.registers()[i.rs4()];
        let r = rs1_value.overflowing_add(rs4_value);
        update_register(machine, i.rs3(), r);
    }
    Ok(())
}

pub fn handle_add3c<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    let i = R5type(inst);
    {
        let rs1_value = machine.registers()[i.rs1()].clone();
        let rs2_value = machine.registers()[i.rs2()].clone();
        let r = rs1_value.overflowing_add(&rs2_value);
        update_register(machine, i.rd(), r);
    }
    {
        let rd_value = &machine.registers()[i.rd()];
        let rs1_value = &machine.registers()[i.rs1()];
        let rs4_value = &machine.registers()[i.rs4()];
        let r = rd_value.lt(rs1_value);
        let r = r.overflowing_add(rs4_value);
        update_register(machine, i.rs3(), r);
    }
    Ok(())
}

pub fn handle_custom_load_uimm<Mac: Machine>(
    machine: &mut Mac,
    inst: Instruction,
) -> Result<(), Error> {
    let i = Utype(inst);
    update_register(machine, i.rd(), Mac::REG::from_u32(i.immediate_u()));
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

pub fn handle_unloaded<Mac: Machine>(machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    handle_invalid_op(machine, inst)
}

pub fn handle_custom_trace_end<Mac: Machine>(
    machine: &mut Mac,
    inst: Instruction,
) -> Result<(), Error> {
    handle_invalid_op(machine, inst)
}

pub fn handle_custom_asm_trace_jump<Mac: Machine>(
    machine: &mut Mac,
    inst: Instruction,
) -> Result<(), Error> {
    handle_invalid_op(machine, inst)
}

pub fn handle_invalid_op<Mac: Machine>(_machine: &mut Mac, inst: Instruction) -> Result<(), Error> {
    Err(Error::InvalidOp(extract_opcode(inst)))
}

macro_rules! handle_single_opcode {
    ($name:ident, $real_name:ident, $code:expr, $machine:ident, $inst:ident) => {
        paste! {
            Ok([< handle_ $real_name:lower >]($machine, $inst)?)
        }
    };
}

pub fn execute_instruction<Mac: Machine>(
    inst: Instruction,
    machine: &mut Mac,
) -> Result<(), Error> {
    let op = extract_opcode(inst);
    for_each_inst_match2!(
        handle_single_opcode,
        op,
        handle_invalid_op(machine, inst),
        machine,
        inst
    )
}

pub fn execute<Mac: Machine>(inst: Instruction, machine: &mut Mac) -> Result<(), Error> {
    let instruction_size = instruction_length(inst);
    let next_pc = machine
        .pc()
        .overflowing_add(&Mac::REG::from_u8(instruction_size));
    machine.update_pc(next_pc);
    let r = execute_instruction(inst, machine);
    machine.commit_pc();
    r
}

pub fn execute_with_thread<Mac: Machine>(
    inst: Instruction,
    machine: &mut Mac,
    thread: &Thread<Mac>,
) -> Result<(), Error> {
    let instruction_size = instruction_length(inst);
    let next_pc = machine
        .pc()
        .overflowing_add(&Mac::REG::from_u8(instruction_size));
    machine.update_pc(next_pc);
    let r = thread(machine, inst);
    machine.commit_pc();
    r
}

pub type Thread<Mac> = fn(&mut Mac, Instruction) -> Result<(), Error>;

const FASTPATH_THREADS: usize = insts::MAXIMUM_OPCODE as usize + 1 - insts::MINIMAL_OPCODE as usize;

pub struct ThreadFactory<Mac: Machine> {
    // Right now we are only dealing with fastpath opcodes, later we might
    // (or might not?) expand this with some opcodes in the slowpath category.
    threads: [Thread<Mac>; FASTPATH_THREADS],
}

macro_rules! thread_func_item {
    ($name:ident, $real_name:ident, $code:expr, $t:ident) => {
        paste! {
            [< handle_ $real_name:lower >]::<$t> as Thread<$t>
        }
    };
}

impl<Mac: Machine> ThreadFactory<Mac> {
    pub fn create() -> Self {
        let threads = for_each_inst_array1!(thread_func_item, Mac);
        Self { threads }
    }

    pub fn get(&self, op: InstructionOpcode) -> Option<&Thread<Mac>> {
        self.threads
            .get((op as usize).wrapping_sub(insts::MINIMAL_OPCODE as usize))
    }

    pub fn get_cloned(&self, op: InstructionOpcode) -> Option<Thread<Mac>> {
        self.get(op).cloned()
    }
}

impl<Mac: Machine> std::ops::Index<InstructionOpcode> for ThreadFactory<Mac> {
    type Output = Thread<Mac>;

    fn index(&self, opcode: InstructionOpcode) -> &Thread<Mac> {
        self.get(opcode).unwrap()
    }
}
