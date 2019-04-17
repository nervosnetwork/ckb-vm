use super::super::registers::SP;
use super::register::Register;
use super::utils::{rd, x, xs};
use super::{blank_instruction, Instruction, Itype, Rtype, Stype, Utype};
use ckb_vm_definitions::instructions as insts;

// Notice the location of rs2 in RVC encoding is different from full encoding
#[inline(always)]
fn c_rs2(instruction_bits: u32) -> usize {
    x(instruction_bits, 2, 5, 0) as usize
}

// This function extract 3 bits from least_bit to form a register number,
// here since we are only using 3 bits, we can only reference the most popular
// used registers x8 - x15. In other words, a number of 0 extracted here means
// x8, 1 means x9, etc.
#[inline(always)]
fn compact_register_number(instruction_bits: u32, least_bit: usize) -> usize {
    x(instruction_bits, least_bit, 3, 0) as usize + 8
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
                    Utype::new(
                        insts::OP_RVC_ADDI4SPN,
                        compact_register_number(instruction_bits, 2),
                        nzuimm,
                    )
                    .0,
                )
            } else {
                // Illegal instruction
                None
            }
        }
        0b_010_00000000000_00 => Some(
            Itype::new(
                insts::OP_RVC_LW,
                compact_register_number(instruction_bits, 2),
                compact_register_number(instruction_bits, 7),
                sw_uimmediate(instruction_bits),
            )
            .0,
        ),
        0b_011_00000000000_00 => {
            if rv32 {
                None
            } else {
                Some(
                    Itype::new(
                        insts::OP_RVC_LD,
                        compact_register_number(instruction_bits, 2),
                        compact_register_number(instruction_bits, 7),
                        fld_uimmediate(instruction_bits),
                    )
                    .0,
                )
            }
        }
        // Reserved
        0b_100_00000000000_00 => None,
        0b_110_00000000000_00 => Some(
            Stype::new(
                insts::OP_RVC_SW,
                sw_uimmediate(instruction_bits),
                compact_register_number(instruction_bits, 7),
                compact_register_number(instruction_bits, 2),
            )
            .0,
        ),
        0b_111_00000000000_00 => {
            if rv32 {
                None
            } else {
                Some(
                    Stype::new(
                        insts::OP_RVC_SD,
                        fld_uimmediate(instruction_bits),
                        compact_register_number(instruction_bits, 7),
                        compact_register_number(instruction_bits, 2),
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
                Some(Itype::new_s(insts::OP_RVC_ADDI, rd, rd, nzimm).0)
            } else if nzimm == 0 && rd == 0 {
                Some(blank_instruction(insts::OP_RVC_NOP))
            } else {
                // Invalid instruction
                None
            }
        }
        0b_001_00000000000_01 => {
            if rv32 {
                Some(Utype::new_s(insts::OP_RVC_JAL, 0, j_immediate(instruction_bits)).0)
            } else {
                let rd = rd(instruction_bits);
                if rd != 0 {
                    Some(Itype::new_s(insts::OP_RVC_ADDIW, rd, rd, immediate(instruction_bits)).0)
                } else {
                    None
                }
            }
        }
        0b_010_00000000000_01 => {
            let rd = rd(instruction_bits);
            if rd != 0 {
                Some(Utype::new_s(insts::OP_RVC_LI, rd, immediate(instruction_bits)).0)
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
                        Itype::new_s(
                            insts::OP_RVC_ADDI16SP,
                            0,
                            0,
                            (x(instruction_bits, 6, 1, 4)
                                | x(instruction_bits, 2, 1, 5)
                                | x(instruction_bits, 5, 1, 6)
                                | x(instruction_bits, 3, 2, 7)
                                | xs(instruction_bits, 12, 1, 9))
                                as i32,
                        )
                        .0,
                    )
                } else if rd != 0 {
                    Some(Utype::new_s(insts::OP_RVC_LUI, rd, imm).0)
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
                0b_0_00_000_00000_00 if instruction_bits & 0b_111_00 == 0 => {
                    Some(blank_instruction(insts::OP_RVC_SRLI64))
                }
                // SRAI64
                0b_0_01_000_00000_00 if instruction_bits & 0b_111_00 == 0 => {
                    Some(blank_instruction(insts::OP_RVC_SRAI64))
                }
                // SUB
                0b_0_11_000_00000_00 => Some(
                    Rtype::new(
                        insts::OP_RVC_SUB,
                        rd,
                        rd,
                        compact_register_number(instruction_bits, 2),
                    )
                    .0,
                ),
                // XOR
                0b_0_11_000_01000_00 => Some(
                    Rtype::new(
                        insts::OP_RVC_XOR,
                        rd,
                        rd,
                        compact_register_number(instruction_bits, 2),
                    )
                    .0,
                ),
                // OR
                0b_0_11_000_10000_00 => Some(
                    Rtype::new(
                        insts::OP_RVC_OR,
                        rd,
                        rd,
                        compact_register_number(instruction_bits, 2),
                    )
                    .0,
                ),
                // AND
                0b_0_11_000_11000_00 => Some(
                    Rtype::new(
                        insts::OP_RVC_AND,
                        rd,
                        rd,
                        compact_register_number(instruction_bits, 2),
                    )
                    .0,
                ),
                // SUBW
                0b_1_11_000_00000_00 if rv64 => Some(
                    Rtype::new(
                        insts::OP_RVC_SUBW,
                        rd,
                        rd,
                        compact_register_number(instruction_bits, 2),
                    )
                    .0,
                ),
                // ADDW
                0b_1_11_000_01000_00 if rv64 => Some(
                    Rtype::new(
                        insts::OP_RVC_ADDW,
                        rd,
                        rd,
                        compact_register_number(instruction_bits, 2),
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
                            Some(Itype::new(insts::OP_RVC_SRLI, rd, rd, uimm).0)
                        }
                        // Invalid instruction
                        (0b_01_000_00000_00, 0) => None,
                        // SRAI
                        (0b_01_000_00000_00, uimm) => {
                            Some(Itype::new(insts::OP_RVC_SRAI, rd, rd, uimm).0)
                        }
                        // ANDI
                        (0b_10_000_00000_00, _) => Some(
                            Itype::new_s(insts::OP_RVC_ANDI, rd, rd, immediate(instruction_bits)).0,
                        ),
                        _ => None,
                    }
                }
            }
        }
        0b_101_00000000000_01 => {
            Some(Utype::new_s(insts::OP_RVC_J, 0, j_immediate(instruction_bits)).0)
        }
        0b_110_00000000000_01 => Some(
            Stype::new_s(
                insts::OP_RVC_BEQZ,
                b_immediate(instruction_bits),
                compact_register_number(instruction_bits, 7),
                0,
            )
            .0,
        ),
        0b_111_00000000000_01 => Some(
            Stype::new_s(
                insts::OP_RVC_BNEZ,
                b_immediate(instruction_bits),
                compact_register_number(instruction_bits, 7),
                0,
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
                Some(Itype::new(insts::OP_RVC_SLLI, rd, rd, uimm).0)
            } else {
                Some(blank_instruction(insts::OP_RVC_SLLI64))
            }
        }
        0b_010_00000000000_10 => {
            let rd = rd(instruction_bits);
            if rd != 0 {
                Some(Utype::new(insts::OP_RVC_LWSP, rd, lwsp_uimmediate(instruction_bits)).0)
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
                    Some(Utype::new(insts::OP_RVC_LDSP, rd, fldsp_uimmediate(instruction_bits)).0)
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
                        Some(Stype::new(insts::OP_RVC_JR, 0, rd, 0).0)
                    } else {
                        Some(Rtype::new(insts::OP_RVC_MV, rd, 0, rs2).0)
                    }
                }
                0b_1_00000_00000_00 => {
                    let rd = rd(instruction_bits);
                    let rs2 = c_rs2(instruction_bits);
                    match (rd, rs2) {
                        (0, 0) => Some(blank_instruction(insts::OP_RVC_EBREAK)),
                        (rs1, 0) => Some(Stype::new(insts::OP_RVC_JALR, 0, rs1, 0).0),
                        (rd, rs2) if rd != 0 => Some(Rtype::new(insts::OP_RVC_ADD, rd, rd, rs2).0),
                        // Invalid instruction
                        _ => None,
                    }
                }
                _ => unreachable!(),
            }
        }
        0b_110_00000000000_10 => Some(
            Stype::new(
                insts::OP_RVC_SWSP,
                swsp_uimmediate(instruction_bits),
                0,
                c_rs2(instruction_bits),
            )
            .0,
        ),
        0b_111_00000000000_10 => {
            if rv32 {
                None
            } else {
                Some(
                    Stype::new(
                        insts::OP_RVC_SDSP,
                        fsdsp_uimmediate(instruction_bits),
                        0,
                        c_rs2(instruction_bits),
                    )
                    .0,
                )
            }
        }
        _ => None,
    }
}
