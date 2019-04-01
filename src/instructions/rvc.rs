use super::super::machine::Machine;
use super::super::{Error, SP as CRATE_SP};
use super::register::Register;
use super::utils::{rd, update_register, x, xs};
use super::{
    assemble_no_argument_instruction, common, extract_opcode, Instruction, InstructionOp, Itype,
    Module, Rtype, Stype, Utype,
};

const SP: u8 = CRATE_SP as u8;

// Notice the location of rs2 in RVC encoding is different from full encoding
#[inline(always)]
fn c_rs2(instruction_bits: u32) -> u8 {
    x(instruction_bits, 2, 5, 0) as u8
}

// This function extract 3 bits from least_bit to form a register number,
// here since we are only using 3 bits, we can only reference the most popular
// used registers x8 - x15. In other words, a number of 0 extracted here means
// x8, 1 means x9, etc.
#[inline(always)]
fn compact_register_number(instruction_bits: u32, least_bit: usize) -> u8 {
    x(instruction_bits, least_bit, 3, 0) as u8 + 8
}

// [12]  => imm[5]
// [6:2] => imm[4:0]
fn immediate(instruction_bits: u32) -> i32 {
    (x(instruction_bits, 2, 5, 0) | xs(instruction_bits, 12, 1, 5)) as i32
}

// [12]  => imm[5]
// [6:2] => imm[4:0]
fn uimmediate(instruction_bits: u32) -> u32 {
    (x(instruction_bits, 2, 5, 0) | x(instruction_bits, 12, 1, 5))
}

// [12:2] => imm[11|4|9:8|10|6|7|3:1|5]
fn j_immediate(instruction_bits: u32) -> i32 {
    (x(instruction_bits, 3, 3, 1)
        | x(instruction_bits, 11, 1, 4)
        | x(instruction_bits, 2, 1, 5)
        | x(instruction_bits, 7, 1, 6)
        | x(instruction_bits, 6, 1, 7)
        | x(instruction_bits, 9, 2, 8)
        | x(instruction_bits, 8, 1, 10)
        | xs(instruction_bits, 12, 1, 11)) as i32
}

// [12:10] => uimm[5:3]
// [6:5]   => uimm[7:6]
fn fld_uimmediate(instruction_bits: u32) -> u32 {
    x(instruction_bits, 10, 3, 3) | x(instruction_bits, 5, 2, 6)
}

// [10:12] => uimm[5:3]
// [5:6]   => uimm[2|6]
fn sw_uimmediate(instruction_bits: u32) -> u32 {
    x(instruction_bits, 6, 1, 2) | x(instruction_bits, 10, 3, 3) | x(instruction_bits, 5, 1, 6)
}

// [12]  => uimm[5]
// [6:2] => uimm[4:2|7:6]
fn lwsp_uimmediate(instruction_bits: u32) -> u32 {
    x(instruction_bits, 4, 3, 2) | x(instruction_bits, 12, 1, 5) | x(instruction_bits, 2, 2, 6)
}

// [12]  => uimm[5]
// [6:2] => uimm[4:3|8:6]
fn fldsp_uimmediate(instruction_bits: u32) -> u32 {
    x(instruction_bits, 5, 2, 3) | x(instruction_bits, 12, 1, 5) | x(instruction_bits, 2, 3, 6)
}

// [12:7] => uimm[5:3|8:6]
fn fsdsp_uimmediate(instruction_bits: u32) -> u32 {
    x(instruction_bits, 10, 3, 3) | x(instruction_bits, 7, 3, 6)
}

// [12:7] => uimm[5:2|7:6]
fn swsp_uimmediate(instruction_bits: u32) -> u32 {
    x(instruction_bits, 9, 4, 2) | x(instruction_bits, 7, 2, 6)
}

// [12:10] => imm[8|4:3]
// [6:2]   => imm[7:6|2:1|5]
fn b_immediate(instruction_bits: u32) -> i32 {
    (x(instruction_bits, 3, 2, 1)
        | x(instruction_bits, 10, 2, 3)
        | x(instruction_bits, 2, 1, 5)
        | x(instruction_bits, 5, 2, 6)
        | xs(instruction_bits, 12, 1, 8)) as i32
}

pub fn execute<Mac: Machine>(inst: Instruction, machine: &mut Mac) -> Result<(), Error> {
    let op = extract_opcode(inst)?;
    let next_pc: Option<Mac::REG> = match op {
        InstructionOp::SUB => {
            let i = Rtype(inst);
            common::sub(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        InstructionOp::ADD => {
            let i = Rtype(inst);
            common::add(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        InstructionOp::XOR => {
            let i = Rtype(inst);
            common::xor(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        InstructionOp::OR => {
            let i = Rtype(inst);
            common::or(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        InstructionOp::AND => {
            let i = Rtype(inst);
            common::and(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        // > C.SUBW (RV64/128; RV32 RES)
        InstructionOp::SUBW => {
            let i = Rtype(inst);
            common::subw(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        // > C.ADDW (RV64/128; RV32 RES)
        InstructionOp::ADDW => {
            let i = Rtype(inst);
            common::addw(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        InstructionOp::ADDI => {
            let i = Itype(inst);
            common::addi(machine, i.rd(), i.rs1(), i.immediate_s());
            None
        }
        InstructionOp::ANDI => {
            let i = Itype(inst);
            common::andi(machine, i.rd(), i.rs1(), i.immediate_s());
            None
        }
        InstructionOp::ADDIW => {
            let i = Itype(inst);
            common::addiw(machine, i.rd(), i.rs1(), i.immediate_s());
            None
        }
        InstructionOp::SLLI => {
            let i = Itype(inst);
            common::slli(machine, i.rd(), i.rs1(), i.immediate());
            None
        }
        InstructionOp::SRLI => {
            let i = Itype(inst);
            common::srli(machine, i.rd(), i.rs1(), i.immediate());
            None
        }
        InstructionOp::SRAI => {
            let i = Itype(inst);
            common::srai(machine, i.rd(), i.rs1(), i.immediate());
            None
        }
        InstructionOp::LW => {
            let i = Itype(inst);
            common::lw(machine, i.rd(), i.rs1(), i.immediate_s())?;
            None
        }
        InstructionOp::LD => {
            let i = Itype(inst);
            common::ld(machine, i.rd(), i.rs1(), i.immediate_s())?;
            None
        }
        InstructionOp::SW => {
            let i = Stype(inst);
            common::sw(machine, i.rs1(), i.rs2(), i.immediate_s())?;
            None
        }
        InstructionOp::SD => {
            let i = Stype(inst);
            common::sd(machine, i.rs1(), i.rs2(), i.immediate_s())?;
            None
        }
        InstructionOp::LI => {
            let i = Utype(inst);
            update_register(machine, i.rd(), Mac::REG::from_i32(i.immediate_s()));
            None
        }
        InstructionOp::LUI => {
            let i = Utype(inst);
            update_register(machine, i.rd(), Mac::REG::from_i32(i.immediate_s()));
            None
        }
        InstructionOp::ADDI4SPN => {
            let i = Utype(inst);
            let value =
                machine.registers()[CRATE_SP].overflowing_add(&Mac::REG::from_u32(i.immediate()));
            update_register(machine, i.rd(), value);
            None
        }
        InstructionOp::LWSP => {
            let i = Utype(inst);
            common::lw(machine, i.rd(), SP, i.immediate_s())?;
            None
        }
        InstructionOp::LDSP => {
            let i = Utype(inst);
            common::ld(machine, i.rd(), SP, i.immediate_s())?;
            None
        }
        InstructionOp::SWSP => {
            let i = Stype(inst);
            common::sw(machine, SP, i.rs2(), i.immediate_s())?;
            None
        }
        InstructionOp::SDSP => {
            let i = Stype(inst);
            common::sd(machine, SP, i.rs2(), i.immediate_s())?;
            None
        }
        InstructionOp::BEQZ => {
            let i = Stype(inst);
            let condition = machine.registers()[i.rs1() as usize].eq(&Mac::REG::zero());
            let next_pc_offset = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()),
                &Mac::REG::from_usize(2),
            );
            Some(machine.pc().overflowing_add(&next_pc_offset))
        }
        InstructionOp::BNEZ => {
            let i = Stype(inst);
            let condition = machine.registers()[i.rs1() as usize]
                .eq(&Mac::REG::zero())
                .logical_not();
            let next_pc_offset = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()),
                &Mac::REG::from_usize(2),
            );
            Some(machine.pc().overflowing_add(&next_pc_offset))
        }
        InstructionOp::MV => {
            let i = Rtype(inst);
            let value = &machine.registers()[i.rs2() as usize];
            update_register(machine, i.rd(), value.clone());
            None
        }
        InstructionOp::JAL => {
            let i = Utype(inst);
            common::jal(machine, 1, i.immediate_s(), 2)
        }
        InstructionOp::J => {
            let i = Utype(inst);
            Some(
                machine
                    .pc()
                    .overflowing_add(&Mac::REG::from_i32(i.immediate_s())),
            )
        }
        InstructionOp::JR => {
            let i = Stype(inst);
            Some(machine.registers()[i.rs1() as usize].clone())
        }
        InstructionOp::JALR => {
            let i = Stype(inst);
            let link = machine.pc().overflowing_add(&Mac::REG::from_usize(2));
            update_register(machine, 1, link);
            Some(machine.registers()[i.rs1() as usize].clone())
        }
        InstructionOp::ADDI16SP => {
            let i = Itype(inst);
            let value =
                machine.registers()[CRATE_SP].overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
            update_register(machine, SP, value);
            None
        }
        InstructionOp::SRLI64 => None,
        InstructionOp::SRAI64 => None,
        InstructionOp::SLLI64 => None,
        InstructionOp::NOP => None,
        InstructionOp::EBREAK => {
            machine.ebreak()?;
            None
        }
        _ => return Err(Error::InvalidOp(op as u8)),
    };
    let default_next_pc = machine.pc().overflowing_add(&Mac::REG::from_usize(2));
    machine.set_pc(next_pc.unwrap_or(default_next_pc));
    Ok(())
}

#[allow(clippy::cyclomatic_complexity)]
pub fn factory<R: Register>(instruction_bits: u32) -> Option<Instruction> {
    let bit_length = R::BITS;
    if bit_length != 32 && bit_length != 64 {
        return None;
    }
    let rv32 = bit_length == 32;
    let rv64 = bit_length == 64;
    match instruction_bits & 0b_111_00000000000_11 {
        // == Quadrant 0
        0b_000_00000000000_00 => {
            let nzuimm = x(instruction_bits, 6, 1, 2)
                | x(instruction_bits, 5, 1, 3)
                | x(instruction_bits, 11, 2, 4)
                | x(instruction_bits, 7, 4, 6);
            if nzuimm != 0 {
                Some(
                    Utype::assemble(
                        InstructionOp::ADDI4SPN,
                        compact_register_number(instruction_bits, 2),
                        nzuimm,
                        Module::RVC,
                    )
                    .0,
                )
            } else {
                // Illegal instruction
                None
            }
        }
        0b_010_00000000000_00 => Some(
            Itype::assemble(
                InstructionOp::LW,
                compact_register_number(instruction_bits, 2),
                compact_register_number(instruction_bits, 7),
                sw_uimmediate(instruction_bits),
                Module::RVC,
            )
            .0,
        ),
        0b_011_00000000000_00 => {
            if rv32 {
                None
            } else {
                Some(
                    Itype::assemble(
                        InstructionOp::LD,
                        compact_register_number(instruction_bits, 2),
                        compact_register_number(instruction_bits, 7),
                        fld_uimmediate(instruction_bits),
                        Module::RVC,
                    )
                    .0,
                )
            }
        }
        // Reserved
        0b_100_00000000000_00 => None,
        0b_110_00000000000_00 => Some(
            Stype::assemble(
                InstructionOp::SW,
                sw_uimmediate(instruction_bits),
                compact_register_number(instruction_bits, 7),
                compact_register_number(instruction_bits, 2),
                Module::RVC,
            )
            .0,
        ),
        0b_111_00000000000_00 => {
            if rv32 {
                None
            } else {
                Some(
                    Stype::assemble(
                        InstructionOp::SD,
                        fld_uimmediate(instruction_bits),
                        compact_register_number(instruction_bits, 7),
                        compact_register_number(instruction_bits, 2),
                        Module::RVC,
                    )
                    .0,
                )
            }
        }
        // == Quadrant 1
        0b_000_00000000000_01 => {
            let nzimm = immediate(instruction_bits);
            let rd = rd(instruction_bits);
            if nzimm != 0 && rd != 0 {
                Some(Itype::assemble_s(InstructionOp::ADDI, rd, rd, nzimm, Module::RVC).0)
            } else if nzimm == 0 && rd == 0 {
                Some(assemble_no_argument_instruction(
                    InstructionOp::NOP,
                    Module::RVC,
                ))
            } else {
                // Invalid instruction
                None
            }
        }
        0b_001_00000000000_01 => {
            if rv32 {
                Some(
                    Utype::assemble_s(
                        InstructionOp::JAL,
                        0,
                        j_immediate(instruction_bits),
                        Module::RVC,
                    )
                    .0,
                )
            } else {
                let rd = rd(instruction_bits);
                if rd != 0 {
                    Some(
                        Itype::assemble_s(
                            InstructionOp::ADDIW,
                            rd,
                            rd,
                            immediate(instruction_bits),
                            Module::RVC,
                        )
                        .0,
                    )
                } else {
                    None
                }
            }
        }
        0b_010_00000000000_01 => {
            let rd = rd(instruction_bits);
            if rd != 0 {
                Some(
                    Utype::assemble_s(
                        InstructionOp::LI,
                        rd,
                        immediate(instruction_bits),
                        Module::RVC,
                    )
                    .0,
                )
            } else {
                None
            }
        }
        0b_011_00000000000_01 => {
            let imm = immediate(instruction_bits) << 12;
            if imm != 0 {
                let rd = rd(instruction_bits);
                if rd == SP {
                    Some(
                        Itype::assemble_s(
                            InstructionOp::ADDI16SP,
                            0,
                            0,
                            (x(instruction_bits, 6, 1, 4)
                                | x(instruction_bits, 2, 1, 5)
                                | x(instruction_bits, 5, 1, 6)
                                | x(instruction_bits, 3, 2, 7)
                                | xs(instruction_bits, 12, 1, 9))
                                as i32,
                            Module::RVC,
                        )
                        .0,
                    )
                } else if rd != 0 {
                    Some(Utype::assemble_s(InstructionOp::LUI, rd, imm, Module::RVC).0)
                } else {
                    None
                }
            } else {
                None
            }
        }
        0b_100_00000000000_01 => {
            let rd = compact_register_number(instruction_bits, 7);
            match instruction_bits & 0b_1_11_000_11000_00 {
                // SRLI64
                0b_0_00_000_00000_00 if instruction_bits & 0b_111_00 == 0 => Some(
                    assemble_no_argument_instruction(InstructionOp::SRLI64, Module::RVC),
                ),
                // SRAI64
                0b_0_01_000_00000_00 if instruction_bits & 0b_111_00 == 0 => Some(
                    assemble_no_argument_instruction(InstructionOp::SRAI64, Module::RVC),
                ),
                // SUB
                0b_0_11_000_00000_00 => Some(
                    Rtype::assemble(
                        InstructionOp::SUB,
                        rd,
                        rd,
                        compact_register_number(instruction_bits, 2),
                        Module::RVC,
                    )
                    .0,
                ),
                // XOR
                0b_0_11_000_01000_00 => Some(
                    Rtype::assemble(
                        InstructionOp::XOR,
                        rd,
                        rd,
                        compact_register_number(instruction_bits, 2),
                        Module::RVC,
                    )
                    .0,
                ),
                // OR
                0b_0_11_000_10000_00 => Some(
                    Rtype::assemble(
                        InstructionOp::OR,
                        rd,
                        rd,
                        compact_register_number(instruction_bits, 2),
                        Module::RVC,
                    )
                    .0,
                ),
                // AND
                0b_0_11_000_11000_00 => Some(
                    Rtype::assemble(
                        InstructionOp::AND,
                        rd,
                        rd,
                        compact_register_number(instruction_bits, 2),
                        Module::RVC,
                    )
                    .0,
                ),
                // SUBW
                0b_1_11_000_00000_00 if rv64 => Some(
                    Rtype::assemble(
                        InstructionOp::SUBW,
                        rd,
                        rd,
                        compact_register_number(instruction_bits, 2),
                        Module::RVC,
                    )
                    .0,
                ),
                // ADDW
                0b_1_11_000_01000_00 if rv64 => Some(
                    Rtype::assemble(
                        InstructionOp::ADDW,
                        rd,
                        rd,
                        compact_register_number(instruction_bits, 2),
                        Module::RVC,
                    )
                    .0,
                ),
                // Reserved
                0b_1_11_000_10000_00 => None,
                // Reserved
                0b_1_11_000_11000_00 => None,
                _ => {
                    let uimm = uimmediate(instruction_bits);
                    match (instruction_bits & 0b_11_000_00000_00, uimm) {
                        // Invalid instruction
                        (0b_00_000_00000_00, 0) => None,
                        // SRLI
                        (0b_00_000_00000_00, uimm) => {
                            Some(Itype::assemble(InstructionOp::SRLI, rd, rd, uimm, Module::RVC).0)
                        }
                        // Invalid instruction
                        (0b_01_000_00000_00, 0) => None,
                        // SRAI
                        (0b_01_000_00000_00, uimm) => {
                            Some(Itype::assemble(InstructionOp::SRAI, rd, rd, uimm, Module::RVC).0)
                        }
                        // ANDI
                        (0b_10_000_00000_00, _) => Some(
                            Itype::assemble_s(
                                InstructionOp::ANDI,
                                rd,
                                rd,
                                immediate(instruction_bits),
                                Module::RVC,
                            )
                            .0,
                        ),
                        _ => None,
                    }
                }
            }
        }
        0b_101_00000000000_01 => Some(
            Utype::assemble_s(
                InstructionOp::J,
                0,
                j_immediate(instruction_bits),
                Module::RVC,
            )
            .0,
        ),
        0b_110_00000000000_01 => Some(
            Stype::assemble_s(
                InstructionOp::BEQZ,
                b_immediate(instruction_bits),
                compact_register_number(instruction_bits, 7),
                0,
                Module::RVC,
            )
            .0,
        ),
        0b_111_00000000000_01 => Some(
            Stype::assemble_s(
                InstructionOp::BNEZ,
                b_immediate(instruction_bits),
                compact_register_number(instruction_bits, 7),
                0,
                Module::RVC,
            )
            .0,
        ),
        // == Quadrant 2
        0b_000_00000000000_10 => {
            let uimm = uimmediate(instruction_bits);
            let rd = rd(instruction_bits);
            if rd == 0 {
                // Reserved
                None
            } else if uimm != 0 {
                Some(Itype::assemble(InstructionOp::SLLI, rd, rd, uimm, Module::RVC).0)
            } else {
                Some(assemble_no_argument_instruction(
                    InstructionOp::SLLI64,
                    Module::RVC,
                ))
            }
        }
        0b_010_00000000000_10 => {
            let rd = rd(instruction_bits);
            if rd != 0 {
                Some(
                    Utype::assemble(
                        InstructionOp::LWSP,
                        rd,
                        lwsp_uimmediate(instruction_bits),
                        Module::RVC,
                    )
                    .0,
                )
            } else {
                // Reserved
                None
            }
        }
        0b_011_00000000000_10 => {
            if rv32 {
                None
            } else {
                let rd = rd(instruction_bits);
                if rd != 0 {
                    Some(
                        Utype::assemble(
                            InstructionOp::LDSP,
                            rd,
                            fldsp_uimmediate(instruction_bits),
                            Module::RVC,
                        )
                        .0,
                    )
                } else {
                    // Reserved
                    None
                }
            }
        }
        0b_100_00000000000_10 => {
            match instruction_bits & 0b_1_00000_00000_00 {
                0b_0_00000_00000_00 => {
                    let rd = rd(instruction_bits);
                    let rs2 = c_rs2(instruction_bits);
                    if rd == 0 {
                        None
                    } else if rs2 == 0 {
                        Some(Stype::assemble(InstructionOp::JR, 0, rd, 0, Module::RVC).0)
                    } else {
                        Some(Rtype::assemble(InstructionOp::MV, rd, 0, rs2, Module::RVC).0)
                    }
                }
                0b_1_00000_00000_00 => {
                    let rd = rd(instruction_bits);
                    let rs2 = c_rs2(instruction_bits);
                    match (rd, rs2) {
                        (0, 0) => Some(assemble_no_argument_instruction(
                            InstructionOp::EBREAK,
                            Module::RVC,
                        )),
                        (rs1, 0) => {
                            Some(Stype::assemble(InstructionOp::JALR, 0, rs1, 0, Module::RVC).0)
                        }
                        (rd, rs2) if rd != 0 => {
                            Some(Rtype::assemble(InstructionOp::ADD, rd, rd, rs2, Module::RVC).0)
                        }
                        // Invalid instruction
                        _ => None,
                    }
                }
                _ => unreachable!(),
            }
        }
        0b_110_00000000000_10 => Some(
            Stype::assemble(
                InstructionOp::SWSP,
                swsp_uimmediate(instruction_bits),
                0,
                c_rs2(instruction_bits),
                Module::RVC,
            )
            .0,
        ),
        0b_111_00000000000_10 => {
            if rv32 {
                None
            } else {
                Some(
                    Stype::assemble(
                        InstructionOp::SDSP,
                        fsdsp_uimmediate(instruction_bits),
                        0,
                        c_rs2(instruction_bits),
                        Module::RVC,
                    )
                    .0,
                )
            }
        }
        _ => None,
    }
}
