use super::super::machine::Machine;
use super::super::Error;
use super::utils::{
    btype_immediate, funct3, funct7, itype_immediate, jtype_immediate, opcode, rd, rs1, rs2,
    stype_immediate, update_register, utype_immediate,
};
use super::Register;
use super::{
    assemble_no_argument_instruction, common, extract_opcode, Instruction, Itype, Rtype, Stype,
    Utype, MODULE_I,
};
use crate::instructions as insts;

// The FENCE instruction is used to order device I/O and memory accesses
// as viewed by other RISC- V harts and external devices or coprocessors.
#[derive(Debug, Clone, Copy)]
pub struct FenceType(Instruction);

impl FenceType {
    pub fn assemble(fm: u8, pred: u8, succ: u8) -> Self {
        FenceType(Rtype::assemble(insts::OP_FENCE, fm, pred, succ, MODULE_I).0)
    }

    pub fn fm(self) -> u8 {
        Rtype(self.0).rd()
    }

    pub fn pred(self) -> u8 {
        Rtype(self.0).rs1()
    }

    pub fn succ(self) -> u8 {
        Rtype(self.0).rs2()
    }
}

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
            let shift_value = machine.registers()[i.rs2() as usize].clone()
                & Mac::REG::from_usize(Mac::REG::SHIFT_MASK);
            let value = machine.registers()[i.rs1() as usize].clone() << shift_value;
            update_register(machine, i.rd(), value);
            None
        }
        insts::OP_SLLW => {
            let i = Rtype(inst);
            let shift_value =
                machine.registers()[i.rs2() as usize].clone() & Mac::REG::from_usize(0x1F);
            let value = machine.registers()[i.rs1() as usize].clone() << shift_value;
            update_register(
                machine,
                i.rd(),
                value.sign_extend(&Mac::REG::from_usize(32)),
            );
            None
        }
        insts::OP_SRL => {
            let i = Rtype(inst);
            let shift_value = machine.registers()[i.rs2() as usize].clone()
                & Mac::REG::from_usize(Mac::REG::SHIFT_MASK);
            let value = machine.registers()[i.rs1() as usize].clone() >> shift_value;
            update_register(machine, i.rd(), value);
            None
        }
        insts::OP_SRLW => {
            let i = Rtype(inst);
            let shift_value =
                machine.registers()[i.rs2() as usize].clone() & Mac::REG::from_usize(0x1F);
            let value = machine.registers()[i.rs1() as usize]
                .zero_extend(&Mac::REG::from_usize(32))
                >> shift_value;
            update_register(
                machine,
                i.rd(),
                value.sign_extend(&Mac::REG::from_usize(32)),
            );
            None
        }
        insts::OP_SRA => {
            let i = Rtype(inst);
            let shift_value = machine.registers()[i.rs2() as usize].clone()
                & Mac::REG::from_usize(Mac::REG::SHIFT_MASK);
            let value = machine.registers()[i.rs1() as usize].signed_shr(&shift_value);
            update_register(machine, i.rd(), value);
            None
        }
        insts::OP_SRAW => {
            let i = Rtype(inst);
            let shift_value =
                machine.registers()[i.rs2() as usize].clone() & Mac::REG::from_usize(0x1F);
            let value = machine.registers()[i.rs1() as usize]
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
            let rs1_value = &machine.registers()[i.rs1() as usize];
            let rs2_value = &machine.registers()[i.rs2() as usize];
            let value = rs1_value.lt_s(&rs2_value);
            update_register(machine, i.rd(), value);
            None
        }
        insts::OP_SLTU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1() as usize];
            let rs2_value = &machine.registers()[i.rs2() as usize];
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
            let rs1_value = &machine.registers()[i.rs1() as usize];
            let imm_value = Mac::REG::from_i32(i.immediate_s());
            let value = rs1_value.lt_s(&imm_value);
            update_register(machine, i.rd(), value);
            None
        }
        insts::OP_SLTIU => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1() as usize];
            let imm_value = Mac::REG::from_i32(i.immediate_s());
            let value = rs1_value.lt(&imm_value);
            update_register(machine, i.rd(), value);
            None
        }
        insts::OP_JALR => {
            let i = Itype(inst);
            let link = machine.pc().overflowing_add(&Mac::REG::from_usize(4));
            let mut next_pc = machine.registers()[i.rs1() as usize]
                .overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
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
            let rs1_value = &machine.registers()[i.rs1() as usize];
            let rs2_value = &machine.registers()[i.rs2() as usize];
            let condition = rs1_value.eq(&rs2_value);
            let offset = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()),
                &Mac::REG::from_usize(4),
            );
            Some(machine.pc().overflowing_add(&offset))
        }
        insts::OP_BNE => {
            let i = Stype(inst);
            let rs1_value = &machine.registers()[i.rs1() as usize];
            let rs2_value = &machine.registers()[i.rs2() as usize];
            let condition = rs1_value.ne(&rs2_value);
            let offset = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()),
                &Mac::REG::from_usize(4),
            );
            Some(machine.pc().overflowing_add(&offset))
        }
        insts::OP_BLT => {
            let i = Stype(inst);
            let rs1_value = &machine.registers()[i.rs1() as usize];
            let rs2_value = &machine.registers()[i.rs2() as usize];
            let condition = rs1_value.lt_s(&rs2_value);
            let offset = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()),
                &Mac::REG::from_usize(4),
            );
            Some(machine.pc().overflowing_add(&offset))
        }
        insts::OP_BGE => {
            let i = Stype(inst);
            let rs1_value = &machine.registers()[i.rs1() as usize];
            let rs2_value = &machine.registers()[i.rs2() as usize];
            let condition = rs1_value.ge_s(&rs2_value);
            let offset = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()),
                &Mac::REG::from_usize(4),
            );
            Some(machine.pc().overflowing_add(&offset))
        }
        insts::OP_BLTU => {
            let i = Stype(inst);
            let rs1_value = &machine.registers()[i.rs1() as usize];
            let rs2_value = &machine.registers()[i.rs2() as usize];
            let condition = rs1_value.lt(&rs2_value);
            let offset = condition.cond(
                &Mac::REG::from_i32(i.immediate_s()),
                &Mac::REG::from_usize(4),
            );
            Some(machine.pc().overflowing_add(&offset))
        }
        insts::OP_BGEU => {
            let i = Stype(inst);
            let rs1_value = &machine.registers()[i.rs1() as usize];
            let rs2_value = &machine.registers()[i.rs2() as usize];
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
        _ => return Err(Error::InvalidOp(op as u8)),
    };
    let default_next_pc = machine.pc().overflowing_add(&Mac::REG::from_usize(4));
    machine.set_pc(next_pc.unwrap_or(default_next_pc));
    Ok(())
}

pub fn factory<R: Register>(instruction_bits: u32) -> Option<Instruction> {
    let bit_length = R::BITS;
    if bit_length != 32 && bit_length != 64 {
        return None;
    }
    let rv64 = bit_length == 64;
    match opcode(instruction_bits) {
        0b_0110111 => Some(
            Utype::assemble_s(
                insts::OP_LUI,
                rd(instruction_bits),
                utype_immediate(instruction_bits),
                MODULE_I,
            )
            .0,
        ),
        0b_0010111 => Some(
            Utype::assemble_s(
                insts::OP_AUIPC,
                rd(instruction_bits),
                utype_immediate(instruction_bits),
                MODULE_I,
            )
            .0,
        ),
        0b_1101111 => Some(
            Utype::assemble_s(
                insts::OP_JAL,
                rd(instruction_bits),
                jtype_immediate(instruction_bits),
                MODULE_I,
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
                Itype::assemble_s(
                    inst,
                    rd(instruction_bits),
                    rs1(instruction_bits),
                    itype_immediate(instruction_bits),
                    MODULE_I,
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
                Itype::assemble_s(
                    inst,
                    rd(instruction_bits),
                    rs1(instruction_bits),
                    itype_immediate(instruction_bits),
                    MODULE_I,
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
                        Itype::assemble_s(
                            inst,
                            rd(instruction_bits),
                            rs1(instruction_bits),
                            itype_immediate(instruction_bits) & R::SHIFT_MASK as i32,
                            MODULE_I,
                        )
                        .0
                    });
                }
                _ => None,
            };

            inst_opt.map(|inst| {
                Itype::assemble_s(
                    inst,
                    rd(instruction_bits),
                    rs1(instruction_bits),
                    itype_immediate(instruction_bits),
                    MODULE_I,
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
                Stype::assemble_s(
                    inst,
                    btype_immediate(instruction_bits),
                    rs1(instruction_bits),
                    rs2(instruction_bits),
                    MODULE_I,
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
                Stype::assemble_s(
                    inst,
                    stype_immediate(instruction_bits),
                    rs1(instruction_bits),
                    rs2(instruction_bits),
                    MODULE_I,
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
                Rtype::assemble(
                    inst,
                    rd(instruction_bits),
                    rs1(instruction_bits),
                    rs2(instruction_bits),
                    MODULE_I,
                )
                .0
            })
        }
        0b_0001111 => {
            const FENCE_LOW_BITS: u32 = 0b_00000_000_00000_0001111;
            const FENCEI_VALUE: u32 = 0b_0000_0000_0000_00000_001_00000_0001111;
            if instruction_bits == FENCEI_VALUE {
                Some(assemble_no_argument_instruction(insts::OP_FENCEI, MODULE_I))
            } else if instruction_bits & 0x000_FFFFF == FENCE_LOW_BITS {
                Some(
                    FenceType::assemble(
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
            0b_000000000000_00000_000_00000_1110011 => {
                Some(assemble_no_argument_instruction(insts::OP_ECALL, MODULE_I))
            }
            0b_000000000001_00000_000_00000_1110011 => {
                Some(assemble_no_argument_instruction(insts::OP_EBREAK, MODULE_I))
            }
            _ => None,
        },
        0b_0011011 if rv64 => {
            let funct3_value = funct3(instruction_bits);
            match funct3_value {
                0b_000 => Some(
                    Itype::assemble_s(
                        insts::OP_ADDIW,
                        rd(instruction_bits),
                        rs1(instruction_bits),
                        itype_immediate(instruction_bits),
                        MODULE_I,
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
                        Itype::assemble_s(
                            inst,
                            rd(instruction_bits),
                            rs1(instruction_bits),
                            itype_immediate(instruction_bits) & 0x1F,
                            MODULE_I,
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
                Rtype::assemble(
                    inst,
                    rd(instruction_bits),
                    rs1(instruction_bits),
                    rs2(instruction_bits),
                    MODULE_I,
                )
                .0
            })
        }
        _ => None,
    }
}

pub fn nop() -> Instruction {
    Itype::assemble(insts::OP_ADDI, 0, 0, 0, MODULE_I).0
}
