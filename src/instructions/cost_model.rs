use super::{extract_opcode, is_slowpath_opcode};
use ckb_vm_definitions::instructions::{self as insts, Instruction, InstructionOpcode};

fn v_instruction_cycles(
    opcode: InstructionOpcode,
    vl: u64,
    sew: u64,
    skip_counting: bool,
) -> Option<u64> {
    // Not V Instruction
    if !is_slowpath_opcode(opcode) {
        return None;
    }
    if skip_counting {
        return Some(0);
    }
    let base;
    match opcode {
        // `setvl` operations are constant, not changed by `vl` and `sew`
        insts::OP_VSETVLI | insts::OP_VSETIVLI | insts::OP_VSETVL => return Some(1),
        // vadd.vv as unit, 1
        insts::OP_VADD_VV => base = 1,
        // vmul.vv
        insts::OP_VMUL_VV => base = 4,
        // TODO: get base according to opcodes
        _ => base = 1,
    }
    let factor = if sew >= 64 { sew / 64 } else { 1 };
    Some(base * factor * vl)
}

/// Returns the spent cycles to execute the specific instruction.
pub fn instruction_cycles(i: Instruction, vl: u64, sew: u64, skip_counting: bool) -> u64 {
    match extract_opcode(i) {
        // IMC
        insts::OP_JALR => 3,
        insts::OP_LD => 2,
        insts::OP_LW => 3,
        insts::OP_LH => 3,
        insts::OP_LB => 3,
        insts::OP_LWU => 3,
        insts::OP_LHU => 3,
        insts::OP_LBU => 3,
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
        // MOP
        insts::OP_WIDE_MUL => 5,
        insts::OP_WIDE_MULU => 5,
        insts::OP_WIDE_MULSU => 5,
        insts::OP_WIDE_DIV => 32,
        insts::OP_WIDE_DIVU => 32,
        insts::OP_FAR_JUMP_REL => 3,
        insts::OP_FAR_JUMP_ABS => 3,
        opcode => {
            // RVV
            if let Some(cycles) = v_instruction_cycles(opcode, vl, sew, skip_counting) {
                cycles
            } else {
                1
            }
        }
    }
}
