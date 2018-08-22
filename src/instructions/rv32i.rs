use super::super::machine::Machine;
use super::super::memory::Memory;
use super::super::Error;
use super::utils::{
    btype_immediate, funct3, funct7, itype_immediate, opcode, rd, rs1, rs2, stype_immediate,
    update_register, utype_immediate,
};
use super::{Instruction as GenericInstruction, Instruction::RV32I};

type Register = usize;
type Immediate = i32;
type UImmediate = u32;

pub trait Executable {
    fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<Option<u32>, Error>;
}

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
pub struct Rtype {
    rs2: Register,
    rs1: Register,
    rd: Register,
    inst: RtypeInstruction,
}

impl Executable for Rtype {
    fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<Option<u32>, Error> {
        match &self.inst {
            RtypeInstruction::ADD => {}
            RtypeInstruction::SUB => {}
            RtypeInstruction::SLL => {
                let shift_value = machine.registers[self.rs2] & 0x1F;
                let value = machine.registers[self.rs1] << shift_value;
                update_register(machine, self.rd, value);
            }
            RtypeInstruction::SLT => {}
            RtypeInstruction::SLTU => {}
            RtypeInstruction::XOR => {}
            RtypeInstruction::SRL => {}
            RtypeInstruction::SRA => {}
            RtypeInstruction::OR => {}
            RtypeInstruction::AND => {}
        }
        Ok(None)
    }
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
pub struct Itype {
    rs1: Register,
    rd: Register,
    imm: Immediate,
    inst: ItypeInstruction,
}

impl Executable for Itype {
    fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<Option<u32>, Error> {
        match &self.inst {
            ItypeInstruction::JALR => {
                let link = machine.pc + 4;
                let (mut next_pc, _) = machine.registers[self.rs1].overflowing_add(self.imm as u32);
                next_pc &= !(1 as u32);
                machine.pc = next_pc;
                update_register(machine, self.rd, link);
                return Ok(Some(next_pc));
            }
            ItypeInstruction::LB => {}
            ItypeInstruction::LH => {}
            ItypeInstruction::LW => {
                let (address, _) = machine.registers[self.rs1].overflowing_add(self.imm as u32);
                let value = machine.memory.load32(address as usize)?;
                update_register(machine, self.rd, value);
            }
            ItypeInstruction::LBU => {
                let (address, _) = machine.registers[self.rs1].overflowing_add(self.imm as u32);
                let value = machine.memory.load8(address as usize)?;
                update_register(machine, self.rd, value as u32);
            }
            ItypeInstruction::LHU => {}
            ItypeInstruction::ADDI => {
                let (value, _) = machine.registers[self.rs1].overflowing_add(self.imm as u32);
                update_register(machine, self.rd, value);
            }
            ItypeInstruction::SLTI => {}
            ItypeInstruction::SLTIU => {}
            ItypeInstruction::XORI => {}
            ItypeInstruction::ORI => {}
            ItypeInstruction::ANDI => {
                let value = machine.registers[self.rs1] & (self.imm as u32);
                update_register(machine, self.rd, value);
            }
        }
        Ok(None)
    }
}

#[derive(Debug)]
pub enum ItypeShiftInstruction {
    SLLI,
    SRLI,
    SRAI,
}

#[derive(Debug)]
pub struct ItypeShift {
    rs1: Register,
    rd: Register,
    // FIXME: utils.rs, Type
    shamt: Immediate,
    inst: ItypeShiftInstruction,
}

impl Executable for ItypeShift {
    fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<Option<u32>, Error> {
        match &self.inst {
            ItypeShiftInstruction::SLLI => {
                let value = machine.registers[self.rs1] << self.shamt;
                update_register(machine, self.rd, value);
            }
            ItypeShiftInstruction::SRLI => {}
            ItypeShiftInstruction::SRAI => {}
        }
        Ok(None)
    }
}

#[derive(Debug)]
pub enum StypeInstruction {
    SB,
    SH,
    SW,
}

#[derive(Debug)]
pub struct Stype {
    rs2: Register,
    rs1: Register,
    imm: Immediate,
    inst: StypeInstruction,
}

impl Executable for Stype {
    fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<Option<u32>, Error> {
        match &self.inst {
            StypeInstruction::SB => {
                let (address, _) = machine.registers[self.rs1].overflowing_add(self.imm as u32);
                let value = machine.registers[self.rs2] as u8;
                machine.memory.store8(address as usize, value)?;
            }
            StypeInstruction::SH => {}
            StypeInstruction::SW => {
                let (address, _) = machine.registers[self.rs1].overflowing_add(self.imm as u32);
                let value = machine.registers[self.rs2] as u32;
                machine.memory.store32(address as usize, value)?;
            }
        }
        Ok(None)
    }
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
pub struct Btype {
    rs2: Register,
    rs1: Register,
    imm: Immediate,
    inst: BtypeInstruction,
}

impl Executable for Btype {
    fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<Option<u32>, Error> {
        match &self.inst {
            BtypeInstruction::BEQ => {
                let rs1_value: u32 = machine.registers[self.rs1];
                let rs2_value: u32 = machine.registers[self.rs2];
                if rs1_value == rs2_value {
                    let (next_pc, _) = machine.pc.overflowing_add(self.imm as u32);
                    return Ok(Some(next_pc));
                }
            }
            BtypeInstruction::BNE => {
                let rs1_value: u32 = machine.registers[self.rs1];
                let rs2_value: u32 = machine.registers[self.rs2];
                if rs1_value != rs2_value {
                    let (next_pc, _) = machine.pc.overflowing_add(self.imm as u32);
                    return Ok(Some(next_pc));
                }
            }
            BtypeInstruction::BLT => {
                let rs1_value: i32 = machine.registers[self.rs1] as i32;
                let rs2_value: i32 = machine.registers[self.rs2] as i32;
                if rs1_value < rs2_value {
                    let (next_pc, _) = machine.pc.overflowing_add(self.imm as u32);
                    return Ok(Some(next_pc));
                }
            }
            BtypeInstruction::BGE => {
                let rs1_value: i32 = machine.registers[self.rs1] as i32;
                let rs2_value: i32 = machine.registers[self.rs2] as i32;
                if rs1_value >= rs2_value {
                    let (next_pc, _) = machine.pc.overflowing_add(self.imm as u32);
                    return Ok(Some(next_pc));
                }
            }
            BtypeInstruction::BLTU => {
                let rs1_value: u32 = machine.registers[self.rs1];
                let rs2_value: u32 = machine.registers[self.rs2];
                if rs1_value < rs2_value {
                    let (next_pc, _) = machine.pc.overflowing_add(self.imm as u32);
                    return Ok(Some(next_pc));
                }
            }
            BtypeInstruction::BGEU => {
                let rs1_value: u32 = machine.registers[self.rs1];
                let rs2_value: u32 = machine.registers[self.rs2];
                if rs1_value >= rs2_value {
                    let (next_pc, _) = machine.pc.overflowing_add(self.imm as u32);
                    return Ok(Some(next_pc));
                }
            }
        }
        Ok(None)
    }
}

#[derive(Debug)]
pub enum UtypeInstruction {
    LUI,
    AUIPC,
}

#[derive(Debug)]
pub struct Utype {
    rd: Register,
    imm: Immediate,
    inst: UtypeInstruction,
}

impl Executable for Utype {
    fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<Option<u32>, Error> {
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

#[derive(Debug)]
pub struct Jtype {
    rd: usize,
    imm: i32,
}

impl Executable for Jtype {
    fn execute<M: Memory>(&self, _machine: &mut Machine<M>) -> Result<Option<u32>, Error> {
        Ok(None)
    }
}

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

impl Executable for FenceType {
    fn execute<M: Memory>(&self, _machine: &mut Machine<M>) -> Result<Option<u32>, Error> {
        Ok(None)
    }
}

#[derive(Debug)]
pub enum EnvInstruction {
    ECALL,
    EBREAK,
}

impl Executable for EnvInstruction {
    fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<Option<u32>, Error> {
        match self {
            EnvInstruction::ECALL => {
                // The semantic of ECALL is determined by the hardware, which
                // is not part of the spec, hence here the implementation is
                // deferred to the machine. This way custom ECALLs might be
                // provided for different environments.
                return machine.ecall().map(|_| None);
            }
            EnvInstruction::EBREAK => {}
        }
        Ok(None)
    }
}

#[derive(Debug)]
pub enum CsrInstruction {
    CSRRW,
    CSRRS,
    CSRRC,
}

#[derive(Debug)]
pub struct CsrType {
    csr: UImmediate,
    rs1: Register,
    rd: Register,
    inst: CsrInstruction,
}

impl Executable for CsrType {
    fn execute<M: Memory>(&self, _machine: &mut Machine<M>) -> Result<Option<u32>, Error> {
        match &self.inst {
            CsrInstruction::CSRRW => {}
            CsrInstruction::CSRRS => {}
            CsrInstruction::CSRRC => {}
        }
        Ok(None)
    }
}

#[derive(Debug)]
pub enum CsrIInstruction {
    CSRRWI,
    CSRRSI,
    CSRRCI,
}

#[derive(Debug)]
pub struct CsrIType {
    csr: UImmediate,
    zimm: UImmediate,
    rd: Register,
    inst: CsrIInstruction,
}

impl Executable for CsrIType {
    fn execute<M: Memory>(&self, _machine: &mut Machine<M>) -> Result<Option<u32>, Error> {
        match &self.inst {
            CsrIInstruction::CSRRWI => {}
            CsrIInstruction::CSRRSI => {}
            CsrIInstruction::CSRRCI => {}
        }
        Ok(None)
    }
}

//
//  31       27 26 25 24     20 19    15 14    12 11          7 6      0
// ======================================================================
// | funct7          |   rs2   |   rs1  | funct3 |  rd         | opcode | R-type
// +--------------------------------------------------------------------+
// |          imm[11:0]        |   rs1  | funct3 |  rd         | opcode | I-type
// +--------------------------------------------------------------------+
// |   imm[11:5]     |   rs2   |   rs1  | funct3 | imm[4:0]    | opcode | S-type
// +--------------------------------------------------------------------+
// |   imm[12|10:5]  |   rs2   |   rs1  | funct3 | imm[4:1|11] | opcode | B-type
// +--------------------------------------------------------------------+
// |             imm[31:12]                      |  rd         | opcode | U-type
// +--------------------------------------------------------------------+
// |             imm[20|10:1|11|19:12]           |  rd         | opcode | J-type
// ======================================================================
//
#[derive(Debug)]
pub enum Instruction {
    R(Rtype),
    I(Itype),
    IShift(ItypeShift),
    S(Stype),
    B(Btype),
    U(Utype),
    J(Jtype),
    Fence(FenceType),
    FenceI,
    Env(EnvInstruction),
    Csr(CsrType),
    CsrI(CsrIType),
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
            Instruction::J(inst) => inst.execute(machine)?,
            Instruction::Fence(inst) => inst.execute(machine)?,
            Instruction::FenceI => unimplemented!(),
            Instruction::Env(inst) => inst.execute(machine)?,
            Instruction::Csr(inst) => inst.execute(machine)?,
            Instruction::CsrI(inst) => inst.execute(machine)?,
        };
        machine.pc = next_pc.unwrap_or(machine.pc + 4);
        Ok(())
    }
}

mod code {
    // Instructions: LUI
    pub const U_TYPE_LUI: u32 = 0b_0110111;
    // Instructions: AUIPC
    pub const U_TYPE_AUIPC: u32 = 0b_0010111;
    // Instructions: JAL
    pub const J_TYPE: u32 = 0b_1101111;
    // Instructions: JALR
    pub const I_TYPE_JUMP: u32 = 0b_1100111;
    // Instructions: LB LH LW LBU LHU
    pub const I_TYPE_LOAD: u32 = 0b_0000011;
    // Instructions: ADDI SLTI SLTIU XORI ORI ANDI SLLI SRLI SRAI
    pub const I_TYPE_ALU: u32 = 0b_0010011;
    // Instructions: BEQ BNE BLT BGE BLTU BGEU
    pub const B_TYPE: u32 = 0b_1100011;
    // Instructions: SB SH SW
    pub const S_TYPE: u32 = 0b_0100011;
    // Instructions: ADD SUB SLL SLT SLTU XOR SRL SRA OR AND
    pub const R_TYPE: u32 = 0b_0110011;
    // Instructions: FENCE FENCE.I
    pub const MISC_MEM: u32 = 0b_0001111;
    // Instructions: ECALL EBREAK CSRRW CSRRS CSRRC CSRRWI CSRRSI CSRRCI
    //   (Execution and CSR instructions.)
    pub const SYSTEM: u32 = 0b_1110011;
}

pub fn factory(instruction_bits: u32) -> Option<GenericInstruction> {
    let current_code = opcode(instruction_bits);
    let instruction_opt = match current_code {
        code::U_TYPE_LUI | code::U_TYPE_AUIPC => {
            let inst = match current_code {
                code::U_TYPE_LUI => UtypeInstruction::LUI,
                code::U_TYPE_AUIPC => UtypeInstruction::AUIPC,
                _ => unreachable!(),
            };
            Some(Instruction::U(Utype {
                rd: rd(instruction_bits),
                imm: utype_immediate(instruction_bits),
                inst,
            }))
        }
        code::J_TYPE => {
            Some(Instruction::J(Jtype {
                rd: rd(instruction_bits),
                // FIXME: jtype_immediate
                imm: utype_immediate(instruction_bits),
            }))
        }
        code::I_TYPE_JUMP | code::I_TYPE_LOAD | code::I_TYPE_ALU => {
            let funct3_value = funct3(instruction_bits);
            let inst_opt = match (current_code, funct3_value) {
                (code::I_TYPE_JUMP, 0b_000) => Some(ItypeInstruction::JALR),

                (code::I_TYPE_LOAD, 0b_000) => Some(ItypeInstruction::LB),
                (code::I_TYPE_LOAD, 0b_001) => Some(ItypeInstruction::LH),
                (code::I_TYPE_LOAD, 0b_010) => Some(ItypeInstruction::LW),
                (code::I_TYPE_LOAD, 0b_100) => Some(ItypeInstruction::LBU),
                (code::I_TYPE_LOAD, 0b_101) => Some(ItypeInstruction::LHU),

                (code::I_TYPE_ALU, 0b_000) => Some(ItypeInstruction::ADDI),
                (code::I_TYPE_ALU, 0b_010) => Some(ItypeInstruction::SLTI),
                (code::I_TYPE_ALU, 0b_011) => Some(ItypeInstruction::SLTIU),
                (code::I_TYPE_ALU, 0b_100) => Some(ItypeInstruction::XORI),
                (code::I_TYPE_ALU, 0b_110) => Some(ItypeInstruction::ORI),
                (code::I_TYPE_ALU, 0b_111) => Some(ItypeInstruction::ANDI),

                (code::I_TYPE_ALU, 0b_001) | (code::I_TYPE_ALU, 0b_101) => {
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
        code::B_TYPE => {
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
        code::S_TYPE => {
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
        code::R_TYPE => {
            let inst_opt = match funct3(instruction_bits) {
                0b_000 => match funct7(instruction_bits) {
                    0b_0000000 => Some(RtypeInstruction::ADD),
                    0b_0100000 => Some(RtypeInstruction::SUB),
                    _ => None,
                },
                0b_001 => Some(RtypeInstruction::SLL),
                0b_010 => Some(RtypeInstruction::SLT),
                0b_011 => Some(RtypeInstruction::SLTU),
                0b_100 => Some(RtypeInstruction::XOR),
                0b_101 => match funct7(instruction_bits) {
                    0b_0000000 => Some(RtypeInstruction::SRL),
                    0b_0100000 => Some(RtypeInstruction::SRA),
                    _ => None,
                },
                0b_110 => Some(RtypeInstruction::OR),
                0b_111 => Some(RtypeInstruction::AND),
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
        code::MISC_MEM => {
            const FENCE_MASK: u32 = 0b_0000_0000_0000_00000_001_00000_0001111;
            if instruction_bits == FENCE_MASK {
                Some(Instruction::FenceI)
            } else if instruction_bits & 0x000_FFFFF == FENCE_MASK {
                Some(Instruction::Fence(FenceType {
                    fm: (instruction_bits & 0xF00_00000) >> 8,
                    pred: (instruction_bits & 0x0F0_00000) >> 4,
                    succ: instruction_bits & 0x00F_00000,
                }))
            } else {
                None
            }
        }
        code::SYSTEM => match instruction_bits {
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
    instruction_opt.map(|instruction| RV32I(instruction))
}
