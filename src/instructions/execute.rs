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

pub fn execute_instruction<Mac: Machine>(
    inst: Instruction,
    machine: &mut Mac,
) -> Result<(), Error> {
    let op = extract_opcode(inst);
    match op {
        insts::OP_SUB => {
            let i = Rtype(inst);
            common::sub(machine, i.rd(), i.rs1(), i.rs2());
        }
        insts::OP_SUBW => {
            let i = Rtype(inst);
            common::subw(machine, i.rd(), i.rs1(), i.rs2());
        }
        insts::OP_ADD => {
            let i = Rtype(inst);
            common::add(machine, i.rd(), i.rs1(), i.rs2());
        }
        insts::OP_ADDW => {
            let i = Rtype(inst);
            common::addw(machine, i.rd(), i.rs1(), i.rs2());
        }
        insts::OP_XOR => {
            let i = Rtype(inst);
            common::xor(machine, i.rd(), i.rs1(), i.rs2());
        }
        insts::OP_OR => {
            let i = Rtype(inst);
            common::or(machine, i.rd(), i.rs1(), i.rs2());
        }
        insts::OP_AND => {
            let i = Rtype(inst);
            common::and(machine, i.rd(), i.rs1(), i.rs2());
        }
        insts::OP_SLL => {
            let i = Rtype(inst);
            let shift_value =
                machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = machine.registers()[i.rs1()].clone() << shift_value;
            update_register(machine, i.rd(), value);
        }
        insts::OP_SLLW => {
            let i = Rtype(inst);
            let shift_value = machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(0x1F);
            let value = machine.registers()[i.rs1()].clone() << shift_value;
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_SRL => {
            let i = Rtype(inst);
            let shift_value =
                machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = machine.registers()[i.rs1()].clone() >> shift_value;
            update_register(machine, i.rd(), value);
        }
        insts::OP_SRLW => {
            let i = Rtype(inst);
            let shift_value = machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(0x1F);
            let value =
                machine.registers()[i.rs1()].zero_extend(&Mac::REG::from_u8(32)) >> shift_value;
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_SRA => {
            let i = Rtype(inst);
            let shift_value =
                machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = machine.registers()[i.rs1()].signed_shr(&shift_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SRAW => {
            let i = Rtype(inst);
            let shift_value = machine.registers()[i.rs2()].clone() & Mac::REG::from_u8(0x1F);
            let value = machine.registers()[i.rs1()]
                .sign_extend(&Mac::REG::from_u8(32))
                .signed_shr(&shift_value);
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_SLT => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.lt_s(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SLTU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.lt(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_LB => {
            let i = Itype(inst);
            common::lb(
                machine,
                i.rd(),
                i.rs1(),
                i.immediate_s(),
                machine.version() == 0,
            )?;
        }
        insts::OP_LH => {
            let i = Itype(inst);
            common::lh(
                machine,
                i.rd(),
                i.rs1(),
                i.immediate_s(),
                machine.version() == 0,
            )?;
        }
        insts::OP_LW => {
            let i = Itype(inst);
            common::lw(
                machine,
                i.rd(),
                i.rs1(),
                i.immediate_s(),
                machine.version() == 0,
            )?;
        }
        insts::OP_LD => {
            let i = Itype(inst);
            common::ld(
                machine,
                i.rd(),
                i.rs1(),
                i.immediate_s(),
                machine.version() == 0,
            )?;
        }
        insts::OP_LBU => {
            let i = Itype(inst);
            common::lbu(
                machine,
                i.rd(),
                i.rs1(),
                i.immediate_s(),
                machine.version() == 0,
            )?;
        }
        insts::OP_LHU => {
            let i = Itype(inst);
            common::lhu(
                machine,
                i.rd(),
                i.rs1(),
                i.immediate_s(),
                machine.version() == 0,
            )?;
        }
        insts::OP_LWU => {
            let i = Itype(inst);
            common::lwu(
                machine,
                i.rd(),
                i.rs1(),
                i.immediate_s(),
                machine.version() == 0,
            )?;
        }
        insts::OP_ADDI => {
            let i = Itype(inst);
            common::addi(machine, i.rd(), i.rs1(), i.immediate_s());
        }
        insts::OP_ADDIW => {
            let i = Itype(inst);
            common::addiw(machine, i.rd(), i.rs1(), i.immediate_s());
        }
        insts::OP_XORI => {
            let i = Itype(inst);
            common::xori(machine, i.rd(), i.rs1(), i.immediate_s());
        }
        insts::OP_ORI => {
            let i = Itype(inst);
            common::ori(machine, i.rd(), i.rs1(), i.immediate_s());
        }
        insts::OP_ANDI => {
            let i = Itype(inst);
            common::andi(machine, i.rd(), i.rs1(), i.immediate_s());
        }
        insts::OP_SLTI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let imm_value = Mac::REG::from_i32(i.immediate_s());
            let value = rs1_value.lt_s(&imm_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SLTIU => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let imm_value = Mac::REG::from_i32(i.immediate_s());
            let value = rs1_value.lt(&imm_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_JALR => {
            let i = Itype(inst);
            let size = instruction_length(inst);
            let link = machine.pc().overflowing_add(&Mac::REG::from_u8(size));
            if machine.version() >= 1 {
                let mut next_pc = machine.registers()[i.rs1()]
                    .overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
                next_pc = next_pc & (!Mac::REG::one());
                update_register(machine, i.rd(), link);
                machine.update_pc(next_pc);
            } else {
                update_register(machine, i.rd(), link);
                let mut next_pc = machine.registers()[i.rs1()]
                    .overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
                next_pc = next_pc & (!Mac::REG::one());
                machine.update_pc(next_pc);
            }
        }
        insts::OP_SLLI => {
            let i = Itype(inst);
            common::slli(machine, i.rd(), i.rs1(), i.immediate_u());
        }
        insts::OP_SRLI => {
            let i = Itype(inst);
            common::srli(machine, i.rd(), i.rs1(), i.immediate_u());
        }
        insts::OP_SRAI => {
            let i = Itype(inst);
            common::srai(machine, i.rd(), i.rs1(), i.immediate_u());
        }
        insts::OP_SLLIW => {
            let i = Itype(inst);
            common::slliw(machine, i.rd(), i.rs1(), i.immediate_u());
        }
        insts::OP_SRLIW => {
            let i = Itype(inst);
            common::srliw(machine, i.rd(), i.rs1(), i.immediate_u());
        }
        insts::OP_SRAIW => {
            let i = Itype(inst);
            common::sraiw(machine, i.rd(), i.rs1(), i.immediate_u());
        }
        insts::OP_SB => {
            let i = Stype(inst);
            common::sb(machine, i.rs1(), i.rs2(), i.immediate_s())?;
        }
        insts::OP_SH => {
            let i = Stype(inst);
            common::sh(machine, i.rs1(), i.rs2(), i.immediate_s())?;
        }
        insts::OP_SW => {
            let i = Stype(inst);
            common::sw(machine, i.rs1(), i.rs2(), i.immediate_s())?;
        }
        insts::OP_SD => {
            let i = Stype(inst);
            common::sd(machine, i.rs1(), i.rs2(), i.immediate_s())?;
        }
        insts::OP_BEQ => {
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
        }
        insts::OP_BNE => {
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
        }
        insts::OP_BLT => {
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
        }
        insts::OP_BGE => {
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
        }
        insts::OP_BLTU => {
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
        }
        insts::OP_BGEU => {
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
        }
        insts::OP_LUI => {
            let i = Utype(inst);
            update_register(machine, i.rd(), Mac::REG::from_i32(i.immediate_s()));
        }
        insts::OP_AUIPC => {
            let i = Utype(inst);
            let value = machine
                .pc()
                .overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
            update_register(machine, i.rd(), value);
        }
        insts::OP_ECALL => {
            // The semantic of ECALL is determined by the hardware, which
            // is not part of the spec, hence here the implementation is
            // deferred to the machine. This way custom ECALLs might be
            // provided for different environments.
            machine.ecall()?;
        }
        insts::OP_EBREAK => {
            machine.ebreak()?;
        }
        insts::OP_FENCEI => {}
        insts::OP_FENCE => {}
        insts::OP_JAL => {
            let i = Utype(inst);
            common::jal(machine, i.rd(), i.immediate_s(), instruction_length(inst));
        }
        insts::OP_MUL => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_mul(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MULW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value
                .zero_extend(&Mac::REG::from_u8(32))
                .overflowing_mul(&rs2_value.zero_extend(&Mac::REG::from_u8(32)));
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_MULH => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_mul_high_signed(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MULHSU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_mul_high_signed_unsigned(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MULHU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_mul_high_unsigned(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_DIV => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_div_signed(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_DIVW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_value = rs1_value.sign_extend(&Mac::REG::from_u8(32));
            let rs2_value = rs2_value.sign_extend(&Mac::REG::from_u8(32));
            let value = rs1_value.overflowing_div_signed(&rs2_value);
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_DIVU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_div(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_DIVUW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_value = rs1_value.zero_extend(&Mac::REG::from_u8(32));
            let rs2_value = rs2_value.zero_extend(&Mac::REG::from_u8(32));
            let value = rs1_value.overflowing_div(&rs2_value);
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_REM => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_rem_signed(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_REMW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_value = rs1_value.sign_extend(&Mac::REG::from_u8(32));
            let rs2_value = rs2_value.sign_extend(&Mac::REG::from_u8(32));
            let value = rs1_value.overflowing_rem_signed(&rs2_value);
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_REMU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_rem(&rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_REMUW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_value = rs1_value.zero_extend(&Mac::REG::from_u8(32));
            let rs2_value = rs2_value.zero_extend(&Mac::REG::from_u8(32));
            let value = rs1_value.overflowing_rem(&rs2_value);
            update_register(machine, i.rd(), value.sign_extend(&Mac::REG::from_u8(32)));
        }
        insts::OP_ADDUW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_u = rs1_value.zero_extend(&Mac::REG::from_u8(32));
            let value = rs2_value.overflowing_add(&rs1_u);
            update_register(machine, i.rd(), value);
        }
        insts::OP_ANDN => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.clone() & !rs2_value.clone();
            update_register(machine, i.rd(), value);
        }
        insts::OP_BCLR => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.clone() & !(Mac::REG::one() << shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_BCLRI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate_u());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.clone() & !(Mac::REG::one() << shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_BEXT => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = Mac::REG::one() & (rs1_value.clone() >> shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_BEXTI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate_u());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = Mac::REG::one() & (rs1_value.clone() >> shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_BINV => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.clone() ^ (Mac::REG::one() << shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_BINVI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate_u());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.clone() ^ (Mac::REG::one() << shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_BSET => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.clone() | (Mac::REG::one() << shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_BSETI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate_u());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.clone() | (Mac::REG::one() << shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_CLMUL => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.clmul(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_CLMULH => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.clmulh(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_CLMULR => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.clmulr(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_CLZ => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = rs1_value.clz();
            update_register(machine, i.rd(), value);
        }
        insts::OP_CLZW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = rs1_value
                .zero_extend(&Mac::REG::from_u8(32))
                .clz()
                .overflowing_sub(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), value);
        }
        insts::OP_CPOP => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = rs1_value.cpop();
            update_register(machine, i.rd(), value);
        }
        insts::OP_CPOPW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = rs1_value.zero_extend(&Mac::REG::from_u8(32)).cpop();
            update_register(machine, i.rd(), value);
        }
        insts::OP_CTZ => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = rs1_value.ctz();
            update_register(machine, i.rd(), value);
        }
        insts::OP_CTZW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = (rs1_value.clone() | Mac::REG::from_u64(0xffff_ffff_0000_0000)).ctz();
            update_register(machine, i.rd(), value);
        }
        insts::OP_MAX => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.ge_s(&rs2_value).cond(&rs1_value, &rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MAXU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.ge(&rs2_value).cond(&rs1_value, &rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MIN => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.lt_s(&rs2_value).cond(&rs1_value, &rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MINU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.lt(&rs2_value).cond(&rs1_value, &rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_ORCB => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = rs1_value.orcb();
            update_register(machine, i.rd(), value);
        }
        insts::OP_ORN => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.clone() | !rs2_value.clone();
            update_register(machine, i.rd(), value);
        }
        insts::OP_REV8 => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = rs1_value.rev8();
            update_register(machine, i.rd(), value);
        }
        insts::OP_ROL => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.rol(&shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_ROLW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(31);
            let twins = rs1_value
                .zero_extend(&Mac::REG::from_u8(32))
                .overflowing_mul(&Mac::REG::from_u64(0x_0000_0001_0000_0001));
            let value = twins.rol(&shamt).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), value);
        }
        insts::OP_ROR => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.ror(&shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_RORI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate_u());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.ror(&shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_RORIW => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate_u());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(31);
            let twins = rs1_value
                .zero_extend(&Mac::REG::from_u8(32))
                .overflowing_mul(&Mac::REG::from_u64(0x_0000_0001_0000_0001));
            let value = twins.ror(&shamt).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), value);
        }
        insts::OP_RORW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let shamt = rs2_value.clone() & Mac::REG::from_u8(31);
            let twins = rs1_value
                .zero_extend(&Mac::REG::from_u8(32))
                .overflowing_mul(&Mac::REG::from_u64(0x_0000_0001_0000_0001));
            let value = twins.ror(&shamt).sign_extend(&Mac::REG::from_u8(32));
            update_register(machine, i.rd(), value);
        }
        insts::OP_SEXTB => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let shift = &Mac::REG::from_u8(Mac::REG::BITS - 8);
            let value = rs1_value.signed_shl(shift).signed_shr(shift);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SEXTH => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let shift = &Mac::REG::from_u8(Mac::REG::BITS - 16);
            let value = rs1_value.signed_shl(shift).signed_shr(shift);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SH1ADD => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = (rs1_value.clone() << Mac::REG::from_u32(1)).overflowing_add(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SH1ADDUW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_z = rs1_value.clone().zero_extend(&Mac::REG::from_u8(32));
            let value = (rs1_z << Mac::REG::from_u32(1)).overflowing_add(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SH2ADD => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = (rs1_value.clone() << Mac::REG::from_u32(2)).overflowing_add(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SH2ADDUW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_z = rs1_value.clone().zero_extend(&Mac::REG::from_u8(32));
            let value = (rs1_z << Mac::REG::from_u32(2)).overflowing_add(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SH3ADD => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = (rs1_value.clone() << Mac::REG::from_u32(3)).overflowing_add(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SH3ADDUW => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let rs1_z = rs1_value.clone().zero_extend(&Mac::REG::from_u8(32));
            let value = (rs1_z << Mac::REG::from_u32(3)).overflowing_add(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SLLIUW => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = Mac::REG::from_u32(i.immediate_u());
            let rs1_u = rs1_value.clone().zero_extend(&Mac::REG::from_u8(32));
            let shamt = rs2_value & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_u << shamt;
            update_register(machine, i.rd(), value);
        }
        insts::OP_XNOR => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.clone() ^ !rs2_value.clone();
            update_register(machine, i.rd(), value);
        }
        insts::OP_ZEXTH => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let value = rs1_value.zero_extend(&Mac::REG::from_u8(16));
            update_register(machine, i.rd(), value);
        }
        insts::OP_WIDE_MUL => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value_h = rs1_value.overflowing_mul_high_signed(&rs2_value);
            let value_l = rs1_value.overflowing_mul(&rs2_value);
            update_register(machine, i.rd(), value_h);
            update_register(machine, i.rs3(), value_l);
        }
        insts::OP_WIDE_MULU => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value_h = rs1_value.overflowing_mul_high_unsigned(&rs2_value);
            let value_l = rs1_value.overflowing_mul(&rs2_value);
            update_register(machine, i.rd(), value_h);
            update_register(machine, i.rs3(), value_l);
        }
        insts::OP_WIDE_MULSU => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value_h = rs1_value.overflowing_mul_high_signed_unsigned(&rs2_value);
            let value_l = rs1_value.overflowing_mul(&rs2_value);
            update_register(machine, i.rd(), value_h);
            update_register(machine, i.rs3(), value_l);
        }
        insts::OP_WIDE_DIV => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value_h = rs1_value.overflowing_div_signed(&rs2_value);
            let value_l = rs1_value.overflowing_rem_signed(&rs2_value);
            update_register(machine, i.rd(), value_h);
            update_register(machine, i.rs3(), value_l);
        }
        insts::OP_WIDE_DIVU => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value_h = rs1_value.overflowing_div(&rs2_value);
            let value_l = rs1_value.overflowing_rem(&rs2_value);
            update_register(machine, i.rd(), value_h);
            update_register(machine, i.rs3(), value_l);
        }
        insts::OP_FAR_JUMP_REL => {
            let i = Utype(inst);
            let size = instruction_length(inst);
            let link = machine.pc().overflowing_add(&Mac::REG::from_u8(size));
            let next_pc = machine
                .pc()
                .overflowing_add(&Mac::REG::from_i32(i.immediate_s()))
                & (!Mac::REG::one());
            update_register(machine, RA, link);
            machine.update_pc(next_pc);
        }
        insts::OP_FAR_JUMP_ABS => {
            let i = Utype(inst);
            let size = instruction_length(inst);
            let link = machine.pc().overflowing_add(&Mac::REG::from_u8(size));
            let next_pc = Mac::REG::from_i32(i.immediate_s()) & (!Mac::REG::one());
            update_register(machine, RA, link);
            machine.update_pc(next_pc);
        }
        insts::OP_ADC => {
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
        }
        insts::OP_SBB => {
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
        }
        insts::OP_LD_SIGN_EXTENDED_32_CONSTANT => {
            let i = Utype(inst);
            update_register(machine, i.rd(), Mac::REG::from_i32(i.immediate_s()));
        }
        insts::OP_CUSTOM_LOAD_IMM => {
            let i = Utype(inst);
            let value = Mac::REG::from_i32(i.immediate_s());
            update_register(machine, i.rd(), value);
        }
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
            m_vv_loop_u!(inst, machine, alu::sltu);
        }
        insts::OP_VMSLTU_VX => {
            m_vx_loop_u!(inst, machine, alu::sltu);
        }
        insts::OP_VMSLT_VV => {
            m_vv_loop_s!(inst, machine, alu::slt);
        }
        insts::OP_VMSLT_VX => {
            m_vx_loop_s!(inst, machine, alu::slt);
        }
        insts::OP_VMSLEU_VV => {
            m_vv_loop_u!(inst, machine, alu::sleu);
        }
        insts::OP_VMSLEU_VX => {
            m_vx_loop_u!(inst, machine, alu::sleu);
        }
        insts::OP_VMSLEU_VI => {
            m_vi_loop_u!(inst, machine, alu::sleu);
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
            m_vx_loop_u!(inst, machine, alu::sgtu);
        }
        insts::OP_VMSGTU_VI => {
            m_vi_loop_u!(inst, machine, alu::sgtu);
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
        insts::OP_VAND_VV => {
            v_vv_loop_s!(inst, machine, alu::and);
        }
        insts::OP_VOR_VV => {
            v_vv_loop_s!(inst, machine, alu::or);
        }
        insts::OP_VXOR_VV => {
            v_vv_loop_s!(inst, machine, alu::xor);
        }
        insts::OP_VAND_VX => {
            v_vx_loop_s!(inst, machine, alu::and);
        }
        insts::OP_VOR_VX => {
            v_vx_loop_s!(inst, machine, alu::or);
        }
        insts::OP_VXOR_VX => {
            v_vx_loop_s!(inst, machine, alu::xor);
        }
        insts::OP_VAND_VI => {
            v_vi_loop_s!(inst, machine, alu::and);
        }
        insts::OP_VOR_VI => {
            v_vi_loop_s!(inst, machine, alu::or);
        }
        insts::OP_VXOR_VI => {
            v_vi_loop_s!(inst, machine, alu::xor);
        }
        insts::OP_VMV1R_V => {
            let i = VItype(inst);
            let data = machine.element_ref(i.vs2(), (VLEN as u64) * 1, 0).to_vec();
            machine
                .element_mut(i.vd(), (VLEN as u64) * 1, 0)
                .copy_from_slice(&data);
        }
        insts::OP_VMV2R_V => {
            let i = VItype(inst);
            let data = machine.element_ref(i.vs2(), (VLEN as u64) * 2, 0).to_vec();
            machine
                .element_mut(i.vd(), (VLEN as u64) * 2, 0)
                .copy_from_slice(&data);
        }
        insts::OP_VMV4R_V => {
            let i = VItype(inst);
            let data = machine.element_ref(i.vs2(), (VLEN as u64) * 4, 0).to_vec();
            machine
                .element_mut(i.vd(), (VLEN as u64) * 4, 0)
                .copy_from_slice(&data);
        }
        insts::OP_VMV8R_V => {
            let i = VItype(inst);
            let data = machine.element_ref(i.vs2(), (VLEN as u64) * 8, 0).to_vec();
            machine
                .element_mut(i.vd(), (VLEN as u64) * 8, 0)
                .copy_from_slice(&data);
        }
        insts::OP_VSADDU_VV => {
            v_vv_loop_u!(inst, machine, alu::saddu);
        }
        insts::OP_VSADDU_VX => {
            v_vx_loop_u!(inst, machine, alu::saddu);
        }
        insts::OP_VSADDU_VI => {
            v_vi_loop_u!(inst, machine, alu::saddu);
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
        insts::OP_VWADDU_VV => {
            w_vv_loop_u!(inst, machine, Eint::widening_add_u);
        }
        insts::OP_VWADDU_VX => {
            w_vx_loop_u!(inst, machine, Eint::widening_add_u);
        }
        insts::OP_VWADDU_WV => {
            w_wv_loop_u!(inst, machine, Eint::wrapping_add);
        }
        insts::OP_VWADDU_WX => {
            w_wx_loop_u!(inst, machine, Eint::wrapping_add);
        }
        insts::OP_VWADD_WX => {
            w_wx_loop_s!(inst, machine, Eint::wrapping_add);
        }
        insts::OP_VWADD_VV => {
            w_vv_loop_s!(inst, machine, Eint::widening_add_s);
        }
        insts::OP_VWADD_VX => {
            w_vx_loop_s!(inst, machine, Eint::widening_add_s);
        }
        insts::OP_VWADD_WV => {
            w_wv_loop_s!(inst, machine, Eint::wrapping_add);
        }
        insts::OP_VWSUBU_VV => {
            w_vv_loop_u!(inst, machine, Eint::widening_sub_u);
        }
        insts::OP_VWSUBU_WV => {
            w_wv_loop_u!(inst, machine, Eint::wrapping_sub);
        }
        insts::OP_VWSUBU_VX => {
            w_vx_loop_u!(inst, machine, Eint::widening_sub_u);
        }
        insts::OP_VWSUB_VV => {
            w_vv_loop_s!(inst, machine, Eint::widening_sub_s);
        }
        insts::OP_VWSUB_VX => {
            w_vx_loop_s!(inst, machine, Eint::widening_sub_s);
        }
        insts::OP_VWSUB_WV => {
            w_wv_loop_s!(inst, machine, Eint::wrapping_sub);
        }
        insts::OP_VWSUBU_WX => {
            w_wx_loop_u!(inst, machine, Eint::wrapping_sub);
        }
        insts::OP_VWSUB_WX => {
            w_wx_loop_s!(inst, machine, Eint::wrapping_sub);
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
        insts::OP_VMV_V_V => {
            v_vv_loop_s!(inst, machine, alu::mv);
        }
        insts::OP_VMV_V_X => {
            v_vx_loop_s!(inst, machine, alu::mv);
        }
        insts::OP_VMV_V_I => {
            v_vi_loop_s!(inst, machine, alu::mv);
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
        insts::OP_VMADC_VV => {
            m_vv_loop_s!(inst, machine, alu::madc);
        }
        insts::OP_VMADC_VX => {
            m_vx_loop_s!(inst, machine, alu::madc);
        }
        insts::OP_VMADC_VI => {
            m_vi_loop_s!(inst, machine, alu::madc);
        }
        insts::OP_VMSBC_VV => {
            m_vv_loop_s!(inst, machine, alu::msbc);
        }
        insts::OP_VMSBC_VX => {
            m_vx_loop_s!(inst, machine, alu::msbc);
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
        insts::OP_VMAND_MM => {
            m_mm_loop!(inst, machine, |b, a| b & a);
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
        insts::OP_VL1RE8_V => {
            ld_whole!(inst, machine, VLEN as u64 / 8);
        }
        insts::OP_VL1RE16_V => {
            ld_whole!(inst, machine, VLEN as u64 / 16);
        }
        insts::OP_VL1RE32_V => {
            ld_whole!(inst, machine, VLEN as u64 / 32);
        }
        insts::OP_VL1RE64_V => {
            ld_whole!(inst, machine, VLEN as u64 / 64);
        }
        insts::OP_VL2RE8_V => {
            ld_whole!(inst, machine, VLEN as u64 / 4);
        }
        insts::OP_VL2RE16_V => {
            ld_whole!(inst, machine, VLEN as u64 / 8);
        }
        insts::OP_VL2RE32_V => {
            ld_whole!(inst, machine, VLEN as u64 / 16);
        }
        insts::OP_VL2RE64_V => {
            ld_whole!(inst, machine, VLEN as u64 / 32);
        }
        insts::OP_VL4RE8_V => {
            ld_whole!(inst, machine, VLEN as u64 / 2);
        }
        insts::OP_VL4RE16_V => {
            ld_whole!(inst, machine, VLEN as u64 / 4);
        }
        insts::OP_VL4RE32_V => {
            ld_whole!(inst, machine, VLEN as u64 / 8);
        }
        insts::OP_VL4RE64_V => {
            ld_whole!(inst, machine, VLEN as u64 / 16);
        }
        insts::OP_VL8RE8_V => {
            ld_whole!(inst, machine, VLEN as u64 / 1);
        }
        insts::OP_VL8RE16_V => {
            ld_whole!(inst, machine, VLEN as u64 / 2);
        }
        insts::OP_VL8RE32_V => {
            ld_whole!(inst, machine, VLEN as u64 / 4);
        }
        insts::OP_VL8RE64_V => {
            ld_whole!(inst, machine, VLEN as u64 / 8);
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
        insts::OP_VMADD_VX => {
            v_vx_loop_destructive_s!(inst, machine, alu::madd);
        }
        insts::OP_VNMSUB_VV => {
            v_vv_loop_destructive_s!(inst, machine, alu::nmsub);
        }
        insts::OP_VNMSUB_VX => {
            v_vx_loop_destructive_s!(inst, machine, alu::nmsub);
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
        insts::OP_VSMUL_VV => {
            v_vv_loop_s!(inst, machine, alu::smul);
        }
        insts::OP_VSMUL_VX => {
            v_vx_loop_s!(inst, machine, alu::smul);
        }
        insts::OP_VWMACCU_VV => {
            w_vv_loop_destructive_s!(inst, machine, alu::wmaccu);
        }
        insts::OP_VWMACCU_VX => {
            w_vx_loop_destructive_s!(inst, machine, alu::wmaccu);
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
            w_vx_loop_destructive_s!(inst, machine, alu::wmaccus);
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
        insts::OP_VNCLIPU_WV => {
            v_wv_loop_u!(inst, machine, alu::srl);
        }
        insts::OP_VNCLIPU_WX => {
            v_wx_loop_u!(inst, machine, alu::srl);
        }
        insts::OP_VNCLIPU_WI => {
            v_wi_loop_u!(inst, machine, alu::srl);
        }
        insts::OP_VNCLIP_WV => {
            v_wv_loop_u!(inst, machine, alu::sra);
        }
        insts::OP_VNCLIP_WX => {
            v_wx_loop_u!(inst, machine, alu::sra);
        }
        insts::OP_VNCLIP_WI => {
            v_wi_loop_u!(inst, machine, alu::sra);
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
            w_vs_loop_s!(inst, machine, Eint::wrapping_add);
        }
        insts::OP_VWREDSUM_VS => {
            w_vs_loop_u!(inst, machine, Eint::wrapping_add);
        }
        insts::OP_VMSBF_M => {
            if machine.vill() {
                return Err(Error::Vill);
            }
            let i = VVtype(inst);
            let mut found_first_mask = false;
            for j in 0..VLEN {
                if i.vm() == 0 && !machine.get_bit(0, j) {
                    continue;
                }
                let m = machine.get_bit(i.vs2(), j);
                if !found_first_mask && m {
                    found_first_mask = true;
                }
                if found_first_mask {
                    machine.clr_bit(i.vd(), j);
                } else {
                    machine.set_bit(i.vd(), j);
                }
            }
        }
        insts::OP_VMSOF_M => {
            if machine.vill() {
                return Err(Error::Vill);
            }
            let i = VVtype(inst);
            let mut found_first_mask = false;
            for j in 0..VLEN {
                if i.vm() == 0 && !machine.get_bit(0, j) {
                    continue;
                }
                let m = machine.get_bit(i.vs2(), j);
                if !found_first_mask && m {
                    found_first_mask = true;
                    machine.set_bit(i.vd(), j);
                    continue;
                } else {
                    machine.clr_bit(i.vd(), j);
                }
            }
        }
        insts::OP_VMSIF_M => {
            if machine.vill() {
                return Err(Error::Vill);
            }
            let i = VVtype(inst);
            let mut found_first_mask = false;
            for j in 0..VLEN {
                if i.vm() == 0 && !machine.get_bit(0, j) {
                    continue;
                }
                let m = machine.get_bit(i.vs2(), j);
                if !found_first_mask && m {
                    found_first_mask = true;
                    machine.set_bit(i.vd(), j);
                    continue;
                }
                if found_first_mask {
                    machine.clr_bit(i.vd(), j);
                } else {
                    machine.set_bit(i.vd(), j);
                }
            }
        }
        insts::OP_VIOTA_M => {
            if machine.vill() {
                return Err(Error::Vill);
            }
            let i = VVtype(inst);
            let sew = machine.vsew();
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
                    _ => return Err(Error::Unexpected("".into())),
                }
                if machine.get_bit(i.vs2(), j) {
                    iota += 1;
                }
            }
        }
        insts::OP_VID_V => {
            if machine.vill() {
                return Err(Error::Vill);
            }
            let i = VVtype(inst);
            let sew = machine.vsew();
            let mut index: u64 = 0;
            for j in 0..machine.vl() as usize {
                if i.vm() == 0 && !machine.get_bit(0, j) {
                    continue;
                }
                match sew {
                    8 => E8::from(index as u8).put(machine.element_mut(i.vd(), sew, j)),
                    16 => E16::from(index as u16).put(machine.element_mut(i.vd(), sew, j)),
                    32 => E32::from(index as u16).put(machine.element_mut(i.vd(), sew, j)),
                    64 => E64::from(index as u16).put(machine.element_mut(i.vd(), sew, j)),
                    128 => E128::from(index as u16).put(machine.element_mut(i.vd(), sew, j)),
                    256 => E256::from(index as u16).put(machine.element_mut(i.vd(), sew, j)),
                    512 => E512::from(index as u16).put(machine.element_mut(i.vd(), sew, j)),
                    1024 => E1024::from(index as u16).put(machine.element_mut(i.vd(), sew, j)),
                    _ => return Err(Error::Unexpected("".into())),
                }
                index += 1;
            }
        }
        insts::OP_VMV_X_S => {
            if machine.vill() {
                return Err(Error::Vill);
            }
            let i = VVtype(inst);
            let sew = machine.vsew();
            let r = match sew {
                8 => E8::get(machine.element_ref(i.vs2(), sew, 0)).0 as i8 as i64 as u64,
                16 => E16::get(machine.element_ref(i.vs2(), sew, 0)).0 as i16 as i64 as u64,
                32 => E32::get(machine.element_ref(i.vs2(), sew, 0)).0 as i32 as i64 as u64,
                64 => E64::get(machine.element_ref(i.vs2(), sew, 0)).u64(),
                128 => E128::get(machine.element_ref(i.vs2(), sew, 0)).u64(),
                256 => E256::get(machine.element_ref(i.vs2(), sew, 0)).u64(),
                512 => E512::get(machine.element_ref(i.vs2(), sew, 0)).u64(),
                1024 => E1024::get(machine.element_ref(i.vs2(), sew, 0)).u64(),
                _ => return Err(Error::Unexpected("".into())),
            };
            update_register(machine, i.vd(), Mac::REG::from_u64(r));
        }
        insts::OP_VMV_S_X => {
            if machine.vill() {
                return Err(Error::Vill);
            }
            let i = VVtype(inst);
            let sew = machine.vsew();
            match sew {
                8 => E8::from(machine.registers()[i.vs1()].to_u64()).put(machine.element_mut(
                    i.vd(),
                    sew,
                    0,
                )),
                16 => E16::from(machine.registers()[i.vs1()].to_u64()).put(machine.element_mut(
                    i.vd(),
                    sew,
                    0,
                )),
                32 => E32::from(machine.registers()[i.vs1()].to_u64()).put(machine.element_mut(
                    i.vd(),
                    sew,
                    0,
                )),
                64 => E64::from(machine.registers()[i.vs1()].to_u64()).put(machine.element_mut(
                    i.vd(),
                    sew,
                    0,
                )),
                128 => E128::from(machine.registers()[i.vs1()].to_u64()).put(machine.element_mut(
                    i.vd(),
                    sew,
                    0,
                )),
                256 => E256::from(machine.registers()[i.vs1()].to_u64()).put(machine.element_mut(
                    i.vd(),
                    sew,
                    0,
                )),
                512 => E512::from(machine.registers()[i.vs1()].to_u64()).put(machine.element_mut(
                    i.vd(),
                    sew,
                    0,
                )),
                1024 => E1024::from(machine.registers()[i.vs2()].to_u64())
                    .put(machine.element_mut(i.vd(), sew, 0)),
                _ => return Err(Error::Unexpected("".into())),
            };
        }
        insts::OP_VCOMPRESS_VM => {
            if machine.vill() {
                return Err(Error::Vill);
            }
            let i = VVtype(inst);
            let sew = machine.vsew();
            let mut k = 0;
            for j in 0..machine.vl() as usize {
                if machine.get_bit(i.vs1(), j) {
                    let data = machine.element_ref(i.vs2(), sew, j).to_vec();
                    machine.element_mut(i.vd(), sew, k).copy_from_slice(&data);
                    k += 1;
                }
            }
        }
        insts::OP_VSLIDE1UP_VX => {
            if machine.vill() {
                return Err(Error::Vill);
            }
            let i = VXtype(inst);
            let sew = machine.vsew();
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
                    let vd0 = E128::from(machine.registers()[i.rs1()].to_u64());
                    vd0.put(machine.element_mut(i.vd(), sew, 0));
                }
                256 => {
                    let vd0 = E256::from(machine.registers()[i.rs1()].to_u64());
                    vd0.put(machine.element_mut(i.vd(), sew, 0));
                }
                512 => {
                    let vd0 = E512::from(machine.registers()[i.rs1()].to_u64());
                    vd0.put(machine.element_mut(i.vd(), sew, 0));
                }
                1024 => {
                    let vd0 = E1024::from(machine.registers()[i.rs1()].to_u64());
                    vd0.put(machine.element_mut(i.vd(), sew, 0));
                }
                _ => return Err(Error::Unexpected("".into())),
            }
            for j in 1..machine.vl() {
                if i.vm() == 0 && machine.get_bit(0, j as usize) {
                    continue;
                }
                let data = machine.element_ref(i.vs2(), sew, (j - 1) as usize).to_vec();
                machine
                    .element_mut(i.vd(), sew, j as usize)
                    .copy_from_slice(&data);
            }
        }
        insts::OP_VSLIDEUP_VX => {
            if machine.vill() {
                return Err(Error::Vill);
            }
            let i = VXtype(inst);
            let sew = machine.vsew();
            let offset = machine.registers()[i.rs1()].to_u64();
            if offset < machine.vl() {
                for j in offset..machine.vl() {
                    if i.vm() == 0 && machine.get_bit(0, j as usize) {
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
        }
        insts::OP_VSLIDEUP_VI => {
            if machine.vill() {
                return Err(Error::Vill);
            }
            let i = VItype(inst);
            let sew = machine.vsew();
            let offset = i.immediate_u() as u64;
            if offset < machine.vl() {
                for j in offset..machine.vl() {
                    if i.vm() == 0 && machine.get_bit(0, j as usize) {
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
        }
        insts::OP_VSLIDE1DOWN_VX => {
            if machine.vill() {
                return Err(Error::Vill);
            }
            let i = VXtype(inst);
            let sew = machine.vsew();
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
                    let vd0 = E128::from(machine.registers()[i.rs1()].to_u64());
                    vd0.put(machine.element_mut(i.vd(), sew, (machine.vl() - 1) as usize));
                }
                256 => {
                    let vd0 = E256::from(machine.registers()[i.rs1()].to_u64());
                    vd0.put(machine.element_mut(i.vd(), sew, (machine.vl() - 1) as usize));
                }
                512 => {
                    let vd0 = E512::from(machine.registers()[i.rs1()].to_u64());
                    vd0.put(machine.element_mut(i.vd(), sew, (machine.vl() - 1) as usize));
                }
                1024 => {
                    let vd0 = E1024::from(machine.registers()[i.rs1()].to_u64());
                    vd0.put(machine.element_mut(i.vd(), sew, (machine.vl() - 1) as usize));
                }
                _ => return Err(Error::Unexpected("".into())),
            }
            for j in 0..machine.vl() {
                if i.vm() == 0 && machine.get_bit(0, j as usize) {
                    continue;
                }
                if (j + 1) < machine.vlmax() {
                    let data = machine.element_ref(i.vs2(), sew, j as usize + 1).to_vec();
                    machine
                        .element_mut(i.vd(), sew, j as usize)
                        .copy_from_slice(&data);
                }
            }
        }
        insts::OP_VSLIDEDOWN_VX => {
            if machine.vill() {
                return Err(Error::Vill);
            }
            let i = VXtype(inst);
            let sew = machine.vsew();
            let offset = machine.registers()[i.rs1()].to_u64();
            for j in 0..machine.vl() {
                if i.vm() == 0 && machine.get_bit(0, j as usize) {
                    continue;
                }
                if (j + offset) < machine.vlmax() {
                    let data = machine
                        .element_ref(i.vs2(), sew, (j + offset) as usize)
                        .to_vec();
                    machine
                        .element_mut(i.vd(), sew, j as usize)
                        .copy_from_slice(&data);
                } else {
                    machine
                        .element_mut(i.vd(), sew, j as usize)
                        .copy_from_slice(&vec![0; sew as usize >> 3]);
                }
            }
        }
        insts::OP_VSLIDEDOWN_VI => {
            if machine.vill() {
                return Err(Error::Vill);
            }
            let i = VItype(inst);
            let sew = machine.vsew();
            let offset = i.immediate_u() as u64;
            for j in 0..machine.vl() {
                if i.vm() == 0 && machine.get_bit(0, j as usize) {
                    continue;
                }
                if (j + offset) < machine.vlmax() {
                    let data = machine
                        .element_ref(i.vs2(), sew, (j + offset) as usize)
                        .to_vec();
                    machine
                        .element_mut(i.vd(), sew, j as usize)
                        .copy_from_slice(&data);
                } else {
                    machine
                        .element_mut(i.vd(), sew, j as usize)
                        .copy_from_slice(&vec![0; sew as usize >> 3]);
                }
            }
        }
        insts::OP_VRGATHER_VV => {
            if machine.vill() {
                return Err(Error::Vill);
            }
            let i = VVtype(inst);
            let sew = machine.vsew();
            for j in 0..machine.vl() as usize {
                if i.vm() == 0 && machine.get_bit(0, j) {
                    continue;
                }
                let mut data = machine.element_ref(i.vs1(), sew, j).to_vec();
                data.resize(8, 0);
                let index = E64::get(&data).u64();
                if index < machine.vlmax() {
                    let data = machine.element_ref(i.vs2(), sew, index as usize).to_vec();
                    machine.element_mut(i.vd(), sew, j).copy_from_slice(&data);
                } else {
                    let data = vec![0; sew as usize >> 3];
                    machine.element_mut(i.vd(), sew, j).copy_from_slice(&data);
                }
            }
        }
        insts::OP_VRGATHER_VX => {
            if machine.vill() {
                return Err(Error::Vill);
            }
            let i = VXtype(inst);
            let sew = machine.vsew();
            for j in 0..machine.vl() as usize {
                if i.vm() == 0 && machine.get_bit(0, j) {
                    continue;
                }
                let index = machine.registers()[i.rs1()].to_u64();
                if index < machine.vlmax() {
                    let data = machine.element_ref(i.vs2(), sew, index as usize).to_vec();
                    machine.element_mut(i.vd(), sew, j).copy_from_slice(&data);
                } else {
                    let data = vec![0; sew as usize >> 3];
                    machine.element_mut(i.vd(), sew, j).copy_from_slice(&data);
                }
            }
        }
        insts::OP_VRGATHER_VI => {
            if machine.vill() {
                return Err(Error::Vill);
            }
            let i = VItype(inst);
            let sew = machine.vsew();
            for j in 0..machine.vl() as usize {
                if i.vm() == 0 && machine.get_bit(0, j) {
                    continue;
                }
                let index = i.immediate_u() as u64;
                if index < machine.vlmax() {
                    let data = machine.element_ref(i.vs2(), sew, index as usize).to_vec();
                    machine.element_mut(i.vd(), sew, j).copy_from_slice(&data);
                } else {
                    let data = vec![0; sew as usize >> 3];
                    machine.element_mut(i.vd(), sew, j).copy_from_slice(&data);
                }
            }
        }
        insts::OP_VRGATHEREI16_VV => {
            if machine.vill() {
                return Err(Error::Vill);
            }
            let i = VVtype(inst);
            let sew = machine.vsew();
            for j in 0..machine.vl() as usize {
                if i.vm() == 0 && machine.get_bit(0, j) {
                    continue;
                }
                let index = E16::get(&machine.element_ref(i.vs1(), 16, j).to_vec()).u64();
                if index < machine.vlmax() {
                    let data = machine.element_ref(i.vs2(), sew, index as usize).to_vec();
                    machine.element_mut(i.vd(), sew, j).copy_from_slice(&data);
                } else {
                    let data = vec![0; sew as usize >> 3];
                    machine.element_mut(i.vd(), sew, j).copy_from_slice(&data);
                }
            }
        }
        insts::OP_VFIRST_M => {
            if machine.vill() {
                return Err(Error::Vill);
            }
            let i = VVtype(inst);
            let m = if i.vm() == 0 {
                E2048::get(machine.element_ref(i.vs2(), VLEN as u64, 0))
                    & E2048::get(machine.element_ref(0, VLEN as u64, 0))
            } else {
                E2048::get(machine.element_ref(i.vs2(), VLEN as u64, 0))
            } & (E2048::MAX_U >> (VLEN as u32 - machine.vl() as u32));
            let r = m.ctz();
            if r == 2048 {
                update_register(machine, i.vd(), Mac::REG::from_u64(0xffff_ffff_ffff_ffff));
            } else {
                update_register(machine, i.vd(), Mac::REG::from_u32(r));
            }
        }
        insts::OP_VCPOP_M => {
            if machine.vill() {
                return Err(Error::Vill);
            }
            let i = VVtype(inst);
            let m = if i.vm() == 0 {
                E2048::get(machine.element_ref(i.vs2(), VLEN as u64, 0))
                    & E2048::get(machine.element_ref(0, VLEN as u64, 0))
            } else {
                E2048::get(machine.element_ref(i.vs2(), VLEN as u64, 0))
            } & (E2048::MAX_U >> (VLEN as u32 - machine.vl() as u32));
            let r = m.cpop();
            update_register(machine, i.vd(), Mac::REG::from_u32(r));
        }
        _ => return Err(Error::InvalidOp(op)),
    };
    Ok(())
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
