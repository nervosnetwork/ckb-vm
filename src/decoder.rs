use ckb_vm_definitions::instructions::{self as insts};
use ckb_vm_definitions::registers::RA;

use crate::instructions::{
    b, extract_opcode, i, instruction_length, m, rvc, set_instruction_length_n, Instruction,
    InstructionFactory, Itype, R4type, Register, Rtype, Utype,
};
use crate::memory::Memory;
use crate::{Error, ISA_B, ISA_MOP, RISCV_PAGESIZE};

const RISCV_PAGESIZE_MASK: u64 = RISCV_PAGESIZE as u64 - 1;

#[derive(Default)]
pub struct Decoder {
    factories: Vec<InstructionFactory>,
    mop: bool,
}

impl Decoder {
    pub fn new() -> Decoder {
        Decoder {
            factories: vec![],
            mop: false,
        }
    }

    pub fn add_instruction_factory(&mut self, factory: InstructionFactory) {
        self.factories.push(factory);
    }

    // This method is used to decode instruction raw bits from memory pointed
    // by current PC. Right now we support 32-bit instructions and RVC compressed
    // instructions. In future version we might add support for longer instructions.
    //
    // This decode method actually leverages a trick from little endian encoding:
    // the format for a full 32 bit RISC-V instruction is as follows:
    //
    // WWWWWWWWZZZZZZZZYYYYYYYYXXXXXX11
    //
    // While the format for a 16 bit RVC RIST-V instruction is one of the following 3:
    //
    // YYYYYYYYXXXXXX00
    // YYYYYYYYXXXXXX01
    // YYYYYYYYXXXXXX10
    //
    // Here X, Y, Z and W stands for arbitrary bits.
    // However the above is the representation in a 16-bit or 32-bit integer, since
    // we are using little endian, in memory it's actually in following reversed order:
    //
    // XXXXXX11 YYYYYYYY ZZZZZZZZ WWWWWWWW
    // XXXXXX00 YYYYYYYY
    // XXXXXX01 YYYYYYYY
    // XXXXXX10 YYYYYYYY
    //
    // One observation here, is the first byte in memory is always the least
    // significant byte whether we load a 32-bit or 16-bit integer.
    // So when we are decoding an instruction, we can first load 2 bytes forming
    // a 16-bit integer, then we check the 2 least significant bits, if the 2 bitss
    // are 0b11, we know this is a 32-bit instruction, we should load another 2 bytes
    // from memory and concat the 2 16-bit integers into a full 32-bit integers.
    // Otherwise, we know we are loading a RVC integer, and we are done here.
    // Also, due to RISC-V encoding behavior, it's totally okay when we cast a 16-bit
    // RVC instruction into a 32-bit instruction, the meaning of the instruction stays
    // unchanged in the cast conversion.
    fn decode_bits<M: Memory>(&self, memory: &mut M, pc: u64) -> Result<u32, Error> {
        // when the address is not the last 2 bytes of an executable page,
        // use a faster path to load instruction bits
        if pc & RISCV_PAGESIZE_MASK < RISCV_PAGESIZE_MASK - 1 {
            let mut instruction_bits = memory.execute_load32(pc)?;
            if instruction_bits & 0x3 != 0x3 {
                instruction_bits &= 0xffff;
            }
            Ok(instruction_bits)
        } else {
            let mut instruction_bits = u32::from(memory.execute_load16(pc)?);
            if instruction_bits & 0x3 == 0x3 {
                instruction_bits |= u32::from(memory.execute_load16(pc + 2)?) << 16;
            }
            Ok(instruction_bits)
        }
    }

    pub fn decode_raw<M: Memory>(&self, memory: &mut M, pc: u64) -> Result<Instruction, Error> {
        let instruction_bits = self.decode_bits(memory, pc)?;
        for factory in &self.factories {
            if let Some(instruction) = factory(instruction_bits) {
                return Ok(instruction);
            }
        }
        Err(Error::InvalidInstruction(instruction_bits))
    }

    // Macro-Operation Fusion (also Macro-Op Fusion, MOP Fusion, or Macrofusion) is a hardware optimization technique found
    // in many modern microarchitectures whereby a series of adjacent macro-operations are merged into a single
    // macro-operation prior or during decoding. Those instructions are later decoded into fused-µOPs.
    //
    // - https://riscv.org/wp-content/uploads/2016/07/Tue1130celio-fusion-finalV2.pdf
    // - https://en.wikichip.org/wiki/macro-operation_fusion#Proposed_fusion_operations
    // - https://carrv.github.io/2017/papers/clark-rv8-carrv2017.pdf
    pub fn decode_mop<M: Memory>(&self, memory: &mut M, pc: u64) -> Result<Instruction, Error> {
        let head_instruction = self.decode_raw(memory, pc)?;
        let head_opcode = extract_opcode(head_instruction);
        match head_opcode {
            insts::OP_LUI => {
                let head_inst = Utype(head_instruction);
                let head_size = instruction_length(head_instruction);
                let next_instruction_opt = self.decode_raw(memory, pc + head_size as u64);
                if next_instruction_opt.is_err() {
                    return Ok(head_instruction);
                }
                let next_instruction = next_instruction_opt.unwrap();
                let next_opcode = extract_opcode(next_instruction);
                match next_opcode {
                    insts::OP_JALR => {
                        let next_inst = Itype(next_instruction);
                        if next_inst.rs1() == head_inst.rd() && next_inst.rd() == RA {
                            let fuze_imm = head_inst.immediate_s() + next_inst.immediate_s();
                            let fuze_inst = Utype::new_s(insts::OP_FAR_JUMP_ABS, RA, fuze_imm);
                            let next_size = instruction_length(next_instruction);
                            let fuze_size = head_size + next_size;
                            Ok(set_instruction_length_n(fuze_inst.0, fuze_size))
                        } else {
                            Ok(head_instruction)
                        }
                    }
                    insts::OP_ADDIW => {
                        let next_inst = Itype(next_instruction);
                        if next_inst.rs1() == next_inst.rd() && next_inst.rd() == head_inst.rd() {
                            let fuze_imm = head_inst.immediate_s() + next_inst.immediate_s();
                            let fuze_inst = Utype::new_s(
                                insts::OP_LD_SIGN_EXTENDED_32_CONSTANT,
                                head_inst.rd(),
                                fuze_imm,
                            );
                            let next_size = instruction_length(next_instruction);
                            let fuze_size = head_size + next_size;
                            Ok(set_instruction_length_n(fuze_inst.0, fuze_size))
                        } else {
                            Ok(head_instruction)
                        }
                    }
                    _ => Ok(head_instruction),
                }
            }
            insts::OP_AUIPC => {
                let head_inst = Utype(head_instruction);
                let head_size = instruction_length(head_instruction);
                let next_instruction_opt = self.decode_raw(memory, pc + head_size as u64);
                if next_instruction_opt.is_err() {
                    return Ok(head_instruction);
                }
                let next_instruction = next_instruction_opt.unwrap();
                let next_opcode = extract_opcode(next_instruction);
                match next_opcode {
                    insts::OP_JALR => {
                        let next_inst = Itype(next_instruction);
                        if next_inst.rs1() == head_inst.rd() && next_inst.rd() == RA {
                            let fuze_imm = head_inst.immediate_s() + next_inst.immediate_s();
                            let fuze_inst = Utype::new_s(insts::OP_FAR_JUMP_REL, RA, fuze_imm);
                            let next_size = instruction_length(next_instruction);
                            let fuze_size = head_size + next_size;
                            Ok(set_instruction_length_n(fuze_inst.0, fuze_size))
                        } else {
                            Ok(head_instruction)
                        }
                    }
                    _ => Ok(head_instruction),
                }
            }
            insts::OP_MULH => {
                let head_inst = Rtype(head_instruction);
                let head_size = instruction_length(head_instruction);
                let next_instruction_opt = self.decode_raw(memory, pc + head_size as u64);
                if next_instruction_opt.is_err() {
                    return Ok(head_instruction);
                }
                let next_instruction = next_instruction_opt.unwrap();
                let next_opcode = extract_opcode(next_instruction);
                match next_opcode {
                    insts::OP_MUL => {
                        let next_inst = Rtype(next_instruction);
                        if head_inst.rd() != head_inst.rs1()
                            && head_inst.rd() != head_inst.rs2()
                            && head_inst.rs1() == next_inst.rs1()
                            && head_inst.rs2() == next_inst.rs2()
                        {
                            let next_size = instruction_length(next_instruction);
                            let fuze_inst = R4type::new(
                                insts::OP_WIDE_MUL,
                                head_inst.rd(),
                                head_inst.rs1(),
                                head_inst.rs2(),
                                next_inst.rd(),
                            );
                            let fuze_size = head_size + next_size;
                            Ok(set_instruction_length_n(fuze_inst.0, fuze_size))
                        } else {
                            Ok(head_instruction)
                        }
                    }
                    _ => Ok(head_instruction),
                }
            }
            insts::OP_MULHU => {
                let head_inst = Rtype(head_instruction);
                let head_size = instruction_length(head_instruction);
                let next_instruction_opt = self.decode_raw(memory, pc + head_size as u64);
                if next_instruction_opt.is_err() {
                    return Ok(head_instruction);
                }
                let next_instruction = next_instruction_opt.unwrap();
                let next_opcode = extract_opcode(next_instruction);
                match next_opcode {
                    insts::OP_MUL => {
                        let next_inst = Rtype(next_instruction);
                        if head_inst.rd() != head_inst.rs1()
                            && head_inst.rd() != head_inst.rs2()
                            && head_inst.rs1() == next_inst.rs1()
                            && head_inst.rs2() == next_inst.rs2()
                        {
                            let next_size = instruction_length(next_instruction);
                            let fuze_inst = R4type::new(
                                insts::OP_WIDE_MULU,
                                head_inst.rd(),
                                head_inst.rs1(),
                                head_inst.rs2(),
                                next_inst.rd(),
                            );
                            let fuze_size = head_size + next_size;
                            Ok(set_instruction_length_n(fuze_inst.0, fuze_size))
                        } else {
                            Ok(head_instruction)
                        }
                    }
                    _ => Ok(head_instruction),
                }
            }
            insts::OP_MULHSU => {
                let head_inst = Rtype(head_instruction);
                let head_size = instruction_length(head_instruction);
                let next_instruction_opt = self.decode_raw(memory, pc + head_size as u64);
                if next_instruction_opt.is_err() {
                    return Ok(head_instruction);
                }
                let next_instruction = next_instruction_opt.unwrap();
                let next_opcode = extract_opcode(next_instruction);
                match next_opcode {
                    insts::OP_MUL => {
                        let next_inst = Rtype(next_instruction);
                        if head_inst.rd() != head_inst.rs1()
                            && head_inst.rd() != head_inst.rs2()
                            && head_inst.rs1() == next_inst.rs1()
                            && head_inst.rs2() == next_inst.rs2()
                        {
                            let next_size = instruction_length(next_instruction);
                            let fuze_inst = R4type::new(
                                insts::OP_WIDE_MULSU,
                                head_inst.rd(),
                                head_inst.rs1(),
                                head_inst.rs2(),
                                next_inst.rd(),
                            );
                            let fuze_size = head_size + next_size;
                            Ok(set_instruction_length_n(fuze_inst.0, fuze_size))
                        } else {
                            Ok(head_instruction)
                        }
                    }
                    _ => Ok(head_instruction),
                }
            }
            insts::OP_DIV => {
                let head_inst = Rtype(head_instruction);
                let head_size = instruction_length(head_instruction);
                let next_instruction_opt = self.decode_raw(memory, pc + head_size as u64);
                if next_instruction_opt.is_err() {
                    return Ok(head_instruction);
                }
                let next_instruction = next_instruction_opt.unwrap();
                let next_opcode = extract_opcode(next_instruction);
                match next_opcode {
                    insts::OP_REM => {
                        let next_inst = Rtype(next_instruction);
                        if head_inst.rd() != head_inst.rs1()
                            && head_inst.rd() != head_inst.rs2()
                            && head_inst.rs1() == next_inst.rs1()
                            && head_inst.rs2() == next_inst.rs2()
                        {
                            let next_size = instruction_length(next_instruction);
                            let fuze_inst = R4type::new(
                                insts::OP_WIDE_DIV,
                                head_inst.rd(),
                                head_inst.rs1(),
                                head_inst.rs2(),
                                next_inst.rd(),
                            );
                            let fuze_size = head_size + next_size;
                            Ok(set_instruction_length_n(fuze_inst.0, fuze_size))
                        } else {
                            Ok(head_instruction)
                        }
                    }
                    _ => Ok(head_instruction),
                }
            }
            insts::OP_DIVU => {
                let head_inst = Rtype(head_instruction);
                let head_size = instruction_length(head_instruction);
                let next_instruction_opt = self.decode_raw(memory, pc + head_size as u64);
                if next_instruction_opt.is_err() {
                    return Ok(head_instruction);
                }
                let next_instruction = next_instruction_opt.unwrap();
                let next_opcode = extract_opcode(next_instruction);
                match next_opcode {
                    insts::OP_REMU => {
                        let next_inst = Rtype(next_instruction);
                        if head_inst.rd() != head_inst.rs1()
                            && head_inst.rd() != head_inst.rs2()
                            && head_inst.rs1() == next_inst.rs1()
                            && head_inst.rs2() == next_inst.rs2()
                        {
                            let next_size = instruction_length(next_instruction);
                            let fuze_inst = R4type::new(
                                insts::OP_WIDE_DIVU,
                                head_inst.rd(),
                                head_inst.rs1(),
                                head_inst.rs2(),
                                next_inst.rd(),
                            );
                            let fuze_size = head_size + next_size;
                            Ok(set_instruction_length_n(fuze_inst.0, fuze_size))
                        } else {
                            Ok(head_instruction)
                        }
                    }
                    _ => Ok(head_instruction),
                }
            }
            _ => Ok(head_instruction),
        }
    }

    pub fn decode<M: Memory>(&self, memory: &mut M, pc: u64) -> Result<Instruction, Error> {
        if self.mop {
            self.decode_mop(memory, pc)
        } else {
            self.decode_raw(memory, pc)
        }
    }
}

pub fn build_decoder<R: Register>(isa: u8) -> Decoder {
    let mut decoder = Decoder::new();
    decoder.add_instruction_factory(rvc::factory::<R>);
    decoder.add_instruction_factory(i::factory::<R>);
    decoder.add_instruction_factory(m::factory::<R>);
    if isa & ISA_B != 0 {
        decoder.add_instruction_factory(b::factory::<R>);
    }
    decoder.mop = isa & ISA_MOP != 0;
    decoder
}
