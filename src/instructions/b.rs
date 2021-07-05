// RISC-V Bitmanip (Bit Manipulation) Extension
// See https://github.com/riscv/riscv-bitmanip/releases/download/1.0.0/bitmanip-1.0.0.pdf

use ckb_vm_definitions::instructions as insts;

use super::utils::{self, funct3, funct7, opcode, rd, rs1, rs2};
use super::{set_instruction_length_4, Instruction, Itype, Register, Rtype};

pub fn factory<R: Register>(instruction_bits: u32) -> Option<Instruction> {
    let bit_length = R::BITS;
    if bit_length != 32 && bit_length != 64 {
        return None;
    }
    let rv64 = bit_length == 64;
    let inst = match opcode(instruction_bits) {
        0b_0111011 => {
            let funct3_value = funct3(instruction_bits);
            let funct7_value = funct7(instruction_bits);
            let inst_opt = match (funct3_value, funct7_value) {
                (0b_000, 0b_0000100) => Some(insts::OP_ADDUW),
                (0b_001, 0b_0110000) => Some(insts::OP_ROLW),
                (0b_010, 0b_0010000) => Some(insts::OP_SH1ADDUW),
                (0b_100, 0b_0000100) => {
                    if rv64 && rs2(instruction_bits) == 0 {
                        Some(insts::OP_ZEXTH)
                    } else {
                        None
                    }
                }
                (0b_100, 0b_0010000) => Some(insts::OP_SH2ADDUW),
                (0b_101, 0b_0110000) => Some(insts::OP_RORW),
                (0b_110, 0b_0010000) => Some(insts::OP_SH3ADDUW),
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
        0b_0110011 => {
            let funct3_value = funct3(instruction_bits);
            let funct7_value = funct7(instruction_bits);
            let inst_opt = match (funct3_value, funct7_value) {
                (0b_111, 0b_0100000) => Some(insts::OP_ANDN),
                (0b_110, 0b_0100000) => Some(insts::OP_ORN),
                (0b_100, 0b_0100000) => Some(insts::OP_XNOR),
                (0b_001, 0b_0110000) => Some(insts::OP_ROL),
                (0b_101, 0b_0110000) => Some(insts::OP_ROR),
                (0b_001, 0b_0110100) => Some(insts::OP_BINV),
                (0b_001, 0b_0010100) => Some(insts::OP_BSET),
                (0b_001, 0b_0100100) => Some(insts::OP_BCLR),
                (0b_101, 0b_0100100) => Some(insts::OP_BEXT),
                (0b_010, 0b_0010000) => Some(insts::OP_SH1ADD),
                (0b_100, 0b_0010000) => Some(insts::OP_SH2ADD),
                (0b_110, 0b_0010000) => Some(insts::OP_SH3ADD),
                (0b_001, 0b_0000101) => Some(insts::OP_CLMUL),
                (0b_011, 0b_0000101) => Some(insts::OP_CLMULH),
                (0b_010, 0b_0000101) => Some(insts::OP_CLMULR),
                (0b_100, 0b_0000101) => Some(insts::OP_MIN),
                (0b_101, 0b_0000101) => Some(insts::OP_MINU),
                (0b_110, 0b_0000101) => Some(insts::OP_MAX),
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
        0b_0010011 => {
            let funct3_value = funct3(instruction_bits);
            let funct7_value = funct7(instruction_bits);
            let rs2_value = rs2(instruction_bits);
            let inst_opt = match (funct7_value, funct3_value, rs2_value) {
                (0b_0010100, 0b_101, 0b_00111) => Some(insts::OP_ORCB),
                (0b_0110101, 0b_101, 0b_11000) => Some(insts::OP_REV8),
                (0b_0110000, 0b_001, 0b_00000) => Some(insts::OP_CLZ),
                (0b_0110000, 0b_001, 0b_00010) => Some(insts::OP_CPOP),
                (0b_0110000, 0b_001, 0b_00001) => Some(insts::OP_CTZ),
                (0b_0110000, 0b_001, 0b_00100) => Some(insts::OP_SEXTB),
                (0b_0110000, 0b_001, 0b_00101) => Some(insts::OP_SEXTH),
                _ => None,
            };
            if let Some(inst) = inst_opt {
                Some(
                    Rtype::new(
                        inst,
                        rd(instruction_bits),
                        rs1(instruction_bits),
                        rs2(instruction_bits),
                    )
                    .0,
                )
            } else {
                let inst_opt = match (funct7_value >> 1, funct3_value) {
                    (0b_010010, 0b_001) => Some(insts::OP_BCLRI),
                    (0b_010010, 0b_101) => Some(insts::OP_BEXTI),
                    (0b_011010, 0b_001) => Some(insts::OP_BINVI),
                    (0b_001010, 0b_001) => Some(insts::OP_BSETI),
                    (0b_011000, 0b_101) => Some(insts::OP_RORI),
                    _ => None,
                };
                inst_opt.map(|inst| {
                    Itype::new(
                        inst,
                        rd(instruction_bits),
                        rs1(instruction_bits),
                        utils::x(instruction_bits, 20, 6, 0),
                    )
                    .0
                })
            }
        }
        0b_0011011 => {
            let funct3_value = funct3(instruction_bits);
            let funct7_value = funct7(instruction_bits);
            let rs2_value = rs2(instruction_bits);

            match funct7_value {
                0b_0110000 => match funct3_value {
                    0b_001 => {
                        let inst_opt = match rs2_value {
                            0b_00000 => Some(insts::OP_CLZW),
                            0b_00010 => Some(insts::OP_CPOPW),
                            0b_00001 => Some(insts::OP_CTZW),
                            _ => None,
                        };
                        inst_opt.map(|inst| {
                            Rtype::new(inst, rd(instruction_bits), rs1(instruction_bits), rs2_value)
                                .0
                        })
                    }
                    0b_101 => Some(
                        Itype::new(
                            insts::OP_RORIW,
                            rd(instruction_bits),
                            rs1(instruction_bits),
                            utils::x(instruction_bits, 20, 5, 0),
                        )
                        .0,
                    ),
                    _ => None,
                },
                _ => {
                    if funct7_value >> 1 == 0b_000010 && funct3_value == 0b_001 {
                        Some(
                            Itype::new(
                                insts::OP_SLLIUW,
                                rd(instruction_bits),
                                rs1(instruction_bits),
                                utils::x(instruction_bits, 20, 5, 0),
                            )
                            .0,
                        )
                    } else {
                        None
                    }
                }
            }
        }
        _ => None,
    };

    inst.map(set_instruction_length_4)
}
