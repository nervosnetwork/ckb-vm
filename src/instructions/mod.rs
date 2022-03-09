mod common;
mod execute;
mod register;
mod utils;

pub mod ast;
pub mod b;
pub mod i;
pub mod m;
pub mod rvc;
pub mod tagged;

pub use self::register::Register;
use super::Error;
pub use ckb_vm_definitions::{
    instructions::{self as insts, instruction_opcode_name, Instruction, InstructionOpcode},
    registers::REGISTER_ABI_NAMES,
};
use core::fmt;
pub use execute::{execute, execute_instruction};

pub type RegisterIndex = usize;
pub type SImmediate = i32;
pub type UImmediate = u32;

#[inline(always)]
pub fn extract_opcode(i: Instruction) -> InstructionOpcode {
    (((i >> 8) & 0xff00) | (i & 0x00ff)) as u16
}

pub type InstructionFactory = fn(instruction_bits: u32, version: u32) -> Option<Instruction>;

// Blank instructions need no register indices nor immediates, they only have opcode
// and module bit set.
pub fn blank_instruction(op: InstructionOpcode) -> Instruction {
    (op as u64 >> 8 << 16) | (op as u64 & 0xff)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rtype(pub Instruction);

impl Rtype {
    pub fn new(
        op: InstructionOpcode,
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    ) -> Self {
        Rtype(
            (u64::from(op) >> 8 << 16)
                | u64::from(op as u8)
                | (u64::from(rd as u8) << 8)
                | (u64::from(rs1 as u8) << 32)
                | (u64::from(rs2 as u8) << 40),
        )
    }

    pub fn op(self) -> InstructionOpcode {
        ((self.0 >> 16 << 8) | (self.0 & 0xFF)) as InstructionOpcode
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

impl fmt::Display for Rtype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {},{},{}",
            instruction_opcode_name(self.op()).to_lowercase(),
            REGISTER_ABI_NAMES[self.rd()],
            REGISTER_ABI_NAMES[self.rs1()],
            REGISTER_ABI_NAMES[self.rs2()]
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Itype(pub Instruction);

impl Itype {
    pub fn new_u(
        op: InstructionOpcode,
        rd: RegisterIndex,
        rs1: RegisterIndex,
        immediate_u: UImmediate,
    ) -> Self {
        Itype(
            (u64::from(op) >> 8 << 16) |
            u64::from(op as u8) |
              (u64::from(rd as u8) << 8) |
              (u64::from(rs1 as u8) << 32) |
              // Per RISC-V spec, I-type uses 12 bits at most, so it's perfectly
              // fine we store them in 3-byte location.
              (u64::from(immediate_u) << 40),
        )
    }

    pub fn new_s(
        op: InstructionOpcode,
        rd: RegisterIndex,
        rs1: RegisterIndex,
        immediate_s: SImmediate,
    ) -> Self {
        Self::new_u(op, rd, rs1, immediate_s as UImmediate)
    }

    pub fn op(self) -> InstructionOpcode {
        ((self.0 >> 16 << 8) | (self.0 & 0xFF)) as InstructionOpcode
    }

    pub fn rd(self) -> RegisterIndex {
        (self.0 >> 8) as u8 as RegisterIndex
    }

    pub fn rs1(self) -> RegisterIndex {
        (self.0 >> 32) as u8 as RegisterIndex
    }

    pub fn immediate_u(self) -> UImmediate {
        self.immediate_s() as UImmediate
    }

    pub fn immediate_s(self) -> SImmediate {
        ((self.0 as i64) >> 40) as SImmediate
    }
}

impl fmt::Display for Itype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: there are 2 simplifications here:
        // 1. It will print `addi a1,s0,-64` as `addi a1,-64(s0)`, and also print
        // `ld ra,88(sp)` as `ld ra,88(sp)`
        // 2. It will always use signed immediate numbers.
        // It is debatable if we should do a per-instruction pattern match to show
        // more patterns.
        write!(
            f,
            "{} {},{}({})",
            instruction_opcode_name(self.op()).to_lowercase(),
            REGISTER_ABI_NAMES[self.rd()],
            self.immediate_s(),
            REGISTER_ABI_NAMES[self.rs1()]
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stype(pub Instruction);

impl Stype {
    pub fn new_u(
        op: InstructionOpcode,
        immediate_u: UImmediate,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    ) -> Self {
        Stype(
            (u64::from(op) >> 8 << 16) |
            u64::from(op as u8) |
              (u64::from(rs2 as u8) << 8) |
              (u64::from(rs1 as u8) << 32) |
              // Per RISC-V spec, S/B type uses 13 bits at most, so it's perfectly
              // fine we store them in 3-byte location.
              (u64::from(immediate_u) << 40),
        )
    }

    pub fn new_s(
        op: InstructionOpcode,
        immediate_s: SImmediate,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
    ) -> Self {
        Self::new_u(op, immediate_s as UImmediate, rs1, rs2)
    }

    pub fn op(self) -> InstructionOpcode {
        ((self.0 >> 16 << 8) | (self.0 & 0xFF)) as InstructionOpcode
    }

    pub fn rs1(self) -> RegisterIndex {
        (self.0 >> 32) as u8 as RegisterIndex
    }

    pub fn rs2(self) -> RegisterIndex {
        (self.0 >> 8) as u8 as RegisterIndex
    }

    pub fn immediate_u(self) -> UImmediate {
        self.immediate_s() as UImmediate
    }

    pub fn immediate_s(self) -> SImmediate {
        ((self.0 as i64) >> 40) as SImmediate
    }
}

impl fmt::Display for Stype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {},{},{}",
            instruction_opcode_name(self.op()).to_lowercase(),
            REGISTER_ABI_NAMES[self.rs1()],
            REGISTER_ABI_NAMES[self.rs2()],
            self.immediate_s()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Utype(pub Instruction);

impl Utype {
    pub fn new(op: InstructionOpcode, rd: RegisterIndex, immediate_u: UImmediate) -> Self {
        Utype(
            (u64::from(op) >> 8 << 16)
                | u64::from(op as u8)
                | (u64::from(rd as u8) << 8)
                | (u64::from(immediate_u) << 32),
        )
    }

    pub fn new_s(op: InstructionOpcode, rd: RegisterIndex, immediate_s: SImmediate) -> Self {
        Self::new(op, rd, immediate_s as UImmediate)
    }

    pub fn op(self) -> InstructionOpcode {
        ((self.0 >> 16 << 8) | (self.0 & 0xFF)) as InstructionOpcode
    }

    pub fn rd(self) -> RegisterIndex {
        (self.0 >> 8) as u8 as RegisterIndex
    }

    pub fn immediate_u(self) -> UImmediate {
        self.immediate_s() as UImmediate
    }

    pub fn immediate_s(self) -> SImmediate {
        ((self.0 as i64) >> 32) as SImmediate
    }
}

impl fmt::Display for Utype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {},{}",
            instruction_opcode_name(self.op()).to_lowercase(),
            REGISTER_ABI_NAMES[self.rd()],
            self.immediate_s()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct R4type(pub Instruction);

impl R4type {
    pub fn new(
        op: InstructionOpcode,
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
        rs3: RegisterIndex,
    ) -> Self {
        R4type(
            (u64::from(op) >> 8 << 16)
                | u64::from(op as u8)
                | (u64::from(rd as u8) << 8)
                | (u64::from(rs1 as u8) << 32)
                | (u64::from(rs2 as u8) << 40)
                | (u64::from(rs3 as u8) << 48),
        )
    }

    pub fn op(self) -> InstructionOpcode {
        ((self.0 >> 16 << 8) | (self.0 & 0xFF)) as InstructionOpcode
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

    pub fn rs3(self) -> RegisterIndex {
        (self.0 >> 48) as u8 as RegisterIndex
    }
}

impl fmt::Display for R4type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {},{},{},{}",
            instruction_opcode_name(self.op()).to_lowercase(),
            REGISTER_ABI_NAMES[self.rd()],
            REGISTER_ABI_NAMES[self.rs1()],
            REGISTER_ABI_NAMES[self.rs2()],
            REGISTER_ABI_NAMES[self.rs3()]
        )
    }
}

pub fn is_slowpath_instruction(i: Instruction) -> bool {
    i as u8 >= 0xF0
}

pub fn is_basic_block_end_instruction(i: Instruction) -> bool {
    matches!(
        extract_opcode(i),
        insts::OP_AUIPC
            | insts::OP_JALR
            | insts::OP_BEQ
            | insts::OP_BNE
            | insts::OP_BLT
            | insts::OP_BGE
            | insts::OP_BLTU
            | insts::OP_BGEU
            | insts::OP_ECALL
            | insts::OP_EBREAK
            | insts::OP_JAL
            | insts::OP_FAR_JUMP_ABS
            | insts::OP_FAR_JUMP_REL
    ) | is_slowpath_instruction(i)
}

#[inline(always)]
pub fn set_instruction_length_2(i: u64) -> u64 {
    i | 0x1000000
}

#[inline(always)]
pub fn set_instruction_length_4(i: u64) -> u64 {
    i | 0x2000000
}

#[inline(always)]
pub fn set_instruction_length_n(i: u64, n: u8) -> u64 {
    debug_assert!(n % 2 == 0);
    debug_assert!(n <= 30);
    i | ((n as u64 & 0x1f) >> 1 << 24)
}

#[inline(always)]
pub fn instruction_length(i: Instruction) -> u8 {
    (((i >> 24) & 0x0f) << 1) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn test_instruction_op_should_fit_in_byte() {
        assert_eq!(2, size_of::<InstructionOpcode>());
    }
}
