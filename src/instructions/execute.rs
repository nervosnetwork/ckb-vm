use super::{
    super::{machine::Machine, Error},
    common, extract_opcode, instruction_length,
    utils::update_register,
    Instruction, Itype, R4type, Register, Rtype, Stype, Utype, VItype, VRegister, VVtype, VXtype,
    U1024, U256, U512,
};
use crate::instructions::v_register::{
    vfunc_add_vi, vfunc_add_vv, vfunc_add_vx, vfunc_div_vv, vfunc_div_vx, vfunc_divu_vv,
    vfunc_divu_vx, vfunc_mseq_vi, vfunc_mseq_vv, vfunc_mseq_vx, vfunc_msgt_vi, vfunc_msgt_vx,
    vfunc_msgtu_vi, vfunc_msgtu_vx, vfunc_msle_vi, vfunc_msle_vv, vfunc_msle_vx, vfunc_msleu_vi,
    vfunc_msleu_vv, vfunc_msleu_vx, vfunc_mslt_vv, vfunc_mslt_vx, vfunc_msltu_vv, vfunc_msltu_vx,
    vfunc_msne_vi, vfunc_msne_vv, vfunc_msne_vx, vfunc_mul_vv, vfunc_mul_vx, vfunc_rem_vv,
    vfunc_rem_vx, vfunc_remu_vv, vfunc_remu_vx, vfunc_rsub_vi, vfunc_rsub_vx, vfunc_sll_vi,
    vfunc_sll_vv, vfunc_sra_vi, vfunc_sra_vv, vfunc_srl_vi, vfunc_srl_vv, vfunc_sub_vv,
    vfunc_sub_vx,
};
use crate::memory::Memory;
use ckb_vm_definitions::{instructions as insts, registers::RA, VLEN};
use uintxx::{Element, U128, U16, U32, U64, U8};

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
            common::slli(machine, i.rd(), i.rs1(), i.immediate());
        }
        insts::OP_SRLI => {
            let i = Itype(inst);
            common::srli(machine, i.rd(), i.rs1(), i.immediate());
        }
        insts::OP_SRAI => {
            let i = Itype(inst);
            common::srai(machine, i.rd(), i.rs1(), i.immediate());
        }
        insts::OP_SLLIW => {
            let i = Itype(inst);
            common::slliw(machine, i.rd(), i.rs1(), i.immediate());
        }
        insts::OP_SRLIW => {
            let i = Itype(inst);
            common::srliw(machine, i.rd(), i.rs1(), i.immediate());
        }
        insts::OP_SRAIW => {
            let i = Itype(inst);
            common::sraiw(machine, i.rd(), i.rs1(), i.immediate());
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
            let rs2_value = &Mac::REG::from_u32(i.immediate());
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
            let rs2_value = &Mac::REG::from_u32(i.immediate());
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
            let rs2_value = &Mac::REG::from_u32(i.immediate());
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
            let rs2_value = &Mac::REG::from_u32(i.immediate());
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
            let rs2_value = &Mac::REG::from_u32(i.immediate());
            let shamt = rs2_value.clone() & Mac::REG::from_u8(Mac::REG::SHIFT_MASK);
            let value = rs1_value.ror(&shamt);
            update_register(machine, i.rd(), value);
        }
        insts::OP_RORIW => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &Mac::REG::from_u32(i.immediate());
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
            let rs2_value = Mac::REG::from_u32(i.immediate());
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
                machine.registers()[i.rs1()].to_u32(),
                i.immediate(),
            )?;
        }
        insts::OP_VSETIVLI => {
            let i = Itype(inst);
            common::set_vl(machine, i.rd(), 33, i.rs1() as u32, i.immediate())?;
        }
        insts::OP_VSETVL => {
            let i = Rtype(inst);
            common::set_vl(
                machine,
                i.rd(),
                i.rs1(),
                machine.registers()[i.rs1()].to_u32(),
                machine.registers()[i.rs2()].to_u32(),
            )?;
        }
        insts::OP_VLE8_V => {
            let i = Itype(inst);
            let rd = i.rd();
            let addr = machine.registers()[i.rs1()].clone();
            for i in 0..machine.get_vl() {
                let rd = rd + i as usize / 256;
                let addr = addr.overflowing_add(&Mac::REG::from_u32(i as u32 * 1));
                let elem = machine.memory_mut().load8(&addr)?.to_u8();
                let vreg = machine.get_vregister(rd);
                if let VRegister::U8(ref mut data) = vreg {
                    data[i as usize % 256] = U8(elem);
                } else {
                    return Err(Error::Unexpected);
                }
            }
        }
        insts::OP_VLE16_V => {
            let i = Itype(inst);
            let rd = i.rd();
            let addr = machine.registers()[i.rs1()].clone();
            for i in 0..machine.get_vl() {
                let rd = rd + i as usize / 128;
                let addr = addr.overflowing_add(&Mac::REG::from_u32(i as u32 * 2));
                let elem = machine.memory_mut().load16(&addr)?.to_u16();
                let vreg = machine.get_vregister(rd);
                if let VRegister::U16(ref mut data) = vreg {
                    data[i as usize % 128] = U16(elem);
                } else {
                    return Err(Error::Unexpected);
                }
            }
        }
        insts::OP_VLE32_V => {
            let i = Itype(inst);
            let rd = i.rd();
            let addr = machine.registers()[i.rs1()].clone();
            for i in 0..machine.get_vl() {
                let rd = rd + i as usize / 64;
                let addr = addr.overflowing_add(&Mac::REG::from_u32(i as u32 * 4));
                let elem = machine.memory_mut().load32(&addr)?.to_u32();
                let vreg = machine.get_vregister(rd);
                if let VRegister::U32(ref mut data) = vreg {
                    data[i as usize % 64] = U32(elem);
                } else {
                    return Err(Error::Unexpected);
                }
            }
        }
        insts::OP_VLE64_V => {
            let i = Itype(inst);
            let rd = i.rd();
            let addr = machine.registers()[i.rs1()].clone();
            for i in 0..machine.get_vl() {
                let rd = rd + i as usize / 32;
                let addr = addr.overflowing_add(&Mac::REG::from_u32(i as u32 * 8));
                let elem = machine.memory_mut().load64(&addr)?.to_u64();
                let vreg = machine.get_vregister(rd);
                if let VRegister::U64(ref mut data) = vreg {
                    data[i as usize % 32] = U64(elem);
                } else {
                    return Err(Error::Unexpected);
                }
            }
        }
        insts::OP_VLE128_V => {
            let i = Itype(inst);
            let rd = i.rd();
            let addr = machine.registers()[i.rs1()].clone();
            let mut buf = [0u8; 16];
            for i in 0..machine.get_vl() {
                let rd = rd + i as usize / 16;
                let addr = addr.overflowing_add(&Mac::REG::from_u32(i as u32 * 16));
                buf.copy_from_slice(&machine.memory_mut().load_bytes(addr.to_u64(), 16)?);
                let elem = u128::from_le_bytes(buf);
                let vreg = machine.get_vregister(rd);
                if let VRegister::U128(ref mut data) = vreg {
                    data[i as usize % 16] = U128(elem);
                } else {
                    return Err(Error::Unexpected);
                }
            }
        }
        insts::OP_VLE256_V => {
            let i = Itype(inst);
            let rd = i.rd();
            let addr = machine.registers()[i.rs1()].clone();
            let mut buf = [0u8; 32];
            for i in 0..machine.get_vl() {
                let rd = rd + i as usize / 8;
                let addr = addr.overflowing_add(&Mac::REG::from_u32(i as u32 * 32));
                buf.copy_from_slice(&machine.memory_mut().load_bytes(addr.to_u64(), 32)?);
                let elem = U256::from_le_bytes(buf);
                let vreg = machine.get_vregister(rd);
                if let VRegister::U256(ref mut data) = vreg {
                    data[i as usize % 8] = elem;
                } else {
                    return Err(Error::Unexpected);
                }
            }
        }
        insts::OP_VLE512_V => {
            let i = Itype(inst);
            let rd = i.rd();
            let addr = machine.registers()[i.rs1()].clone();
            let mut buf = [0u8; 64];
            for i in 0..machine.get_vl() {
                let rd = rd + i as usize / 4;
                let addr = addr.overflowing_add(&Mac::REG::from_u32(i as u32 * 64));
                buf.copy_from_slice(&machine.memory_mut().load_bytes(addr.to_u64(), 64)?);
                let elem = U512::from_le_bytes(buf);
                let vreg = machine.get_vregister(rd);
                if let VRegister::U512(ref mut data) = vreg {
                    data[i as usize % 4] = elem;
                } else {
                    return Err(Error::Unexpected);
                }
            }
        }
        insts::OP_VLE1024_V => {
            let i = Itype(inst);
            let rd = i.rd();
            let addr = machine.registers()[i.rs1()].clone();
            let mut buf = [0u8; 128];
            for i in 0..machine.get_vl() {
                let rd = rd + i as usize / 2;
                let addr = addr.overflowing_add(&Mac::REG::from_u32(i as u32 * 128));
                buf.copy_from_slice(&machine.memory_mut().load_bytes(addr.to_u64(), 128)?);
                let elem = U1024::from_le_bytes(buf);
                let vreg = machine.get_vregister(rd);
                if let VRegister::U1024(ref mut data) = vreg {
                    data[i as usize % 2] = elem;
                } else {
                    return Err(Error::Unexpected);
                }
            }
        }
        insts::OP_VSE8_V => {
            let i = Itype(inst);
            let rd = i.rd();
            let addr = machine.registers()[i.rs1()].clone();
            let bits: usize = 8;
            for i in 0..machine.get_vl() {
                let rd = rd + i as usize / (VLEN as usize / bits);
                let addr = addr.overflowing_add(&Mac::REG::from_u32(i as u32 * (bits as u32 / 8)));
                let vreg = machine.vregisters()[rd];
                if let VRegister::U8(data) = vreg {
                    let elem = data[i as usize % (VLEN as usize / bits)];
                    machine
                        .memory_mut()
                        .store_bytes(addr.to_u64(), &elem.to_le_bytes())?;
                } else {
                    return Err(Error::Unexpected);
                }
            }
        }
        insts::OP_VSE16_V => {
            let i = Itype(inst);
            let rd = i.rd();
            let addr = machine.registers()[i.rs1()].clone();
            let bits: usize = 16;
            for i in 0..machine.get_vl() {
                let rd = rd + i as usize / (VLEN as usize / bits);
                let addr = addr.overflowing_add(&Mac::REG::from_u32(i as u32 * (bits as u32 / 8)));
                let vreg = machine.vregisters()[rd];
                if let VRegister::U16(data) = vreg {
                    let elem = data[i as usize % (VLEN as usize / bits)];
                    machine
                        .memory_mut()
                        .store_bytes(addr.to_u64(), &elem.to_le_bytes())?;
                } else {
                    return Err(Error::Unexpected);
                }
            }
        }
        insts::OP_VSE32_V => {
            let i = Itype(inst);
            let rd = i.rd();
            let addr = machine.registers()[i.rs1()].clone();
            let bits: usize = 32;
            for i in 0..machine.get_vl() {
                let rd = rd + i as usize / (VLEN as usize / bits);
                let addr = addr.overflowing_add(&Mac::REG::from_u32(i as u32 * (bits as u32 / 8)));
                let vreg = machine.vregisters()[rd];
                if let VRegister::U32(data) = vreg {
                    let elem = data[i as usize % (VLEN as usize / bits)];
                    machine
                        .memory_mut()
                        .store_bytes(addr.to_u64(), &elem.to_le_bytes())?;
                } else {
                    return Err(Error::Unexpected);
                }
            }
        }
        insts::OP_VSE64_V => {
            let i = Itype(inst);
            let rd = i.rd();
            let addr = machine.registers()[i.rs1()].clone();
            let bits: usize = 64;
            for i in 0..machine.get_vl() {
                let rd = rd + i as usize / (VLEN as usize / bits);
                let addr = addr.overflowing_add(&Mac::REG::from_u32(i as u32 * (bits as u32 / 8)));
                let vreg = machine.vregisters()[rd];
                if let VRegister::U64(data) = vreg {
                    let elem = data[i as usize % (VLEN as usize / bits)];
                    machine
                        .memory_mut()
                        .store_bytes(addr.to_u64(), &elem.to_le_bytes())?;
                } else {
                    return Err(Error::Unexpected);
                }
            }
        }
        insts::OP_VSE128_V => {
            let i = Itype(inst);
            let rd = i.rd();
            let addr = machine.registers()[i.rs1()].clone();
            let bits: usize = 128;
            for i in 0..machine.get_vl() {
                let rd = rd + i as usize / (VLEN as usize / bits);
                let addr = addr.overflowing_add(&Mac::REG::from_u32(i as u32 * (bits as u32 / 8)));
                let vreg = machine.vregisters()[rd];
                if let VRegister::U128(data) = vreg {
                    let elem = data[i as usize % (VLEN as usize / bits)];
                    machine
                        .memory_mut()
                        .store_bytes(addr.to_u64(), &elem.to_le_bytes())?;
                } else {
                    return Err(Error::Unexpected);
                }
            }
        }
        insts::OP_VSE256_V => {
            let i = Itype(inst);
            let rd = i.rd();
            let addr = machine.registers()[i.rs1()].clone();
            let bits: usize = 256;
            for i in 0..machine.get_vl() {
                let rd = rd + i as usize / (VLEN as usize / bits);
                let addr = addr.overflowing_add(&Mac::REG::from_u32(i as u32 * (bits as u32 / 8)));
                let vreg = machine.vregisters()[rd];
                if let VRegister::U256(data) = vreg {
                    let elem = data[i as usize % (VLEN as usize / bits)];
                    machine
                        .memory_mut()
                        .store_bytes(addr.to_u64(), &elem.to_le_bytes())?;
                } else {
                    return Err(Error::Unexpected);
                }
            }
        }
        insts::OP_VSE512_V => {
            let i = Itype(inst);
            let rd = i.rd();
            let addr = machine.registers()[i.rs1()].clone();
            let bits: usize = 512;
            for i in 0..machine.get_vl() {
                let rd = rd + i as usize / (VLEN as usize / bits);
                let addr = addr.overflowing_add(&Mac::REG::from_u32(i as u32 * (bits as u32 / 8)));
                let vreg = machine.vregisters()[rd];
                if let VRegister::U512(data) = vreg {
                    let elem = data[i as usize % (VLEN as usize / bits)];
                    machine
                        .memory_mut()
                        .store_bytes(addr.to_u64(), &elem.to_le_bytes())?;
                } else {
                    return Err(Error::Unexpected);
                }
            }
        }
        insts::OP_VSE1024_V => {
            let i = Itype(inst);
            let rd = i.rd();
            let addr = machine.registers()[i.rs1()].clone();
            let bits: usize = 1024;
            for i in 0..machine.get_vl() {
                let rd = rd + i as usize / (VLEN as usize / bits);
                let addr = addr.overflowing_add(&Mac::REG::from_u32(i as u32 * (bits as u32 / 8)));
                let vreg = machine.vregisters()[rd];
                if let VRegister::U1024(data) = vreg {
                    let elem = data[i as usize % (VLEN as usize / bits)];
                    machine
                        .memory_mut()
                        .store_bytes(addr.to_u64(), &elem.to_le_bytes())?;
                } else {
                    return Err(Error::Unexpected);
                }
            }
        }
        insts::OP_VADD_VV => {
            let i = VVtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs1 = machine.vregisters()[i.vs1() as usize + j as usize];
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_add_vv(&vs2, &vs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VADD_VX => {
            let i = VXtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let rs1 = machine.registers()[i.rs1() as usize + j as usize].to_u64();
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_add_vx(&vs2, rs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VADD_VI => {
            let i = VItype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let imm = i.immediate_s();
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_add_vi(&vs2, imm, &mut vd, num as usize)?;
            }
        }
        insts::OP_VSUB_VV => {
            let i = VVtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs1 = machine.vregisters()[i.vs1() as usize + j as usize];
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_sub_vv(&vs2, &vs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VSUB_VX => {
            let i = VXtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let rs1 = machine.registers()[i.rs1() as usize + j as usize].to_u64();
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_sub_vx(&vs2, rs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VRSUB_VX => {
            let i = Rtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let rs1 = machine.registers()[i.rs1() as usize + j as usize].to_u64();
                let vs2 = machine.vregisters()[i.rs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.rd() + j as usize);
                vfunc_rsub_vx(&vs2, rs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VRSUB_VI => {
            let i = VItype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let imm = i.immediate_s();
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_rsub_vi(&vs2, imm, &mut vd, num as usize)?;
            }
        }
        insts::OP_VMUL_VV => {
            let i = VVtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs1 = machine.vregisters()[i.vs1() as usize + j as usize];
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_mul_vv(&vs2, &vs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VMUL_VX => {
            let i = VXtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let rs1 = machine.registers()[i.rs1() as usize + j as usize].to_u64();
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_mul_vx(&vs2, rs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VDIVU_VV => {
            let i = VVtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs1 = machine.vregisters()[i.vs1() as usize + j as usize];
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_divu_vv(&vs2, &vs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VDIVU_VX => {
            let i = VXtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let rs1 = machine.registers()[i.rs1() as usize + j as usize].to_u64();
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_divu_vx(&vs2, rs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VDIV_VV => {
            let i = VVtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs1 = machine.vregisters()[i.vs1() as usize + j as usize];
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_div_vv(&vs2, &vs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VDIV_VX => {
            let i = VXtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let rs1 = machine.registers()[i.rs1() as usize + j as usize].to_u64();
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_div_vx(&vs2, rs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VREMU_VV => {
            let i = VVtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs1 = machine.vregisters()[i.vs1() as usize + j as usize];
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_remu_vv(&vs2, &vs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VREMU_VX => {
            let i = VXtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let rs1 = machine.registers()[i.rs1() as usize + j as usize].to_u64();
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_remu_vx(&vs2, rs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VREM_VV => {
            let i = VVtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs1 = machine.vregisters()[i.vs1() as usize + j as usize];
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_rem_vv(&vs2, &vs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VREM_VX => {
            let i = VXtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let rs1 = machine.registers()[i.rs1() as usize + j as usize].to_u64();
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_rem_vx(&vs2, rs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VSLL_VV => {
            let i = VVtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs1 = machine.vregisters()[i.vs1() as usize + j as usize];
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_sll_vv(&vs2, &vs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VSLL_VX => {
            let i = VXtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let rs1 = machine.registers()[i.rs1() as usize + j as usize].to_u32();
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_sll_vi(&vs2, rs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VSLL_VI => {
            let i = VItype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let imm = i.immediate_u();
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_sll_vi(&vs2, imm, &mut vd, num as usize)?;
            }
        }
        insts::OP_VSRL_VV => {
            let i = VVtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs1 = machine.vregisters()[i.vs1() as usize + j as usize];
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_srl_vv(&vs2, &vs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VSRL_VX => {
            let i = VXtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let rs1 = machine.registers()[i.rs1() as usize + j as usize].to_u32();
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_srl_vi(&vs2, rs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VSRL_VI => {
            let i = VItype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let imm = i.immediate_u();
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_srl_vi(&vs2, imm, &mut vd, num as usize)?;
            }
        }
        insts::OP_VSRA_VV => {
            let i = VVtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs1 = machine.vregisters()[i.vs1() as usize + j as usize];
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_sra_vv(&vs2, &vs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VSRA_VX => {
            let i = VXtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let rs1 = machine.registers()[i.rs1() as usize + j as usize].to_u32();
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_sra_vi(&vs2, rs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VSRA_VI => {
            let i = VItype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let imm = i.immediate_u();
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_sra_vi(&vs2, imm, &mut vd, num as usize)?;
            }
        }
        insts::OP_VMSEQ_VV => {
            let i = VVtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs1 = machine.vregisters()[i.vs1() as usize + j as usize];
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_mseq_vv(&vs2, &vs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VMSEQ_VX => {
            let i = VXtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let rs1 = machine.registers()[i.rs1() as usize + j as usize].to_u64();
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_mseq_vx(&vs2, rs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VMSEQ_VI => {
            let i = VItype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let imm = i.immediate_s();
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_mseq_vi(&vs2, imm, &mut vd, num as usize)?;
            }
        }
        insts::OP_VMSNE_VV => {
            let i = VVtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs1 = machine.vregisters()[i.vs1() as usize + j as usize];
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_msne_vv(&vs2, &vs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VMSNE_VX => {
            let i = VXtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let rs1 = machine.registers()[i.rs1() as usize + j as usize].to_u64();
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_msne_vx(&vs2, rs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VMSNE_VI => {
            let i = VItype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let imm = i.immediate_s();
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_msne_vi(&vs2, imm, &mut vd, num as usize)?;
            }
        }
        insts::OP_VMSLTU_VV => {
            let i = VVtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs1 = machine.vregisters()[i.vs1() as usize + j as usize];
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_msltu_vv(&vs2, &vs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VMSLTU_VX => {
            let i = VXtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let rs1 = machine.registers()[i.rs1() as usize + j as usize].to_u64();
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_msltu_vx(&vs2, rs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VMSLT_VV => {
            let i = VVtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs1 = machine.vregisters()[i.vs1() as usize + j as usize];
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_mslt_vv(&vs2, &vs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VMSLT_VX => {
            let i = VXtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let rs1 = machine.registers()[i.rs1() as usize + j as usize].to_u64();
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_mslt_vx(&vs2, rs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VMSLEU_VV => {
            let i = VVtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs1 = machine.vregisters()[i.vs1() as usize + j as usize];
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_msleu_vv(&vs2, &vs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VMSLEU_VX => {
            let i = VXtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let rs1 = machine.registers()[i.rs1() as usize + j as usize].to_u64();
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_msleu_vx(&vs2, rs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VMSLEU_VI => {
            let i = VItype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let imm = i.immediate_s();
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_msleu_vi(&vs2, imm, &mut vd, num as usize)?;
            }
        }
        insts::OP_VMSLE_VV => {
            let i = VVtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs1 = machine.vregisters()[i.vs1() as usize + j as usize];
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_msle_vv(&vs2, &vs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VMSLE_VX => {
            let i = VXtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let rs1 = machine.registers()[i.rs1() as usize + j as usize].to_u64();
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_msle_vx(&vs2, rs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VMSLE_VI => {
            let i = VItype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let imm = i.immediate_s();
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_msle_vi(&vs2, imm, &mut vd, num as usize)?;
            }
        }
        insts::OP_VMSGTU_VX => {
            let i = VXtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let rs1 = machine.registers()[i.rs1() as usize + j as usize].to_u64();
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_msgtu_vx(&vs2, rs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VMSGTU_VI => {
            let i = VItype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let imm = i.immediate_s();
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_msgtu_vi(&vs2, imm, &mut vd, num as usize)?;
            }
        }
        insts::OP_VMSGT_VX => {
            let i = VXtype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let rs1 = machine.registers()[i.rs1() as usize + j as usize].to_u64();
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_msgt_vx(&vs2, rs1, &mut vd, num as usize)?;
            }
        }
        insts::OP_VMSGT_VI => {
            let i = VItype(inst);
            let vlmax = VLEN / machine.get_vsew();
            for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
                let num = if machine.get_vl() > vlmax {
                    vlmax
                } else {
                    machine.get_vl()
                };
                let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
                let imm = i.immediate_s();
                let mut vd = machine.get_vregister(i.vd() + j as usize);
                vfunc_msgt_vi(&vs2, imm, &mut vd, num as usize)?;
            }
        }
        insts::OP_VFIRST_M => {
            let i = Rtype(inst);
            let vs2 = machine.vregisters()[i.rs2() as usize];
            let mut r = u64::MAX;
            match vs2 {
                VRegister::U1024(data) => {
                    for (j, e) in data.iter().enumerate() {
                        if e == &U1024::from(1u8) {
                            r = j as u64;
                            break;
                        }
                    }
                }
                VRegister::U512(data) => {
                    for (j, e) in data.iter().enumerate() {
                        if e == &U512::from(1u8) {
                            r = j as u64;
                            break;
                        }
                    }
                }
                VRegister::U256(data) => {
                    for (j, e) in data.iter().enumerate() {
                        if e == &U256::from(1u8) {
                            r = j as u64;
                            break;
                        }
                    }
                }
                VRegister::U128(data) => {
                    for (j, e) in data.iter().enumerate() {
                        if *e == U128::ONE {
                            r = j as u64;
                            break;
                        }
                    }
                }
                VRegister::U64(data) => {
                    for (j, e) in data.iter().enumerate() {
                        if *e == U64::ONE {
                            r = j as u64;
                            break;
                        }
                    }
                }
                VRegister::U32(data) => {
                    for (j, e) in data.iter().enumerate() {
                        if *e == U32::ONE {
                            r = j as u64;
                            break;
                        }
                    }
                }
                VRegister::U16(data) => {
                    for (j, e) in data.iter().enumerate() {
                        if *e == U16::ONE {
                            r = j as u64;
                            break;
                        }
                    }
                }
                VRegister::U8(data) => {
                    for (j, e) in data.iter().enumerate() {
                        if *e == U8::ONE {
                            r = j as u64;
                            break;
                        }
                    }
                }
            }
            update_register(machine, i.rd(), Mac::REG::from_u64(r));
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
