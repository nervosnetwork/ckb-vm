use super::super::machine::Machine;
use super::super::memory::Memory;
use super::super::{Error, SP};
use super::register::Register;
use super::utils::{rd, update_register, x, xs};
use super::{
    common, Execute, Immediate, Instruction as GenericInstruction, Instruction::RVC, RegisterIndex,
    UImmediate,
};

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

#[derive(Debug)]
pub enum RtypeInstruction {
    SUB,
    XOR,
    OR,
    AND,
    SUBW,
    ADDW,
    ADD,
}

#[derive(Debug)]
pub enum ItypeInstruction {
    ADDI,
    ADDIW,
    ANDI,
}

#[derive(Debug)]
pub enum ItypeUInstruction {
    FLD,
    // LQ,
    LW,
    FLW,
    LD,
    SRLI,
    SRAI,
    SLLI,
}

#[derive(Debug)]
pub enum StypeUInstruction {
    FSD,
    // SQ,
    SW,
    FSW,
    SD,
}

#[derive(Debug)]
pub enum UtypeInstruction {
    LI,
    LUI,
}

#[derive(Debug)]
pub enum UtypeUInstruction {
    ADDI4SPN,
    FLDSP,
    // LQSP,
    LWSP,
    FLWSP,
    LDSP,
}

#[derive(Debug)]
pub enum CSSformatInstruction {
    FSDSP,
    // SQSP,
    SWSP,
    FSWSP,
    SDSP,
}

// ## Compressed 16-bit RVC instruction formats
//
//  Format |    Meaning              15 14 13   12    11 10   9 8 7   6 5   4  3  2   1 0
//  -------+-----------------      +----------+-----+-------+-------+-----+---------+----+
//    CR   |    Register           |     funct4     |    rd/rs1     |      rs2      | op |
//         |                       +----------+-----+---------------+---------------+----+
//    CI   |   Immediate           |  funct3  | imm |    rd/rs1     |      imm      | op |
//         |                       +----------+-----+---------------+---------------+----+
//    CSS  | Stack-relative Store  |  funct3  |         imm         |      rs2      | op |
//         |                       +----------+---------------------+-----+---------+----+
//    CIW  |   Wide Immediate      |  funct3  |            imm            |   rd′   | op |
//         |                       +----------+-------------+-------+-----+---------+----+
//    CL   |     Load              |  funct3  | imm         | rs1′  | imm |   rd′   | op |
//         |                       +----------+-------------+-------+-----+---------+----+
//    CS   |     Store             |  funct3  | imm         | rs1′  | imm |   rs2′  | op |
//         |                       +----------+-------------+-------+-----+---------+----+
//    CB   |     Branch            |  funct3  | offset      | rs1′  |    offset     | op |
//         |                       +----------+-------------+-------+---------------+----+
//    CJ   |     Jump              |  funct3  |             jump target             | op |
//  -------+-----------------      +----------+-------------------------------------+----+

// CR format
type Rtype = super::Rtype<RtypeInstruction>;
// CI/CL format
type Itype = super::Itype<Immediate, ItypeInstruction>;
// CI/CL format
type ItypeU = super::Itype<UImmediate, ItypeUInstruction>;
// CS format
type StypeU = super::Stype<UImmediate, StypeUInstruction>;
// CIW format
type Utype = super::Utype<Immediate, UtypeInstruction>;
// CIW format
type UtypeU = super::Utype<UImmediate, UtypeUInstruction>;

#[derive(Debug)]
pub struct CSSformat {
    rs2: RegisterIndex,
    imm: UImmediate,
    inst: CSSformatInstruction,
}

impl CSSformat {
    pub fn inst(&self) -> &CSSformatInstruction {
        &self.inst
    }
}

impl ItypeU {
    pub fn new(instruction_bits: u32, imm: UImmediate, inst: ItypeUInstruction) -> ItypeU {
        ItypeU {
            rs1: compact_register_number(instruction_bits, 7),
            rd: compact_register_number(instruction_bits, 2),
            imm,
            inst,
        }
    }
}

impl StypeU {
    pub fn new(instruction_bits: u32, imm: UImmediate, inst: StypeUInstruction) -> StypeU {
        StypeU {
            rs1: compact_register_number(instruction_bits, 7),
            rs2: compact_register_number(instruction_bits, 2),
            imm,
            inst,
        }
    }
}

impl Execute for ItypeU {
    fn execute<Mac: Machine<R, M>, R: Register, M: Memory>(
        &self,
        machine: &mut Mac,
    ) -> Result<Option<R>, Error> {
        match &self.inst {
            ItypeUInstruction::SLLI => common::slli(machine, self.rd, self.rs1, self.imm),
            ItypeUInstruction::SRLI => common::srli(machine, self.rd, self.rs1, self.imm),
            ItypeUInstruction::SRAI => common::srai(machine, self.rd, self.rs1, self.imm),
            ItypeUInstruction::LW => common::lw(machine, self.rd, self.rs1, self.imm as i32)?,

            // > RV32FC-only
            ItypeUInstruction::FLW => unimplemented!(),
            // > RV32DC/RV64DC-only
            ItypeUInstruction::FLD => unimplemented!(),
            ItypeUInstruction::LD => common::ld(machine, self.rd, self.rs1, self.imm as i32)?,
            // ItypeUInstruction::LQ => {},
        }
        Ok(None)
    }
}

impl Execute for StypeU {
    fn execute<Mac: Machine<R, M>, R: Register, M: Memory>(
        &self,
        machine: &mut Mac,
    ) -> Result<Option<R>, Error> {
        match &self.inst {
            StypeUInstruction::SW => common::sw(machine, self.rs1, self.rs2, self.imm as i32)?,
            // > RV32FC-only
            StypeUInstruction::FSW => unimplemented!(),
            // > RV32DC/RV64DC-only
            StypeUInstruction::FSD => unimplemented!(),
            // StypeUInstruction::SQ => {},
            StypeUInstruction::SD => common::sd(machine, self.rs1, self.rs2, self.imm as i32)?,
        }
        Ok(None)
    }
}

impl Execute for Itype {
    fn execute<Mac: Machine<R, M>, R: Register, M: Memory>(
        &self,
        machine: &mut Mac,
    ) -> Result<Option<R>, Error> {
        match &self.inst {
            ItypeInstruction::ADDI => common::addi(machine, self.rd, self.rs1, self.imm),
            ItypeInstruction::ANDI => common::andi(machine, self.rd, self.rs1, self.imm),
            ItypeInstruction::ADDIW => common::addiw(machine, self.rd, self.rs1, self.imm),
        }
        Ok(None)
    }
}

impl Execute for Utype {
    fn execute<Mac: Machine<R, M>, R: Register, M: Memory>(
        &self,
        machine: &mut Mac,
    ) -> Result<Option<R>, Error> {
        match &self.inst {
            UtypeInstruction::LI => {
                update_register(machine, self.rd, R::from_i32(self.imm));
            }
            UtypeInstruction::LUI => {
                update_register(machine, self.rd, R::from_i32(self.imm));
            }
        }
        Ok(None)
    }
}

impl Execute for UtypeU {
    fn execute<Mac: Machine<R, M>, R: Register, M: Memory>(
        &self,
        machine: &mut Mac,
    ) -> Result<Option<R>, Error> {
        match &self.inst {
            UtypeUInstruction::ADDI4SPN => {
                let (value, _) = machine.registers()[SP].overflowing_add(R::from_u32(self.imm));
                update_register(machine, self.rd, value);
            }
            UtypeUInstruction::LWSP => common::lw(machine, self.rd, SP, self.imm as i32)?,

            // > RV32FC-only
            UtypeUInstruction::FLWSP => unimplemented!(),
            // > RV32DC/RV64DC-only
            UtypeUInstruction::FLDSP => unimplemented!(),
            UtypeUInstruction::LDSP => common::ld(machine, self.rd, SP, self.imm as i32)?,
            // UtypeUInstruction::LQSP => {},
        }
        Ok(None)
    }
}

impl Execute for Rtype {
    fn execute<Mac: Machine<R, M>, R: Register, M: Memory>(
        &self,
        machine: &mut Mac,
    ) -> Result<Option<R>, Error> {
        match &self.inst {
            RtypeInstruction::SUB => common::sub(machine, self.rd, self.rs1, self.rs2),
            RtypeInstruction::ADD => common::add(machine, self.rd, self.rd, self.rs2),
            RtypeInstruction::XOR => common::xor(machine, self.rd, self.rs1, self.rs2),
            RtypeInstruction::OR => common::or(machine, self.rd, self.rs1, self.rs2),
            RtypeInstruction::AND => common::and(machine, self.rd, self.rs1, self.rs2),

            // > C.SUBW (RV64/128; RV32 RES)
            RtypeInstruction::SUBW => common::subw(machine, self.rd, self.rs1, self.rs2),
            // > C.ADDW (RV64/128; RV32 RES)
            RtypeInstruction::ADDW => common::addw(machine, self.rd, self.rd, self.rs2),
        }
        Ok(None)
    }
}

impl Execute for CSSformat {
    fn execute<Mac: Machine<R, M>, R: Register, M: Memory>(
        &self,
        machine: &mut Mac,
    ) -> Result<Option<R>, Error> {
        match &self.inst {
            CSSformatInstruction::SWSP => common::sw(machine, SP, self.rs2, self.imm as i32)?,
            // > RV32FC-only
            CSSformatInstruction::FSWSP => unimplemented!(),
            // > RV32DC/RV64DC-only
            CSSformatInstruction::FSDSP => unimplemented!(),
            CSSformatInstruction::SDSP => common::sd(machine, SP, self.rs2, self.imm as i32)?,
            // CSSformatInstruction::SQSP => {},
        }
        Ok(None)
    }
}

// ## Map of the major opcodes for RVC
//
//  +-------------+
//  | inst[15:13] |
//  | inst[1:0]   | 000      | 001   | 010   | 011          | 100           | 101   | 110  | 111   |
//  +-------------+----------+-------+-------+--------------+---------------+-------+------+-------+
//                |          | FLD   |       | FLW          |               | FSD   |      | FSW   | RV32
//             00 | ADDI4SPN | FLD   | LW    | LD           | Reserved      | FSD   | SW   | SD    | RV64
//                |          | LQ    |       | LD           |               | SQ    |      | SD    | RV128
//            ----+----------+-------+-------+--------------+---------------+-------+------+-------+-------
//                |          | JAL   |       |              |               |       |      |       | RV32
//             01 | ADDI     | ADDIW | LI    | LUI/ADDI16SP | MISC-ALU      | J     | BEQZ | BNEZ  | RV64
//                |          | ADDIW |       |              |               |       |      |       | RV128
//            ----+----------+-------+-------+--------------+---------------+-------+------+-------+-------
//                |          | FLDSP |       | FLWSP        |               | FSDSP |      | FSWSP | RV32
//             10 | SLLI     | FLDSP | LWSP  | LDSP         | J[AL]R/MV/ADD | FSDSP | SWSP | SDSP  | RV64
//                |          | LQSP  |       | LDSP         |               | SQSP  |      | SDSP  | RV128
//            ----+--------------------------------------------------------------------------------+
//             11 |                                       >16b                                     |
//            =====================================================================================+
//
#[derive(Debug)]
pub enum Instruction {
    // C.FLD (RV32/64)
    // C.LQ (RV128)
    // C.LW
    // C.FLW (RV32)
    // C.LD (RV64/128)
    // C.SRLI (RV32 NSE, nzuimm[5]=1)
    // C.SRAI (RV32 NSE, nzuimm[5]=1)
    // C.SLLI (HINT, rd=0; RV32 NSE, nzuimm[5]=1)
    Iu(ItypeU),

    // C.FSD (RV32/64)
    // C.SQ (RV128)
    // C.SW
    // C.FSW (RV32)
    // C.SD (RV64/128)
    Su(StypeU),

    // C.ADDI (HINT, nzimm=0)
    // C.ADDIW (RV64/128; RES, rd=0)
    // C.ANDI
    I(Itype),

    // C.LI (HINT, rd=0)
    // C.LUI (RES, nzimm=0; HINT, rd=0)
    U(Utype),

    // C.ADDI4SPN (RES, nzuimm=0)
    // C.FLDSP (RV32/64)
    // C.LQSP (RV128; RES, rd=0)
    // C.LWSP (RES, rd=0) C.FLWSP (RV32)
    // C.LDSP (RV64/128; RES, rd=0)
    Uu(UtypeU),

    // C.SUB
    // C.XOR
    // C.OR
    // C.AND
    // C.SUBW (RV64/128; RV32 RES)
    // C.ADDW (RV64/128; RV32 RES)
    // C.ADD (HINT, rd=0)
    R(Rtype),

    // C.FSDSP (RV32/64)
    // C.SQSP (RV128)
    // C.SWSP
    // C.FSWSP (RV32)
    // C.SDSP (RV64/128)
    CSS(CSSformat),

    // C.BEQZ
    BEQZ {
        rs1: RegisterIndex,
        imm: Immediate,
    },
    // C.BNEZ
    BNEZ {
        rs1: RegisterIndex,
        imm: Immediate,
    },
    // C.MV (HINT, rd=0)
    MV {
        rs2: RegisterIndex,
        rd: RegisterIndex,
    },

    // C.JAL (RV32)
    JAL {
        imm: Immediate,
    },
    // C.J
    J {
        imm: Immediate,
    },
    // C.JR (RES, rs1=0)
    JR {
        rs1: RegisterIndex,
    },
    // C.JALR
    JALR {
        rs1: RegisterIndex,
    },

    // C.ADDI16SP (RES, nzimm=0)
    ADDI16SP {
        imm: Immediate,
    },

    // C.NOP (HINT, nzimm̸=0)
    NOP {
        imm: Immediate,
    },
    // C.SRLI64 (RV128; RV32/64 HINT)
    SRLI64 {
        rs1: RegisterIndex,
        rd: RegisterIndex,
    },
    // C.SRAI64 (RV128; RV32/64 HINT)
    SRAI64 {
        rs1: RegisterIndex,
        rd: RegisterIndex,
    },
    // C.SLLI64 (RV128; RV32/64 HINT; HINT, rd=0)
    SLLI64 {
        rs1: RegisterIndex,
        rd: RegisterIndex,
    },
    // C.EBREAK
    EBREAK,
}

impl Instruction {
    pub fn execute<Mac: Machine<R, M>, R: Register, M: Memory>(
        &self,
        machine: &mut Mac,
    ) -> Result<(), Error> {
        let next_pc = match self {
            Instruction::Iu(inst) => inst.execute(machine)?,
            Instruction::Su(inst) => inst.execute(machine)?,
            Instruction::I(inst) => inst.execute(machine)?,
            Instruction::U(inst) => inst.execute(machine)?,
            Instruction::Uu(inst) => inst.execute(machine)?,
            Instruction::R(inst) => inst.execute(machine)?,
            Instruction::CSS(inst) => inst.execute(machine)?,
            Instruction::BEQZ { rs1, imm } => {
                if machine.registers()[*rs1] == R::zero() {
                    Some(machine.pc().overflowing_add(R::from_i32(*imm)).0)
                } else {
                    None
                }
            }
            Instruction::BNEZ { rs1, imm } => {
                if machine.registers()[*rs1] != R::zero() {
                    Some(machine.pc().overflowing_add(R::from_i32(*imm)).0)
                } else {
                    None
                }
            }
            Instruction::MV { rs2, rd } => {
                let value = machine.registers()[*rs2];
                update_register(machine, *rd, value);
                None
            }
            Instruction::JAL { imm } => common::jal(machine, 1, *imm, 2),
            Instruction::J { imm } => Some(machine.pc().overflowing_add(R::from_i32(*imm)).0),
            Instruction::JR { rs1 } => Some(machine.registers()[*rs1]),
            Instruction::JALR { rs1 } => {
                let link = machine.pc().overflowing_add(R::from_usize(2)).0;
                update_register(machine, 1, link);
                Some(machine.registers()[*rs1])
            }
            Instruction::ADDI16SP { imm } => {
                let (value, _) = machine.registers()[SP].overflowing_add(R::from_i32(*imm));
                update_register(machine, SP, value);
                None
            }
            // NOP for now
            Instruction::SRLI64 { .. } => None,
            // NOP for now
            Instruction::SRAI64 { .. } => None,
            // NOP for now
            Instruction::SLLI64 { .. } => None,
            Instruction::NOP { .. } => None,
            Instruction::EBREAK => {
                machine.ebreak()?;
                None
            }
        };
        let default_next_pc = machine.pc().overflowing_add(R::from_usize(2)).0;
        machine.set_pc(next_pc.unwrap_or(default_next_pc));
        Ok(())
    }
}

#[allow(clippy::cyclomatic_complexity)]
pub fn factory<R: Register>(instruction_bits: u32) -> Option<GenericInstruction> {
    let bit_length = R::BITS;
    if bit_length != 32 && bit_length != 64 {
        return None;
    }
    let rv32 = bit_length == 32;
    let rv64 = bit_length == 64;
    let inst_opt = match instruction_bits & 0b_111_00000000000_11 {
        // == Quadrant 0
        0b_000_00000000000_00 => {
            let nzuimm = x(instruction_bits, 6, 1, 2)
                | x(instruction_bits, 5, 1, 3)
                | x(instruction_bits, 11, 2, 4)
                | x(instruction_bits, 7, 4, 6);
            if nzuimm != 0 {
                Some(Instruction::Uu(UtypeU {
                    rd: compact_register_number(instruction_bits, 2),
                    imm: nzuimm,
                    inst: UtypeUInstruction::ADDI4SPN,
                }))
            } else {
                // Illegal instruction
                None
            }
        }
        0b_001_00000000000_00 => Some(Instruction::Iu(ItypeU::new(
            instruction_bits,
            fld_uimmediate(instruction_bits),
            ItypeUInstruction::FLD,
        ))),
        0b_010_00000000000_00 => Some(Instruction::Iu(ItypeU::new(
            instruction_bits,
            sw_uimmediate(instruction_bits),
            ItypeUInstruction::LW,
        ))),
        0b_011_00000000000_00 => {
            if rv32 {
                Some(Instruction::Iu(ItypeU::new(
                    instruction_bits,
                    sw_uimmediate(instruction_bits),
                    ItypeUInstruction::FLW,
                )))
            } else {
                Some(Instruction::Iu(ItypeU::new(
                    instruction_bits,
                    fld_uimmediate(instruction_bits),
                    ItypeUInstruction::LD,
                )))
            }
        }
        // Reserved
        0b_100_00000000000_00 => None,
        0b_101_00000000000_00 => Some(Instruction::Su(StypeU::new(
            instruction_bits,
            fld_uimmediate(instruction_bits),
            StypeUInstruction::FSD,
        ))),
        0b_110_00000000000_00 => Some(Instruction::Su(StypeU::new(
            instruction_bits,
            sw_uimmediate(instruction_bits),
            StypeUInstruction::SW,
        ))),
        0b_111_00000000000_00 => {
            if rv32 {
                Some(Instruction::Su(StypeU::new(
                    instruction_bits,
                    sw_uimmediate(instruction_bits),
                    StypeUInstruction::FSW,
                )))
            } else {
                Some(Instruction::Su(StypeU::new(
                    instruction_bits,
                    fld_uimmediate(instruction_bits),
                    StypeUInstruction::SD,
                )))
            }
        }
        // == Quadrant 1
        0b_000_00000000000_01 => {
            let nzimm = immediate(instruction_bits);
            let rd = rd(instruction_bits);
            if nzimm != 0 && rd != 0 {
                Some(Instruction::I(Itype {
                    rd,
                    rs1: rd,
                    imm: nzimm,
                    inst: ItypeInstruction::ADDI,
                }))
            } else if nzimm == 0 && rd == 0 {
                Some(Instruction::NOP { imm: nzimm })
            } else {
                // Invalid instruction
                None
            }
        }
        0b_001_00000000000_01 => {
            if rv32 {
                Some(Instruction::JAL {
                    imm: j_immediate(instruction_bits),
                })
            } else {
                let rd = rd(instruction_bits);
                if rd != 0 {
                    Some(Instruction::I(Itype {
                        rd,
                        rs1: rd,
                        imm: immediate(instruction_bits),
                        inst: ItypeInstruction::ADDIW,
                    }))
                } else {
                    None
                }
            }
        }
        0b_010_00000000000_01 => {
            let rd = rd(instruction_bits);
            if rd != 0 {
                Some(Instruction::U(Utype {
                    rd,
                    imm: immediate(instruction_bits),
                    inst: UtypeInstruction::LI,
                }))
            } else {
                None
            }
        }
        0b_011_00000000000_01 => {
            let imm = immediate(instruction_bits) << 12;
            if imm != 0 {
                let rd = rd(instruction_bits);
                if rd == SP {
                    Some(Instruction::ADDI16SP {
                        imm: (x(instruction_bits, 6, 1, 4)
                            | x(instruction_bits, 2, 1, 5)
                            | x(instruction_bits, 5, 1, 6)
                            | x(instruction_bits, 3, 2, 7)
                            | xs(instruction_bits, 12, 1, 9)) as i32,
                    })
                } else if rd != 0 {
                    Some(Instruction::U(Utype {
                        rd,
                        imm,
                        inst: UtypeInstruction::LUI,
                    }))
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
                    Some(Instruction::SRLI64 { rd, rs1: rd })
                }
                // SRAI64
                0b_0_01_000_00000_00 if instruction_bits & 0b_111_00 == 0 => {
                    Some(Instruction::SRAI64 { rd, rs1: rd })
                }
                // SUB
                0b_0_11_000_00000_00 => Some(Instruction::R(Rtype {
                    rd,
                    rs1: rd,
                    rs2: compact_register_number(instruction_bits, 2),
                    inst: RtypeInstruction::SUB,
                })),
                // XOR
                0b_0_11_000_01000_00 => Some(Instruction::R(Rtype {
                    rd,
                    rs1: rd,
                    rs2: compact_register_number(instruction_bits, 2),
                    inst: RtypeInstruction::XOR,
                })),
                // OR
                0b_0_11_000_10000_00 => Some(Instruction::R(Rtype {
                    rd,
                    rs1: rd,
                    rs2: compact_register_number(instruction_bits, 2),
                    inst: RtypeInstruction::OR,
                })),
                // AND
                0b_0_11_000_11000_00 => Some(Instruction::R(Rtype {
                    rd,
                    rs1: rd,
                    rs2: compact_register_number(instruction_bits, 2),
                    inst: RtypeInstruction::AND,
                })),
                // SUBW
                0b_1_11_000_00000_00 if rv64 => Some(Instruction::R(Rtype {
                    rd,
                    rs1: rd,
                    rs2: compact_register_number(instruction_bits, 2),
                    inst: RtypeInstruction::SUBW,
                })),
                // ADDW
                0b_1_11_000_01000_00 if rv64 => Some(Instruction::R(Rtype {
                    rd,
                    rs1: rd,
                    rs2: compact_register_number(instruction_bits, 2),
                    inst: RtypeInstruction::ADDW,
                })),
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
                        (0b_00_000_00000_00, uimm) => Some(Instruction::Iu(ItypeU {
                            rd,
                            rs1: rd,
                            imm: uimm,
                            inst: ItypeUInstruction::SRLI,
                        })),
                        // Invalid instruction
                        (0b_01_000_00000_00, 0) => None,
                        // SRAI
                        (0b_01_000_00000_00, uimm) => Some(Instruction::Iu(ItypeU {
                            rd,
                            rs1: rd,
                            imm: uimm,
                            inst: ItypeUInstruction::SRAI,
                        })),
                        // ANDI
                        (0b_10_000_00000_00, _) => Some(Instruction::I(Itype {
                            rd,
                            rs1: rd,
                            imm: immediate(instruction_bits),
                            inst: ItypeInstruction::ANDI,
                        })),
                        _ => None,
                    }
                }
            }
        }
        0b_101_00000000000_01 => Some(Instruction::J {
            imm: j_immediate(instruction_bits),
        }),
        0b_110_00000000000_01 => Some(Instruction::BEQZ {
            rs1: compact_register_number(instruction_bits, 7),
            imm: b_immediate(instruction_bits),
        }),
        0b_111_00000000000_01 => Some(Instruction::BNEZ {
            rs1: compact_register_number(instruction_bits, 7),
            imm: b_immediate(instruction_bits),
        }),
        // == Quadrant 2
        0b_000_00000000000_10 => {
            let uimm = uimmediate(instruction_bits);
            let rd = rd(instruction_bits);
            if rd == 0 {
                // Reserved
                None
            } else if uimm != 0 {
                Some(Instruction::Iu(ItypeU {
                    rs1: rd,
                    rd,
                    imm: uimm,
                    inst: ItypeUInstruction::SLLI,
                }))
            } else {
                Some(Instruction::SLLI64 { rs1: rd, rd })
            }
        }
        0b_001_00000000000_10 => Some(Instruction::Uu(UtypeU {
            rd: rd(instruction_bits),
            imm: fldsp_uimmediate(instruction_bits),
            inst: UtypeUInstruction::FLDSP,
        })),
        0b_010_00000000000_10 => {
            let rd = rd(instruction_bits);
            if rd != 0 {
                Some(Instruction::Uu(UtypeU {
                    rd,
                    imm: lwsp_uimmediate(instruction_bits),
                    inst: UtypeUInstruction::LWSP,
                }))
            } else {
                // Reserved
                None
            }
        }
        0b_011_00000000000_10 => {
            if rv32 {
                Some(Instruction::Uu(UtypeU {
                    rd: rd(instruction_bits),
                    imm: lwsp_uimmediate(instruction_bits),
                    inst: UtypeUInstruction::FLWSP,
                }))
            } else {
                let rd = rd(instruction_bits);
                if rd != 0 {
                    Some(Instruction::Uu(UtypeU {
                        rd,
                        imm: fldsp_uimmediate(instruction_bits),
                        inst: UtypeUInstruction::LDSP,
                    }))
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
                        Some(Instruction::JR { rs1: rd })
                    } else {
                        Some(Instruction::MV { rd, rs2 })
                    }
                }
                0b_1_00000_00000_00 => {
                    let rd = rd(instruction_bits);
                    let rs2 = c_rs2(instruction_bits);
                    match (rd, rs2) {
                        (0, 0) => Some(Instruction::EBREAK),
                        (rs1, 0) => Some(Instruction::JALR { rs1 }),
                        (rd, rs2) if rd != 0 => Some(Instruction::R(Rtype {
                            rd,
                            rs1: rd,
                            rs2,
                            inst: RtypeInstruction::ADD,
                        })),
                        // Invalid instruction
                        _ => None,
                    }
                }
                _ => unreachable!(),
            }
        }
        0b_101_00000000000_10 => Some(Instruction::CSS(CSSformat {
            rs2: c_rs2(instruction_bits),
            imm: fsdsp_uimmediate(instruction_bits),
            inst: CSSformatInstruction::FSDSP,
        })),
        0b_110_00000000000_10 => Some(Instruction::CSS(CSSformat {
            rs2: c_rs2(instruction_bits),
            imm: swsp_uimmediate(instruction_bits),
            inst: CSSformatInstruction::SWSP,
        })),
        0b_111_00000000000_10 => {
            if rv32 {
                Some(Instruction::CSS(CSSformat {
                    rs2: c_rs2(instruction_bits),
                    imm: swsp_uimmediate(instruction_bits),
                    inst: CSSformatInstruction::FSWSP,
                }))
            } else {
                Some(Instruction::CSS(CSSformat {
                    rs2: c_rs2(instruction_bits),
                    imm: fsdsp_uimmediate(instruction_bits),
                    inst: CSSformatInstruction::SDSP,
                }))
            }
        }
        _ => None,
    };
    inst_opt.map(RVC)
}
