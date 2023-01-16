use super::{
    super::{machine::Machine, Error},
    common, extract_opcode, instruction_length,
    utils::update_register,
    Instruction, Itype, R4type, Register, Rtype, Stype, Utype,
};
use ckb_vm_definitions::{instructions as insts, registers::RA};

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
            let value = rs1_value.lt_s(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_SLTU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.lt(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_LB_VERSION0 => {
            let i = Itype(inst);
            common::lb(machine, i.rd(), i.rs1(), i.immediate_s(), true)?;
        }
        insts::OP_LB_VERSION1 => {
            let i = Itype(inst);
            common::lb(machine, i.rd(), i.rs1(), i.immediate_s(), false)?;
        }
        insts::OP_LH_VERSION0 => {
            let i = Itype(inst);
            common::lh(machine, i.rd(), i.rs1(), i.immediate_s(), true)?;
        }
        insts::OP_LH_VERSION1 => {
            let i = Itype(inst);
            common::lh(machine, i.rd(), i.rs1(), i.immediate_s(), false)?;
        }
        insts::OP_LW_VERSION0 => {
            let i = Itype(inst);
            common::lw(machine, i.rd(), i.rs1(), i.immediate_s(), true)?;
        }
        insts::OP_LW_VERSION1 => {
            let i = Itype(inst);
            common::lw(machine, i.rd(), i.rs1(), i.immediate_s(), false)?;
        }
        insts::OP_LD_VERSION0 => {
            let i = Itype(inst);
            common::ld(machine, i.rd(), i.rs1(), i.immediate_s(), true)?;
        }
        insts::OP_LD_VERSION1 => {
            let i = Itype(inst);
            common::ld(machine, i.rd(), i.rs1(), i.immediate_s(), false)?;
        }
        insts::OP_LBU_VERSION0 => {
            let i = Itype(inst);
            common::lbu(machine, i.rd(), i.rs1(), i.immediate_s(), true)?;
        }
        insts::OP_LBU_VERSION1 => {
            let i = Itype(inst);
            common::lbu(machine, i.rd(), i.rs1(), i.immediate_s(), false)?;
        }
        insts::OP_LHU_VERSION0 => {
            let i = Itype(inst);
            common::lhu(machine, i.rd(), i.rs1(), i.immediate_s(), true)?;
        }
        insts::OP_LHU_VERSION1 => {
            let i = Itype(inst);
            common::lhu(machine, i.rd(), i.rs1(), i.immediate_s(), false)?;
        }
        insts::OP_LWU_VERSION0 => {
            let i = Itype(inst);
            common::lwu(machine, i.rd(), i.rs1(), i.immediate_s(), true)?;
        }
        insts::OP_LWU_VERSION1 => {
            let i = Itype(inst);
            common::lwu(machine, i.rd(), i.rs1(), i.immediate_s(), false)?;
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
        insts::OP_JALR_VERSION0 => {
            let i = Itype(inst);
            let size = instruction_length(inst);
            let link = machine.pc().overflowing_add(&Mac::REG::from_u8(size));
            update_register(machine, i.rd(), link);
            let mut next_pc =
                machine.registers()[i.rs1()].overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
            next_pc = next_pc & (!Mac::REG::one());
            machine.update_pc(next_pc);
        }
        insts::OP_JALR_VERSION1 => {
            let i = Itype(inst);
            let size = instruction_length(inst);
            let link = machine.pc().overflowing_add(&Mac::REG::from_u8(size));
            let mut next_pc =
                machine.registers()[i.rs1()].overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
            next_pc = next_pc & (!Mac::REG::one());
            update_register(machine, i.rd(), link);
            machine.update_pc(next_pc);
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
            let condition = rs1_value.eq(rs2_value);
            let new_pc = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()).overflowing_add(pc),
                &Mac::REG::from_u8(instruction_length(inst)).overflowing_add(pc),
            );
            machine.update_pc(new_pc);
        }
        insts::OP_BNE => {
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
        }
        insts::OP_BLT => {
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
        }
        insts::OP_BGE => {
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
        }
        insts::OP_BLTU => {
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
        }
        insts::OP_BGEU => {
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
            let value = rs1_value.overflowing_mul(rs2_value);
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
            let value = rs1_value.overflowing_mul_high_signed(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MULHSU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_mul_high_signed_unsigned(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MULHU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_mul_high_unsigned(rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_DIV => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.overflowing_div_signed(rs2_value);
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
            let value = rs1_value.overflowing_div(rs2_value);
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
            let value = rs1_value.overflowing_rem_signed(rs2_value);
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
            let value = rs1_value.overflowing_rem(rs2_value);
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
            let value = rs1_value.ge_s(rs2_value).cond(rs1_value, rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MAXU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.ge(rs2_value).cond(rs1_value, rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MIN => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.lt_s(rs2_value).cond(rs1_value, rs2_value);
            update_register(machine, i.rd(), value);
        }
        insts::OP_MINU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value = rs1_value.lt(rs2_value).cond(rs1_value, rs2_value);
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
            let value_h = rs1_value.overflowing_mul_high_signed(rs2_value);
            let value_l = rs1_value.overflowing_mul(rs2_value);
            update_register(machine, i.rd(), value_h);
            update_register(machine, i.rs3(), value_l);
        }
        insts::OP_WIDE_MULU => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value_h = rs1_value.overflowing_mul_high_unsigned(rs2_value);
            let value_l = rs1_value.overflowing_mul(rs2_value);
            update_register(machine, i.rd(), value_h);
            update_register(machine, i.rs3(), value_l);
        }
        insts::OP_WIDE_MULSU => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value_h = rs1_value.overflowing_mul_high_signed_unsigned(rs2_value);
            let value_l = rs1_value.overflowing_mul(rs2_value);
            update_register(machine, i.rd(), value_h);
            update_register(machine, i.rs3(), value_l);
        }
        insts::OP_WIDE_DIV => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value_h = rs1_value.overflowing_div_signed(rs2_value);
            let value_l = rs1_value.overflowing_rem_signed(rs2_value);
            update_register(machine, i.rd(), value_h);
            update_register(machine, i.rs3(), value_l);
        }
        insts::OP_WIDE_DIVU => {
            let i = R4type(inst);
            let rs1_value = &machine.registers()[i.rs1()];
            let rs2_value = &machine.registers()[i.rs2()];
            let value_h = rs1_value.overflowing_div(rs2_value);
            let value_l = rs1_value.overflowing_rem(rs2_value);
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
        }
        insts::OP_SBB => {
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
        }
        insts::OP_CUSTOM_LOAD_UIMM => {
            let i = Utype(inst);
            update_register(machine, i.rd(), Mac::REG::from_u32(i.immediate_u()));
        }
        insts::OP_CUSTOM_LOAD_IMM => {
            let i = Utype(inst);
            let value = Mac::REG::from_i32(i.immediate_s());
            update_register(machine, i.rd(), value);
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
