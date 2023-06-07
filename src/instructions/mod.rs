mod common;
mod execute;
mod register;
mod utils;

pub mod a;
pub mod ast;
pub mod b;
pub mod i;
pub mod m;
pub mod rvc;
pub mod tagged;

pub use self::register::Register;
use super::Error;
pub use ckb_vm_definitions::{
    instructions::{
        self as insts, instruction_opcode_name, Instruction, InstructionOpcode,
        MAXIMUM_BASIC_BLOCK_END_OPCODE, MINIMAL_BASIC_BLOCK_END_OPCODE, MINIMAL_OPCODE,
    },
    registers::REGISTER_ABI_NAMES,
};
use core::fmt;
pub use execute::{
    execute, execute_instruction, execute_with_thread, handle_invalid_op, Thread, ThreadFactory,
};

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
        match self.op() {
            // Branches are in fact of B-type, we reuse S-type in CKB-VM
            // since they share the same constructs after decoding, but
            // they have different encoding rules in texts.
            insts::OP_BEQ
            | insts::OP_BNE
            | insts::OP_BLT
            | insts::OP_BGE
            | insts::OP_BLTU
            | insts::OP_BGEU => write!(
                f,
                "{} {},{},{}",
                instruction_opcode_name(self.op()).to_lowercase(),
                REGISTER_ABI_NAMES[self.rs1()],
                REGISTER_ABI_NAMES[self.rs2()],
                self.immediate_s()
            ),
            _ => write!(
                f,
                "{} {},{}({})",
                instruction_opcode_name(self.op()).to_lowercase(),
                REGISTER_ABI_NAMES[self.rs2()],
                self.immediate_s(),
                REGISTER_ABI_NAMES[self.rs1()]
            ),
        }
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct R5type(pub Instruction);

impl R5type {
    pub fn new(
        op: InstructionOpcode,
        rd: RegisterIndex,
        rs1: RegisterIndex,
        rs2: RegisterIndex,
        rs3: RegisterIndex,
        rs4: RegisterIndex,
    ) -> Self {
        R5type(
            ((op as u64) >> 8 << 16)
                | (op as u8 as u64)
                | ((rd as u8 as u64) << 8)
                | ((rs1 as u8 as u64) << 32)
                | ((rs2 as u8 as u64) << 40)
                | ((rs3 as u8 as u64) << 48)
                | ((rs4 as u8 as u64) << 56),
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

    pub fn rs4(self) -> RegisterIndex {
        (self.0 >> 56) as u8 as RegisterIndex
    }
}

impl fmt::Display for R5type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {},{},{},{},{}",
            instruction_opcode_name(self.op()).to_lowercase(),
            REGISTER_ABI_NAMES[self.rd()],
            REGISTER_ABI_NAMES[self.rs1()],
            REGISTER_ABI_NAMES[self.rs2()],
            REGISTER_ABI_NAMES[self.rs3()],
            REGISTER_ABI_NAMES[self.rs4()]
        )
    }
}

pub fn is_slowpath_instruction(i: Instruction) -> bool {
    (i as u8 as u16) < MINIMAL_OPCODE
}

pub fn is_basic_block_end_instruction(i: Instruction) -> bool {
    let opcode = extract_opcode(i);
    (MINIMAL_BASIC_BLOCK_END_OPCODE..=MAXIMUM_BASIC_BLOCK_END_OPCODE).contains(&opcode)
        || is_slowpath_instruction(i)
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
    use super::i::factory;
    use super::*;
    use ckb_vm_definitions::{for_each_inst1, instructions::MAXIMUM_OPCODE};
    use std::cmp::{max, min};
    use std::mem::size_of;

    #[test]
    fn test_instruction_op_should_fit_in_byte() {
        assert_eq!(2, size_of::<InstructionOpcode>());
    }

    #[test]
    fn test_stype_display() {
        // This is "sd	a5,568(sp)"
        let sd_inst = 0x22f13c23;
        let decoded = factory::<u64>(sd_inst, u32::max_value()).expect("decoding");
        let stype = Stype(decoded);

        assert_eq!("sd a5,568(sp)", format!("{}", stype));

        // This is "beq	a0,a5,1012e"
        let sd_inst = 0xf4f500e3;
        let decoded = factory::<u64>(sd_inst, u32::max_value()).expect("decoding");
        let stype = Stype(decoded);

        assert_eq!("beq a0,a5,-192", format!("{}", stype));
    }

    macro_rules! update_min_opcode {
        ($name:ident, $real_name:ident, $code:expr, $x:ident) => {
            $x = min($code, $x);
        };
    }

    #[test]
    fn test_minimal_opcode_is_minimal() {
        let mut o = MINIMAL_OPCODE;
        for_each_inst1!(update_min_opcode, o);
        assert_eq!(MINIMAL_OPCODE, o);
    }

    macro_rules! update_max_opcode {
        ($name:ident, $real_name:ident, $code:expr, $x:ident) => {
            $x = max($code, $x);
        };
    }

    #[test]
    fn test_maximal_opcode_is_maximal() {
        let mut o = MAXIMUM_OPCODE;
        for_each_inst1!(update_max_opcode, o);
        assert_eq!(MAXIMUM_OPCODE, o);
    }

    #[test]
    fn test_basic_block_end_opcode_is_in_range() {
        for o in MINIMAL_OPCODE..=MAXIMUM_OPCODE {
            if is_basic_block_end_instruction(blank_instruction(o)) {
                assert!(
                    o >= MINIMAL_BASIC_BLOCK_END_OPCODE,
                    "Opcode {} ({}) is smaller than minimal basic block end opcode!",
                    o,
                    instruction_opcode_name(o)
                );
                assert!(
                    o <= MAXIMUM_BASIC_BLOCK_END_OPCODE,
                    "Opcode {} ({}) is bigger than maximum basic block end opcode!",
                    o,
                    instruction_opcode_name(o)
                );
            }
        }
    }

    macro_rules! test_opcode_with_last {
        ($name:ident, $real_name:ident, $code:expr, $last:ident) => {
            assert_eq!(
                $last + 1,
                $code,
                "Opcode {} ({}) does not follow last opcode!",
                stringify!($real_name),
                $code
            );
            $last = $code;
        };
    }

    #[test]
    fn test_opcodes_are_defined_seqentially() {
        let mut last = MINIMAL_OPCODE - 1;
        for_each_inst1!(test_opcode_with_last, last);
        assert_eq!(last, MAXIMUM_OPCODE);
    }
}
