use super::super::machine::Machine;
use super::super::memory::Memory;
use super::super::Error;
use super::utils::{
    btype_immediate, funct3, funct7, itype_immediate, jtype_immediate, opcode, rd, rs1, rs2,
    stype_immediate, update_register, utype_immediate,
};
use super::{
    common,
    Instruction as GenericInstruction,
    Instruction::RV32I,
    RegisterIndex,
    Immediate,
    UImmediate,
    NextPC,
    Execute,
};


#[derive(Debug)]
pub enum RtypeInstruction {
    ADD,
    SUB,
    SLL,
    SLT,
    SLTU,
    XOR,
    SRL,
    SRA,
    OR,
    AND,
}

#[derive(Debug)]
pub enum ItypeInstruction {
    JALR,
    LB,
    LH,
    LW,
    LBU,
    LHU,
    ADDI,
    SLTI,
    SLTIU,
    XORI,
    ORI,
    ANDI,
}

#[derive(Debug)]
pub enum ItypeShiftInstruction {
    SLLI,
    SRLI,
    SRAI,
}

#[derive(Debug)]
pub enum StypeInstruction {
    SB,
    SH,
    SW,
}

#[derive(Debug)]
pub enum BtypeInstruction {
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
}

#[derive(Debug)]
pub enum UtypeInstruction {
    LUI,
    AUIPC,
}

#[derive(Debug)]
pub enum EnvInstruction {
    ECALL,
    EBREAK,
}

#[derive(Debug)]
pub enum CsrInstruction {
    CSRRW,
    CSRRS,
    CSRRC,
}

#[derive(Debug)]
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
#[derive(Debug)]
pub struct FenceType {
    fm: u32,
    // predecessor
    pred: u32,
    // successor
    succ: u32,
}

#[derive(Debug)]
pub struct CsrType {
    csr: UImmediate,
    rs1: RegisterIndex,
    rd: RegisterIndex,
    inst: CsrInstruction,
}

#[derive(Debug)]
pub struct CsrIType {
    csr: UImmediate,
    zimm: UImmediate,
    rd: RegisterIndex,
    inst: CsrIInstruction,
}

impl Execute for Rtype {
    fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<Option<NextPC>, Error> {
        match &self.inst {
            RtypeInstruction::SUB => common::sub(machine, self.rd, self.rs1, self.rs2),
            RtypeInstruction::ADD => common::add(machine, self.rd, self.rs1, self.rs2),
            RtypeInstruction::XOR => common::xor(machine, self.rd, self.rs1, self.rs2),
            RtypeInstruction::OR => common::or(machine, self.rd, self.rs1, self.rs2),
            RtypeInstruction::AND => common::and(machine, self.rd, self.rs1, self.rs2),
            RtypeInstruction::SLL => {
                let shift_value = machine.registers[self.rs2] & 0x1F;
                let value = machine.registers[self.rs1] << shift_value;
                update_register(machine, self.rd, value);
            }
            RtypeInstruction::SRL => {
                let shift_value = machine.registers[self.rs2] & 0x1F;
                let value = machine.registers[self.rs1] >> shift_value;
                update_register(machine, self.rd, value);
            }
            RtypeInstruction::SRA => {
                let shift_value = machine.registers[self.rs2] & 0x1F;
                let value = (machine.registers[self.rs1] as i32) >> shift_value;
                update_register(machine, self.rd, value as u32);
            }
            RtypeInstruction::SLT => {
                let rs1_value = machine.registers[self.rs1] as i32;
                let rs2_value = machine.registers[self.rs2] as i32;
                let value = if rs1_value < rs2_value { 1 } else { 0 };
                update_register(machine, self.rd, value)
            }
            RtypeInstruction::SLTU => {
                let rs1_value = machine.registers[self.rs1];
                let rs2_value = machine.registers[self.rs2];
                let value = if rs1_value < rs2_value { 1 } else { 0 };
                update_register(machine, self.rd, value)
            }
        }
        Ok(None)
    }
}

impl Execute for Itype {
    fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<Option<NextPC>, Error> {
        match &self.inst {
            ItypeInstruction::LB => common::lb(machine, self.rd, self.rs1, self.imm as u32)?,
            ItypeInstruction::LH => common::lh(machine, self.rd, self.rs1, self.imm as u32)?,
            ItypeInstruction::LW => common::lw(machine, self.rd, self.rs1, self.imm as u32)?,
            ItypeInstruction::LBU => common::lbu(machine, self.rd, self.rs1, self.imm as u32)?,
            ItypeInstruction::LHU => common::lhu(machine, self.rd, self.rs1, self.imm as u32)?,
            ItypeInstruction::ADDI => common::addi(machine, self.rd, self.rs1, self.imm as u32),
            ItypeInstruction::XORI => common::xori(machine, self.rd, self.rs1, self.imm as u32),
            ItypeInstruction::ORI => common::ori(machine, self.rd, self.rs1, self.imm as u32),
            ItypeInstruction::ANDI => common::andi(machine, self.rd, self.rs1, self.imm as u32),
            ItypeInstruction::SLTI => {
                let rs1_value = machine.registers[self.rs1] as i32;
                let imm_value = self.imm as i32;
                let value = if rs1_value < imm_value { 1 } else { 0 };
                update_register(machine, self.rd, value)
            }
            ItypeInstruction::SLTIU => {
                let rs1_value = machine.registers[self.rs1];
                let imm_value = self.imm as u32;
                let value = if rs1_value < imm_value { 1 } else { 0 };
                update_register(machine, self.rd, value)
            }
            ItypeInstruction::JALR => {
                let link = machine.pc + 4;
                let (mut next_pc, _) = machine.registers[self.rs1].overflowing_add(self.imm as u32);
                next_pc &= !(1 as u32);
                update_register(machine, self.rd, link);
                return Ok(Some(next_pc));
            }
        }
        Ok(None)
    }
}

impl Execute for ItypeShift {
    fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<Option<NextPC>, Error> {
        match &self.inst {
            ItypeShiftInstruction::SLLI => common::slli(machine, self.rd, self.rs1, self.shamt as u32),
            ItypeShiftInstruction::SRLI => common::srli(machine, self.rd, self.rs1, self.shamt as u32),
            ItypeShiftInstruction::SRAI => common::srai(machine, self.rd, self.rs1, self.shamt as u32),
        }
        Ok(None)
    }
}

impl Execute for Stype {
    fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<Option<NextPC>, Error> {
        match &self.inst {
            StypeInstruction::SB => common::sb(machine, self.rs1, self.rs2, self.imm as u32)?,
            StypeInstruction::SH => common::sh(machine, self.rs1, self.rs2, self.imm as u32)?,
            StypeInstruction::SW => common::sw(machine, self.rs1, self.rs2, self.imm as u32)?,
        }
        Ok(None)
    }
}

impl Execute for Btype {
    fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<Option<NextPC>, Error> {
        let satisfied = match &self.inst {
            BtypeInstruction::BEQ => machine.registers[self.rs1] == machine.registers[self.rs2],
            BtypeInstruction::BNE => machine.registers[self.rs1] != machine.registers[self.rs2],
            BtypeInstruction::BLT => (machine.registers[self.rs1] as i32) < (machine.registers[self.rs2] as i32),
            BtypeInstruction::BGE => (machine.registers[self.rs1] as i32) >= (machine.registers[self.rs2] as i32),
            BtypeInstruction::BLTU => machine.registers[self.rs1] < machine.registers[self.rs2],
            BtypeInstruction::BGEU => machine.registers[self.rs1] >=  machine.registers[self.rs2],
        };
        match satisfied {
            true => Ok(Some(machine.pc.overflowing_add(self.imm as u32).0)),
            false => Ok(None)
        }
    }
}

impl Execute for Utype {
    fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<Option<NextPC>, Error> {
        match &self.inst {
            UtypeInstruction::LUI => {
                update_register(machine, self.rd, self.imm as u32);
            }
            UtypeInstruction::AUIPC => {
                let (value, _) = machine.pc.overflowing_add(self.imm as u32);
                update_register(machine, self.rd, value);
            }
        }
        Ok(None)
    }
}

impl Execute for FenceType {
    fn execute<M: Memory>(&self, _machine: &mut Machine<M>) -> Result<Option<NextPC>, Error> {
        Ok(None)
    }
}

impl Execute for EnvInstruction {
    fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<Option<NextPC>, Error> {
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
            },
        }
        Ok(None)
    }
}

impl Execute for CsrType {
    fn execute<M: Memory>(&self, _machine: &mut Machine<M>) -> Result<Option<NextPC>, Error> {
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
    fn execute<M: Memory>(&self, _machine: &mut Machine<M>) -> Result<Option<NextPC>, Error> {
        // > All CSR instructions atomically read-modify-write a single CSR.
        // So no need to implement them yet
        match &self.inst {
            CsrIInstruction::CSRRWI => unimplemented!(),
            CsrIInstruction::CSRRSI => unimplemented!(),
            CsrIInstruction::CSRRCI => unimplemented!(),
        }
    }
}

#[derive(Debug)]
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
    JAL{ imm: Immediate, rd: RegisterIndex },
    FENCEI,
}

impl Instruction {
    pub fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<(), Error> {
        let next_pc: Option<u32> = match self {
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
        machine.pc = next_pc.unwrap_or(machine.pc + 4);
        Ok(())
    }
}

pub fn factory(instruction_bits: u32) -> Option<GenericInstruction> {
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
            let inst_opt = match funct3(instruction_bits){
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
                    let funct7_value = funct7(instruction_bits);
                    let inst_opt = match (funct3_value, funct7_value) {
                        (0b_001, 0b_0000000) => Some(ItypeShiftInstruction::SLLI),
                        (0b_101, 0b_0000000) => Some(ItypeShiftInstruction::SRLI),
                        (0b_101, 0b_0100000) => Some(ItypeShiftInstruction::SRAI),
                        _ => None,
                    };
                    return inst_opt.map(|inst| {
                        RV32I(Instruction::IShift(ItypeShift {
                            rs1: rs1(instruction_bits),
                            rd: rd(instruction_bits),
                            shamt: itype_immediate(instruction_bits) & 0x1F,
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
                    fm: (instruction_bits & 0xF00_00000) >> 28,
                    pred: (instruction_bits & 0x0F0_00000) >> 24,
                    succ: (instruction_bits & 0x00F_00000) >> 20,
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
                        csr,
                        rd,
                        zimm: rs1_zimm as u32,
                        inst: CsrIInstruction::CSRRWI,
                    })),
                    0b_110 => Some(Instruction::CsrI(CsrIType {
                        csr,
                        rd,
                        zimm: rs1_zimm as u32,
                        inst: CsrIInstruction::CSRRSI,
                    })),
                    0b_111 => Some(Instruction::CsrI(CsrIType {
                        csr,
                        rd,
                        zimm: rs1_zimm as u32,
                        inst: CsrIInstruction::CSRRCI,
                    })),
                    _ => None,
                }
            }
        },
        _ => None,
    };
    instruction_opt.map(RV32I)
}
