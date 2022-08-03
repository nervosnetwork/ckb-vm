use ckb_vm::instructions::cost_model::instruction_cycles;
use ckb_vm::registers::{A0, A7};
use ckb_vm::{Bytes, CoreMachine, Memory, Register, SupportMachine, Syscalls};
use std::time::SystemTime;

pub struct CustomSyscall {}

impl<Mac: SupportMachine> Syscalls<Mac> for CustomSyscall {
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
        // Zlib::crc32("time") % 10000
        if code.to_i32() != 9285 {
            return Ok(false);
        }

        let now = SystemTime::now();
        let d = now
            .duration_since(self.boot_time.clone())
            .expect("clock goes backwards");

        machine.set_register(A0, Mac::REG::from_u64(d.as_nanos() as u64));

        Ok(true)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let code = std::fs::read(&args[1])?.into();
    let riscv_args: Vec<Bytes> = if args.len() > 2 {
        (&args[2..]).into_iter().map(|s| s.clone().into()).collect()
    } else {
        Vec::new()
    };
    let asm_core = ckb_vm::machine::asm::AsmCoreMachine::new(
        ckb_vm::ISA_IMC | ckb_vm::ISA_B | ckb_vm::ISA_V,
        ckb_vm::machine::VERSION1,
        u64::MAX,
    );
    let core = ckb_vm::DefaultMachineBuilder::new(asm_core.into())
        .instruction_cycle_func(Box::new(instruction_cycles))
        .syscall(Box::new(CustomSyscall {}))
        .syscall(Box::new(TimeSyscall::new()))
        .build();
    let mut machine = ckb_vm::machine::asm::v_trace::VTraceAsmMachine::new(core, None);

    cpuprofiler::PROFILER
        .lock()
        .unwrap()
        .start("./asm64-vtrace.profile")
        .unwrap();

    machine.load_program(&code, &riscv_args).unwrap();

    let exit = machine.run();
    let cycles = machine.machine.cycles();
    println!(
        "asm exit={:?} cycles={:?} r[a1]={:?}",
        exit,
        cycles,
        machine.machine.registers()[ckb_vm::registers::A1]
    );

    std::process::exit(exit.unwrap() as i32);
}
