mod common;
mod execute;
mod register;
mod utils;

pub mod ast;
pub mod b;
pub mod i;
pub mod m;
pub mod rvc;
pub mod v;
pub mod v_register;

pub use self::register::Register;
pub use self::v_register::{VRegister, U1024, U256, U512};
use super::Error;
pub use ckb_vm_definitions::instructions::{
    self as insts, instruction_opcode_name, Instruction, InstructionOpcode,
};
pub use execute::{execute, execute_instruction};

type RegisterIndex = usize;
type Immediate = i32;
type SImmediate = Immediate;
type UImmediate = u32;

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
            (u64::from(op) >> 8 << 16) |
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
        ((self.0 >> 16 << 8) | (self.0 & 0xFF)) as InstructionOpcode
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
            (u64::from(op) >> 8 << 16) |
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
        ((self.0 >> 16 << 8) | (self.0 & 0xFF)) as InstructionOpcode
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
        Utype(
            (u64::from(op) >> 8 << 16)
                | u64::from(op as u8)
                | (u64::from(rd as u8) << 8)
                | (u64::from(immediate) << 32),
        )
    }

    pub fn new_s(op: InstructionOpcode, rd: RegisterIndex, immediate: Immediate) -> Self {
        Self::new(op, rd, immediate as UImmediate)
    }

    pub fn op(self) -> InstructionOpcode {
        ((self.0 >> 16 << 8) | (self.0 & 0xFF)) as InstructionOpcode
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct VVtype(pub Instruction);

impl VVtype {
    pub fn new(
        op: InstructionOpcode,
        vd: RegisterIndex,
        vs1: RegisterIndex,
        vs2: RegisterIndex,
        vm: bool,
    ) -> Self {
        let a = u64::from(op as u8);
        let b = u64::from(op) >> 8 << 16;
        let c = u64::from(vd as u8) << 8;
        let d = u64::from(vs1 as u8) << 32;
        let e = u64::from(vs2 as u8) << 40;
        let f = if vm { 1u64 << 28 } else { 0 };
        VVtype(a | b | c | d | e | f)
    }

    pub fn op(self) -> InstructionOpcode {
        ((self.0 >> 16 << 8) | (self.0 & 0xFF)) as InstructionOpcode
    }

    pub fn vd(self) -> RegisterIndex {
        (self.0 >> 8) as u8 as RegisterIndex
    }

    pub fn vs1(self) -> RegisterIndex {
        (self.0 >> 32) as u8 as RegisterIndex
    }

    pub fn vs2(self) -> RegisterIndex {
        (self.0 >> 40) as u8 as RegisterIndex
    }

    pub fn vm(self) -> bool {
        self.0 & 1u64 << 28 != 0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VXtype(pub Instruction);

impl VXtype {
    pub fn new(
        op: InstructionOpcode,
        vd: RegisterIndex,
        rs1: RegisterIndex,
        vs2: RegisterIndex,
        vm: bool,
    ) -> Self {
        let a = u64::from(op as u8);
        let b = u64::from(op) >> 8 << 16;
        let c = u64::from(vd as u8) << 8;
        let d = u64::from(rs1 as u8) << 32;
        let e = u64::from(vs2 as u8) << 40;
        let f = if vm { 1u64 << 28 } else { 0 };
        VXtype(a | b | c | d | e | f)
    }

    pub fn op(self) -> InstructionOpcode {
        ((self.0 >> 16 << 8) | (self.0 & 0xFF)) as InstructionOpcode
    }

    pub fn vd(self) -> RegisterIndex {
        (self.0 >> 8) as u8 as RegisterIndex
    }

    pub fn rs1(self) -> RegisterIndex {
        (self.0 >> 32) as u8 as RegisterIndex
    }

    pub fn vs2(self) -> RegisterIndex {
        (self.0 >> 40) as u8 as RegisterIndex
    }

    pub fn vm(self) -> bool {
        self.0 & 1u64 << 28 != 0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VItype(pub Instruction);

impl VItype {
    pub fn new(
        op: InstructionOpcode,
        vd: RegisterIndex,
        vs2: RegisterIndex,
        imm: UImmediate,
        vm: bool,
    ) -> Self {
        let a = u64::from(op as u8);
        let b = u64::from(op) >> 8 << 16;
        let c = u64::from(vd as u8) << 8;
        let d = u64::from(vs2 as u8) << 32;
        let e = u64::from(imm) << 40;
        let f = if vm { 1u64 << 28 } else { 0 };
        VItype(a | b | c | d | e | f)
    }

    pub fn op(self) -> InstructionOpcode {
        ((self.0 >> 16 << 8) | (self.0 & 0xFF)) as InstructionOpcode
    }

    pub fn vd(self) -> RegisterIndex {
        (self.0 >> 8) as u8 as RegisterIndex
    }

    pub fn vs2(self) -> RegisterIndex {
        (self.0 >> 32) as u8 as RegisterIndex
    }

    pub fn immediate_u(self) -> UImmediate {
        (self.0 >> 40) as u8 as u32
    }

    pub fn immediate_s(self) -> SImmediate {
        let u = self.immediate_u() as i32;
        if u >= 16 {
            u - 32
        } else {
            u
        }
    }

    pub fn vm(self) -> bool {
        self.0 & 1u64 << 28 != 0
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
