use super::super::machine::Machine;
use super::super::memory::Memory;
use super::super::Error;
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

#[derive(Debug)]
pub enum Instruction {
    ADD { rd: usize, rs2: usize },
    BNEZ { rs1: usize, imm: i32 },
    LI { rd: usize, imm: i32 },
    JAL { imm: i32 },
    MV { rd: usize, rs2: usize },
    SUB { rd: usize, rs2: usize },
}

impl Instruction {
    pub fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<(), Error> {
        match self {
            Instruction::ADD { rd, rs2 } => {
                let rs2_value = machine.registers[*rs2];
                let (value, _) = machine.registers[*rd].overflowing_add(rs2_value);
                update_register(machine, *rd, value);
            },
            Instruction::BNEZ { rs1, imm } => {
                if machine.registers[*rs1] != 0 {
                    let (value, _) = machine.pc.overflowing_add(*imm as u32);
                    machine.pc = value;
                    return Ok(());
                }
            },
            Instruction::JAL { imm } => {
                let link = machine.pc + 2;
                let (value, _) = machine.pc.overflowing_add(*imm as u32);
                machine.pc = value;
                update_register(machine, 1, link);
                return Ok(());
            },
            Instruction::LI { rd, imm } => {
                update_register(machine, *rd, *imm as u32);
            },
            Instruction::MV { rd, rs2 } => {
                let value = machine.registers[*rs2];
                update_register(machine, *rd, value);
            }
            Instruction::SUB { rd, rs2 } => {
                let (value, _) = machine.registers[*rd].overflowing_sub(machine.registers[*rs2]);
                update_register(machine, *rd, value);
            },
        }
        machine.pc += 2;
        Ok(())
    }
}

pub fn factory(instruction_bits: u32) -> Option<GenericInstruction> {
    match opcode(instruction_bits) {
        0x5 => {
            Some(C(Instruction::JAL {
                imm: (x(instruction_bits, 3, 3, 1)
                      | x(instruction_bits, 11, 1, 4)
                      | x(instruction_bits, 2, 1, 5)
                      | x(instruction_bits, 7, 1, 6)
                      | x(instruction_bits, 6, 1, 7)
                      | x(instruction_bits, 9, 2, 8)
                      | x(instruction_bits, 8, 1, 10)
                      | xs(instruction_bits, 12, 1, 11)) as i32,
            }))
        },
        0x9 => {
            let rd = rd(instruction_bits);
            if rd > 0 {
                Some(C(Instruction::LI {
                    rd: rd,
                    imm: (x(instruction_bits, 2, 5, 0)
                          | xs(instruction_bits, 12, 1, 5)) as i32,
                }))
            } else {
                None
            }
        },
        0x11 => match alu_opcode(instruction_bits) {
            0xc => Some(C(Instruction::SUB {
                rd: compact_register_number(instruction_bits, 7),
                rs2: compact_register_number(instruction_bits, 2),
            })),
            _ => None,
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
                    // TODO: implement C.JR
                    None
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
                    // TODO: implement C.JALR
                    None
                } else {
                    // TODO: implement C.EBREAK
                    None
                }
            }
        },
        0x1d => Some(C(Instruction::BNEZ {
            rs1: compact_register_number(instruction_bits, 7),
            imm: (x(instruction_bits, 3, 2, 1)
                  | x(instruction_bits, 10, 2, 3)
                  | x(instruction_bits, 2, 1, 5)
                  | x(instruction_bits, 5, 2, 6)
                  | xs(instruction_bits, 12, 1, 8)) as i32,
        })),
        _ => None,
    }
}
