use ckb_vm::instructions::{extract_opcode, insts};
use ckb_vm::registers::{A0, A7};
use ckb_vm::{Bytes, CoreMachine, Instruction, Memory, Register, SupportMachine, Syscalls};

pub fn instruction_cycles(i: Instruction) -> u64 {
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

pub struct DebugSyscall {}

impl<Mac: SupportMachine> Syscalls<Mac> for DebugSyscall {
    fn initialize(&mut self, _machine: &mut Mac) -> Result<(), ckb_vm::error::Error> {
        Ok(())
    }

    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, ckb_vm::error::Error> {
        let code = &machine.registers()[A7];
        if code.to_i32() != 2177 {
            return Ok(false);
        }

        let mut addr = machine.registers()[A0].to_u64();
        let mut buffer = Vec::new();

        loop {
            let byte = machine
                .memory_mut()
                .load8(&Mac::REG::from_u64(addr))?
                .to_u8();
            if byte == 0 {
                break;
            }
            buffer.push(byte);
            addr += 1;
        }

        let s = String::from_utf8(buffer).unwrap();
        println!("{:?}", s);

        Ok(true)
    }
}

#[cfg(has_aot)]
fn main_aot(code: Bytes, args: Vec<Bytes>) -> Result<(), Box<dyn std::error::Error>> {
    let mut aot_machine = ckb_vm::machine::aot::AotCompilingMachine::load(
        &code,
        Some(Box::new(instruction_cycles)),
        ckb_vm::ISA_IMC | ckb_vm::ISA_B | ckb_vm::ISA_MOP,
        ckb_vm::machine::VERSION1,
    )?;
    let aot_code = aot_machine.compile()?;
    let asm_core = ckb_vm::machine::asm::AsmCoreMachine::new(
        ckb_vm::ISA_IMC | ckb_vm::ISA_B | ckb_vm::ISA_MOP,
        ckb_vm::machine::VERSION1,
        u64::MAX,
    );
    let core =
        ckb_vm::DefaultMachineBuilder::<Box<ckb_vm::machine::asm::AsmCoreMachine>>::new(asm_core)
            .instruction_cycle_func(Box::new(instruction_cycles))
            .syscall(Box::new(DebugSyscall {}))
            .build();
    let mut machine = ckb_vm::machine::asm::AsmMachine::new(core, Some(&aot_code));
    machine.load_program(&code, &args)?;
    let exit = machine.run();
    let cycles = machine.machine.cycles();
    println!(
        "aot exit={:?} cycles={:?} r[a1]={:?}",
        exit,
        cycles,
        machine.machine.registers()[ckb_vm::registers::A1]
    );
    std::process::exit(exit? as i32);
}

#[cfg(all(has_asm, not(has_aot)))]
fn main_asm(code: Bytes, args: Vec<Bytes>) -> Result<(), Box<dyn std::error::Error>> {
    let asm_core = ckb_vm::machine::asm::AsmCoreMachine::new(
        ckb_vm::ISA_IMC | ckb_vm::ISA_B | ckb_vm::ISA_MOP,
        ckb_vm::machine::VERSION1,
        u64::MAX,
    );
    let core = ckb_vm::DefaultMachineBuilder::new(asm_core)
        .instruction_cycle_func(Box::new(instruction_cycles))
        .syscall(Box::new(DebugSyscall {}))
        .build();
    let mut machine = ckb_vm::machine::asm::AsmMachine::new(core, None);
    machine.load_program(&code, &args)?;
    let exit = machine.run();
    let cycles = machine.machine.cycles();
    println!(
        "asm exit={:?} cycles={:?} r[a1]={:?}",
        exit,
        cycles,
        machine.machine.registers()[ckb_vm::registers::A1]
    );
    std::process::exit(exit? as i32);
}

#[cfg(all(not(has_asm), not(has_aot)))]
fn main_int(code: Bytes, args: Vec<Bytes>) -> Result<(), Box<dyn std::error::Error>> {
    let core_machine = ckb_vm::DefaultCoreMachine::<u64, ckb_vm::SparseMemory<u64>>::new(
        ckb_vm::ISA_IMC | ckb_vm::ISA_B | ckb_vm::ISA_MOP,
        ckb_vm::machine::VERSION1,
        u64::MAX,
    );
    let machine_builder = ckb_vm::DefaultMachineBuilder::new(core_machine)
        .instruction_cycle_func(Box::new(instruction_cycles));
    let mut machine = machine_builder.syscall(Box::new(DebugSyscall {})).build();
    machine.load_program(&code, &args)?;
    let exit = machine.run();
    let cycles = machine.cycles();
    println!(
        "int exit={:?} cycles={:?} r[a1]={:?}",
        exit,
        cycles,
        machine.registers()[ckb_vm::registers::A1]
    );
    std::process::exit(exit? as i32);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let code = std::fs::read(&args[1])?.into();
    let riscv_args: Vec<Bytes> = if args.len() > 2 {
        (&args[2..]).into_iter().map(|s| s.clone().into()).collect()
    } else {
        Vec::new()
    };
    #[cfg(has_aot)]
    main_aot(code, riscv_args)?;
    #[cfg(all(has_asm, not(has_aot)))]
    main_asm(code, riscv_args)?;
    #[cfg(all(not(has_asm), not(has_aot)))]
    main_int(code, riscv_args)?;
    Ok(())
}
