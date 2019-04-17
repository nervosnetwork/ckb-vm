use super::utils::{
    btype_immediate, funct3, funct7, itype_immediate, jtype_immediate, opcode, rd, rs1, rs2,
    stype_immediate, utype_immediate,
};
use super::Register;
use super::{blank_instruction, Instruction, Itype, Rtype, Stype, Utype};
use ckb_vm_definitions::instructions as insts;

// The FENCE instruction is used to order device I/O and memory accesses
// as viewed by other RISC- V harts and external devices or coprocessors.
#[derive(Debug, Clone, Copy)]
pub struct FenceType(Instruction);

impl FenceType {
    pub fn new(fm: u8, pred: u8, succ: u8) -> Self {
        FenceType(Rtype::new(insts::OP_FENCE, fm as usize, pred as usize, succ as usize).0)
    }

    pub fn fm(self) -> u8 {
        Rtype(self.0).rd() as u8
    }

    pub fn pred(self) -> u8 {
        Rtype(self.0).rs1() as u8
    }

    pub fn succ(self) -> u8 {
        Rtype(self.0).rs2() as u8
    }
}

pub fn factory<R: Register>(instruction_bits: u32) -> Option<Instruction> {
    let bit_length = R::BITS;
    if bit_length != 32 && bit_length != 64 {
        return None;
    }
    let rv64 = bit_length == 64;
    match opcode(instruction_bits) {
        0b_0110111 => Some(
            Utype::new_s(
                insts::OP_LUI,
                rd(instruction_bits),
                utype_immediate(instruction_bits),
            )
            .0,
        ),
        0b_0010111 => Some(
            Utype::new_s(
                insts::OP_AUIPC,
                rd(instruction_bits),
                utype_immediate(instruction_bits),
            )
            .0,
        ),
        0b_1101111 => Some(
            Utype::new_s(
                insts::OP_JAL,
                rd(instruction_bits),
                jtype_immediate(instruction_bits),
            )
            .0,
        ),
        0b_1100111 => {
            let inst_opt = match funct3(instruction_bits) {
                // I-type jump instructions
                0b_000 => Some(insts::OP_JALR),
                _ => None,
            };
            inst_opt.map(|inst| {
                Itype::new_s(
                    inst,
                    rd(instruction_bits),
                    rs1(instruction_bits),
                    itype_immediate(instruction_bits),
                )
                .0
            })
        }
        0b_0000011 => {
            let inst_opt = match funct3(instruction_bits) {
                // I-type load instructions
                0b_000 => Some(insts::OP_LB),
                0b_001 => Some(insts::OP_LH),
                0b_010 => Some(insts::OP_LW),
                0b_100 => Some(insts::OP_LBU),
                0b_101 => Some(insts::OP_LHU),
                0b_110 if rv64 => Some(insts::OP_LWU),
                0b_011 if rv64 => Some(insts::OP_LD),
                _ => None,
            };
            inst_opt.map(|inst| {
                Itype::new_s(
                    inst,
                    rd(instruction_bits),
                    rs1(instruction_bits),
                    itype_immediate(instruction_bits),
                )
                .0
            })
        }
        0b_0010011 => {
            let funct3_value = funct3(instruction_bits);
            let inst_opt = match funct3_value {
                // I-type ALU instructions
                0b_000 => Some(insts::OP_ADDI),
                0b_010 => Some(insts::OP_SLTI),
                0b_011 => Some(insts::OP_SLTIU),
                0b_100 => Some(insts::OP_XORI),
                0b_110 => Some(insts::OP_ORI),
                0b_111 => Some(insts::OP_ANDI),
                // I-type special ALU instructions
                0b_001 | 0b_101 => {
                    let top6_value = funct7(instruction_bits) >> 1;
                    let inst_opt = match (funct3_value, top6_value) {
                        (0b_001, 0b_000000) => Some(insts::OP_SLLI),
                        (0b_101, 0b_000000) => Some(insts::OP_SRLI),
                        (0b_101, 0b_010000) => Some(insts::OP_SRAI),
                        _ => None,
                    };
                    return inst_opt.map(|inst| {
                        Itype::new_s(
                            inst,
                            rd(instruction_bits),
                            rs1(instruction_bits),
                            itype_immediate(instruction_bits) & R::SHIFT_MASK as i32,
                        )
                        .0
                    });
                }
                _ => None,
            };

            inst_opt.map(|inst| {
                Itype::new_s(
                    inst,
                    rd(instruction_bits),
                    rs1(instruction_bits),
                    itype_immediate(instruction_bits),
                )
                .0
            })
        }
        0b_1100011 => {
            let inst_opt = match funct3(instruction_bits) {
                0b_000 => Some(insts::OP_BEQ),
                0b_001 => Some(insts::OP_BNE),
                0b_100 => Some(insts::OP_BLT),
                0b_101 => Some(insts::OP_BGE),
                0b_110 => Some(insts::OP_BLTU),
                0b_111 => Some(insts::OP_BGEU),
                _ => None,
            };
            inst_opt.map(|inst| {
                Stype::new_s(
                    inst,
                    btype_immediate(instruction_bits),
                    rs1(instruction_bits),
                    rs2(instruction_bits),
                )
                .0
            })
        }
        0b_0100011 => {
            let inst_opt = match funct3(instruction_bits) {
                0b_000 => Some(insts::OP_SB),
                0b_001 => Some(insts::OP_SH),
                0b_010 => Some(insts::OP_SW),
                0b_011 if rv64 => Some(insts::OP_SD),
                _ => None,
            };
            inst_opt.map(|inst| {
                Stype::new_s(
                    inst,
                    stype_immediate(instruction_bits),
                    rs1(instruction_bits),
                    rs2(instruction_bits),
                )
                .0
            })
        }
        0b_0110011 => {
            let inst_opt = match (funct3(instruction_bits), funct7(instruction_bits)) {
                (0b_000, 0b_0000000) => Some(insts::OP_ADD),
                (0b_000, 0b_0100000) => Some(insts::OP_SUB),
                (0b_001, 0b_0000000) => Some(insts::OP_SLL),
                (0b_010, 0b_0000000) => Some(insts::OP_SLT),
                (0b_011, 0b_0000000) => Some(insts::OP_SLTU),
                (0b_100, 0b_0000000) => Some(insts::OP_XOR),
                (0b_101, 0b_0000000) => Some(insts::OP_SRL),
                (0b_101, 0b_0100000) => Some(insts::OP_SRA),
                (0b_110, 0b_0000000) => Some(insts::OP_OR),
                (0b_111, 0b_0000000) => Some(insts::OP_AND),
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
        0b_0001111 => {
            const FENCE_LOW_BITS: u32 = 0b_00000_000_00000_0001111;
            const FENCEI_VALUE: u32 = 0b_0000_0000_0000_00000_001_00000_0001111;
            if instruction_bits == FENCEI_VALUE {
                Some(blank_instruction(insts::OP_FENCEI))
            } else if instruction_bits & 0x000_FFFFF == FENCE_LOW_BITS {
                Some(
                    FenceType::new(
                        ((instruction_bits & 0xF00_00000) >> 28) as u8,
                        ((instruction_bits & 0x0F0_00000) >> 24) as u8,
                        ((instruction_bits & 0x00F_00000) >> 20) as u8,
                    )
                    .0,
                )
            } else {
                None
            }
        }
        0b_1110011 => match instruction_bits {
            0b_000000000000_00000_000_00000_1110011 => Some(blank_instruction(insts::OP_ECALL)),
            0b_000000000001_00000_000_00000_1110011 => Some(blank_instruction(insts::OP_EBREAK)),
            _ => None,
        },
        0b_0011011 if rv64 => {
            let funct3_value = funct3(instruction_bits);
            match funct3_value {
                0b_000 => Some(
                    Itype::new_s(
                        insts::OP_ADDIW,
                        rd(instruction_bits),
                        rs1(instruction_bits),
                        itype_immediate(instruction_bits),
                    )
                    .0,
                ),
                0b_001 | 0b_101 => {
                    let funct7_value = funct7(instruction_bits);
                    let inst_opt = match (funct3_value, funct7_value) {
                        (0b_001, 0b_0000000) => Some(insts::OP_SLLIW),
                        (0b_101, 0b_0000000) => Some(insts::OP_SRLIW),
                        (0b_101, 0b_0100000) => Some(insts::OP_SRAIW),
                        _ => None,
                    };
                    inst_opt.map(|inst| {
                        Itype::new_s(
                            inst,
                            rd(instruction_bits),
                            rs1(instruction_bits),
                            itype_immediate(instruction_bits) & 0x1F,
                        )
                        .0
                    })
                }
                _ => None,
            }
        }
        0b_0111011 if rv64 => {
            let inst_opt = match (funct3(instruction_bits), funct7(instruction_bits)) {
                (0b_000, 0b_0000000) => Some(insts::OP_ADDW),
                (0b_000, 0b_0100000) => Some(insts::OP_SUBW),
                (0b_001, 0b_0000000) => Some(insts::OP_SLLW),
                (0b_101, 0b_0000000) => Some(insts::OP_SRLW),
                (0b_101, 0b_0100000) => Some(insts::OP_SRAW),
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

pub fn nop() -> Instruction {
    Itype::new(insts::OP_ADDI, 0, 0, 0).0
}
