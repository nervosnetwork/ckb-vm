#![cfg(has_asm)]

use ckb_vm::instructions::{extract_opcode, insts};
use ckb_vm::machine::asm::{AsmCoreMachine, AsmMachine};
use ckb_vm::machine::{DefaultMachineBuilder, VERSION1};
use ckb_vm::{Bytes, Instruction, ISA_B, ISA_IMC, ISA_MOP};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let flag_parser = clap::App::new("pprof_cycles")
        .version("0.1")
        .about("A pprof tool for CKB VM")
        .arg(
            clap::Arg::with_name("bin")
                .long("bin")
                .value_name("filename")
                .help("Specify the name of the executable")
                .required(true),
        )
        .arg(
            clap::Arg::with_name("arg")
                .long("arg")
                .value_name("arguments")
                .help("Pass arguments to binary")
                .multiple(true),
        )
        .get_matches();
    let fl_bin = flag_parser.value_of("bin").unwrap();
    let fl_arg: Vec<_> = flag_parser.values_of("arg").unwrap_or_default().collect();

    let code_data = std::fs::read(fl_bin)?;
    let code = Bytes::from(code_data);
    let mut args = vec![fl_bin.to_string().into()];
    args.append(&mut fl_arg.iter().map(|x| Bytes::from(x.to_string())).collect());

    let core_machine = AsmCoreMachine::new(ISA_IMC | ISA_B | ISA_MOP, VERSION1, u64::max_value());
    let wrap_machine = DefaultMachineBuilder::new(core_machine)
        .instruction_cycle_func(Box::new(instruction_cycles))
        .build();
    let mut machine = AsmMachine::new(wrap_machine, None);
    machine.load_program(&code, &args)?;
    machine.run()?;
    machine
        .pprof_logger
        .unwrap()
        .tree_root
        .borrow()
        .display_flamegraph("", &mut std::io::stdout());
    Ok(())
}

fn instruction_cycles(i: Instruction) -> u64 {
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
        _ => 1,
    }
}
