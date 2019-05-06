mod common;
mod execute;
mod register;
mod utils;

pub mod i;
pub mod m;
pub mod rvc;

pub use self::register::Register;
use super::Error;
pub use ckb_vm_definitions::instructions::{
    self as insts, Instruction, InstructionOpcode, MAXIMUM_RVC_OPCODE, MINIMAL_RVC_OPCODE,
};
pub use execute::execute;

type RegisterIndex = usize;
type Immediate = i32;
type UImmediate = u32;

#[inline(always)]
pub fn extract_opcode(i: Instruction) -> InstructionOpcode {
    i as u8
}

pub type InstructionFactory = fn(instruction_bits: u32) -> Option<Instruction>;

// Blank instructions need no register indices nor immediates, they only have opcode
// and module bit set.
pub fn blank_instruction(op: InstructionOpcode) -> Instruction {
    u64::from(op as u8)
}

#[derive(Debug, Clone, Copy)]
pub struct Rtype(pub Instruction);

impl Rtype {
    pub fn new(
        op: InstructionOpcode,
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    ) -> Self {
        Rtype(
            u64::from(op as u8)
                | (u64::from(rd as u8) << 8)
                | (u64::from(rs1 as u8) << 32)
                | (u64::from(rs2 as u8) << 40),
        )
    }

    pub fn op(self) -> InstructionOpcode {
        self.0 as u8 as InstructionOpcode
    }

    pub fn rd(self) -> RegisterIndex {
        (self.0 >> 8) as u8 as RegisterIndex
    }

    pub fn rs1(self) -> RegisterIndex {
        (self.0 >> 32) as u8 as RegisterIndex
    }

    pub fn rs2(self) -> RegisterIndex {
        (self.0 >> 40) as u8 as RegisterIndex
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Itype(pub Instruction);

impl Itype {
    pub fn new(
        op: InstructionOpcode,
        rd: RegisterIndex,
        rs1: RegisterIndex,
        immediate: UImmediate,
    ) -> Self {
        Itype(
            u64::from(op as u8) |
              (u64::from(rd as u8) << 8) |
              (u64::from(rs1 as u8) << 32) |
              // Per RISC-V spec, I-type uses 12 bits at most, so it's perfectly
              // fine we store them in 3-byte location.
              (u64::from(immediate) << 40),
        )
    }

    pub fn new_s(
        op: InstructionOpcode,
        rd: RegisterIndex,
        rs1: RegisterIndex,
        immediate: Immediate,
    ) -> Self {
        Self::new(op, rd, rs1, immediate as UImmediate)
    }

    pub fn op(self) -> InstructionOpcode {
        self.0 as u8 as InstructionOpcode
    }

    pub fn rd(self) -> RegisterIndex {
        (self.0 >> 8) as u8 as RegisterIndex
    }

    pub fn rs1(self) -> RegisterIndex {
        (self.0 >> 32) as u8 as RegisterIndex
    }

    pub fn immediate(self) -> UImmediate {
        self.immediate_s() as UImmediate
    }

    pub fn immediate_s(self) -> Immediate {
        ((self.0 as i64) >> 40) as Immediate
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Stype(pub Instruction);

impl Stype {
    pub fn new(
        op: InstructionOpcode,
        immediate: UImmediate,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    ) -> Self {
        Stype(
            u64::from(op as u8) |
              (u64::from(rs2 as u8) << 8) |
              (u64::from(rs1 as u8) << 32) |
              // Per RISC-V spec, S/B type uses 13 bits at most, so it's perfectly
              // fine we store them in 3-byte location.
              (u64::from(immediate) << 40),
        )
    }

    pub fn new_s(
        op: InstructionOpcode,
        immediate: Immediate,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    ) -> Self {
        Self::new(op, immediate as UImmediate, rs1, rs2)
    }

    pub fn op(self) -> InstructionOpcode {
        self.0 as u8 as InstructionOpcode
    }

    pub fn rs1(self) -> RegisterIndex {
        (self.0 >> 32) as u8 as RegisterIndex
    }

    pub fn rs2(self) -> RegisterIndex {
        (self.0 >> 8) as u8 as RegisterIndex
    }

    pub fn immediate(self) -> UImmediate {
        self.immediate_s() as UImmediate
    }

    pub fn immediate_s(self) -> Immediate {
        ((self.0 as i64) >> 40) as Immediate
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Utype(pub Instruction);

impl Utype {
    pub fn new(op: InstructionOpcode, rd: RegisterIndex, immediate: UImmediate) -> Self {
        Utype(u64::from(op as u8) | (u64::from(rd as u8) << 8) | (u64::from(immediate) << 32))
    }

    pub fn new_s(op: InstructionOpcode, rd: RegisterIndex, immediate: Immediate) -> Self {
        Self::new(op, rd, immediate as UImmediate)
    }

    pub fn op(self) -> InstructionOpcode {
        self.0 as u8 as InstructionOpcode
    }

    pub fn rd(self) -> RegisterIndex {
        (self.0 >> 8) as u8 as RegisterIndex
    }

    pub fn immediate(self) -> UImmediate {
        self.immediate_s() as UImmediate
    }

    pub fn immediate_s(self) -> Immediate {
        ((self.0 as i64) >> 32) as Immediate
    }
}

pub fn is_basic_block_end_instruction(i: Instruction) -> bool {
    match extract_opcode(i) {
        insts::OP_AUIPC => true,
        insts::OP_JALR => true,
        insts::OP_BEQ => true,
        insts::OP_BNE => true,
        insts::OP_BLT => true,
        insts::OP_BGE => true,
        insts::OP_BLTU => true,
        insts::OP_BGEU => true,
        insts::OP_ECALL => true,
        insts::OP_EBREAK => true,
        insts::OP_JAL => true,
        insts::OP_RVC_EBREAK => true,
        insts::OP_RVC_BEQZ => true,
        insts::OP_RVC_BNEZ => true,
        insts::OP_RVC_J => true,
        insts::OP_RVC_JAL => true,
        insts::OP_RVC_JALR => true,
        insts::OP_RVC_JR => true,
        _ => false,
    }
}

#[inline(always)]
pub fn instruction_length(i: Instruction) -> usize {
    let o = extract_opcode(i);
    if o >= MINIMAL_RVC_OPCODE && o <= MAXIMUM_RVC_OPCODE {
        2
    } else {
        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn test_instruction_op_should_fit_in_byte() {
        assert_eq!(1, size_of::<InstructionOpcode>());
    }
}
