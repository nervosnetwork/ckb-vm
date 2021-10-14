use ckb_vm::registers::A7;
use ckb_vm::{CoreMachine, Register, SupportMachine, Syscalls};

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
        Ok(true)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let code = std::fs::read(&args[1])?.into();

    let core_machine = ckb_vm::DefaultCoreMachine::<u64, ckb_vm::SparseMemory<u64>>::new(
        ckb_vm::ISA_IMC | ckb_vm::ISA_B | ckb_vm::ISA_V,
        ckb_vm::machine::VERSION1,
        u64::MAX,
    );

    let machine_builder =
        ckb_vm::DefaultMachineBuilder::new(core_machine).instruction_cycle_func(Box::new(|_| 1));
    let mut machine = machine_builder.syscall(Box::new(CustomSyscall {})).build();
    machine.load_program(&code, &vec!["main".into()]).unwrap();

    let exit = machine.run();
    let cycles = machine.cycles();
    println!(
        "int exit={:?} cycles={:?} r[a1]={:?}",
        exit,
        cycles,
        machine.registers()[ckb_vm::registers::A1]
    );

    std::process::exit(exit.unwrap() as i32);
}
