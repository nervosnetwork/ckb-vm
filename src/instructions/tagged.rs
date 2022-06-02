use crate::{
    error::Error,
    instructions::{extract_opcode, insts, Instruction, Itype, R4type, Rtype, Stype, Utype},
};
use core::convert::TryFrom;
use core::fmt;

// This is used for generating human readable texts from RISC-V instructions.
// For performance reason, ckb-vm will not use this representation internally.
#[derive(Debug, Clone, PartialEq)]
pub enum TaggedInstruction {
    Rtype(Rtype),
    Itype(Itype),
    Stype(Stype),
    Utype(Utype),
    R4type(R4type),
}

impl fmt::Display for TaggedInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaggedInstruction::Rtype(i) => i.fmt(f),
            TaggedInstruction::Itype(i) => i.fmt(f),
            TaggedInstruction::Stype(i) => i.fmt(f),
            TaggedInstruction::Utype(i) => i.fmt(f),
            TaggedInstruction::R4type(i) => i.fmt(f),
        }
    }
}

impl From<Rtype> for TaggedInstruction {
    fn from(i: Rtype) -> Self {
        TaggedInstruction::Rtype(i)
    }
}

impl From<Itype> for TaggedInstruction {
    fn from(i: Itype) -> Self {
        TaggedInstruction::Itype(i)
    }
}

impl From<Stype> for TaggedInstruction {
    fn from(i: Stype) -> Self {
        TaggedInstruction::Stype(i)
    }
}

impl From<Utype> for TaggedInstruction {
    fn from(i: Utype) -> Self {
        TaggedInstruction::Utype(i)
    }
}

impl From<R4type> for TaggedInstruction {
    fn from(i: R4type) -> Self {
        TaggedInstruction::R4type(i)
    }
}

impl From<TaggedInstruction> for Instruction {
    fn from(t: TaggedInstruction) -> Self {
        match t {
            TaggedInstruction::Rtype(i) => i.0,
            TaggedInstruction::Itype(i) => i.0,
            TaggedInstruction::Stype(i) => i.0,
            TaggedInstruction::Utype(i) => i.0,
            TaggedInstruction::R4type(i) => i.0,
        }
    }
}

impl TryFrom<Instruction> for TaggedInstruction {
    type Error = Error;

    fn try_from(i: Instruction) -> Result<Self, Self::Error> {
        let op = extract_opcode(i);
        let tagged_inst = match op {
            insts::OP_UNLOADED => Rtype(i).into(),
            insts::OP_ECALL => Rtype(i).into(),
            insts::OP_EBREAK => Rtype(i).into(),
            insts::OP_FENCEI => Rtype(i).into(),
            insts::OP_FENCE => Rtype(i).into(),
            insts::OP_CUSTOM_TRACE_END => Rtype(i).into(),
            insts::OP_SUB => Rtype(i).into(),
            insts::OP_SUBW => Rtype(i).into(),
            insts::OP_ADD => Rtype(i).into(),
            insts::OP_ADDW => Rtype(i).into(),
            insts::OP_XOR => Rtype(i).into(),
            insts::OP_OR => Rtype(i).into(),
            insts::OP_AND => Rtype(i).into(),
            insts::OP_SLL => Rtype(i).into(),
            insts::OP_SLLW => Rtype(i).into(),
            insts::OP_SRL => Rtype(i).into(),
            insts::OP_SRLW => Rtype(i).into(),
            insts::OP_SRA => Rtype(i).into(),
            insts::OP_SRAW => Rtype(i).into(),
            insts::OP_SLT => Rtype(i).into(),
            insts::OP_SLTU => Rtype(i).into(),
            insts::OP_LB => Itype(i).into(),
            insts::OP_LH => Itype(i).into(),
            insts::OP_LW => Itype(i).into(),
            insts::OP_LD => Itype(i).into(),
            insts::OP_LBU => Itype(i).into(),
            insts::OP_LHU => Itype(i).into(),
            insts::OP_LWU => Itype(i).into(),
            insts::OP_ADDI => Itype(i).into(),
            insts::OP_ADDIW => Itype(i).into(),
            insts::OP_XORI => Itype(i).into(),
            insts::OP_ORI => Itype(i).into(),
            insts::OP_ANDI => Itype(i).into(),
            insts::OP_SLTI => Itype(i).into(),
            insts::OP_SLTIU => Itype(i).into(),
            insts::OP_JALR => Itype(i).into(),
            insts::OP_SLLI => Itype(i).into(),
            insts::OP_SRLI => Itype(i).into(),
            insts::OP_SRAI => Itype(i).into(),
            insts::OP_SLLIW => Itype(i).into(),
            insts::OP_SRLIW => Itype(i).into(),
            insts::OP_SRAIW => Itype(i).into(),
            insts::OP_SB => Stype(i).into(),
            insts::OP_SH => Stype(i).into(),
            insts::OP_SW => Stype(i).into(),
            insts::OP_SD => Stype(i).into(),
            insts::OP_BEQ => Stype(i).into(),
            insts::OP_BNE => Stype(i).into(),
            insts::OP_BLT => Stype(i).into(),
            insts::OP_BGE => Stype(i).into(),
            insts::OP_BLTU => Stype(i).into(),
            insts::OP_BGEU => Stype(i).into(),
            insts::OP_LUI => Utype(i).into(),
            insts::OP_AUIPC => Utype(i).into(),
            insts::OP_JAL => Utype(i).into(),
            insts::OP_MUL => Rtype(i).into(),
            insts::OP_MULW => Rtype(i).into(),
            insts::OP_MULH => Rtype(i).into(),
            insts::OP_MULHSU => Rtype(i).into(),
            insts::OP_MULHU => Rtype(i).into(),
            insts::OP_DIV => Rtype(i).into(),
            insts::OP_DIVW => Rtype(i).into(),
            insts::OP_DIVU => Rtype(i).into(),
            insts::OP_DIVUW => Rtype(i).into(),
            insts::OP_REM => Rtype(i).into(),
            insts::OP_REMW => Rtype(i).into(),
            insts::OP_REMU => Rtype(i).into(),
            insts::OP_REMUW => Rtype(i).into(),
            insts::OP_ADDUW => Rtype(i).into(),
            insts::OP_ANDN => Rtype(i).into(),
            insts::OP_BCLR => Rtype(i).into(),
            insts::OP_BCLRI => Itype(i).into(),
            insts::OP_BEXT => Rtype(i).into(),
            insts::OP_BEXTI => Itype(i).into(),
            insts::OP_BINV => Rtype(i).into(),
            insts::OP_BINVI => Itype(i).into(),
            insts::OP_BSET => Rtype(i).into(),
            insts::OP_BSETI => Itype(i).into(),
            insts::OP_CLMUL => Rtype(i).into(),
            insts::OP_CLMULH => Rtype(i).into(),
            insts::OP_CLMULR => Rtype(i).into(),
            insts::OP_CLZ => Rtype(i).into(),
            insts::OP_CLZW => Rtype(i).into(),
            insts::OP_CPOP => Rtype(i).into(),
            insts::OP_CPOPW => Rtype(i).into(),
            insts::OP_CTZ => Rtype(i).into(),
            insts::OP_CTZW => Rtype(i).into(),
            insts::OP_MAX => Rtype(i).into(),
            insts::OP_MAXU => Rtype(i).into(),
            insts::OP_MIN => Rtype(i).into(),
            insts::OP_MINU => Rtype(i).into(),
            insts::OP_ORCB => Rtype(i).into(),
            insts::OP_ORN => Rtype(i).into(),
            insts::OP_REV8 => Rtype(i).into(),
            insts::OP_ROL => Rtype(i).into(),
            insts::OP_ROLW => Rtype(i).into(),
            insts::OP_ROR => Rtype(i).into(),
            insts::OP_RORI => Itype(i).into(),
            insts::OP_RORIW => Itype(i).into(),
            insts::OP_RORW => Rtype(i).into(),
            insts::OP_SEXTB => Rtype(i).into(),
            insts::OP_SEXTH => Rtype(i).into(),
            insts::OP_SH1ADD => Rtype(i).into(),
            insts::OP_SH1ADDUW => Rtype(i).into(),
            insts::OP_SH2ADD => Rtype(i).into(),
            insts::OP_SH2ADDUW => Rtype(i).into(),
            insts::OP_SH3ADD => Rtype(i).into(),
            insts::OP_SH3ADDUW => Rtype(i).into(),
            insts::OP_SLLIUW => Itype(i).into(),
            insts::OP_XNOR => Rtype(i).into(),
            insts::OP_ZEXTH => Rtype(i).into(),
            insts::OP_WIDE_MUL => R4type(i).into(),
            insts::OP_WIDE_MULU => R4type(i).into(),
            insts::OP_WIDE_MULSU => R4type(i).into(),
            insts::OP_WIDE_DIV => R4type(i).into(),
            insts::OP_WIDE_DIVU => R4type(i).into(),
            insts::OP_FAR_JUMP_REL => Utype(i).into(),
            insts::OP_FAR_JUMP_ABS => Utype(i).into(),
            insts::OP_ADC => Rtype(i).into(),
            insts::OP_SBB => R4type(i).into(),
            insts::OP_LD_SIGN_EXTENDED_32_CONSTANT => Utype(i).into(),
            insts::OP_CUSTOM_LOAD_IMM => Utype(i).into(),
            _ => return Err(Error::InvalidOp(op)),
        };
        Ok(tagged_inst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::{blank_instruction, instruction_opcode_name};

    #[test]
    fn test_all_valid_opcodes_convert_to_tagged_instruction() {
        for i in insts::OP_UNLOADED..=insts::OP_CUSTOM_TRACE_END {
            let inst = blank_instruction(i);
            let result = TaggedInstruction::try_from(inst);
            assert!(
                result.is_ok(),
                "TaggedInstruction does not handle opcode {}({})!",
                i,
                instruction_opcode_name(i)
            );
        }
    }
}
