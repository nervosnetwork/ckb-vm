use super::{
    super::{machine::Machine, registers::SP, Error},
    common, extract_opcode, instruction_length,
    utils::update_register,
    Instruction, Itype, Register, Rtype, Stype, Utype,
};
use ckb_vm_definitions::instructions as insts;

pub fn execute<Mac: Machine>(inst: Instruction, machine: &mut Mac) -> Result<(), Error> {
    let op = extract_opcode(inst);
    let next_pc: Option<Mac::REG> = match op {
        insts::OP_SUB => {
            let i = Rtype(inst);
            common::sub(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        insts::OP_SUBW => {
            let i = Rtype(inst);
            common::subw(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        insts::OP_ADD => {
            let i = Rtype(inst);
            common::add(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        insts::OP_ADDW => {
            let i = Rtype(inst);
            common::addw(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        insts::OP_XOR => {
            let i = Rtype(inst);
            common::xor(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        insts::OP_OR => {
            let i = Rtype(inst);
            common::or(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        insts::OP_AND => {
            let i = Rtype(inst);
            common::and(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        insts::OP_SLL => {
            let i = Rtype(inst);
            let shift_value =
                machine.registers()[i.rs2()].clone() & Mac::REG::from_usize(Mac::REG::SHIFT_MASK);
            let value = machine.registers()[i.rs1()].clone() << shift_value;
            update_register(machine, i.rd(), value);
            None
        }
        insts::OP_SLLW => {
            let i = Rtype(inst);
            let shift_value = machine.registers()[i.rs2()].clone() & Mac::REG::from_usize(0x1F);
            let value = machine.registers()[i.rs1()].clone() << shift_value;
            update_register(
                machine,
                i.rd(),
                value.sign_extend(&Mac::REG::from_usize(32)),
            );
            None
        }
        insts::OP_SRL => {
            let i = Rtype(inst);
            let shift_value =
                machine.registers()[i.rs2()].clone() & Mac::REG::from_usize(Mac::REG::SHIFT_MASK);
            let value = machine.registers()[i.rs1()].clone() >> shift_value;
            update_register(machine, i.rd(), value);
            None
        }
        insts::OP_SRLW => {
            let i = Rtype(inst);
            let shift_value = machine.registers()[i.rs2()].clone() & Mac::REG::from_usize(0x1F);
            let value =
                machine.registers()[i.rs1()].zero_extend(&Mac::REG::from_usize(32)) >> shift_value;
            update_register(
                machine,
                i.rd(),
                value.sign_extend(&Mac::REG::from_usize(32)),
            );
            None
        }
        insts::OP_SRA => {
            let i = Rtype(inst);
            let shift_value =
                machine.registers()[i.rs2()].clone() & Mac::REG::from_usize(Mac::REG::SHIFT_MASK);
            let value = machine.registers()[i.rs1()].signed_shr(&shift_value);
            update_register(machine, i.rd(), value);
            None
        }
        insts::OP_SRAW => {
            let i = Rtype(inst);
            let shift_value = machine.registers()[i.rs2()].clone() & Mac::REG::from_usize(0x1F);
            let value = machine.registers()[i.rs1()]
                .sign_extend(&Mac::REG::from_usize(32))
                .signed_shr(&shift_value);
            update_register(
                machine,
                i.rd(),
                value.sign_extend(&Mac::REG::from_usize(32)),
            );
            None
        }
        insts::OP_SLT => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.lt_s(&rs2_value);
            update_register(machine, i.rd(), value);
            None
        }
        insts::OP_SLTU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.lt(&rs2_value);
            update_register(machine, i.rd(), value);
            None
        }
        insts::OP_LB => {
            let i = Itype(inst);
            common::lb(machine, i.rd(), i.rs1(), i.immediate_s())?;
            None
        }
        insts::OP_LH => {
            let i = Itype(inst);
            common::lh(machine, i.rd(), i.rs1(), i.immediate_s())?;
            None
        }
        insts::OP_LW => {
            let i = Itype(inst);
            common::lw(machine, i.rd(), i.rs1(), i.immediate_s())?;
            None
        }
        insts::OP_LD => {
            let i = Itype(inst);
            common::ld(machine, i.rd(), i.rs1(), i.immediate_s())?;
            None
        }
        insts::OP_LBU => {
            let i = Itype(inst);
            common::lbu(machine, i.rd(), i.rs1(), i.immediate_s())?;
            None
        }
        insts::OP_LHU => {
            let i = Itype(inst);
            common::lhu(machine, i.rd(), i.rs1(), i.immediate_s())?;
            None
        }
        insts::OP_LWU => {
            let i = Itype(inst);
            common::lwu(machine, i.rd(), i.rs1(), i.immediate_s())?;
            None
        }
        insts::OP_ADDI => {
            let i = Itype(inst);
            common::addi(machine, i.rd(), i.rs1(), i.immediate_s());
            None
        }
        insts::OP_ADDIW => {
            let i = Itype(inst);
            common::addiw(machine, i.rd(), i.rs1(), i.immediate_s());
            None
        }
        insts::OP_XORI => {
            let i = Itype(inst);
            common::xori(machine, i.rd(), i.rs1(), i.immediate_s());
            None
        }
        insts::OP_ORI => {
            let i = Itype(inst);
            common::ori(machine, i.rd(), i.rs1(), i.immediate_s());
            None
        }
        insts::OP_ANDI => {
            let i = Itype(inst);
            common::andi(machine, i.rd(), i.rs1(), i.immediate_s());
            None
        }
        insts::OP_SLTI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let imm_value = Mac::REG::from_i32(i.immediate_s());
            let value = rs1_value.lt_s(&imm_value);
            update_register(machine, i.rd(), value);
            None
        }
        insts::OP_SLTIU => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let imm_value = Mac::REG::from_i32(i.immediate_s());
            let value = rs1_value.lt(&imm_value);
            update_register(machine, i.rd(), value);
            None
        }
        insts::OP_JALR => {
            let i = Itype(inst);
            let link = machine.pc().overflowing_add(&Mac::REG::from_usize(4));
            let mut next_pc =
                machine.registers()[i.rs1()].overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
            next_pc = next_pc & (!Mac::REG::one());
            update_register(machine, i.rd(), link);
            Some(next_pc)
        }
        insts::OP_SLLI => {
            let i = Itype(inst);
            common::slli(machine, i.rd(), i.rs1(), i.immediate());
            None
        }
        insts::OP_SRLI => {
            let i = Itype(inst);
            common::srli(machine, i.rd(), i.rs1(), i.immediate());
            None
        }
        insts::OP_SRAI => {
            let i = Itype(inst);
            common::srai(machine, i.rd(), i.rs1(), i.immediate());
            None
        }
        insts::OP_SLLIW => {
            let i = Itype(inst);
            common::slliw(machine, i.rd(), i.rs1(), i.immediate());
            None
        }
        insts::OP_SRLIW => {
            let i = Itype(inst);
            common::srliw(machine, i.rd(), i.rs1(), i.immediate());
            None
        }
        insts::OP_SRAIW => {
            let i = Itype(inst);
            common::sraiw(machine, i.rd(), i.rs1(), i.immediate());
            None
        }
        insts::OP_SB => {
            let i = Stype(inst);
            common::sb(machine, i.rs1(), i.rs2(), i.immediate_s())?;
            None
        }
        insts::OP_SH => {
            let i = Stype(inst);
            common::sh(machine, i.rs1(), i.rs2(), i.immediate_s())?;
            None
        }
        insts::OP_SW => {
            let i = Stype(inst);
            common::sw(machine, i.rs1(), i.rs2(), i.immediate_s())?;
            None
        }
        insts::OP_SD => {
            let i = Stype(inst);
            common::sd(machine, i.rs1(), i.rs2(), i.immediate_s())?;
            None
        }
        insts::OP_BEQ => {
            let i = Stype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let condition = rs1_value.eq(&rs2_value);
            let offset = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()),
                &Mac::REG::from_usize(4),
            );
            Some(machine.pc().overflowing_add(&offset))
        }
        insts::OP_BNE => {
            let i = Stype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let condition = rs1_value.ne(&rs2_value);
            let offset = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()),
                &Mac::REG::from_usize(4),
            );
            Some(machine.pc().overflowing_add(&offset))
        }
        insts::OP_BLT => {
            let i = Stype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let condition = rs1_value.lt_s(&rs2_value);
            let offset = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()),
                &Mac::REG::from_usize(4),
            );
            Some(machine.pc().overflowing_add(&offset))
        }
        insts::OP_BGE => {
            let i = Stype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let condition = rs1_value.ge_s(&rs2_value);
            let offset = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()),
                &Mac::REG::from_usize(4),
            );
            Some(machine.pc().overflowing_add(&offset))
        }
        insts::OP_BLTU => {
            let i = Stype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let condition = rs1_value.lt(&rs2_value);
            let offset = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()),
                &Mac::REG::from_usize(4),
            );
            Some(machine.pc().overflowing_add(&offset))
        }
        insts::OP_BGEU => {
            let i = Stype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let condition = rs1_value.ge(&rs2_value);
            let offset = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()),
                &Mac::REG::from_usize(4),
            );
            Some(machine.pc().overflowing_add(&offset))
        }
        insts::OP_LUI => {
            let i = Utype(inst);
            update_register(machine, i.rd(), Mac::REG::from_i32(i.immediate_s()));
            None
        }
        insts::OP_AUIPC => {
            let i = Utype(inst);
            let value = machine
                .pc()
                .overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
            update_register(machine, i.rd(), value);
            None
        }
        insts::OP_ECALL => {
            // The semantic of ECALL is determined by the hardware, which
            // is not part of the spec, hence here the implementation is
            // deferred to the machine. This way custom ECALLs might be
            // provided for different environments.
            machine.ecall()?;
            None
        }
        insts::OP_EBREAK => {
            machine.ebreak()?;
            None
        }
        insts::OP_FENCEI => None,
        insts::OP_FENCE => None,
        insts::OP_JAL => {
            let i = Utype(inst);
            common::jal(machine, i.rd(), i.immediate_s(), 4)
        }
        insts::OP_MUL => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_mul(&rs2_value);
            update_register(machine, i.rd(), value);
            None
        }
        insts::OP_MULW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value
                .zero_extend(&Mac::REG::from_usize(32))
                .overflowing_mul(&rs2_value.zero_extend(&Mac::REG::from_usize(32)));
            update_register(
                machine,
                i.rd(),
                value.sign_extend(&Mac::REG::from_usize(32)),
            );
            None
        }
        insts::OP_MULH => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_mul_high_signed(&rs2_value);
            update_register(machine, i.rd(), value);
            None
        }
        insts::OP_MULHSU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_mul_high_signed_unsigned(&rs2_value);
            update_register(machine, i.rd(), value);
            None
        }
        insts::OP_MULHU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_mul_high_unsigned(&rs2_value);
            update_register(machine, i.rd(), value);
            None
        }
        insts::OP_DIV => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_div_signed(&rs2_value);
            update_register(machine, i.rd(), value);
            None
        }
        insts::OP_DIVW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_value = rs1_value.sign_extend(&Mac::REG::from_usize(32));
            let rs2_value = rs2_value.sign_extend(&Mac::REG::from_usize(32));
            let value = rs1_value.overflowing_div_signed(&rs2_value);
            update_register(
                machine,
                i.rd(),
                value.sign_extend(&Mac::REG::from_usize(32)),
            );
            None
        }
        insts::OP_DIVU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_div(&rs2_value);
            update_register(machine, i.rd(), value);
            None
        }
        insts::OP_DIVUW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_value = rs1_value.zero_extend(&Mac::REG::from_usize(32));
            let rs2_value = rs2_value.zero_extend(&Mac::REG::from_usize(32));
            let value = rs1_value.overflowing_div(&rs2_value);
            update_register(
                machine,
                i.rd(),
                value.sign_extend(&Mac::REG::from_usize(32)),
            );
            None
        }
        insts::OP_REM => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_rem_signed(&rs2_value);
            update_register(machine, i.rd(), value);
            None
        }
        insts::OP_REMW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_value = rs1_value.sign_extend(&Mac::REG::from_usize(32));
            let rs2_value = rs2_value.sign_extend(&Mac::REG::from_usize(32));
            let value = rs1_value.overflowing_rem_signed(&rs2_value);
            update_register(
                machine,
                i.rd(),
                value.sign_extend(&Mac::REG::from_usize(32)),
            );
            None
        }
        insts::OP_REMU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_rem(&rs2_value);
            update_register(machine, i.rd(), value);
            None
        }
        insts::OP_REMUW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_value = rs1_value.zero_extend(&Mac::REG::from_usize(32));
            let rs2_value = rs2_value.zero_extend(&Mac::REG::from_usize(32));
            let value = rs1_value.overflowing_rem(&rs2_value);
            update_register(
                machine,
                i.rd(),
                value.sign_extend(&Mac::REG::from_usize(32)),
            );
            None
        }
        insts::OP_RVC_SUB => {
            let i = Rtype(inst);
            common::sub(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        insts::OP_RVC_ADD => {
            let i = Rtype(inst);
            common::add(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        insts::OP_RVC_XOR => {
            let i = Rtype(inst);
            common::xor(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        insts::OP_RVC_OR => {
            let i = Rtype(inst);
            common::or(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        insts::OP_RVC_AND => {
            let i = Rtype(inst);
            common::and(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        // > C.SUBW (RV64/128; RV32 RES)
        insts::OP_RVC_SUBW => {
            let i = Rtype(inst);
            common::subw(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        // > C.ADDW (RV64/128; RV32 RES)
        insts::OP_RVC_ADDW => {
            let i = Rtype(inst);
            common::addw(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        insts::OP_RVC_ADDI => {
            let i = Itype(inst);
            common::addi(machine, i.rd(), i.rs1(), i.immediate_s());
            None
        }
        insts::OP_RVC_ANDI => {
            let i = Itype(inst);
            common::andi(machine, i.rd(), i.rs1(), i.immediate_s());
            None
        }
        insts::OP_RVC_ADDIW => {
            let i = Itype(inst);
            common::addiw(machine, i.rd(), i.rs1(), i.immediate_s());
            None
        }
        insts::OP_RVC_SLLI => {
            let i = Itype(inst);
            common::slli(machine, i.rd(), i.rs1(), i.immediate());
            None
        }
        insts::OP_RVC_SRLI => {
            let i = Itype(inst);
            common::srli(machine, i.rd(), i.rs1(), i.immediate());
            None
        }
        insts::OP_RVC_SRAI => {
            let i = Itype(inst);
            common::srai(machine, i.rd(), i.rs1(), i.immediate());
            None
        }
        insts::OP_RVC_LW => {
            let i = Itype(inst);
            common::lw(machine, i.rd(), i.rs1(), i.immediate_s())?;
            None
        }
        insts::OP_RVC_LD => {
            let i = Itype(inst);
            common::ld(machine, i.rd(), i.rs1(), i.immediate_s())?;
            None
        }
        insts::OP_RVC_SW => {
            let i = Stype(inst);
            common::sw(machine, i.rs1(), i.rs2(), i.immediate_s())?;
            None
        }
        insts::OP_RVC_SD => {
            let i = Stype(inst);
            common::sd(machine, i.rs1(), i.rs2(), i.immediate_s())?;
            None
        }
        insts::OP_RVC_LI => {
            let i = Utype(inst);
            update_register(machine, i.rd(), Mac::REG::from_i32(i.immediate_s()));
            None
        }
        insts::OP_RVC_LUI => {
            let i = Utype(inst);
            update_register(machine, i.rd(), Mac::REG::from_i32(i.immediate_s()));
            None
        }
        insts::OP_RVC_ADDI4SPN => {
            let i = Utype(inst);
            let value = machine.registers()[SP].overflowing_add(&Mac::REG::from_u32(i.immediate()));
            update_register(machine, i.rd(), value);
            None
        }
        insts::OP_RVC_LWSP => {
            let i = Utype(inst);
            common::lw(machine, i.rd(), SP, i.immediate_s())?;
            None
        }
        insts::OP_RVC_LDSP => {
            let i = Utype(inst);
            common::ld(machine, i.rd(), SP, i.immediate_s())?;
            None
        }
        insts::OP_RVC_SWSP => {
            let i = Stype(inst);
            common::sw(machine, SP, i.rs2(), i.immediate_s())?;
            None
        }
        insts::OP_RVC_SDSP => {
            let i = Stype(inst);
            common::sd(machine, SP, i.rs2(), i.immediate_s())?;
            None
        }
        insts::OP_RVC_BEQZ => {
            let i = Stype(inst);
            let condition = machine.registers()[i.rs1()].eq(&Mac::REG::zero());
            let next_pc_offset = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()),
                &Mac::REG::from_usize(2),
            );
            Some(machine.pc().overflowing_add(&next_pc_offset))
        }
        insts::OP_RVC_BNEZ => {
            let i = Stype(inst);
            let condition = machine.registers()[i.rs1()]
                .eq(&Mac::REG::zero())
                .logical_not();
            let next_pc_offset = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()),
                &Mac::REG::from_usize(2),
            );
            Some(machine.pc().overflowing_add(&next_pc_offset))
        }
        insts::OP_RVC_MV => {
            let i = Rtype(inst);
            let value = &machine.registers()[i.rs2()];
            update_register(machine, i.rd(), value.clone());
            None
        }
        insts::OP_RVC_JAL => {
            let i = Utype(inst);
            common::jal(machine, 1, i.immediate_s(), 2)
        }
        insts::OP_RVC_J => {
            let i = Utype(inst);
            Some(
                machine
                    .pc()
                    .overflowing_add(&Mac::REG::from_i32(i.immediate_s())),
            )
        }
        insts::OP_RVC_JR => {
            let i = Stype(inst);
            let mut next_pc = machine.registers()[i.rs1()].clone();
            next_pc = next_pc & (!Mac::REG::one());
            Some(next_pc)
        }
        insts::OP_RVC_JALR => {
            let i = Stype(inst);
            let link = machine.pc().overflowing_add(&Mac::REG::from_usize(2));
            let mut next_pc = machine.registers()[i.rs1()].clone();
            next_pc = next_pc & (!Mac::REG::one());
            update_register(machine, 1, link);
            Some(next_pc)
        }
        insts::OP_RVC_ADDI16SP => {
            let i = Itype(inst);
            let value =
                machine.registers()[SP].overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
            update_register(machine, SP, value);
            None
        }
        insts::OP_RVC_SRLI64 => None,
        insts::OP_RVC_SRAI64 => None,
        insts::OP_RVC_SLLI64 => None,
        insts::OP_RVC_NOP => None,
        insts::OP_RVC_EBREAK => {
            machine.ebreak()?;
            None
        }
        insts::OP_CUSTOM_LOAD_IMM => {
            let i = Utype(inst);
            let value = Mac::REG::from_i32(i.immediate_s());
            update_register(machine, i.rd(), value);
            None
        }
        _ => return Err(Error::InvalidOp(op as u8)),
    };
    let default_instruction_size = instruction_length(inst);
    let default_next_pc = machine
        .pc()
        .overflowing_add(&Mac::REG::from_usize(default_instruction_size));
    machine.set_pc(next_pc.unwrap_or(default_next_pc));
    Ok(())
}
