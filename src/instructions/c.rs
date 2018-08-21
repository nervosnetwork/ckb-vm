use super::super::machine::Machine;
use super::super::memory::Memory;
use super::super::{Error, SP};
use super::utils::{rd, update_register, x, xs};
use super::{Instruction as GenericInstruction, Instruction::C};

// Notice the location of rs2 in RVC encoding is different from full encoding
#[inline(always)]
fn c_rs2(instruction_bits: u32) -> usize {
    x(instruction_bits, 2, 5, 0) as usize
}

// This function extract bits [15:13] and bits [1:0], then concat them
// into a 5 bit opcode to decode RVC instructions
#[inline(always)]
fn opcode(instruction_bits: u32) -> u32 {
    x(instruction_bits, 0, 2, 0) | x(instruction_bits, 13, 3, 2)
}

// This function extract bits [12:10] and bits [6:5], then concat them
// into a 5 bit opcode to decode RVC ALU instructions
#[inline(always)]
fn alu_opcode(instruction_bits: u32) -> u32 {
    x(instruction_bits, 5, 2, 0 ) | x(instruction_bits, 10, 3, 2)
}

// This function extract 3 bits from least_bit to form a register number,
// here since we are only using 3 bits, we can only reference the most popular
// used registers x8 - x15. In other words, a number of 0 extracted here means
// x8, 1 means x9, etc.
#[inline(always)]
fn compact_register_number(instruction_bits: u32, least_bit: usize) -> usize {
    x(instruction_bits, least_bit, 3, 0) as usize + 8
}

fn immediate(instruction_bits: u32) -> i32 {
    (x(instruction_bits, 2, 5, 0)
     | xs(instruction_bits, 12, 1, 5)) as i32
}

fn uimmediate(instruction_bits: u32) -> u32 {
    (x(instruction_bits, 2, 5, 0)
     | x(instruction_bits, 12, 1, 5)) as u32
}

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

fn sw_uimmediate(instruction_bits: u32) -> u32 {
    x(instruction_bits, 6, 1, 2)
        | x(instruction_bits, 10, 3, 3)
        | x(instruction_bits, 5, 1, 6)
}

fn lwsp_uimmediate(instruction_bits: u32) -> u32 {
    x(instruction_bits, 4, 3, 2)
        | x(instruction_bits, 12, 1, 5)
        | x(instruction_bits, 2, 2, 6)
}

fn b_immediate(instruction_bits: u32) -> i32 {
    (x(instruction_bits, 3, 2, 1)
     | x(instruction_bits, 10, 2, 3)
     | x(instruction_bits, 2, 1, 5)
     | x(instruction_bits, 5, 2, 6)
     | xs(instruction_bits, 12, 1, 8)) as i32
}

#[derive(Debug)]
pub enum Instruction {
    ADD { rd: usize, rs2: usize },
    ADDI { rd: usize, imm: i32 },
    ANDI { rd: usize, imm: i32 },
    BEQZ { rs1: usize, imm: i32 },
    BNEZ { rs1: usize, imm: i32 },
    LI { rd: usize, imm: i32 },
    J { imm: i32 },
    JAL { imm: i32 },
    JALR { rs1: usize },
    JR { rs1: usize },
    LUI { rd: usize, imm: i32 },
    LW { rd: usize, rs1: usize, uimm: u32 },
    LWSP { rd: usize, uimm: u32 },
    MV { rd: usize, rs2: usize },
    SRAI { rd: usize, uimm: u32 },
    SUB { rd: usize, rs2: usize },
    SW { rs1: usize, rs2: usize, uimm: u32 },
    SWSP { rs2: usize, uimm: u32 },
}

impl Instruction {
    pub fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<(), Error> {
        match self {
            Instruction::ADD { rd, rs2 } => {
                let rs2_value = machine.registers[*rs2];
                let (value, _) = machine.registers[*rd].overflowing_add(rs2_value);
                update_register(machine, *rd, value);
            },
            Instruction::ADDI { rd, imm } => {
                let (value, _) = machine.registers[*rd].overflowing_add(*imm as u32);
                update_register(machine, *rd, value);
            }
            Instruction::ANDI { rd, imm } => {
                let value = machine.registers[*rd] & (*imm as u32);
                update_register(machine, *rd, value);
            }
            Instruction::BEQZ { rs1, imm } => {
                if machine.registers[*rs1] == 0 {
                    let (value, _) = machine.pc.overflowing_add(*imm as u32);
                    machine.pc = value;
                    return Ok(());
                }
            },
            Instruction::BNEZ { rs1, imm } => {
                if machine.registers[*rs1] != 0 {
                    let (value, _) = machine.pc.overflowing_add(*imm as u32);
                    machine.pc = value;
                    return Ok(());
                }
            },
            Instruction::J { imm } => {
                let (value, _) = machine.pc.overflowing_add(*imm as u32);
                machine.pc = value;
                return Ok(());
            },
            Instruction::JAL { imm } => {
                let link = machine.pc + 2;
                let (value, _) = machine.pc.overflowing_add(*imm as u32);
                machine.pc = value;
                update_register(machine, 1, link);
                return Ok(());
            },
            Instruction::JALR { rs1 } => {
                let link = machine.pc + 2;
                let value = machine.registers[*rs1];
                machine.pc = value;
                update_register(machine, 1, link);
                return Ok(());
            },
            Instruction::JR { rs1 } => {
                let value = machine.registers[*rs1];
                machine.pc = value;
                return Ok(());
            },
            Instruction::LI { rd, imm } => {
                update_register(machine, *rd, *imm as u32);
            },
            Instruction::LUI { rd, imm } => {
                update_register(machine, *rd, *imm as u32);
            },
            Instruction::LW { rd, rs1, uimm } => {
                let (address, _) = machine.registers[*rs1].overflowing_add(*uimm);
                let value = machine.memory.load32(address as usize)?;
                update_register(machine, *rd, value);
            },
            Instruction::LWSP { rd, uimm } => {
                let (address, _) = machine.registers[SP].overflowing_add(*uimm);
                let value = machine.memory.load32(address as usize)?;
                println!("Loaded: 0x{:08X} from 0x{:08X}", value, address);
                update_register(machine, *rd, value);
            },
            Instruction::MV { rd, rs2 } => {
                let value = machine.registers[*rs2];
                update_register(machine, *rd, value);
            },
            Instruction::SRAI { rd, uimm } => {
                let value = (machine.registers[*rd] as i32) >> uimm;
                update_register(machine, *rd, value as u32);
            },
            Instruction::SUB { rd, rs2 } => {
                let (value, _) = machine.registers[*rd].overflowing_sub(machine.registers[*rs2]);
                update_register(machine, *rd, value);
            },
            Instruction::SW { rs1, rs2, uimm } => {
                let (address, _) = machine.registers[*rs1].overflowing_add(*uimm);
                let value = machine.registers[*rs2] as u32;
                machine.memory.store32(address as usize, value)?;
            },
            Instruction::SWSP { rs2, uimm } => {
                let (address, _) = machine.registers[SP].overflowing_add(*uimm);
                let value = machine.registers[*rs2] as u32;
                println!("Storing: 0x{:08X} at 0x{:08X}", value, address);
                machine.memory.store32(address as usize, value)?;
            },
        }
        machine.pc += 2;
        Ok(())
    }
}

pub fn factory(instruction_bits: u32) -> Option<GenericInstruction> {
    match opcode(instruction_bits) {
        0x1 => {
            let rd = rd(instruction_bits);
            let nzimm = immediate(instruction_bits);
            if rd != 0 && nzimm != 0 {
                Some(C(Instruction::ADDI {
                    rd,
                    imm: nzimm,
                }))
            } else {
                // TODO: C.NOP
                None
            }
        },
        0x5 => {
            Some(C(Instruction::JAL {
                imm: j_immediate(instruction_bits),
            }))
        },
        0x8 => {
            Some(C(Instruction::LW {
                rs1: compact_register_number(instruction_bits, 7),
                rd: compact_register_number(instruction_bits, 2),
                uimm: sw_uimmediate(instruction_bits),
            }))
        },
        0x9 => {
            let rd = rd(instruction_bits);
            if rd > 0 {
                Some(C(Instruction::LI {
                    rd,
                    imm: immediate(instruction_bits),
                }))
            } else {
                None
            }
        },
        0xa => {
            let rd = rd(instruction_bits);
            if rd > 0 {
                Some(C(Instruction::LWSP {
                    rd,
                    uimm: lwsp_uimmediate(instruction_bits),
                }))
            } else {
                None
            }
        }
        0xd => {
            let rd = rd(instruction_bits);
            let imm = immediate(instruction_bits) << 12;
            if rd != 0 && rd != 2 && imm != 0 {
                Some(C(Instruction::LUI { rd, imm }))
            } else {
                None
            }
        },
        0x11 => match alu_opcode(instruction_bits) {
            0xc => Some(C(Instruction::SUB {
                rd: compact_register_number(instruction_bits, 7),
                rs2: compact_register_number(instruction_bits, 2),
            })),
            _ => match x(instruction_bits, 10, 2, 0) {
                0x1 => {
                    let rd = rd(instruction_bits);
                    let uimm = uimmediate(instruction_bits);
                    if uimm != 0 {
                        Some(C(Instruction::SRAI { rd, uimm }))
                    } else {
                        // C.SRAI64
                        None
                    }
                }
                0x2 => Some(C(Instruction::ANDI {
                    rd: compact_register_number(instruction_bits, 7),
                    imm: immediate(instruction_bits),
                })),
                _ => None,
            },
        },
        0x12 => {
            let rs2 = c_rs2(instruction_bits);
            let rd_or_rs1 = rd(instruction_bits);
            if x(instruction_bits, 12, 1, 0) == 0 {
                if rd_or_rs1 != 0 && rs2 != 0 {
                    Some(C(Instruction::MV {
                        rd: rd_or_rs1,
                        rs2,
                    }))
                } else if rd_or_rs1 != 0 {
                    Some(C(Instruction::JR {
                        rs1: rd_or_rs1,
                    }))
                } else {
                    None
                }
            } else {
                if rd_or_rs1 != 0 && rs2 != 0 {
                    Some(C(Instruction::ADD {
                        rd: rd_or_rs1,
                        rs2,
                    }))
                } else if rd_or_rs1 != 0 {
                    Some(C(Instruction::JALR {
                        rs1: rd_or_rs1,
                    }))
                } else {
                    // TODO: implement C.EBREAK
                    None
                }
            }
        },
        0x15 => {
            Some(C(Instruction::J {
                imm: j_immediate(instruction_bits),
            }))
        },
        0x18 => {
            Some(C(Instruction::SW {
                rs1: compact_register_number(instruction_bits, 7),
                rs2: compact_register_number(instruction_bits, 2),
                uimm: sw_uimmediate(instruction_bits),
            }))
        },
        0x1a => Some(C(Instruction::SWSP {
            rs2: c_rs2(instruction_bits),
            uimm: x(instruction_bits, 9, 4, 2)
                | x(instruction_bits, 7, 2, 6),
        })),
        0x19 => Some(C(Instruction::BEQZ {
            rs1: compact_register_number(instruction_bits, 7),
            imm: b_immediate(instruction_bits),
        })),
        0x1d => Some(C(Instruction::BNEZ {
            rs1: compact_register_number(instruction_bits, 7),
            imm: b_immediate(instruction_bits),
        })),
        _ => None,
    }
}
