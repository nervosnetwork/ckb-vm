use ckb_vm::cost_model::estimate_cycles;
use ckb_vm::registers::{A0, A7};
use ckb_vm::{
    Bytes, CoreMachine, DefaultMachineRunner, Memory, Register, SupportMachine, Syscalls,
};

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

#[cfg(has_asm)]
fn main_asm64(code: Bytes, args: Vec<Bytes>) -> Result<(), Box<dyn std::error::Error>> {
    let asm_core = <ckb_vm::machine::asm::AsmCoreMachine as SupportMachine>::new(
        ckb_vm::ISA_IMC | ckb_vm::ISA_B | ckb_vm::ISA_A | ckb_vm::ISA_MOP,
        ckb_vm::machine::VERSION2,
        u64::MAX,
    );
    let core = ckb_vm::machine::asm::AsmDefaultMachineBuilder::new(asm_core)
        .instruction_cycle_func(Box::new(estimate_cycles))
        .syscall(Box::new(DebugSyscall {}))
        .build();
    let mut machine = ckb_vm::machine::asm::AsmMachine::new(core);
    machine.load_program(&code, args.into_iter().map(Ok))?;
    let exit = machine.run();
    let cycles = machine.machine.cycles();
    println!(
        "asm64 exit={:?} cycles={:?} r[a1]={:?}",
        exit,
        cycles,
        machine.machine.registers()[ckb_vm::registers::A1]
    );
    std::process::exit(exit? as i32);
}

#[cfg(not(has_asm))]
fn main_asm64(_: Bytes, _: Vec<Bytes>) -> Result<(), Box<dyn std::error::Error>> {
    panic!("please use --features=asm to enable asm support.")
}

fn main_interpreter32(code: Bytes, args: Vec<Bytes>) -> Result<(), Box<dyn std::error::Error>> {
    let core_machine = ckb_vm::DefaultCoreMachine::<u32, ckb_vm::SparseMemory<u32>>::new(
        ckb_vm::ISA_IMC | ckb_vm::ISA_B | ckb_vm::ISA_A | ckb_vm::ISA_MOP,
        ckb_vm::machine::VERSION2,
        u64::MAX,
    );
    let machine_builder = ckb_vm::RustDefaultMachineBuilder::new(core_machine)
        .instruction_cycle_func(Box::new(estimate_cycles));
    let mut machine = machine_builder.syscall(Box::new(DebugSyscall {})).build();
    machine.load_program(&code, args.into_iter().map(Ok))?;
    let exit = machine.run();
    let cycles = machine.cycles();
    println!(
        "interpreter32 exit={:?} cycles={:?} r[a1]={:?}",
        exit,
        cycles,
        machine.registers()[ckb_vm::registers::A1]
    );
    std::process::exit(exit? as i32);
}

fn main_interpreter64(code: Bytes, args: Vec<Bytes>) -> Result<(), Box<dyn std::error::Error>> {
    let core_machine = ckb_vm::DefaultCoreMachine::<u64, ckb_vm::SparseMemory<u64>>::new(
        ckb_vm::ISA_IMC | ckb_vm::ISA_B | ckb_vm::ISA_A | ckb_vm::ISA_MOP,
        ckb_vm::machine::VERSION2,
        u64::MAX,
    );
    let machine_builder = ckb_vm::RustDefaultMachineBuilder::new(core_machine)
        .instruction_cycle_func(Box::new(estimate_cycles));
    let mut machine = machine_builder.syscall(Box::new(DebugSyscall {})).build();
    machine.load_program(&code, args.into_iter().map(Ok))?;
    let exit = machine.run();
    let cycles = machine.cycles();
    println!(
        "interpreter64 exit={:?} cycles={:?} r[a1]={:?}",
        exit,
        cycles,
        machine.registers()[ckb_vm::registers::A1]
    );
    std::process::exit(exit? as i32);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mode = String::from("asm64");
    let mut program_path = String::from("");
    let mut program_args: Vec<String> = vec![];
    {
        let mut ap = argparse::ArgumentParser::new();
        ap.set_description("CKB VM Runner");
        ap.refer(&mut mode).add_option(
            &["--mode"],
            argparse::Store,
            "[asm64, interpreter32, interpreter64]",
        );
        ap.refer(&mut program_path)
            .add_argument("program_path", argparse::Store, "program path");
        ap.refer(&mut program_args)
            .add_argument("program_args", argparse::List, "program args");
        ap.parse_args_or_exit();
    }
    let code = std::fs::read(&program_path)?.into();
    let program_args = program_args.into_iter().map(|e| e.into()).collect();
    match mode.as_str() {
        "asm64" => main_asm64(code, program_args)?,
        "interpreter32" => main_interpreter32(code, program_args)?,
        "interpreter64" => main_interpreter64(code, program_args)?,
        _ => panic!("mode should be one of [asm64, interpreter32, interpreter64]"),
    }
    Ok(())
}
