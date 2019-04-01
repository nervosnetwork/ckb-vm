use super::super::machine::Machine;
use super::super::Error;
use super::utils::{
    btype_immediate, funct3, funct7, itype_immediate, jtype_immediate, opcode, rd, rs1, rs2,
    stype_immediate, update_register, utype_immediate,
};
use super::Register;
use super::{
    assemble_no_argument_instruction, common, extract_opcode, Instruction, InstructionOp, Itype,
    Module, Rtype, Stype, Utype,
};

// The FENCE instruction is used to order device I/O and memory accesses
// as viewed by other RISC- V harts and external devices or coprocessors.
#[derive(Debug, Clone, Copy)]
pub struct FenceType(Instruction);

impl FenceType {
    pub fn assemble(fm: u8, pred: u8, succ: u8) -> Self {
        FenceType(Rtype::assemble(InstructionOp::FENCE, fm, pred, succ, Module::I).0)
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
    let op = extract_opcode(inst)?;
    let next_pc: Option<Mac::REG> = match op {
        InstructionOp::SUB => {
            let i = Rtype(inst);
            common::sub(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        InstructionOp::SUBW => {
            let i = Rtype(inst);
            common::subw(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        InstructionOp::ADD => {
            let i = Rtype(inst);
            common::add(machine, i.rd(), i.rs1(), i.rs2());
            None
        }
        InstructionOp::ADDW => {
            let i = Rtype(inst);
            common::addw(machine, i.rd(), i.rs1(), i.rs2());
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
        InstructionOp::SLL => {
            let i = Rtype(inst);
            let shift_value = machine.registers()[i.rs2() as usize].clone()
                & Mac::REG::from_usize(Mac::REG::SHIFT_MASK);
            let value = machine.registers()[i.rs1() as usize].clone() << shift_value;
            update_register(machine, i.rd(), value);
            None
        }
        InstructionOp::SLLW => {
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
        InstructionOp::SRL => {
            let i = Rtype(inst);
            let shift_value = machine.registers()[i.rs2() as usize].clone()
                & Mac::REG::from_usize(Mac::REG::SHIFT_MASK);
            let value = machine.registers()[i.rs1() as usize].clone() >> shift_value;
            update_register(machine, i.rd(), value);
            None
        }
        InstructionOp::SRLW => {
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
        InstructionOp::SRA => {
            let i = Rtype(inst);
            let shift_value = machine.registers()[i.rs2() as usize].clone()
                & Mac::REG::from_usize(Mac::REG::SHIFT_MASK);
            let value = machine.registers()[i.rs1() as usize].signed_shr(&shift_value);
            update_register(machine, i.rd(), value);
            None
        }
        InstructionOp::SRAW => {
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
        InstructionOp::SLT => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1() as usize];
            let rs2_value = &machine.registers()[i.rs2() as usize];
            let value = rs1_value.lt_s(&rs2_value);
            update_register(machine, i.rd(), value);
            None
        }
        InstructionOp::SLTU => {
            let i = Rtype(inst);
            let rs1_value = &machine.registers()[i.rs1() as usize];
            let rs2_value = &machine.registers()[i.rs2() as usize];
            let value = rs1_value.lt(&rs2_value);
            update_register(machine, i.rd(), value);
            None
        }
        InstructionOp::LB => {
            let i = Itype(inst);
            common::lb(machine, i.rd(), i.rs1(), i.immediate_s())?;
            None
        }
        InstructionOp::LH => {
            let i = Itype(inst);
            common::lh(machine, i.rd(), i.rs1(), i.immediate_s())?;
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
        InstructionOp::LBU => {
            let i = Itype(inst);
            common::lbu(machine, i.rd(), i.rs1(), i.immediate_s())?;
            None
        }
        InstructionOp::LHU => {
            let i = Itype(inst);
            common::lhu(machine, i.rd(), i.rs1(), i.immediate_s())?;
            None
        }
        InstructionOp::LWU => {
            let i = Itype(inst);
            common::lwu(machine, i.rd(), i.rs1(), i.immediate_s())?;
            None
        }
        InstructionOp::ADDI => {
            let i = Itype(inst);
            common::addi(machine, i.rd(), i.rs1(), i.immediate_s());
            None
        }
        InstructionOp::ADDIW => {
            let i = Itype(inst);
            common::addiw(machine, i.rd(), i.rs1(), i.immediate_s());
            None
        }
        InstructionOp::XORI => {
            let i = Itype(inst);
            common::xori(machine, i.rd(), i.rs1(), i.immediate_s());
            None
        }
        InstructionOp::ORI => {
            let i = Itype(inst);
            common::ori(machine, i.rd(), i.rs1(), i.immediate_s());
            None
        }
        InstructionOp::ANDI => {
            let i = Itype(inst);
            common::andi(machine, i.rd(), i.rs1(), i.immediate_s());
            None
        }
        InstructionOp::SLTI => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1() as usize];
            let imm_value = Mac::REG::from_i32(i.immediate_s());
            let value = rs1_value.lt_s(&imm_value);
            update_register(machine, i.rd(), value);
            None
        }
        InstructionOp::SLTIU => {
            let i = Itype(inst);
            let rs1_value = &machine.registers()[i.rs1() as usize];
            let imm_value = Mac::REG::from_i32(i.immediate_s());
            let value = rs1_value.lt(&imm_value);
            update_register(machine, i.rd(), value);
            None
        }
        InstructionOp::JALR => {
            let i = Itype(inst);
            let link = machine.pc().overflowing_add(&Mac::REG::from_usize(4));
            let mut next_pc = machine.registers()[i.rs1() as usize]
                .overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
            next_pc = next_pc & (!Mac::REG::one());
            update_register(machine, i.rd(), link);
            Some(next_pc)
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
        InstructionOp::SLLIW => {
            let i = Itype(inst);
            common::slliw(machine, i.rd(), i.rs1(), i.immediate());
            None
        }
        InstructionOp::SRLIW => {
            let i = Itype(inst);
            common::srliw(machine, i.rd(), i.rs1(), i.immediate());
            None
        }
        InstructionOp::SRAIW => {
            let i = Itype(inst);
            common::sraiw(machine, i.rd(), i.rs1(), i.immediate());
            None
        }
        InstructionOp::SB => {
            let i = Stype(inst);
            common::sb(machine, i.rs1(), i.rs2(), i.immediate_s())?;
            None
        }
        InstructionOp::SH => {
            let i = Stype(inst);
            common::sh(machine, i.rs1(), i.rs2(), i.immediate_s())?;
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
        InstructionOp::BEQ => {
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
        InstructionOp::BNE => {
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
        InstructionOp::BLT => {
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
        InstructionOp::BGE => {
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
        InstructionOp::BLTU => {
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
        InstructionOp::BGEU => {
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
        InstructionOp::LUI => {
            let i = Utype(inst);
            update_register(machine, i.rd(), Mac::REG::from_i32(i.immediate_s()));
            None
        }
        InstructionOp::AUIPC => {
            let i = Utype(inst);
            let value = machine
                .pc()
                .overflowing_add(&Mac::REG::from_i32(i.immediate_s()));
            update_register(machine, i.rd(), value);
            None
        }
        InstructionOp::ECALL => {
            // The semantic of ECALL is determined by the hardware, which
            // is not part of the spec, hence here the implementation is
            // deferred to the machine. This way custom ECALLs might be
            // provided for different environments.
            machine.ecall()?;
            None
        }
        InstructionOp::EBREAK => {
            machine.ebreak()?;
            None
        }
        InstructionOp::FENCEI => None,
        InstructionOp::FENCE => None,
        InstructionOp::JAL => {
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
                InstructionOp::LUI,
                rd(instruction_bits),
                utype_immediate(instruction_bits),
                Module::I,
            )
            .0,
        ),
        0b_0010111 => Some(
            Utype::assemble_s(
                InstructionOp::AUIPC,
                rd(instruction_bits),
                utype_immediate(instruction_bits),
                Module::I,
            )
            .0,
        ),
        0b_1101111 => Some(
            Utype::assemble_s(
                InstructionOp::JAL,
                rd(instruction_bits),
                jtype_immediate(instruction_bits),
                Module::I,
            )
            .0,
        ),
        0b_1100111 => {
            let inst_opt = match funct3(instruction_bits) {
                // I-type jump instructions
                0b_000 => Some(InstructionOp::JALR),
                _ => None,
            };
            inst_opt.map(|inst| {
                Itype::assemble_s(
                    inst,
                    rd(instruction_bits),
                    rs1(instruction_bits),
                    itype_immediate(instruction_bits),
                    Module::I,
                )
                .0
            })
        }
        0b_0000011 => {
            let inst_opt = match funct3(instruction_bits) {
                // I-type load instructions
                0b_000 => Some(InstructionOp::LB),
                0b_001 => Some(InstructionOp::LH),
                0b_010 => Some(InstructionOp::LW),
                0b_100 => Some(InstructionOp::LBU),
                0b_101 => Some(InstructionOp::LHU),
                0b_110 if rv64 => Some(InstructionOp::LWU),
                0b_011 if rv64 => Some(InstructionOp::LD),
                _ => None,
            };
            inst_opt.map(|inst| {
                Itype::assemble_s(
                    inst,
                    rd(instruction_bits),
                    rs1(instruction_bits),
                    itype_immediate(instruction_bits),
                    Module::I,
                )
                .0
            })
        }
        0b_0010011 => {
            let funct3_value = funct3(instruction_bits);
            let inst_opt = match funct3_value {
                // I-type ALU instructions
                0b_000 => Some(InstructionOp::ADDI),
                0b_010 => Some(InstructionOp::SLTI),
                0b_011 => Some(InstructionOp::SLTIU),
                0b_100 => Some(InstructionOp::XORI),
                0b_110 => Some(InstructionOp::ORI),
                0b_111 => Some(InstructionOp::ANDI),
                // I-type special ALU instructions
                0b_001 | 0b_101 => {
                    let top6_value = funct7(instruction_bits) >> 1;
                    let inst_opt = match (funct3_value, top6_value) {
                        (0b_001, 0b_000000) => Some(InstructionOp::SLLI),
                        (0b_101, 0b_000000) => Some(InstructionOp::SRLI),
                        (0b_101, 0b_010000) => Some(InstructionOp::SRAI),
                        _ => None,
                    };
                    return inst_opt.map(|inst| {
                        Itype::assemble_s(
                            inst,
                            rd(instruction_bits),
                            rs1(instruction_bits),
                            itype_immediate(instruction_bits) & R::SHIFT_MASK as i32,
                            Module::I,
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
                    Module::I,
                )
                .0
            })
        }
        0b_1100011 => {
            let inst_opt = match funct3(instruction_bits) {
                0b_000 => Some(InstructionOp::BEQ),
                0b_001 => Some(InstructionOp::BNE),
                0b_100 => Some(InstructionOp::BLT),
                0b_101 => Some(InstructionOp::BGE),
                0b_110 => Some(InstructionOp::BLTU),
                0b_111 => Some(InstructionOp::BGEU),
                _ => None,
            };
            inst_opt.map(|inst| {
                Stype::assemble_s(
                    inst,
                    btype_immediate(instruction_bits),
                    rs1(instruction_bits),
                    rs2(instruction_bits),
                    Module::I,
                )
                .0
            })
        }
        0b_0100011 => {
            let inst_opt = match funct3(instruction_bits) {
                0b_000 => Some(InstructionOp::SB),
                0b_001 => Some(InstructionOp::SH),
                0b_010 => Some(InstructionOp::SW),
                0b_011 if rv64 => Some(InstructionOp::SD),
                _ => None,
            };
            inst_opt.map(|inst| {
                Stype::assemble_s(
                    inst,
                    stype_immediate(instruction_bits),
                    rs1(instruction_bits),
                    rs2(instruction_bits),
                    Module::I,
                )
                .0
            })
        }
        0b_0110011 => {
            let inst_opt = match (funct3(instruction_bits), funct7(instruction_bits)) {
                (0b_000, 0b_0000000) => Some(InstructionOp::ADD),
                (0b_000, 0b_0100000) => Some(InstructionOp::SUB),
                (0b_001, 0b_0000000) => Some(InstructionOp::SLL),
                (0b_010, 0b_0000000) => Some(InstructionOp::SLT),
                (0b_011, 0b_0000000) => Some(InstructionOp::SLTU),
                (0b_100, 0b_0000000) => Some(InstructionOp::XOR),
                (0b_101, 0b_0000000) => Some(InstructionOp::SRL),
                (0b_101, 0b_0100000) => Some(InstructionOp::SRA),
                (0b_110, 0b_0000000) => Some(InstructionOp::OR),
                (0b_111, 0b_0000000) => Some(InstructionOp::AND),
                _ => None,
            };
            inst_opt.map(|inst| {
                Rtype::assemble(
                    inst,
                    rd(instruction_bits),
                    rs1(instruction_bits),
                    rs2(instruction_bits),
                    Module::I,
                )
                .0
            })
        }
        0b_0001111 => {
            const FENCE_LOW_BITS: u32 = 0b_00000_000_00000_0001111;
            const FENCEI_VALUE: u32 = 0b_0000_0000_0000_00000_001_00000_0001111;
            if instruction_bits == FENCEI_VALUE {
                Some(assemble_no_argument_instruction(
                    InstructionOp::FENCEI,
                    Module::I,
                ))
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
            0b_000000000000_00000_000_00000_1110011 => Some(assemble_no_argument_instruction(
                InstructionOp::ECALL,
                Module::I,
            )),
            0b_000000000001_00000_000_00000_1110011 => Some(assemble_no_argument_instruction(
                InstructionOp::EBREAK,
                Module::I,
            )),
            _ => None,
        },
        0b_0011011 if rv64 => {
            let funct3_value = funct3(instruction_bits);
            match funct3_value {
                0b_000 => Some(
                    Itype::assemble_s(
                        InstructionOp::ADDIW,
                        rd(instruction_bits),
                        rs1(instruction_bits),
                        itype_immediate(instruction_bits),
                        Module::I,
                    )
                    .0,
                ),
                0b_001 | 0b_101 => {
                    let funct7_value = funct7(instruction_bits);
                    let inst_opt = match (funct3_value, funct7_value) {
                        (0b_001, 0b_0000000) => Some(InstructionOp::SLLIW),
                        (0b_101, 0b_0000000) => Some(InstructionOp::SRLIW),
                        (0b_101, 0b_0100000) => Some(InstructionOp::SRAIW),
                        _ => None,
                    };
                    inst_opt.map(|inst| {
                        Itype::assemble_s(
                            inst,
                            rd(instruction_bits),
                            rs1(instruction_bits),
                            itype_immediate(instruction_bits) & 0x1F,
                            Module::I,
                        )
                        .0
                    })
                }
                _ => None,
            }
        }
        0b_0111011 if rv64 => {
            let inst_opt = match (funct3(instruction_bits), funct7(instruction_bits)) {
                (0b_000, 0b_0000000) => Some(InstructionOp::ADDW),
                (0b_000, 0b_0100000) => Some(InstructionOp::SUBW),
                (0b_001, 0b_0000000) => Some(InstructionOp::SLLW),
                (0b_101, 0b_0000000) => Some(InstructionOp::SRLW),
                (0b_101, 0b_0100000) => Some(InstructionOp::SRAW),
                _ => None,
            };
            inst_opt.map(|inst| {
                Rtype::assemble(
                    inst,
                    rd(instruction_bits),
                    rs1(instruction_bits),
                    rs2(instruction_bits),
                    Module::I,
                )
                .0
            })
        }
        _ => None,
    }
}

pub fn nop() -> Instruction {
    Itype::assemble(InstructionOp::ADDI, 0, 0, 0, Module::I).0
}
