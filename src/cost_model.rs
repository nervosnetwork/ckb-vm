use crate::{
    instructions::{extract_opcode, insts},
    Instruction,
};

// Returns the spent cycles to execute the secific instruction.
// This function is usually used to write test cases, which can visually
// display how many instructions are executed.
pub fn constant_cycles(_: Instruction) -> u64 {
    1
}

// Returns the spent cycles to execute the secific instruction.
// These values come from estimates of hardware execution speed.
pub fn estimate_cycles(i: Instruction) -> u64 {
    match extract_opcode(i) {
        // IMC
        insts::OP_JALR_VERSION0 => 3,
        insts::OP_JALR_VERSION1 => 3,
        insts::OP_LD_VERSION0 => 2,
        insts::OP_LD_VERSION1 => 2,
        insts::OP_LW_VERSION0 => 3,
        insts::OP_LW_VERSION1 => 3,
        insts::OP_LH_VERSION0 => 3,
        insts::OP_LH_VERSION1 => 3,
        insts::OP_LB_VERSION0 => 3,
        insts::OP_LB_VERSION1 => 3,
        insts::OP_LWU_VERSION0 => 3,
        insts::OP_LWU_VERSION1 => 3,
        insts::OP_LHU_VERSION0 => 3,
        insts::OP_LHU_VERSION1 => 3,
        insts::OP_LBU_VERSION0 => 3,
        insts::OP_LBU_VERSION1 => 3,
        insts::OP_SB => 3,
        insts::OP_SH => 3,
        insts::OP_SW => 3,
        insts::OP_SD => 2,
        insts::OP_BEQ => 3,
        insts::OP_BGE => 3,
        insts::OP_BGEU => 3,
        insts::OP_BLT => 3,
        insts::OP_BLTU => 3,
        insts::OP_BNE => 3,
        insts::OP_EBREAK => 500,
        insts::OP_ECALL => 500,
        insts::OP_JAL => 3,
        insts::OP_MUL => 5,
        insts::OP_MULW => 5,
        insts::OP_MULH => 5,
        insts::OP_MULHU => 5,
        insts::OP_MULHSU => 5,
        insts::OP_DIV => 32,
        insts::OP_DIVW => 32,
        insts::OP_DIVU => 32,
        insts::OP_DIVUW => 32,
        insts::OP_REM => 32,
        insts::OP_REMW => 32,
        insts::OP_REMU => 32,
        insts::OP_REMUW => 32,
        // A
        insts::OP_LR_W => 4,
        insts::OP_SC_W => 4,
        insts::OP_AMOSWAP_W => 7,
        insts::OP_AMOADD_W => 7,
        insts::OP_AMOXOR_W => 7,
        insts::OP_AMOAND_W => 7,
        insts::OP_AMOOR_W => 7,
        insts::OP_AMOMIN_W => 7,
        insts::OP_AMOMAX_W => 7,
        insts::OP_AMOMINU_W => 7,
        insts::OP_AMOMAXU_W => 7,
        insts::OP_LR_D => 3,
        insts::OP_SC_D => 3,
        insts::OP_AMOSWAP_D => 5,
        insts::OP_AMOADD_D => 5,
        insts::OP_AMOXOR_D => 5,
        insts::OP_AMOAND_D => 5,
        insts::OP_AMOOR_D => 5,
        insts::OP_AMOMIN_D => 5,
        insts::OP_AMOMAX_D => 5,
        insts::OP_AMOMINU_D => 5,
        insts::OP_AMOMAXU_D => 5,
        // MOP
        insts::OP_WIDE_MUL => 5,
        insts::OP_WIDE_MULU => 5,
        insts::OP_WIDE_MULSU => 5,
        insts::OP_WIDE_DIV => 32,
        insts::OP_WIDE_DIVU => 32,
        insts::OP_FAR_JUMP_REL => 3,
        insts::OP_FAR_JUMP_ABS => 3,
        _ => 1,
    }
}
