use super::{
    super::{machine::Machine, Error},
    common, extract_opcode, instruction_length,
    utils::update_register,
    Instruction, Itype, R4type, Register, Rtype, Stype, Utype, VItype, VRegister, VVtype, VXtype,
};

use crate::memory::Memory;
use ckb_vm_definitions::{instructions as insts, registers::RA, VLEN};
pub use uintxx::{Element, I1024, I256, I512, U1024, U128, U16, U256, U32, U512, U64, U8};

type VVIteratorFunc =
    fn(lhs: &VRegister, rhs: &VRegister, result: &mut VRegister, num: usize) -> Result<(), Error>;

fn loop_vv<Mac: Machine>(
    inst: Instruction,
    machine: &mut Mac,
    func: VVIteratorFunc,
) -> Result<(), Error> {
    let i = VVtype(inst);
    let vlmax = VLEN / machine.get_vsew();
    let num = machine.get_vl() as usize;
    for j in 0..((num as u32 - 1) / vlmax) + 1 {
        let vs1 = machine.vregisters()[i.vs1() as usize + j as usize];
        let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
        let vd = machine.get_vregister(i.vd() + j as usize);
        func(&vs2, &vs1, vd, num)?;
    }
    Ok(())
}
macro_rules! vv_iterator_impl {
    ($func_1024:tt,
    $func_512:tt,
    $func_256:tt,
    $func_128:tt,
    $func_64:tt,
    $func_32:tt,
    $func_16:tt,
    $func_8:tt) => {
        pub fn vv_iterator_func(
            vs2: &VRegister,
            vs1: &VRegister,
            vd: &mut VRegister,
            num: usize,
        ) -> Result<(), Error> {
            match (vs2, vs1, vd) {
                (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_1024(&a[i], &b[i]);
                    }
                }
                (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_512(&a[i], &b[i]);
                    }
                }
                (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_256(&a[i], &b[i]);
                    }
                }
                (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_128(&a[i], &b[i]);
                    }
                }
                (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_64(&a[i], &b[i]);
                    }
                }
                (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_32(&a[i], &b[i]);
                    }
                }
                (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_16(&a[i], &b[i]);
                    }
                }
                (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_8(&a[i], &b[i]);
                    }
                }
                _ => return Err(Error::Unexpected),
            }
            Ok(())
        }
    };
}

type VXIteratorFunc =
    fn(lhs: &VRegister, rhs: u64, result: &mut VRegister, num: usize) -> Result<(), Error>;

fn loop_vx<Mac: Machine>(
    inst: Instruction,
    machine: &mut Mac,
    func: VXIteratorFunc,
) -> Result<(), Error> {
    let i = VXtype(inst);
    let vlmax = VLEN / machine.get_vsew();
    let num = machine.get_vl() as usize;
    for j in 0..((num as u32 - 1) / vlmax) + 1 {
        let rs1 = machine.registers()[i.rs1() as usize + j as usize].to_u64();
        let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
        let vd = machine.get_vregister(i.vd() + j as usize);
        func(&vs2, rs1, vd, num)?;
    }
    Ok(())
}

macro_rules! vx_iterator_impl {
    ($func_1024:tt,
    $func_512:tt,
    $func_256:tt,
    $func_128:tt,
    $func_64:tt,
    $func_32:tt,
    $func_16:tt,
    $func_8:tt) => {
        pub fn vx_iterator_func(
            lhs: &VRegister,
            rhs: u64,
            result: &mut VRegister,
            num: usize,
        ) -> Result<(), Error> {
            match (lhs, result) {
                (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_1024(&a[i], rhs);
                    }
                }
                (VRegister::U512(a), VRegister::U512(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_512(&a[i], rhs);
                    }
                }
                (VRegister::U256(a), VRegister::U256(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_256(&a[i], rhs);
                    }
                }
                (VRegister::U128(a), VRegister::U128(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_128(&a[i], rhs);
                    }
                }
                (VRegister::U64(a), VRegister::U64(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_64(&a[i], rhs);
                    }
                }
                (VRegister::U32(a), VRegister::U32(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_32(&a[i], rhs);
                    }
                }
                (VRegister::U16(a), VRegister::U16(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_16(&a[i], rhs);
                    }
                }
                (VRegister::U8(a), VRegister::U8(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_8(&a[i], rhs);
                    }
                }
                _ => return Err(Error::Unexpected),
            }
            Ok(())
        }
    };
}

type VIIteratorFunc =
    fn(lhs: &VRegister, rhs: i32, result: &mut VRegister, num: usize) -> Result<(), Error>;

fn loop_vi<Mac: Machine>(
    inst: Instruction,
    machine: &mut Mac,
    func: VIIteratorFunc,
) -> Result<(), Error> {
    let i = VItype(inst);
    let vlmax = VLEN / machine.get_vsew();
    let num = machine.get_vl() as usize;
    for j in 0..((num as u32 - 1) / vlmax) + 1 {
        let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
        let imm = i.immediate_s();
        let vd = machine.get_vregister(i.vd() + j as usize);
        func(&vs2, imm, vd, num)?;
    }
    Ok(())
}

macro_rules! vi_iterator_impl {
    ($func_1024:tt,
    $func_512:tt,
    $func_256:tt,
    $func_128:tt,
    $func_64:tt,
    $func_32:tt,
    $func_16:tt,
    $func_8:tt) => {
        pub fn vi_iterator_func(
            lhs: &VRegister,
            imm: i32,
            result: &mut VRegister,
            num: usize,
        ) -> Result<(), Error> {
            match (lhs, result) {
                (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_1024(&a[i], imm);
                    }
                }
                (VRegister::U512(a), VRegister::U512(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_512(&a[i], imm);
                    }
                }
                (VRegister::U256(a), VRegister::U256(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_256(&a[i], imm);
                    }
                }
                (VRegister::U128(a), VRegister::U128(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_128(&a[i], imm);
                    }
                }
                (VRegister::U64(a), VRegister::U64(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_64(&a[i], imm);
                    }
                }
                (VRegister::U32(a), VRegister::U32(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_32(&a[i], imm);
                    }
                }
                (VRegister::U16(a), VRegister::U16(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_16(&a[i], imm);
                    }
                }
                (VRegister::U8(a), VRegister::U8(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_8(&a[i], imm);
                    }
                }
                _ => return Err(Error::Unexpected),
            }
            Ok(())
        }
    };
}

type VI2IteratorFunc =
    fn(lhs: &VRegister, rhs: u32, result: &mut VRegister, num: usize) -> Result<(), Error>;

fn loop_vi2<Mac: Machine>(
    inst: Instruction,
    machine: &mut Mac,
    func: VI2IteratorFunc,
) -> Result<(), Error> {
    let i = VItype(inst);
    let vlmax = VLEN / machine.get_vsew();
    let num = machine.get_vl() as usize;
    for j in 0..((machine.get_vl() - 1) / vlmax) + 1 {
        let vs2 = machine.vregisters()[i.vs2() as usize + j as usize];
        let imm = i.immediate_u();
        let vd = machine.get_vregister(i.vd() + j as usize);
        func(&vs2, imm, vd, num)?;
    }
    Ok(())
}

macro_rules! vi2_iterator_impl {
    ($func_1024:tt,
    $func_512:tt,
    $func_256:tt,
    $func_128:tt,
    $func_64:tt,
    $func_32:tt,
    $func_16:tt,
    $func_8:tt) => {
        pub fn vi2_iterator_func(
            lhs: &VRegister,
            imm: u32,
            result: &mut VRegister,
            num: usize,
        ) -> Result<(), Error> {
            match (lhs, result) {
                (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_1024(&a[i], imm);
                    }
                }
                (VRegister::U512(a), VRegister::U512(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_512(&a[i], imm);
                    }
                }
                (VRegister::U256(a), VRegister::U256(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_256(&a[i], imm);
                    }
                }
                (VRegister::U128(a), VRegister::U128(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_128(&a[i], imm);
                    }
                }
                (VRegister::U64(a), VRegister::U64(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_64(&a[i], imm);
                    }
                }
                (VRegister::U32(a), VRegister::U32(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_32(&a[i], imm);
                    }
                }
                (VRegister::U16(a), VRegister::U16(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_16(&a[i], imm);
                    }
                }
                (VRegister::U8(a), VRegister::U8(ref mut r)) => {
                    for i in 0..num {
                        r[i] = $func_8(&a[i], imm);
                    }
                }
                _ => return Err(Error::Unexpected),
            }
            Ok(())
        }
    };
}

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
            vv_iterator_impl! {
                {|a: &U1024, b: &U1024| a.wrapping_add(*b)},
                {|a: &U512, b: &U512| a.wrapping_add(*b)},
                {|a: &U256, b: &U256| a.wrapping_add(*b)},
                {|a: &U128, b: &U128| a.wrapping_add(*b)},
                {|a: &U64, b: &U64| a.wrapping_add(*b)},
                {|a: &U32, b: &U32| a.wrapping_add(*b)},
                {|a: &U16, b: &U16| a.wrapping_add(*b)},
                {|a: &U8, b: &U8| a.wrapping_add(*b)}
            };
            loop_vv(inst, machine, vv_iterator_func)?;
        }
        insts::OP_VADD_VX => {
            vx_iterator_impl! {
                {|a:&U1024, rhs: u64| a.wrapping_add(U1024::vx(rhs))},
                {|a:&U512, rhs: u64| a.wrapping_add(U512::vx(rhs))},
                {|a:&U256, rhs: u64| a.wrapping_add(U256::vx(rhs))},
                {|a:&U128, rhs: u64| a.wrapping_add(U128::vx(rhs))},
                {|a:&U64, rhs: u64| a.wrapping_add(U64::vx(rhs))},
                {|a:&U32, rhs: u64| a.wrapping_add(U32::vx(rhs))},
                {|a:&U16, rhs: u64| a.wrapping_add(U16::vx(rhs))},
                {|a:&U8, rhs: u64| a.wrapping_add(U8::vx(rhs))}
            };
            loop_vx(inst, machine, vx_iterator_func)?;
        }
        insts::OP_VADD_VI => {
            vi_iterator_impl! {
                {|a:&U1024, imm: i32| a.wrapping_add(U1024::vi(imm)) },
                {|a:&U512, imm: i32| a.wrapping_add(U512::vi(imm)) },
                {|a:&U256, imm: i32| a.wrapping_add(U256::vi(imm))  },
                {|a:&U128, imm: i32| a.wrapping_add(U128::vi(imm))  },
                {|a:&U64, imm: i32| a.wrapping_add(U64::vi(imm))  },
                {|a:&U32, imm: i32| a.wrapping_add(U32::vi(imm))  },
                {|a:&U16, imm: i32| a.wrapping_add(U16::vi(imm))  },
                {|a:&U8, imm: i32| a.wrapping_add(U8::vi(imm)) }
            };
            loop_vi(inst, machine, vi_iterator_func)?;
        }
        insts::OP_VSUB_VV => {
            vv_iterator_impl! {
                {|a: &U1024, b: &U1024| a.wrapping_sub(*b)},
                {|a: &U512, b: &U512| a.wrapping_sub(*b)},
                {|a: &U256, b: &U256| a.wrapping_sub(*b)},
                {|a: &U128, b: &U128| a.wrapping_sub(*b)},
                {|a: &U64, b: &U64| a.wrapping_sub(*b)},
                {|a: &U32, b: &U32| a.wrapping_sub(*b)},
                {|a: &U16, b: &U16| a.wrapping_sub(*b)},
                {|a: &U8, b: &U8| a.wrapping_sub(*b)}
            };
            loop_vv(inst, machine, vv_iterator_func)?;
        }
        insts::OP_VSUB_VX => {
            vx_iterator_impl! {
                {|a:&U1024, rhs: u64| a.wrapping_sub(U1024::vx(rhs))},
                {|a:&U512, rhs: u64| a.wrapping_sub(U512::vx(rhs))},
                {|a:&U256, rhs: u64| a.wrapping_sub(U256::vx(rhs))},
                {|a:&U128, rhs: u64| a.wrapping_sub(U128::vx(rhs))},
                {|a:&U64, rhs: u64| a.wrapping_sub(U64::vx(rhs))},
                {|a:&U32, rhs: u64| a.wrapping_sub(U32::vx(rhs))},
                {|a:&U16, rhs: u64| a.wrapping_sub(U16::vx(rhs))},
                {|a:&U8, rhs: u64| a.wrapping_sub(U8::vx(rhs))}
            };
            loop_vx(inst, machine, vx_iterator_func)?;
        }
        insts::OP_VRSUB_VX => {
            vx_iterator_impl! {
                {|a:&U1024, rhs: u64| U1024::vx(rhs).wrapping_sub(*a) },
                {|a:&U512, rhs: u64| U512::vx(rhs).wrapping_sub(*a) },
                {|a:&U256, rhs: u64| U256::vx(rhs).wrapping_sub(*a)  },
                {|a:&U128, rhs: u64| U128::vx(rhs).wrapping_sub(*a)  },
                {|a:&U64, rhs: u64| U64::vx(rhs).wrapping_sub(*a)  },
                {|a:&U32, rhs: u64| U32::vx(rhs).wrapping_sub(*a)  },
                {|a:&U16, rhs: u64| U16::vx(rhs).wrapping_sub(*a) },
                {|a:&U8, rhs: u64| U8::vx(rhs).wrapping_sub(*a) }
            };
            loop_vx(inst, machine, vx_iterator_func)?;
        }
        insts::OP_VRSUB_VI => {
            vi_iterator_impl! {
                {|a:&U1024, imm: i32| U1024::vi(imm).wrapping_sub(*a) },
                {|a:&U512, imm: i32| U512::vi(imm).wrapping_sub(*a) },
                {|a:&U256, imm: i32| U256::vi(imm).wrapping_sub(*a) },
                {|a:&U128, imm: i32| U128::vi(imm).wrapping_sub(*a)  },
                {|a:&U64, imm: i32| U64::vi(imm).wrapping_sub(*a)  },
                {|a:&U32, imm: i32| U32::vi(imm).wrapping_sub(*a)  },
                {|a:&U16, imm: i32| U16::vi(imm).wrapping_sub(*a)  },
                {|a:&U8, imm: i32| U8::vi(imm).wrapping_sub(*a) }
            };
            loop_vi(inst, machine, vi_iterator_func)?;
        }

        insts::OP_VMUL_VV => {
            vv_iterator_impl! {
                {|a: &U1024, b: &U1024| a.wrapping_mul(*b)},
                {|a: &U512, b: &U512| a.wrapping_mul(*b)},
                {|a: &U256, b: &U256| a.wrapping_mul(*b)},
                {|a: &U128, b: &U128| a.wrapping_mul(*b)},
                {|a: &U64, b: &U64| a.wrapping_mul(*b)},
                {|a: &U32, b: &U32| a.wrapping_mul(*b)},
                {|a: &U16, b: &U16| a.wrapping_mul(*b)},
                {|a: &U8, b: &U8| a.wrapping_mul(*b)}
            };
            loop_vv(inst, machine, vv_iterator_func)?;
        }
        insts::OP_VMUL_VX => {
            vx_iterator_impl! {
                {|a:&U1024, rhs: u64| a.wrapping_mul(U1024::vx(rhs)) },
                {|a:&U512, rhs: u64| a.wrapping_mul(U512::vx(rhs)) },
                {|a:&U256, rhs: u64| a.wrapping_mul(U256::vx(rhs))  },
                {|a:&U128, rhs: u64| a.wrapping_mul(U128::vx(rhs))  },
                {|a:&U64, rhs: u64| a.wrapping_mul(U64::vx(rhs))  },
                {|a:&U32, rhs: u64| a.wrapping_mul(U32::vx(rhs))  },
                {|a:&U16, rhs: u64| a.wrapping_mul(U16::vx(rhs)) },
                {|a:&U8, rhs: u64| a.wrapping_mul(U8::vx(rhs)) }
            };
            loop_vx(inst, machine, vx_iterator_func)?;
        }
        insts::OP_VDIVU_VV => {
            vv_iterator_impl! {
                {|a: &U1024, b: &U1024| a.wrapping_div(*b)},
                {|a: &U512, b: &U512| a.wrapping_div(*b)},
                {|a: &U256, b: &U256| a.wrapping_div(*b)},
                {|a: &U128, b: &U128| a.wrapping_div(*b)},
                {|a: &U64, b: &U64| a.wrapping_div(*b)},
                {|a: &U32, b: &U32| a.wrapping_div(*b)},
                {|a: &U16, b: &U16| a.wrapping_div(*b)},
                {|a: &U8, b: &U8| a.wrapping_div(*b)}
            };
            loop_vv(inst, machine, vv_iterator_func)?;
        }
        insts::OP_VDIVU_VX => {
            vx_iterator_impl! {
                {|a:&U1024, rhs: u64| a.wrapping_div(U1024::vx(rhs)) },
                {|a:&U512, rhs: u64| a.wrapping_div(U512::vx(rhs)) },
                {|a:&U256, rhs: u64| a.wrapping_div(U256::vx(rhs))  },
                {|a:&U128, rhs: u64| a.wrapping_div(U128::vx(rhs)) },
                {|a:&U64, rhs: u64| a.wrapping_div(U64::vx(rhs))  },
                {|a:&U32, rhs: u64| a.wrapping_div(U32::vx(rhs))  },
                {|a:&U16, rhs: u64| a.wrapping_div(U16::vx(rhs)) },
                {|a:&U8, rhs: u64| a.wrapping_div(U8::vx(rhs))  }
            };
            loop_vx(inst, machine, vx_iterator_func)?;
        }
        insts::OP_VDIV_VV => {
            vv_iterator_impl! {
                {|a: &U1024, b: &U1024| I1024::from(*a).wrapping_div(I1024::from(*b)).uint},
                {|a: &U512, b: &U512| I512::from(*a).wrapping_div(I512::from(*b)).uint},
                {|a: &U256, b: &U256| I256::from(*a).wrapping_div(I256::from(*b)).uint},
                {|a: &U128, b: &U128| if b.0 == 0 {
                    U128::MAX
                } else if a.0 == 1 << 127 && b.0 == u128::MAX {
                    U128(1u128 << 127)
                } else {
                    U128((a.0 as i128).wrapping_div(b.0 as i128) as u128)
                }},
                {|a: &U64, b: &U64| if b.0 == 0 {
                    U64(u64::MAX)
                } else if a.0 == 1 << 63 && b.0 == u64::MAX {
                    U64(1u64 << 63)
                } else {
                    U64((a.0 as i64).wrapping_div(b.0 as i64) as u64)
                }},
                {|a: &U32, b: &U32| if b.0 == 0 {
                    U32(u32::MAX)
                } else if a.0 == 1 << 31 && b.0 == u32::MAX {
                    U32(1u32 << 31)
                } else {
                    U32((a.0 as i32).wrapping_div(b.0 as i32) as u32)
                }},
                {|a: &U16, b: &U16| if b.0 == 0 {
                    U16(u16::MAX)
                } else if a.0 == 1 << 15 && b.0 == u16::MAX {
                    U16(1u16 << 15)
                } else {
                    U16((a.0 as i16).wrapping_div(b.0 as i16) as u16)
                }},
                {|a: &U8, b: &U8| if b.0 == 0 {
                    U8(u8::MAX)
                } else if a.0 == 1 << 7 && b.0 == u8::MAX {
                    U8(1u8 << 7)
                } else {
                    U8((a.0 as i8).wrapping_div(b.0 as i8) as u8)
                }}
            };
            loop_vv(inst, machine, vv_iterator_func)?;
        }
        insts::OP_VDIV_VX => {
            vx_iterator_impl! {
                {|a:&U1024, rhs: u64|
                    I1024::from(*a)
                    .wrapping_div(I1024::from(U1024::from(rhs as i64)))
                    .uint
                },
                {|a:&U512, rhs: u64|
                    I512::from(*a)
                    .wrapping_div(I512::from(U512::from(rhs as i64)))
                    .uint },
                {|a:&U256, rhs: u64|
                    I256::from(*a)
                    .wrapping_div(I256::from(U256::from(rhs as i64)))
                    .uint},
                {|a:&U128, rhs: u64|
                    if rhs == 0 {
                        U128(u128::MAX)
                    } else if a.0 == 1 << 127 && rhs == u64::MAX {
                        U128(1u128 << 127)
                    } else {
                        U128((a.0 as i128).wrapping_div(rhs as i128) as u128)
                    }
                },
                {|a:&U64, rhs: u64|
                    if rhs == 0 {
                        U64(u64::MAX)
                    } else if a.0 == 1 << 63 && rhs == u64::MAX {
                        U64(1u64 << 63)
                    } else {
                        U64((a.0 as i64).wrapping_div(rhs as i64) as u64)
                    }
                },
                {|a:&U32, rhs: u64|
                    if rhs == 0 {
                        U32(u32::MAX)
                    } else if a.0 == 1 << 31 && rhs as u32 == u32::MAX {
                        U32(1u32 << 31)
                    } else {
                        U32((a.0 as i32).wrapping_div(rhs as i32) as u32)
                    }
                },
                {|a:&U16, rhs: u64|
                    if rhs == 0 {
                        U16(u16::MAX)
                    } else if a.0 == 1 << 15 && rhs as u16 == u16::MAX {
                        U16(1u16 << 15)
                    } else {
                        U16((a.0 as i16).wrapping_div(rhs as i16) as u16)
                    }
                },
                {|a:&U8, rhs: u64|
                    if rhs == 0 {
                        U8(u8::MAX)
                    } else if a.0 == 1 << 7 && rhs as u8 == u8::MAX {
                        U8(1u8 << 7)
                    } else {
                        U8((a.0 as i8).wrapping_div(rhs as i8) as u8)
                    }
                }
            };
            loop_vx(inst, machine, vx_iterator_func)?;
        }
        insts::OP_VREMU_VV => {
            vv_iterator_impl! {
                {|a: &U1024, b: &U1024| a.wrapping_rem(*b)},
                {|a: &U512, b: &U512| a.wrapping_rem(*b)},
                {|a: &U256, b: &U256| a.wrapping_rem(*b)},
                {|a: &U128, b: &U128| a.wrapping_rem(*b)},
                {|a: &U64, b: &U64| a.wrapping_rem(*b)},
                {|a: &U32, b: &U32| a.wrapping_rem(*b)},
                {|a: &U16, b: &U16| a.wrapping_rem(*b)},
                {|a: &U8, b: &U8| a.wrapping_rem(*b)}
            };
            loop_vv(inst, machine, vv_iterator_func)?;
        }
        insts::OP_VREMU_VX => {
            vx_iterator_impl! {
                {|a:&U1024, rhs: u64|  a.wrapping_rem(U1024::vx(rhs)) },
                {|a:&U512, rhs: u64| a.wrapping_rem(U512::vx(rhs)) },
                {|a:&U256, rhs: u64| a.wrapping_rem(U256::vx(rhs))  },
                {|a:&U128, rhs: u64| a.wrapping_rem(U128::vx(rhs))},
                {|a:&U64, rhs: u64| a.wrapping_rem(U64::vx(rhs))  },
                {|a:&U32, rhs: u64| a.wrapping_rem(U32::vx(rhs))  },
                {|a:&U16, rhs: u64| a.wrapping_rem(U16::vx(rhs)) },
                {|a:&U8, rhs: u64| a.wrapping_rem(U8::vx(rhs)) }
            };
            loop_vx(inst, machine, vx_iterator_func)?;
        }
        insts::OP_VREM_VV => {
            vv_iterator_impl! {
                {|a: &U1024, b: &U1024| I1024::from(*a).wrapping_rem(I1024::from(*b)).uint},
                {|a: &U512, b: &U512| I512::from(*a).wrapping_rem(I512::from(*b)).uint},
                {|a: &U256, b: &U256| I256::from(*a).wrapping_rem(I256::from(*b)).uint},
                {|a: &U128, b: &U128| if b.0 == 0 {
                    *a
                } else if a.0 == 1 << 127 && b.0 == u128::MAX {
                    U128(0)
                } else {
                    U128((a.0 as i128).wrapping_rem(b.0 as i128) as u128)
                }},
                {|a: &U64, b: &U64| if b.0 == 0 {
                    *a
                } else if a.0 == 1 << 63 && b.0 == u64::MAX {
                    U64(0)
                } else {
                    U64((a.0 as i64).wrapping_rem(b.0 as i64) as u64)
                }},
                {|a: &U32, b: &U32| if b.0 == 0 {
                    *a
                } else if a.0 == 1 << 31 && b.0 == u32::MAX {
                    U32(0)
                } else {
                    U32((a.0 as i32).wrapping_rem(b.0 as i32) as u32)
                }},
                {|a: &U16, b: &U16| if b.0 == 0 {
                    *a
                } else if a.0 == 1 << 15 && b.0 == u16::MAX {
                    U16(0)
                } else {
                    U16((a.0 as i16).wrapping_rem(b.0 as i16) as u16)
                }},
                {|a: &U8, b: &U8| if b.0 == 0 {
                    *a
                } else if a.0 == 1 << 7 && b.0 == u8::MAX {
                    U8(0)
                } else {
                    U8((a.0 as i8).wrapping_rem(b.0 as i8) as u8)
                }}
            };
            loop_vv(inst, machine, vv_iterator_func)?;
        }
        insts::OP_VREM_VX => {
            vx_iterator_impl! {
                {|a:&U1024, rhs: u64|
                    I1024::from(*a)
                    .wrapping_rem(I1024::from(U1024::from(rhs as i64)))
                    .uint },
                {|a:&U512, rhs: u64|
                    I512::from(*a)
                    .wrapping_rem(I512::from(U512::from(rhs as i64)))
                    .uint
                },
                {|a:&U256, rhs: u64|
                    I256::from(*a)
                    .wrapping_rem(I256::from(U256::from(rhs as i64)))
                    .uint
                },
                {|a:&U128, rhs: u64|
                    if rhs == 0 {
                        *a
                    } else if a.0 == 1 << 127 && rhs == u64::MAX {
                        U128(0)
                    } else {
                        U128((a.0 as i128).wrapping_rem(rhs as i128) as u128)
                    }
                },
                {|a:&U64, rhs: u64|
                    if rhs == 0 {
                        *a
                    } else if a.0 == 1 << 63 && rhs == u64::MAX {
                        U64(0)
                    } else {
                        U64((a.0 as i64).wrapping_rem(rhs as i64) as u64)
                    }
                },
                {|a:&U32, rhs: u64|
                    if rhs == 0 {
                        *a
                    } else if a.0 == 1 << 31 && rhs as u32 == u32::MAX {
                        U32(0)
                    } else {
                        U32((a.0 as i32).wrapping_rem(rhs as i32) as u32)
                    }
                },
                {|a:&U16, rhs: u64|
                     if rhs == 0 {
                        *a
                    } else if a.0 == 1 << 15 && rhs as u16 == u16::MAX {
                        U16(0)
                    } else {
                        U16((a.0 as i16).wrapping_rem(rhs as i16) as u16)
                    }
                },
                {|a:&U8, rhs: u64|
                    if rhs == 0 {
                        *a
                    } else if a.0 == 1 << 7 && rhs as u8 == u8::MAX {
                        U8(0)
                    } else {
                        U8((a.0 as i8).wrapping_rem(rhs as i8) as u8)
                    }
                }
            };
            loop_vx(inst, machine, vx_iterator_func)?;
        }
        insts::OP_VSLL_VV => {
            vv_iterator_impl! {
                {|a: &U1024, b: &U1024| *a << b.u32()},
                {|a: &U512, b: &U512| *a << b.u32()},
                {|a: &U256, b: &U256| *a << b.u32()},
                {|a: &U128, b: &U128| *a << b.u32()},
                {|a: &U64, b: &U64| *a << b.u32()},
                {|a: &U32, b: &U32| *a << b.u32()},
                {|a: &U16, b: &U16| *a << b.u32()},
                {|a: &U8, b: &U8| *a << b.u32()}
            };
            loop_vv(inst, machine, vv_iterator_func)?;
        }
        insts::OP_VSLL_VX => {
            vx_iterator_impl! {
                {|a:&U1024, rhs: u64|   *a << rhs as u32 },
                {|a:&U512, rhs: u64| *a << rhs as u32},
                {|a:&U256, rhs: u64|  *a << rhs as u32},
                {|a:&U128, rhs: u64|  *a << rhs as u32} ,
                {|a:&U64, rhs: u64|  *a << rhs as u32},
                {|a:&U32, rhs: u64|  *a << rhs as u32},
                {|a:&U16, rhs: u64|  *a << rhs as u32 },
                {|a:&U8, rhs: u64|  *a << rhs as u32}
            };
            loop_vx(inst, machine, vx_iterator_func)?;
        }
        insts::OP_VSLL_VI => {
            vi2_iterator_impl! {
                {|a:&U1024, imm: u32| *a << imm },
                {|a:&U512, imm: u32|  *a << imm },
                {|a:&U256, imm: u32|  *a << imm  },
                {|a:&U128, imm: u32|  *a << imm  },
                {|a:&U64, imm: u32|  *a << imm },
                {|a:&U32, imm: u32|  *a << imm },
                {|a:&U16, imm: u32|  *a << imm  },
                {|a:&U8, imm: u32|  *a << imm }
            };
            loop_vi2(inst, machine, vi2_iterator_func)?;
        }
        insts::OP_VSRL_VV => {
            vv_iterator_impl! {
                {|a: &U1024, b: &U1024| *a >> b.u32()},
                {|a: &U512, b: &U512| *a >> b.u32()},
                {|a: &U256, b: &U256| *a >> b.u32()},
                {|a: &U128, b: &U128| *a >> b.u32()},
                {|a: &U64, b: &U64| *a >> b.u32()},
                {|a: &U32, b: &U32| *a >> b.u32()},
                {|a: &U16, b: &U16| *a >> b.u32()},
                {|a: &U8, b: &U8| *a >> b.u32()}
            };
            loop_vv(inst, machine, vv_iterator_func)?;
        }
        insts::OP_VSRL_VX => {
            vx_iterator_impl! {
                {|a:&U1024, rhs: u64|  *a >> rhs as u32 },
                {|a:&U512, rhs: u64| *a >> rhs as u32 },
                {|a:&U256, rhs: u64| *a >> rhs as u32  },
                {|a:&U128, rhs: u64| *a >> rhs as u32 },
                {|a:&U64, rhs: u64| *a >> rhs as u32  },
                {|a:&U32, rhs: u64| *a >> rhs as u32  },
                {|a:&U16, rhs: u64| *a >> rhs as u32 },
                {|a:&U8, rhs: u64| *a >> rhs as u32 }
            };
            loop_vx(inst, machine, vx_iterator_func)?;
        }
        insts::OP_VSRL_VI => {
            vi2_iterator_impl! {
                {|a:&U1024, imm: u32| *a >> imm },
                {|a:&U512, imm: u32| *a >> imm },
                {|a:&U256, imm: u32| *a >> imm },
                {|a:&U128, imm: u32| *a >> imm  },
                {|a:&U64, imm: u32| *a >> imm },
                {|a:&U32, imm: u32| *a >> imm },
                {|a:&U16, imm: u32| *a >> imm },
                {|a:&U8, imm: u32| *a >> imm }
            };
            loop_vi2(inst, machine, vi2_iterator_func)?;
        }
        insts::OP_VSRA_VV => {
            vv_iterator_impl! {
                {|a: &U1024, b: &U1024| a.wrapping_sra(b.u32())},
                {|a: &U512, b: &U512| a.wrapping_sra(b.u32())},
                {|a: &U256, b: &U256| a.wrapping_sra(b.u32())},
                {|a: &U128, b: &U128| a.wrapping_sra(b.u32())},
                {|a: &U64, b: &U64| a.wrapping_sra(b.u32())},
                {|a: &U32, b: &U32| a.wrapping_sra(b.u32())},
                {|a: &U16, b: &U16| a.wrapping_sra(b.u32())},
                {|a: &U8, b: &U8| a.wrapping_sra(b.u32())}
            };
            loop_vv(inst, machine, vv_iterator_func)?;
        }
        insts::OP_VSRA_VX => {
            vx_iterator_impl! {
                {|a:&U1024, rhs: u64|   a.wrapping_sra(rhs as u32) },
                {|a:&U512, rhs: u64| a.wrapping_sra(rhs as u32) },
                {|a:&U256, rhs: u64| a.wrapping_sra(rhs as u32)  },
                {|a:&U128, rhs: u64| a.wrapping_sra(rhs as u32) },
                {|a:&U64, rhs: u64| a.wrapping_sra(rhs as u32)   },
                {|a:&U32, rhs: u64| a.wrapping_sra(rhs as u32)   },
                {|a:&U16, rhs: u64| a.wrapping_sra(rhs as u32)  },
                {|a:&U8, rhs: u64| a.wrapping_sra(rhs as u32)  }
            };
            loop_vx(inst, machine, vx_iterator_func)?;
        }
        insts::OP_VSRA_VI => {
            vi2_iterator_impl! {
                {|a:&U1024, imm: u32|  a.wrapping_sra(imm) },
                {|a:&U512, imm: u32| a.wrapping_sra(imm)},
                {|a:&U256, imm: u32| a.wrapping_sra(imm)  },
                {|a:&U128, imm: u32| a.wrapping_sra(imm) },
                {|a:&U64, imm: u32| a.wrapping_sra(imm) },
                {|a:&U32, imm: u32| a.wrapping_sra(imm)  },
                {|a:&U16, imm: u32| a.wrapping_sra(imm)  },
                {|a:&U8, imm: u32| a.wrapping_sra(imm) }
            };
            loop_vi2(inst, machine, vi2_iterator_func)?;
        }
        insts::OP_VMSEQ_VV => {
            vv_iterator_impl! {
                {|a: &U1024, b: &U1024| (a == b).into()},
                {|a: &U512, b: &U512| (a == b).into()},
                {|a: &U256, b: &U256| (a == b).into()},
                {|a: &U128, b: &U128| (a == b).into()},
                {|a: &U64, b: &U64| (a == b).into()},
                {|a: &U32, b: &U32| (a == b).into()},
                {|a: &U16, b: &U16| (a == b).into()},
                {|a: &U8, b: &U8| (a == b).into()}
            };
            loop_vv(inst, machine, vv_iterator_func)?;
        }
        insts::OP_VMSEQ_VX => {
            vx_iterator_impl! {
                {|a:&U1024, rhs: u64|  U1024::from(a == &U1024::vx(rhs)) },
                {|a:&U512, rhs: u64| U512::from(a == &U512::vx(rhs)) },
                {|a:&U256, rhs: u64| U256::from(a == &U256::vx(rhs))  },
                {|a:&U128, rhs: u64|  U128::from(a == &U128::vx(rhs)) },
                {|a:&U64, rhs: u64| U64::from(a == &U64::vx(rhs)) },
                {|a:&U32, rhs: u64| U32::from(a == &U32::vx(rhs))  },
                {|a:&U16, rhs: u64| U16::from(a == &U16::vx(rhs)) },
                {|a:&U8, rhs: u64| U8::from(a == &U8::vx(rhs)) }
            };
            loop_vx(inst, machine, vx_iterator_func)?;
        }
        insts::OP_VMSEQ_VI => {
            vi_iterator_impl! {
                {|a:&U1024, imm: i32| U1024::from(a == &U1024::vi(imm)) },
                {|a:&U512, imm: i32| U512::from(a == &U512::vi(imm)) },
                {|a:&U256, imm: i32| U256::from(a == &U256::vi(imm))},
                {|a:&U128, imm: i32| U128::from(a == &U128::vi(imm)) },
                {|a:&U64, imm: i32| U64::from(a == &U64::vi(imm)) },
                {|a:&U32, imm: i32| U32::from(a == &U32::vi(imm))  },
                {|a:&U16, imm: i32| U16::from(a == &U16::vi(imm)) },
                {|a:&U8, imm: i32| U8::from(a == &U8::vi(imm)) }
            };
            loop_vi(inst, machine, vi_iterator_func)?;
        }
        insts::OP_VMSNE_VV => {
            vv_iterator_impl! {
                {|a: &U1024, b: &U1024| (a != b).into()},
                {|a: &U512, b: &U512| (a != b).into()},
                {|a: &U256, b: &U256| (a != b).into()},
                {|a: &U128, b: &U128| (a != b).into()},
                {|a: &U64, b: &U64| (a != b).into()},
                {|a: &U32, b: &U32| (a != b).into()},
                {|a: &U16, b: &U16| (a != b).into()},
                {|a: &U8, b: &U8| (a != b).into()}
            };
            loop_vv(inst, machine, vv_iterator_func)?;
        }
        insts::OP_VMSNE_VX => {
            vx_iterator_impl! {
                {|a:&U1024, rhs: u64|  U1024::from(a != &U1024::vx(rhs))},
                {|a:&U512, rhs: u64| U512::from(a != &U512::vx(rhs)) },
                {|a:&U256, rhs: u64| U256::from(a != &U256::vx(rhs)) },
                {|a:&U128, rhs: u64| U128::from(a != &U128::vx(rhs)) },
                {|a:&U64, rhs: u64| U64::from(a != &U64::vx(rhs)) },
                {|a:&U32, rhs: u64| U32::from(a != &U32::vx(rhs)) },
                {|a:&U16, rhs: u64| U16::from(a != &U16::vx(rhs)) },
                {|a:&U8, rhs: u64| U8::from(a != &U8::vx(rhs)) }
            };
            loop_vx(inst, machine, vx_iterator_func)?;
        }
        insts::OP_VMSNE_VI => {
            vi_iterator_impl! {
                {|a:&U1024, imm: i32| U1024::from(a != &U1024::vi(imm)) },
                {|a:&U512, imm: i32| U512::from(a != &U512::vi(imm)) },
                {|a:&U256, imm: i32| U256::from(a != &U256::vi(imm))  },
                {|a:&U128, imm: i32| U128::from(a != &U128::vi(imm))  },
                {|a:&U64, imm: i32| U64::from(a != &U64::vi(imm)) },
                {|a:&U32, imm: i32| U32::from(a != &U32::vi(imm)) },
                {|a:&U16, imm: i32| U16::from(a != &U16::vi(imm)) },
                {|a:&U8, imm: i32| U8::from(a != &U8::vi(imm)) }
            };
            loop_vi(inst, machine, vi_iterator_func)?;
        }
        insts::OP_VMSLTU_VV => {
            vv_iterator_impl! {
                {|a: &U1024, b: &U1024| (a < b).into()},
                {|a: &U512, b: &U512| (a < b).into()},
                {|a: &U256, b: &U256| (a < b).into()},
                {|a: &U128, b: &U128| (a < b).into()},
                {|a: &U64, b: &U64| (a < b).into()},
                {|a: &U32, b: &U32| (a < b).into()},
                {|a: &U16, b: &U16| (a < b).into()},
                {|a: &U8, b: &U8| (a < b).into()}
            };
            loop_vv(inst, machine, vv_iterator_func)?;
        }
        insts::OP_VMSLTU_VX => {
            vx_iterator_impl! {
                {|a:&U1024, rhs: u64|  U1024::from(a < &U1024::vx(rhs)) },
                {|a:&U512, rhs: u64| U512::from(a < &U512::vx(rhs)) },
                {|a:&U256, rhs: u64| U256::from(a < &U256::vx(rhs))  },
                {|a:&U128, rhs: u64| U128::from(a < &U128::vx(rhs)) },
                {|a:&U64, rhs: u64| U64::from(a < &U64::vx(rhs))  },
                {|a:&U32, rhs: u64| U32::from(a < &U32::vx(rhs)) },
                {|a:&U16, rhs: u64| U16::from(a < &U16::vx(rhs)) },
                {|a:&U8, rhs: u64| U8::from(a < &U8::vx(rhs)) }
            };
            loop_vx(inst, machine, vx_iterator_func)?;
        }
        insts::OP_VMSLT_VV => {
            vv_iterator_impl! {
                {|a: &U1024, b: &U1024| if I1024::from(*a) < I1024::from(*b) {
                    U1024::ONE
                } else {
                    U1024::MIN
                }},
                {|a: &U512, b: &U512| if I512::from(*a) < I512::from(*b) {
                    U512::ONE
                } else {
                    U512::MIN
                }},
                {|a: &U256, b: &U256| if I256::from(*a) < I256::from(*b) {
                    U256::ONE
                } else {
                    U256::MIN
                }},
                {|a: &U128, b: &U128| if (a.0 as i128) < (b.0 as i128) {
                    U128::ONE
                } else {
                    U128::ZERO
                }},
                {|a: &U64, b: &U64| if (a.0 as i64) < (b.0 as i64) {
                    U64::ONE
                } else {
                    U64::ZERO
                }},
                {|a: &U32, b: &U32| if (a.0 as i32) < (b.0 as i32) {
                    U32::ONE
                } else {
                    U32::ZERO
                }},
                {|a: &U16, b: &U16| if (a.0 as i16) < (b.0 as i16) {
                    U16::ONE
                } else {
                    U16::ZERO
                }},
                {|a: &U8, b: &U8| if (a.0 as i8) < (b.0 as i8) {
                    U8::ONE
                } else {
                    U8::ZERO
                }}
            };
            loop_vv(inst, machine, vv_iterator_func)?;
        }
        insts::OP_VMSLT_VX => {
            vx_iterator_impl! {
                {|a:&U1024, rhs: u64|
                     if I1024::from(*a) < I1024::from(U1024::from(rhs as i64)) {
                        U1024::ONE
                    } else {
                        U1024::MIN
                    }
                },
                {|a:&U512, rhs: u64|
                    if I512::from(*a) < I512::from(U512::from(rhs as i64)) {
                            U512::ONE
                    } else {
                        U512::MIN
                    }
                },
                {|a:&U256, rhs: u64|
                    if I256::from(*a) < I256::from(U256::from(rhs as i64)) {
                        U256::ONE
                    } else {
                        U256::MIN
                    }
                },
                {|a:&U128, rhs: u64|
                    if (a.0 as i128) < (rhs as i64 as i128) {
                        U128::ONE
                    } else {
                        U128::ZERO
                    }
                },
                {|a:&U64, rhs: u64|
                    if (a.0 as i64) < (rhs as i64) {
                        U64::ONE
                    } else {
                        U64::ZERO
                    }
                },
                {|a:&U32, rhs: u64|
                    if (a.0 as i32) < (rhs as u32 as i32) {
                        U32::ONE
                    } else {
                        U32::ZERO
                    }
                },
                {|a:&U16, rhs: u64|
                    if (a.0 as i16) < (rhs as u16 as i16) {
                        U16::ONE
                    } else {
                        U16::ZERO
                    }
                },
                {|a:&U8, rhs: u64|
                    if (a.0 as i8) < (rhs as u8 as i8) {
                        U8::ONE
                    } else {
                        U8::ZERO
                    }
                }
            };
            loop_vx(inst, machine, vx_iterator_func)?;
        }
        insts::OP_VMSLEU_VV => {
            vv_iterator_impl! {
                {|a: &U1024, b: &U1024| (a <= b).into()},
                {|a: &U512, b: &U512| (a <= b).into()},
                {|a: &U256, b: &U256| (a <= b).into()},
                {|a: &U128, b: &U128| (a <= b).into()},
                {|a: &U64, b: &U64| (a <= b).into()},
                {|a: &U32, b: &U32| (a <= b).into()},
                {|a: &U16, b: &U16| (a <= b).into()},
                {|a: &U8, b: &U8| (a <= b).into()}
            };
            loop_vv(inst, machine, vv_iterator_func)?;
        }
        insts::OP_VMSLEU_VX => {
            vx_iterator_impl! {
                {|a:&U1024, rhs: u64|  U1024::from(a <= &U1024::vx(rhs)) },
                {|a:&U512, rhs: u64| U512::from(a <= &U512::vx(rhs)) },
                {|a:&U256, rhs: u64| U256::from(a <= &U256::vx(rhs))  },
                {|a:&U128, rhs: u64| U128::from(a <= &U128::vx(rhs)) },
                {|a:&U64, rhs: u64| U64::from(a <= &U64::vx(rhs))  },
                {|a:&U32, rhs: u64| U32::from(a <= &U32::vx(rhs)) },
                {|a:&U16, rhs: u64| U16::from(a <= &U16::vx(rhs)) },
                {|a:&U8, rhs: u64| U8::from(a <= &U8::vx(rhs)) }
            };
            loop_vx(inst, machine, vx_iterator_func)?;
        }
        insts::OP_VMSLEU_VI => {
            vi_iterator_impl! {
                {|a:&U1024, imm: i32| U1024::from(*a <= U1024::vi(imm)) },
                {|a:&U512, imm: i32|  U512::from(*a <= U512::vi(imm)) },
                {|a:&U256, imm: i32| U256::from(*a <= U256::vi(imm))  },
                {|a:&U128, imm: i32|  U128::from(*a <= U128::vi(imm))  },
                {|a:&U64, imm: i32|  U64::from(*a <= U64::vi(imm))  },
                {|a:&U32, imm: i32|  U32::from(*a <= U32::vi(imm))  },
                {|a:&U16, imm: i32|  U16::from(*a <= U16::vi(imm))  },
                {|a:&U8, imm: i32|  U8::from(*a <= U8::vi(imm)) }
            };
            loop_vi(inst, machine, vi_iterator_func)?;
        }
        insts::OP_VMSLE_VV => {
            vv_iterator_impl! {
                {|a: &U1024, b: &U1024| if I1024::from(*a) <= I1024::from(*b) {
                    U1024::ONE
                } else {
                    U1024::MIN
                }},
                {|a: &U512, b: &U512| if I512::from(*a) <= I512::from(*b) {
                    U512::ONE
                } else {
                    U512::MIN
                }},
                {|a: &U256, b: &U256| if I256::from(*a) <= I256::from(*b) {
                    U256::ONE
                } else {
                    U256::MIN
                }},
                {|a: &U128, b: &U128| if (a.0 as i128) <= (b.0 as i128) {
                    U128::ONE
                } else {
                    U128::ZERO
                }},
                {|a: &U64, b: &U64| if (a.0 as i64) <= (b.0 as i64) {
                    U64::ONE
                } else {
                    U64::ZERO
                }},
                {|a: &U32, b: &U32| if (a.0 as i32) <= (b.0 as i32) {
                    U32::ONE
                } else {
                    U32::ZERO
                }},
                {|a: &U16, b: &U16| if (a.0 as i16) <= (b.0 as i16) {
                    U16::ONE
                } else {
                    U16::ZERO
                }},
                {|a: &U8, b: &U8| if (a.0 as i8) <= (b.0 as i8) {
                    U8::ONE
                } else {
                    U8::ZERO
                }}
            };
            loop_vv(inst, machine, vv_iterator_func)?;
        }
        insts::OP_VMSLE_VX => {
            vx_iterator_impl! {
                {|a:&U1024, rhs: u64|
                    if I1024::from(*a) <= I1024::from(U1024::from(rhs as i64)) {
                        U1024::ONE
                    } else {
                        U1024::MIN
                    }
                },
                {|a:&U512, rhs: u64|
                    if I512::from(*a) <= I512::from(U512::from(rhs as i64)) {
                        U512::ONE
                    } else {
                        U512::MIN
                    }
                },
                {|a:&U256, rhs: u64|
                    if I256::from(*a) <= I256::from(U256::from(rhs as i64)) {
                        U256::ONE
                    } else {
                        U256::MIN
                    }
                },
                {|a:&U128, rhs: u64|
                    if (a.0 as i128) <= (rhs as i64 as i128) {
                        U128::ONE
                    } else {
                        U128::ZERO
                    }
                },
                {|a:&U64, rhs: u64|
                    if (a.0 as i64) <= (rhs as i64) {
                        U64::ONE
                    } else {
                        U64::ZERO
                    }
                },
                {|a:&U32, rhs: u64|
                    if (a.0 as i32) <= (rhs as u32 as i32) {
                        U32::ONE
                    } else {
                        U32::ZERO
                    }
                },
                {|a:&U16, rhs: u64|
                    if (a.0 as i16) <= (rhs as u16 as i16) {
                        U16::ONE
                    } else {
                        U16::ZERO
                    }
                },
                {|a:&U8, rhs: u64|
                    if (a.0 as i8) <= (rhs as u8 as i8) {
                        U8::ONE
                    } else {
                        U8::ZERO
                    }
                }
            };
            loop_vx(inst, machine, vx_iterator_func)?;
        }
        insts::OP_VMSLE_VI => {
            vi_iterator_impl! {
               {|a:&U1024, imm: i32|
                    if I1024::from(*a) <= I1024::from(U1024::from(imm)) {
                        U1024::ONE
                    } else {
                        U1024::MIN
                    }
                },
                {|a:&U512, imm: i32|
                    if I512::from(*a) <= I512::from(U512::from(imm)) {
                        U512::ONE
                    } else {
                        U512::MIN
                    }
                },
                {|a:&U256, imm: i32|
                    if I256::from(*a) <= I256::from(U256::from(imm)) {
                        U256::ONE
                    } else {
                        U256::MIN
                    }
                },
                {|a:&U128, imm: i32|
                    if a.0 as i128 <= imm as i128 {
                        U128::ONE
                    } else {
                        U128::ZERO
                    }
                },
                {|a:&U64, imm: i32|
                    if a.0 as i64 <= imm as i64 {
                        U64::ONE
                    } else {
                        U64::ZERO
                    }
                },
                {|a:&U32, imm: i32|
                    if a.0 as i32 <= imm {
                        U32::ONE
                    } else {
                        U32::ZERO
                    }
                },
                {|a:&U16, imm: i32|
                    if a.0 as i16 <= imm as i16 {
                        U16::ONE
                    } else {
                        U16::ZERO
                    }
                },
                {|a:&U8, imm: i32|
                    if a.0 as i8 <= imm as i8 {
                        U8::ONE
                    } else {
                        U8::ZERO
                    }
                }
            };
            loop_vi(inst, machine, vi_iterator_func)?;
        }
        insts::OP_VMSGTU_VX => {
            vx_iterator_impl! {
                {|a:&U1024, rhs: u64|  U1024::from(a > &U1024::vx(rhs)) },
                {|a:&U512, rhs: u64| U512::from(a > &U512::vx(rhs)) },
                {|a:&U256, rhs: u64| U256::from(a > &U256::vx(rhs)) },
                {|a:&U128, rhs: u64| U128::from(a > &U128::vx(rhs)) },
                {|a:&U64, rhs: u64| U64::from(a > &U64::vx(rhs))  },
                {|a:&U32, rhs: u64| U32::from(a > &U32::vx(rhs))  },
                {|a:&U16, rhs: u64| U16::from(a > &U16::vx(rhs)) },
                {|a:&U8, rhs: u64| U8::from(a > &U8::vx(rhs)) }
            };
            loop_vx(inst, machine, vx_iterator_func)?;
        }
        insts::OP_VMSGTU_VI => {
            vi_iterator_impl! {
                {|a:&U1024, imm: i32| U1024::from(a > &U1024::vi(imm)) },
                {|a:&U512, imm: i32| U512::from(a > &U512::vi(imm))},
                {|a:&U256, imm: i32| U256::from(a > &U256::vi(imm))  },
                {|a:&U128, imm: i32| U128::from(a > &U128::vi(imm))  },
                {|a:&U64, imm: i32| U64::from(a > &U64::vi(imm)) },
                {|a:&U32, imm: i32| U32::from(a > &U32::vi(imm)) },
                {|a:&U16, imm: i32| U16::from(a > &U16::vi(imm))  },
                {|a:&U8, imm: i32| U8::from(a > &U8::vi(imm)) }
            };
            loop_vi(inst, machine, vi_iterator_func)?;
        }
        insts::OP_VMSGT_VX => {
            vx_iterator_impl! {
                {|a:&U1024, rhs: u64|
                    if I1024::from(*a) > I1024::from(U1024::from(rhs as i64)) {
                        U1024::ONE
                    } else {
                        U1024::MIN
                    }
                },
                {|a:&U512, rhs: u64|
                    if I512::from(*a) > I512::from(U512::from(rhs as i64)) {
                        U512::ONE
                    } else {
                        U512::MIN
                    }
                },
                {|a:&U256, rhs: u64|
                    if I256::from(*a) > I256::from(U256::from(rhs as i64)) {
                        U256::ONE
                    } else {
                        U256::MIN
                    }
                },
                {|a:&U128, rhs: u64|
                    if a.0 as i128 > rhs as i64 as i128 {
                        U128::ONE
                    } else {
                        U128::ZERO
                    }
                },
                {|a:&U64, rhs: u64|
                    if a.0 as i64 > rhs as i64 {
                        U64::ONE
                    } else {
                        U64::ZERO
                    }
                },
                {|a:&U32, rhs: u64|
                    if a.0 as i32 > rhs as u32 as i32 {
                        U32::ONE
                    } else {
                        U32::ZERO
                    }
                },
                {|a:&U16, rhs: u64|
                    if a.0 as i16 > rhs as u16 as i16 {
                        U16::ONE
                    } else {
                        U16::ZERO
                    }
                },
                {|a:&U8, rhs: u64|
                     if a.0 as i8 > rhs as u8 as i8 {
                        U8::ONE
                    } else {
                        U8::ZERO
                    }
                }
            };
            loop_vx(inst, machine, vx_iterator_func)?;
        }
        insts::OP_VMSGT_VI => {
            vi_iterator_impl! {
                {|a:&U1024, imm: i32|
                    if I1024::from(*a) > I1024::from(U1024::from(imm)) {
                        U1024::ONE
                    } else {
                        U1024::MIN
                    }
                },
                {|a:&U512, imm: i32|
                    if I512::from(*a) > I512::from(U512::from(imm)) {
                        U512::ONE
                    } else {
                        U512::MIN
                    }
                },
                {|a:&U256, imm: i32|
                    if I256::from(*a) > I256::from(U256::from(imm)) {
                        U256::ONE
                    } else {
                        U256::MIN
                    }
                },
                {|a:&U128, imm: i32|
                    if a.0 as i128 > imm as i128 {
                        U128::ONE
                    } else {
                        U128::ZERO
                    }
                },
                {|a:&U64, imm: i32|
                    if a.0 as i64 > imm as i64 {
                        U64::ONE
                    } else {
                        U64::ZERO
                    }
                },
                {|a:&U32, imm: i32|
                    if a.0 as i32 > imm {
                        U32::ONE
                    } else {
                        U32::ZERO
                    }
                },
                {|a:&U16, imm: i32|
                    if a.0 as i16 > imm as i16 {
                        U16::ONE
                    } else {
                        U16::ZERO
                    }
                },
                {|a:&U8, imm: i32|
                    if a.0 as i8 > imm as i8 {
                        U8::ONE
                    } else {
                        U8::ZERO
                    }
                }
            };
            loop_vi(inst, machine, vi_iterator_func)?;
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
