use super::super::machine::Machine;
use super::super::Error;
use super::utils::{
    btype_immediate, funct3, funct7, itype_immediate, jtype_immediate, opcode, rd, rs1, rs2,
    stype_immediate, update_register, utype_immediate,
};
use super::Register;
use super::{
    common, Execute, Immediate, Instruction as GenericInstruction, Instruction::I, RegisterIndex,
    UImmediate, UShortImmediate,
};

#[derive(Debug, Clone)]
pub enum RtypeInstruction {
    ADD,
    ADDW,
    SUB,
    SUBW,
    SLL,
    SLLW,
    SRL,
    SRLW,
    SRA,
    SRAW,
    SLT,
    SLTU,
    XOR,
    OR,
    AND,
}

#[derive(Debug, Clone)]
pub enum ItypeInstruction {
    JALR,
    LB,
    LH,
    LW,
    LD,
    LBU,
    LHU,
    LWU,
    ADDI,
    ADDIW,
    SLTI,
    SLTIU,
    XORI,
    ORI,
    ANDI,
}

#[derive(Debug, Clone)]
pub enum ItypeShiftInstruction {
    SLLI,
    SRLI,
    SRAI,
    SLLIW,
    SRLIW,
    SRAIW,
}

#[derive(Debug, Clone)]
pub enum StypeInstruction {
    SB,
    SH,
    SW,
    SD,
}

#[derive(Debug, Clone)]
pub enum BtypeInstruction {
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
}

#[derive(Debug, Clone)]
pub enum UtypeInstruction {
    LUI,
    AUIPC,
}

#[derive(Debug, Clone)]
pub enum EnvInstruction {
    ECALL,
    EBREAK,
}

#[derive(Debug, Clone)]
pub enum CsrInstruction {
    CSRRW,
    CSRRS,
    CSRRC,
}

#[derive(Debug, Clone)]
pub enum CsrIInstruction {
    CSRRWI,
    CSRRSI,
    CSRRCI,
}

type Rtype = super::Rtype<RtypeInstruction>;
type Itype = super::Itype<Immediate, ItypeInstruction>;
type Stype = super::Stype<Immediate, StypeInstruction>;
type Btype = super::Btype<Immediate, BtypeInstruction>;
type Utype = super::Utype<Immediate, UtypeInstruction>;
type ItypeShift = super::ItypeShift<Immediate, ItypeShiftInstruction>;

// The FENCE instruction is used to order device I/O and memory accesses
// as viewed by other RISC- V harts and external devices or coprocessors.
#[derive(Debug, Clone)]
pub struct FenceType {
    fm: u8,
    // predecessor
    pred: u8,
    // successor
    succ: u8,
}

#[derive(Debug, Clone)]
pub struct CsrType {
    csr: UImmediate,
    rs1: RegisterIndex,
    rd: RegisterIndex,
    inst: CsrInstruction,
}

#[derive(Debug, Clone)]
pub struct CsrIType {
    csr: UShortImmediate,
    zimm: UShortImmediate,
    rd: RegisterIndex,
    inst: CsrIInstruction,
}

impl Execute for Rtype {
    fn execute<Mac: Machine>(&self, machine: &mut Mac) -> Result<Option<Mac::REG>, Error> {
        match &self.inst {
            RtypeInstruction::SUB => common::sub(machine, self.rd, self.rs1, self.rs2),
            RtypeInstruction::SUBW => common::subw(machine, self.rd, self.rs1, self.rs2),
            RtypeInstruction::ADD => common::add(machine, self.rd, self.rs1, self.rs2),
            RtypeInstruction::ADDW => common::addw(machine, self.rd, self.rs1, self.rs2),
            RtypeInstruction::XOR => common::xor(machine, self.rd, self.rs1, self.rs2),
            RtypeInstruction::OR => common::or(machine, self.rd, self.rs1, self.rs2),
            RtypeInstruction::AND => common::and(machine, self.rd, self.rs1, self.rs2),
            RtypeInstruction::SLL => {
                let shift_value = machine.registers()[self.rs2 as usize].clone()
                    & Mac::REG::from_usize(Mac::REG::SHIFT_MASK);
                let value = machine.registers()[self.rs1 as usize].clone() << shift_value;
                update_register(machine, self.rd, value);
            }
            RtypeInstruction::SLLW => {
                let shift_value =
                    machine.registers()[self.rs2 as usize].clone() & Mac::REG::from_usize(0x1F);
                let value = machine.registers()[self.rs1 as usize].clone() << shift_value;
                update_register(
                    machine,
                    self.rd,
                    value.sign_extend(&Mac::REG::from_usize(32)),
                );
            }
            RtypeInstruction::SRL => {
                let shift_value = machine.registers()[self.rs2 as usize].clone()
                    & Mac::REG::from_usize(Mac::REG::SHIFT_MASK);
                let value = machine.registers()[self.rs1 as usize].clone() >> shift_value;
                update_register(machine, self.rd, value);
            }
            RtypeInstruction::SRLW => {
                let shift_value =
                    machine.registers()[self.rs2 as usize].clone() & Mac::REG::from_usize(0x1F);
                let value = machine.registers()[self.rs1 as usize]
                    .zero_extend(&Mac::REG::from_usize(32))
                    >> shift_value;
                update_register(
                    machine,
                    self.rd,
                    value.sign_extend(&Mac::REG::from_usize(32)),
                );
            }
            RtypeInstruction::SRA => {
                let shift_value = machine.registers()[self.rs2 as usize].clone()
                    & Mac::REG::from_usize(Mac::REG::SHIFT_MASK);
                let value = machine.registers()[self.rs1 as usize].signed_shr(&shift_value);
                update_register(machine, self.rd, value);
            }
            RtypeInstruction::SRAW => {
                let shift_value =
                    machine.registers()[self.rs2 as usize].clone() & Mac::REG::from_usize(0x1F);
                let value = machine.registers()[self.rs1 as usize]
                    .sign_extend(&Mac::REG::from_usize(32))
                    .signed_shr(&shift_value);
                update_register(
                    machine,
                    self.rd,
                    value.sign_extend(&Mac::REG::from_usize(32)),
                );
            }
            RtypeInstruction::SLT => {
                let rs1_value = &machine.registers()[self.rs1 as usize];
                let rs2_value = &machine.registers()[self.rs2 as usize];
                let value = rs1_value.lt_s(&rs2_value);
                update_register(machine, self.rd, value);
            }
            RtypeInstruction::SLTU => {
                let rs1_value = &machine.registers()[self.rs1 as usize];
                let rs2_value = &machine.registers()[self.rs2 as usize];
                let value = rs1_value.lt(&rs2_value);
                update_register(machine, self.rd, value);
            }
        }
        Ok(None)
    }
}

impl Execute for Itype {
    fn execute<Mac: Machine>(&self, machine: &mut Mac) -> Result<Option<Mac::REG>, Error> {
        match &self.inst {
            ItypeInstruction::LB => common::lb(machine, self.rd, self.rs1, self.imm)?,
            ItypeInstruction::LH => common::lh(machine, self.rd, self.rs1, self.imm)?,
            ItypeInstruction::LW => common::lw(machine, self.rd, self.rs1, self.imm)?,
            ItypeInstruction::LD => common::ld(machine, self.rd, self.rs1, self.imm)?,
            ItypeInstruction::LBU => common::lbu(machine, self.rd, self.rs1, self.imm)?,
            ItypeInstruction::LHU => common::lhu(machine, self.rd, self.rs1, self.imm)?,
            ItypeInstruction::LWU => common::lwu(machine, self.rd, self.rs1, self.imm)?,
            ItypeInstruction::ADDI => common::addi(machine, self.rd, self.rs1, self.imm),
            ItypeInstruction::ADDIW => common::addiw(machine, self.rd, self.rs1, self.imm),
            ItypeInstruction::XORI => common::xori(machine, self.rd, self.rs1, self.imm),
            ItypeInstruction::ORI => common::ori(machine, self.rd, self.rs1, self.imm),
            ItypeInstruction::ANDI => common::andi(machine, self.rd, self.rs1, self.imm),
            ItypeInstruction::SLTI => {
                let rs1_value = &machine.registers()[self.rs1 as usize];
                let imm_value = Mac::REG::from_i32(self.imm);
                let value = rs1_value.lt_s(&imm_value);
                update_register(machine, self.rd, value);
            }
            ItypeInstruction::SLTIU => {
                let rs1_value = &machine.registers()[self.rs1 as usize];
                let imm_value = Mac::REG::from_i32(self.imm);
                let value = rs1_value.lt(&imm_value);
                update_register(machine, self.rd, value);
            }
            ItypeInstruction::JALR => {
                let link = machine.pc().overflowing_add(&Mac::REG::from_usize(4));
                let mut next_pc = machine.registers()[self.rs1 as usize]
                    .overflowing_add(&Mac::REG::from_i32(self.imm));
                next_pc = next_pc & (!Mac::REG::one());
                update_register(machine, self.rd, link);
                return Ok(Some(next_pc));
            }
        }
        Ok(None)
    }
}

impl Execute for ItypeShift {
    fn execute<Mac: Machine>(&self, machine: &mut Mac) -> Result<Option<Mac::REG>, Error> {
        match &self.inst {
            ItypeShiftInstruction::SLLI => {
                common::slli(machine, self.rd, self.rs1, self.shamt as u32)
            }
            ItypeShiftInstruction::SRLI => {
                common::srli(machine, self.rd, self.rs1, self.shamt as u32)
            }
            ItypeShiftInstruction::SRAI => {
                common::srai(machine, self.rd, self.rs1, self.shamt as u32)
            }
            ItypeShiftInstruction::SLLIW => {
                common::slliw(machine, self.rd, self.rs1, self.shamt as u32)
            }
            ItypeShiftInstruction::SRLIW => {
                common::srliw(machine, self.rd, self.rs1, self.shamt as u32)
            }
            ItypeShiftInstruction::SRAIW => {
                common::sraiw(machine, self.rd, self.rs1, self.shamt as u32)
            }
        }
        Ok(None)
    }
}

impl Execute for Stype {
    fn execute<Mac: Machine>(&self, machine: &mut Mac) -> Result<Option<Mac::REG>, Error> {
        match &self.inst {
            StypeInstruction::SB => common::sb(machine, self.rs1, self.rs2, self.imm)?,
            StypeInstruction::SH => common::sh(machine, self.rs1, self.rs2, self.imm)?,
            StypeInstruction::SW => common::sw(machine, self.rs1, self.rs2, self.imm)?,
            StypeInstruction::SD => common::sd(machine, self.rs1, self.rs2, self.imm)?,
        }
        Ok(None)
    }
}

impl Execute for Btype {
    fn execute<Mac: Machine>(&self, machine: &mut Mac) -> Result<Option<Mac::REG>, Error> {
        let rs1_value = &machine.registers()[self.rs1 as usize];
        let rs2_value = &machine.registers()[self.rs2 as usize];
        let condition = match &self.inst {
            BtypeInstruction::BEQ => rs1_value.eq(&rs2_value),
            BtypeInstruction::BNE => rs1_value.ne(&rs2_value),
            BtypeInstruction::BLT => rs1_value.lt_s(&rs2_value),
            BtypeInstruction::BGE => rs1_value.ge_s(&rs2_value),
            BtypeInstruction::BLTU => rs1_value.lt(&rs2_value),
            BtypeInstruction::BGEU => rs1_value.ge(&rs2_value),
        };
        let next_pc_offset =
            condition.cond(&Mac::REG::from_i32(self.imm), &Mac::REG::from_usize(4));
        Ok(Some(machine.pc().overflowing_add(&next_pc_offset)))
    }
}

impl Execute for Utype {
    fn execute<Mac: Machine>(&self, machine: &mut Mac) -> Result<Option<Mac::REG>, Error> {
        match &self.inst {
            UtypeInstruction::LUI => {
                update_register(machine, self.rd, Mac::REG::from_i32(self.imm));
            }
            UtypeInstruction::AUIPC => {
                let value = machine.pc().overflowing_add(&Mac::REG::from_i32(self.imm));
                update_register(machine, self.rd, value);
            }
        }
        Ok(None)
    }
}

impl Execute for FenceType {
    fn execute<Mac: Machine>(&self, _machine: &mut Mac) -> Result<Option<Mac::REG>, Error> {
        Ok(None)
    }
}

impl Execute for EnvInstruction {
    fn execute<Mac: Machine>(&self, machine: &mut Mac) -> Result<Option<Mac::REG>, Error> {
        match self {
            EnvInstruction::ECALL => {
                // The semantic of ECALL is determined by the hardware, which
                // is not part of the spec, hence here the implementation is
                // deferred to the machine. This way custom ECALLs might be
                // provided for different environments.
                machine.ecall()?;
            }
            EnvInstruction::EBREAK => {
                machine.ebreak()?;
            }
        }
        Ok(None)
    }
}

impl Execute for CsrType {
    fn execute<Mac: Machine>(&self, _machine: &mut Mac) -> Result<Option<Mac::REG>, Error> {
        // > All CSR instructions atomically read-modify-write a single CSR.
        // So no need to implement them yet
        match &self.inst {
            CsrInstruction::CSRRW => unimplemented!(),
            CsrInstruction::CSRRS => unimplemented!(),
            CsrInstruction::CSRRC => unimplemented!(),
        }
    }
}

impl Execute for CsrIType {
    fn execute<Mac: Machine>(&self, _machine: &mut Mac) -> Result<Option<Mac::REG>, Error> {
        // > All CSR instructions atomically read-modify-write a single CSR.
        // So no need to implement them yet
        match &self.inst {
            CsrIInstruction::CSRRWI => unimplemented!(),
            CsrIInstruction::CSRRSI => unimplemented!(),
            CsrIInstruction::CSRRCI => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    R(Rtype),
    I(Itype),
    IShift(ItypeShift),
    S(Stype),
    B(Btype),
    U(Utype),
    Fence(FenceType),
    Env(EnvInstruction),
    Csr(CsrType),
    CsrI(CsrIType),
    JAL { imm: Immediate, rd: RegisterIndex },
    FENCEI,
}

impl Instruction {
    pub fn execute<Mac: Machine>(&self, machine: &mut Mac) -> Result<(), Error> {
        let next_pc: Option<Mac::REG> = match self {
            Instruction::R(inst) => inst.execute(machine)?,
            Instruction::I(inst) => inst.execute(machine)?,
            Instruction::IShift(inst) => inst.execute(machine)?,
            Instruction::S(inst) => inst.execute(machine)?,
            Instruction::B(inst) => inst.execute(machine)?,
            Instruction::U(inst) => inst.execute(machine)?,
            Instruction::Fence(inst) => inst.execute(machine)?,
            Instruction::Env(inst) => inst.execute(machine)?,
            Instruction::Csr(inst) => inst.execute(machine)?,
            Instruction::CsrI(inst) => inst.execute(machine)?,
            Instruction::JAL { imm, rd } => common::jal(machine, *rd, *imm, 4),
            Instruction::FENCEI => unimplemented!(),
        };
        let default_next_pc = machine.pc().overflowing_add(&Mac::REG::from_usize(4));
        machine.set_pc(next_pc.unwrap_or(default_next_pc));
        Ok(())
    }
}

pub fn factory<R: Register>(instruction_bits: u32) -> Option<GenericInstruction> {
    let bit_length = R::BITS;
    if bit_length != 32 && bit_length != 64 {
        return None;
    }
    let rv64 = bit_length == 64;
    let instruction_opt = match opcode(instruction_bits) {
        0b_0110111 => Some(Instruction::U(Utype {
            rd: rd(instruction_bits),
            imm: utype_immediate(instruction_bits),
            inst: UtypeInstruction::LUI,
        })),
        0b_0010111 => Some(Instruction::U(Utype {
            rd: rd(instruction_bits),
            imm: utype_immediate(instruction_bits),
            inst: UtypeInstruction::AUIPC,
        })),
        0b_1101111 => Some(Instruction::JAL {
            rd: rd(instruction_bits),
            imm: jtype_immediate(instruction_bits),
        }),
        0b_1100111 => {
            let inst_opt = match funct3(instruction_bits) {
                // I-type jump instructions
                0b_000 => Some(ItypeInstruction::JALR),
                _ => None,
            };
            inst_opt.map(|inst| {
                Instruction::I(Itype {
                    rs1: rs1(instruction_bits),
                    rd: rd(instruction_bits),
                    imm: itype_immediate(instruction_bits),
                    inst,
                })
            })
        }
        0b_0000011 => {
            let inst_opt = match funct3(instruction_bits) {
                // I-type load instructions
                0b_000 => Some(ItypeInstruction::LB),
                0b_001 => Some(ItypeInstruction::LH),
                0b_010 => Some(ItypeInstruction::LW),
                0b_100 => Some(ItypeInstruction::LBU),
                0b_101 => Some(ItypeInstruction::LHU),
                0b_110 if rv64 => Some(ItypeInstruction::LWU),
                0b_011 if rv64 => Some(ItypeInstruction::LD),
                _ => None,
            };
            inst_opt.map(|inst| {
                Instruction::I(Itype {
                    rs1: rs1(instruction_bits),
                    rd: rd(instruction_bits),
                    imm: itype_immediate(instruction_bits),
                    inst,
                })
            })
        }
        0b_0010011 => {
            let funct3_value = funct3(instruction_bits);
            let inst_opt = match funct3_value {
                // I-type ALU instructions
                0b_000 => Some(ItypeInstruction::ADDI),
                0b_010 => Some(ItypeInstruction::SLTI),
                0b_011 => Some(ItypeInstruction::SLTIU),
                0b_100 => Some(ItypeInstruction::XORI),
                0b_110 => Some(ItypeInstruction::ORI),
                0b_111 => Some(ItypeInstruction::ANDI),
                // I-type special ALU instructions
                0b_001 | 0b_101 => {
                    let top6_value = funct7(instruction_bits) >> 1;
                    let inst_opt = match (funct3_value, top6_value) {
                        (0b_001, 0b_000000) => Some(ItypeShiftInstruction::SLLI),
                        (0b_101, 0b_000000) => Some(ItypeShiftInstruction::SRLI),
                        (0b_101, 0b_010000) => Some(ItypeShiftInstruction::SRAI),
                        _ => None,
                    };
                    return inst_opt.map(|inst| {
                        I(Instruction::IShift(ItypeShift {
                            rs1: rs1(instruction_bits),
                            rd: rd(instruction_bits),
                            shamt: itype_immediate(instruction_bits) & R::SHIFT_MASK as i32,
                            inst,
                        }))
                    });
                }
                _ => None,
            };

            inst_opt.map(|inst| {
                Instruction::I(Itype {
                    rs1: rs1(instruction_bits),
                    rd: rd(instruction_bits),
                    imm: itype_immediate(instruction_bits),
                    inst,
                })
            })
        }
        0b_1100011 => {
            let inst_opt = match funct3(instruction_bits) {
                0b_000 => Some(BtypeInstruction::BEQ),
                0b_001 => Some(BtypeInstruction::BNE),
                0b_100 => Some(BtypeInstruction::BLT),
                0b_101 => Some(BtypeInstruction::BGE),
                0b_110 => Some(BtypeInstruction::BLTU),
                0b_111 => Some(BtypeInstruction::BGEU),
                _ => None,
            };
            inst_opt.map(|inst| {
                Instruction::B(Btype {
                    rs2: rs2(instruction_bits),
                    rs1: rs1(instruction_bits),
                    imm: btype_immediate(instruction_bits),
                    inst,
                })
            })
        }
        0b_0100011 => {
            let inst_opt = match funct3(instruction_bits) {
                0b_000 => Some(StypeInstruction::SB),
                0b_001 => Some(StypeInstruction::SH),
                0b_010 => Some(StypeInstruction::SW),
                0b_011 if rv64 => Some(StypeInstruction::SD),
                _ => None,
            };
            inst_opt.map(|inst| {
                Instruction::S(Stype {
                    rs2: rs2(instruction_bits),
                    rs1: rs1(instruction_bits),
                    imm: stype_immediate(instruction_bits),
                    inst,
                })
            })
        }
        0b_0110011 => {
            let inst_opt = match (funct3(instruction_bits), funct7(instruction_bits)) {
                (0b_000, 0b_0000000) => Some(RtypeInstruction::ADD),
                (0b_000, 0b_0100000) => Some(RtypeInstruction::SUB),
                (0b_001, 0b_0000000) => Some(RtypeInstruction::SLL),
                (0b_010, 0b_0000000) => Some(RtypeInstruction::SLT),
                (0b_011, 0b_0000000) => Some(RtypeInstruction::SLTU),
                (0b_100, 0b_0000000) => Some(RtypeInstruction::XOR),
                (0b_101, 0b_0000000) => Some(RtypeInstruction::SRL),
                (0b_101, 0b_0100000) => Some(RtypeInstruction::SRA),
                (0b_110, 0b_0000000) => Some(RtypeInstruction::OR),
                (0b_111, 0b_0000000) => Some(RtypeInstruction::AND),
                _ => None,
            };
            inst_opt.map(|inst| {
                Instruction::R(Rtype {
                    rs2: rs2(instruction_bits),
                    rs1: rs1(instruction_bits),
                    rd: rd(instruction_bits),
                    inst,
                })
            })
        }
        0b_0001111 => {
            const FENCE_LOW_BITS: u32 = 0b_00000_000_00000_0001111;
            const FENCEI_VALUE: u32 = 0b_0000_0000_0000_00000_001_00000_0001111;
            if instruction_bits == FENCEI_VALUE {
                Some(Instruction::FENCEI)
            } else if instruction_bits & 0x000_FFFFF == FENCE_LOW_BITS {
                Some(Instruction::Fence(FenceType {
                    fm: ((instruction_bits & 0xF00_00000) >> 28) as u8,
                    pred: ((instruction_bits & 0x0F0_00000) >> 24) as u8,
                    succ: ((instruction_bits & 0x00F_00000) >> 20) as u8,
                }))
            } else {
                None
            }
        }
        0b_1110011 => match instruction_bits {
            0b_000000000000_00000_000_00000_1110011 => {
                Some(Instruction::Env(EnvInstruction::ECALL))
            }
            0b_000000000001_00000_000_00000_1110011 => {
                Some(Instruction::Env(EnvInstruction::EBREAK))
            }
            _ => {
                let csr = instruction_bits >> 20;
                let rs1_zimm = rs1(instruction_bits);
                let rd = rd(instruction_bits);
                match funct3(instruction_bits) {
                    0b_001 => Some(Instruction::Csr(CsrType {
                        csr,
                        rd,
                        rs1: rs1_zimm,
                        inst: CsrInstruction::CSRRW,
                    })),
                    0b_010 => Some(Instruction::Csr(CsrType {
                        csr,
                        rd,
                        rs1: rs1_zimm,
                        inst: CsrInstruction::CSRRS,
                    })),
                    0b_011 => Some(Instruction::Csr(CsrType {
                        csr,
                        rd,
                        rs1: rs1_zimm,
                        inst: CsrInstruction::CSRRC,
                    })),
                    0b_101 => Some(Instruction::CsrI(CsrIType {
                        csr: csr as u16,
                        rd,
                        zimm: u16::from(rs1_zimm),
                        inst: CsrIInstruction::CSRRWI,
                    })),
                    0b_110 => Some(Instruction::CsrI(CsrIType {
                        csr: csr as u16,
                        rd,
                        zimm: u16::from(rs1_zimm),
                        inst: CsrIInstruction::CSRRSI,
                    })),
                    0b_111 => Some(Instruction::CsrI(CsrIType {
                        csr: csr as u16,
                        rd,
                        zimm: u16::from(rs1_zimm),
                        inst: CsrIInstruction::CSRRCI,
                    })),
                    _ => None,
                }
            }
        },
        0b_0011011 if rv64 => {
            let funct3_value = funct3(instruction_bits);
            match funct3_value {
                0b_000 => Some(Instruction::I(Itype {
                    rs1: rs1(instruction_bits),
                    rd: rd(instruction_bits),
                    imm: itype_immediate(instruction_bits),
                    inst: ItypeInstruction::ADDIW,
                })),
                0b_001 | 0b_101 => {
                    let funct7_value = funct7(instruction_bits);
                    let inst_opt = match (funct3_value, funct7_value) {
                        (0b_001, 0b_0000000) => Some(ItypeShiftInstruction::SLLIW),
                        (0b_101, 0b_0000000) => Some(ItypeShiftInstruction::SRLIW),
                        (0b_101, 0b_0100000) => Some(ItypeShiftInstruction::SRAIW),
                        _ => None,
                    };
                    return inst_opt.map(|inst| {
                        I(Instruction::IShift(ItypeShift {
                            rs1: rs1(instruction_bits),
                            rd: rd(instruction_bits),
                            shamt: itype_immediate(instruction_bits) & 0x1F,
                            inst,
                        }))
                    });
                }
                _ => None,
            }
        }
        0b_0111011 if rv64 => {
            let inst_opt = match (funct3(instruction_bits), funct7(instruction_bits)) {
                (0b_000, 0b_0000000) => Some(RtypeInstruction::ADDW),
                (0b_000, 0b_0100000) => Some(RtypeInstruction::SUBW),
                (0b_001, 0b_0000000) => Some(RtypeInstruction::SLLW),
                (0b_101, 0b_0000000) => Some(RtypeInstruction::SRLW),
                (0b_101, 0b_0100000) => Some(RtypeInstruction::SRAW),
                _ => None,
            };
            inst_opt.map(|inst| {
                Instruction::R(Rtype {
                    rs2: rs2(instruction_bits),
                    rs1: rs1(instruction_bits),
                    rd: rd(instruction_bits),
                    inst,
                })
            })
        }
        _ => None,
    };
    instruction_opt.map(I)
}

pub fn nop() -> GenericInstruction {
    I(Instruction::I(Itype {
        rs1: 0,
        rd: 0,
        imm: 0,
        inst: ItypeInstruction::ADDI,
    }))
}
