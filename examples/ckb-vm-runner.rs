use byteorder::WriteBytesExt;
use ckb_vm::instructions::cost_model::instruction_cycles;
use ckb_vm::registers::{A0, A7};
use ckb_vm::{Bytes, CoreMachine, Memory, Register, SupportMachine, Syscalls};
use std::io::Write;
use std::time::SystemTime;

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

        std::io::stdout().write(&buffer)?;
        if buffer.last().copied() != Some('\n' as u8) {
            std::io::stdout().write_u8('\n' as u8)?;
        }

        Ok(true)
    }
}

pub struct TimeSyscall {
    boot_time: SystemTime,
}

impl TimeSyscall {
    fn new() -> Self {
        Self {
            boot_time: SystemTime::now(),
        }
    }
}

impl<Mac: SupportMachine> Syscalls<Mac> for TimeSyscall {
    fn initialize(&mut self, _machine: &mut Mac) -> Result<(), ckb_vm::error::Error> {
        Ok(())
    }

    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, ckb_vm::error::Error> {
        let code = &machine.registers()[A7];
        if code.to_i32() != 9001 {
            return Ok(false);
        }
        let now = SystemTime::now();
        let d = now.duration_since(self.boot_time.clone()).unwrap();
        machine.set_register(A0, Mac::REG::from_u64(d.as_nanos() as u64));
        Ok(true)
    }
}

#[cfg(has_aot)]
fn main_aot(code: Bytes, args: Vec<Bytes>) -> Result<(), Box<dyn std::error::Error>> {
    let mut aot_machine = ckb_vm::machine::aot::AotCompilingMachine::load(
        &code,
        Some(Box::new(instruction_cycles)),
        ckb_vm::ISA_IMC | ckb_vm::ISA_B | ckb_vm::ISA_MOP | ckb_vm::ISA_V,
        ckb_vm::machine::VERSION1,
    )?;
    let aot_code = aot_machine.compile()?;
    let asm_core = ckb_vm::machine::asm::AsmCoreMachine::new(
        ckb_vm::ISA_IMC | ckb_vm::ISA_B | ckb_vm::ISA_MOP | ckb_vm::ISA_V,
        ckb_vm::machine::VERSION1,
        u64::MAX,
    );
    let asm_glue = ckb_vm::machine::asm::AsmGlueMachine::new(asm_core);
    let core = ckb_vm::DefaultMachineBuilder::new(asm_glue)
        .instruction_cycle_func(Box::new(instruction_cycles))
        .syscall(Box::new(DebugSyscall {}))
        .syscall(Box::new(TimeSyscall::new()))
        .build();
    let mut machine =
        ckb_vm::machine::asm::AsmMachine::new(core, Some(std::sync::Arc::new(aot_code)));
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
    use probe::probe;

    let asm_core = ckb_vm::machine::asm::AsmCoreMachine::new(
        ckb_vm::ISA_IMC | ckb_vm::ISA_B | ckb_vm::ISA_MOP | ckb_vm::ISA_V,
        ckb_vm::machine::VERSION1,
        u64::MAX,
    );
    let asm_glue = ckb_vm::machine::asm::AsmGlueMachine::new(asm_core);
    let ptr = (&asm_glue.imc.pc) as *const u64;
    probe!(default, machine_pc_location, ptr as isize);

    let core = ckb_vm::DefaultMachineBuilder::new(asm_glue)
        .instruction_cycle_func(Box::new(instruction_cycles))
        .syscall(Box::new(DebugSyscall {}))
        .syscall(Box::new(TimeSyscall::new()))
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
        ckb_vm::ISA_IMC | ckb_vm::ISA_B | ckb_vm::ISA_MOP | ckb_vm::ISA_V,
        ckb_vm::machine::VERSION1,
        u64::MAX,
    );
    let machine_builder = ckb_vm::DefaultMachineBuilder::new(core_machine)
        .instruction_cycle_func(Box::new(instruction_cycles));
    let mut machine = machine_builder
        .syscall(Box::new(DebugSyscall {}))
        .syscall(Box::new(TimeSyscall::new()))
        .build();
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
