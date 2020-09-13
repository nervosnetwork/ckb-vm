// RISC-V Bitmanip (Bit Manipulation) Extension
// See https://github.com/riscv/riscv-bitmanip/blob/master/bitmanip-0.92.pdf

use super::utils::{self, funct3, funct7, itype_immediate, opcode, rd, rs1, rs2, rs3};
use super::Register;
use super::{Instruction, Itype, R4type, Rtype};
use ckb_vm_definitions::instructions as insts;

pub fn factory<R: Register>(instruction_bits: u32, _: u32) -> Option<Instruction> {
    let bit_length = R::BITS;
    if bit_length != 32 && bit_length != 64 {
        return None;
    }
    let funct3_value = funct3(instruction_bits);
    match opcode(instruction_bits) {
        0b_0110011 => {
            let funct7_value = funct7(instruction_bits);
            let funct2_value = funct7_value & 0x3;

            match funct2_value {
                0b_00 => {
                    let inst_opt = match (funct3_value, funct7_value) {
                        (0b_111, 0b_0100000) => Some(insts::OP_ANDN),
                        (0b_110, 0b_0100000) => Some(insts::OP_ORN),
                        (0b_100, 0b_0100000) => Some(insts::OP_XNOR),
                        (0b_001, 0b_0010000) => Some(insts::OP_SLO),
                        (0b_101, 0b_0010000) => Some(insts::OP_SRO),
                        (0b_001, 0b_0110000) => Some(insts::OP_ROL),
                        (0b_101, 0b_0110000) => Some(insts::OP_ROR),
                        (0b_010, 0b_0010000) => Some(insts::OP_SH1ADD),
                        (0b_100, 0b_0010000) => Some(insts::OP_SH2ADD),
                        (0b_110, 0b_0010000) => Some(insts::OP_SH3ADD),
                        (0b_001, 0b_0100100) => Some(insts::OP_SBCLR),
                        (0b_001, 0b_0010100) => Some(insts::OP_SBSET),
                        (0b_001, 0b_0110100) => Some(insts::OP_SBINV),
                        (0b_101, 0b_0100100) => Some(insts::OP_SBEXT),
                        (0b_101, 0b_0010100) => Some(insts::OP_GORC),
                        (0b_101, 0b_0110100) => Some(insts::OP_GREV),
                        (0b_001, 0b_0000100) => Some(insts::OP_SHFL),
                        (0b_101, 0b_0000100) => Some(insts::OP_UNSHFL),
                        (0b_110, 0b_0100100) => Some(insts::OP_BDEP),
                        (0b_110, 0b_0000100) => Some(insts::OP_BEXT),
                        (0b_100, 0b_0000100) => Some(insts::OP_PACK),
                        (0b_100, 0b_0100100) => Some(insts::OP_PACKU),
                        (0b_011, 0b_0000100) => Some(insts::OP_BMATOR),
                        (0b_011, 0b_0100100) => Some(insts::OP_BMATXOR),
                        (0b_111, 0b_0000100) => Some(insts::OP_PACKH),
                        (0b_111, 0b_0100100) => Some(insts::OP_BFP),
                        _ => None,
                    };
                    inst_opt.map(|inst| {
                        Rtype::new(
                            inst,
                            rd(instruction_bits),
                            rs1(instruction_bits),
                            rs2(instruction_bits),
                        )
                        .0
                    })
                }
                0b_01 => {
                    let inst_opt = match (funct3_value, funct7_value) {
                        (0b_001, 0b_0000101) => Some(insts::OP_CLMUL),
                        (0b_010, 0b_0000101) => Some(insts::OP_CLMULR),
                        (0b_011, 0b_0000101) => Some(insts::OP_CLMULH),
                        (0b_100, 0b_0000101) => Some(insts::OP_MIN),
                        (0b_101, 0b_0000101) => Some(insts::OP_MAX),
                        (0b_110, 0b_0000101) => Some(insts::OP_MINU),
                        (0b_111, 0b_0000101) => Some(insts::OP_MAXU),
                        _ => None,
                    };
                    inst_opt.map(|inst| {
                        Rtype::new(
                            inst,
                            rd(instruction_bits),
                            rs1(instruction_bits),
                            rs2(instruction_bits),
                        )
                        .0
                    })
                }
                0b_10 => {
                    let inst_opt = match funct3_value {
                        0b_001 => Some(insts::OP_FSL),
                        0b_101 => Some(insts::OP_FSR),
                        _ => None,
                    };
                    inst_opt.map(|inst| {
                        R4type::new(
                            inst,
                            rd(instruction_bits),
                            rs1(instruction_bits),
                            rs2(instruction_bits),
                            rs3(instruction_bits),
                        )
                        .0
                    })
                }
                0b_11 => {
                    let inst_opt = match funct3_value {
                        0b_001 => Some(insts::OP_CMIX),
                        0b_101 => Some(insts::OP_CMOV),
                        _ => None,
                    };
                    inst_opt.map(|inst| {
                        R4type::new(
                            inst,
                            rd(instruction_bits),
                            rs1(instruction_bits),
                            rs2(instruction_bits),
                            rs3(instruction_bits),
                        )
                        .0
                    })
                }
                _ => None,
            }
        }
        0b_0010011 => {
            let top12_value = instruction_bits >> 20;
            let top7_value = top12_value >> 5;
            if top7_value == 0b_0110000 {
                let rs2_value = rs2(instruction_bits);
                let inst_opt = match (funct3_value, rs2_value) {
                    (0b_001, 0b_00000) => Some(insts::OP_CLZ),
                    (0b_001, 0b_00001) => Some(insts::OP_CTZ),
                    (0b_001, 0b_00010) => Some(insts::OP_PCNT),
                    (0b_001, 0b_00011) => Some(insts::OP_BMATFLIP),
                    (0b_001, 0b_00100) => Some(insts::OP_SEXTB),
                    (0b_001, 0b_00101) => Some(insts::OP_SEXTH),
                    (0b_001, 0b_10000) => Some(insts::OP_CRC32B),
                    (0b_001, 0b_10001) => Some(insts::OP_CRC32H),
                    (0b_001, 0b_10010) => Some(insts::OP_CRC32W),
                    (0b_001, 0b_10011) => Some(insts::OP_CRC32D),
                    (0b_001, 0b_11000) => Some(insts::OP_CRC32CB),
                    (0b_001, 0b_11001) => Some(insts::OP_CRC32CH),
                    (0b_001, 0b_11010) => Some(insts::OP_CRC32CW),
                    (0b_001, 0b_11011) => Some(insts::OP_CRC32CD),
                    _ => None,
                };
                if inst_opt.is_some() {
                    return inst_opt.map(|inst| {
                        Rtype::new(inst, rd(instruction_bits), rs1(instruction_bits), rs2_value).0
                    });
                }
            }
            let top6_value = top7_value >> 1;
            if top6_value == 0b_000010 {
                let inst_opt = match funct3_value {
                    0b_001 => Some(insts::OP_SHFLI),
                    0b_101 => Some(insts::OP_UNSHFLI),
                    _ => None,
                };
                return inst_opt.map(|inst| {
                    Itype::new_s(
                        inst,
                        rd(instruction_bits),
                        rs1(instruction_bits),
                        utils::xs(instruction_bits, 20, 6, 0) as i32,
                    )
                    .0
                });
            }
            if (instruction_bits >> 26) & 1 == 1 {
                Some(
                    Itype::new_s(
                        insts::OP_FSRI,
                        rd(instruction_bits),
                        rs1(instruction_bits),
                        itype_immediate(instruction_bits) & 0xfff,
                    )
                    .0,
                )
            } else {
                let top5_value = top6_value >> 1;
                let inst_opt = match (funct3_value, top5_value) {
                    (0b_001, 0b_00100) => Some(insts::OP_SLOI),
                    (0b_101, 0b_00100) => Some(insts::OP_SROI),
                    (0b_101, 0b_01100) => Some(insts::OP_RORI),
                    (0b_001, 0b_01001) => Some(insts::OP_SBCLRI),
                    (0b_001, 0b_00101) => Some(insts::OP_SBSETI),
                    (0b_001, 0b_01101) => Some(insts::OP_SBINVI),
                    (0b_101, 0b_01001) => Some(insts::OP_SBEXTI),
                    (0b_101, 0b_00101) => Some(insts::OP_GORCI),
                    (0b_101, 0b_01101) => Some(insts::OP_GREVI),
                    _ => None,
                };
                inst_opt.map(|inst| {
                    Itype::new_s(
                        inst,
                        rd(instruction_bits),
                        rs1(instruction_bits),
                        utils::xs(instruction_bits, 20, 7, 0) as i32,
                    )
                    .0
                })
            }
        }
        0b_0011011 => match funct3_value {
            0b_001 => {
                let funct7_value = funct7(instruction_bits);
                let top5_value = funct7_value >> 2;
                if top5_value == 0b_00001 {
                    return Some(
                        Itype::new_s(
                            insts::OP_SLLIUW,
                            rd(instruction_bits),
                            rs1(instruction_bits),
                            utils::xs(instruction_bits, 20, 7, 0) as i32,
                        )
                        .0,
                    );
                }
                if funct7_value == 0b_0110000 {
                    let inst_opt = match rs2(instruction_bits) {
                        0b_00000 => Some(insts::OP_CLZW),
                        0b_00001 => Some(insts::OP_CTZW),
                        0b_00010 => Some(insts::OP_PCNTW),
                        _ => None,
                    };
                    return inst_opt.map(|inst| {
                        Rtype::new(
                            inst,
                            rd(instruction_bits),
                            rs1(instruction_bits),
                            rs2(instruction_bits),
                        )
                        .0
                    });
                }
                let inst_opt = match funct7_value {
                    0b_0100100 => Some(insts::OP_SBCLRIW),
                    0b_0010100 => Some(insts::OP_SBSETIW),
                    0b_0110100 => Some(insts::OP_SBINVIW),
                    0b_0010000 => Some(insts::OP_SLOIW),
                    _ => None,
                };
                inst_opt.map(|inst| {
                    Itype::new(
                        inst,
                        rd(instruction_bits),
                        rs1(instruction_bits),
                        utils::xs(instruction_bits, 20, 5, 0),
                    )
                    .0
                })
            }
            0b_100 => Some(
                Itype::new_s(
                    insts::OP_ADDIWU,
                    rd(instruction_bits),
                    rs1(instruction_bits),
                    itype_immediate(instruction_bits),
                )
                .0,
            ),
            0b_101 => {
                let funct7_value = funct7(instruction_bits);
                if funct7_value & 0b_11 == 0b_10 {
                    Some(
                        R4type::new(
                            insts::OP_FSRIW,
                            rd(instruction_bits),
                            rs1(instruction_bits),
                            rs2(instruction_bits),
                            rs3(instruction_bits),
                        )
                        .0,
                    )
                } else {
                    let inst_opt = match funct7_value {
                        0b_0010000 => Some(insts::OP_SROIW),
                        0b_0110000 => Some(insts::OP_RORIW),
                        0b_0010100 => Some(insts::OP_GORCIW),
                        0b_0110100 => Some(insts::OP_GREVIW),
                        _ => None,
                    };
                    inst_opt.map(|inst| {
                        Itype::new_s(
                            inst,
                            rd(instruction_bits),
                            rs1(instruction_bits),
                            utils::xs(instruction_bits, 20, 5, 0) as i32,
                        )
                        .0
                    })
                }
            }
            _ => None,
        },
        0b_0111011 => {
            let funct7_value = funct7(instruction_bits);
            if funct7_value & 0b_11 == 0b_10 {
                let inst_opt = match funct3_value {
                    0b_001 => Some(insts::OP_FSLW),
                    0b_101 => Some(insts::OP_FSRW),
                    _ => None,
                };
                return inst_opt.map(|inst| {
                    R4type::new(
                        inst,
                        rd(instruction_bits),
                        rs1(instruction_bits),
                        rs2(instruction_bits),
                        rs3(instruction_bits),
                    )
                    .0
                });
            }
            let inst_opt = match (funct3_value, funct7_value) {
                (0b_000, 0b_0000101) => Some(insts::OP_ADDWU),
                (0b_000, 0b_0100101) => Some(insts::OP_SUBWU),
                (0b_000, 0b_0000100) => Some(insts::OP_ADDUW),
                (0b_000, 0b_0100100) => Some(insts::OP_SUBUW),
                (0b_001, 0b_0010000) => Some(insts::OP_SLOW),
                (0b_101, 0b_0010000) => Some(insts::OP_SROW),
                (0b_001, 0b_0110000) => Some(insts::OP_ROLW),
                (0b_101, 0b_0110000) => Some(insts::OP_RORW),
                (0b_010, 0b_0010000) => Some(insts::OP_SH1ADDUW),
                (0b_100, 0b_0010000) => Some(insts::OP_SH2ADDUW),
                (0b_110, 0b_0010000) => Some(insts::OP_SH3ADDUW),
                (0b_001, 0b_0100100) => Some(insts::OP_SBCLRW),
                (0b_001, 0b_0010100) => Some(insts::OP_SBSETW),
                (0b_001, 0b_0110100) => Some(insts::OP_SBINVW),
                (0b_101, 0b_0100100) => Some(insts::OP_SBEXTW),
                (0b_101, 0b_0010100) => Some(insts::OP_GORCW),
                (0b_101, 0b_0110100) => Some(insts::OP_GREVW),
                (0b_001, 0b_0000101) => Some(insts::OP_CLMULW),
                (0b_010, 0b_0000101) => Some(insts::OP_CLMULRW),
                (0b_011, 0b_0000101) => Some(insts::OP_CLMULHW),
                (0b_001, 0b_0000100) => Some(insts::OP_SHFLW),
                (0b_101, 0b_0000100) => Some(insts::OP_UNSHFLW),
                (0b_110, 0b_0100100) => Some(insts::OP_BDEPW),
                (0b_110, 0b_0000100) => Some(insts::OP_BEXTW),
                (0b_100, 0b_0000100) => Some(insts::OP_PACKW),
                (0b_100, 0b_0100100) => Some(insts::OP_PACKUW),
                (0b_111, 0b_0100100) => Some(insts::OP_BFPW),
                _ => None,
            };
            inst_opt.map(|inst| {
                Rtype::new(
                    inst,
                    rd(instruction_bits),
                    rs1(instruction_bits),
                    rs2(instruction_bits),
                )
                .0
            })
        }
        _ => None,
    }
}
