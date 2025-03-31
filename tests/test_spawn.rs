use bytes::Bytes;
use ckb_vm::cost_model::constant_cycles;
#[cfg(has_asm)]
use ckb_vm::machine::asm::{AsmCoreMachine, AsmDefaultMachineBuilder, AsmMachine};
use ckb_vm::machine::{DefaultCoreMachine, VERSION2, trace::TraceMachine};
use ckb_vm::memory::load_c_string_byte_by_byte;
use ckb_vm::registers::{A0, A1, A2, A7};
use ckb_vm::{
    DefaultMachineRunner, Error, FlattenedArgsReader, ISA_B, ISA_IMC, ISA_MOP, Register,
    RustDefaultMachineBuilder, SparseMemory, SupportMachine, Syscalls, WXorXMemory,
};
use std::sync::{Arc, Mutex};

// There is a spawn system call in ckb, we must ensure that in the worst case, ckb will not crashed by stack overflow.

fn stack_depth() -> u64 {
    let x = 0;
    unsafe {
        let raw: u64 = std::mem::transmute(&x);
        raw
    }
}

pub struct IntSpawnSyscall {
    min_sp: Arc<Mutex<u64>>,
}

impl<Mac: SupportMachine> Syscalls<Mac> for IntSpawnSyscall {
    fn initialize(&mut self, _machine: &mut Mac) -> Result<(), Error> {
        Ok(())
    }

    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, Error> {
        let code = &machine.registers()[A7];
        if code.to_i32() != 1001 {
            return Ok(false);
        }

        {
            let sp = stack_depth();
            let mut min_sp_lock = self.min_sp.lock().unwrap();
            if sp < *min_sp_lock {
                *min_sp_lock = sp
            }
        }

        let addr = machine.registers()[A0].clone();
        let path_byte = load_c_string_byte_by_byte(machine.memory_mut(), &addr).unwrap();
        let path = std::str::from_utf8(&path_byte).unwrap();
        let argc = machine.registers()[A1].clone();
        let argv = machine.registers()[A2].clone();
        let args_iter = FlattenedArgsReader::new(machine.memory_mut(), argc.clone(), argv);
        let buffer: Bytes = std::fs::read(path).unwrap().into();
        let machine_core = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
            ISA_IMC | ISA_B | ISA_MOP,
            VERSION2,
            u64::MAX,
        );
        let mut machine_child = TraceMachine::new(
            RustDefaultMachineBuilder::new(machine_core)
                .instruction_cycle_func(Box::new(constant_cycles))
                .syscall(Box::new(IntSpawnSyscall {
                    min_sp: self.min_sp.clone(),
                }))
                .build(),
        );
        machine_child.load_program(&buffer, args_iter).unwrap();
        let exit = machine_child.run().unwrap();
        machine.set_register(A0, Mac::REG::from_i8(exit));
        Ok(true)
    }
}

#[cfg(has_asm)]
pub struct AsmSpawnSyscall {
    min_sp: Arc<Mutex<u64>>,
}

#[cfg(has_asm)]
impl<Mac: SupportMachine> Syscalls<Mac> for AsmSpawnSyscall {
    fn initialize(&mut self, _machine: &mut Mac) -> Result<(), Error> {
        Ok(())
    }

    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, Error> {
        let code = &machine.registers()[A7];
        if code.to_i32() != 1001 {
            return Ok(false);
        }

        {
            let sp = stack_depth();
            let mut min_sp_lock = self.min_sp.lock().unwrap();
            if sp < *min_sp_lock {
                *min_sp_lock = sp
            }
        }

        let addr = machine.registers()[A0].clone();
        let path_byte = load_c_string_byte_by_byte(machine.memory_mut(), &addr).unwrap();
        let path = std::str::from_utf8(&path_byte).unwrap();
        let argc = machine.registers()[A1].clone();
        let argv = machine.registers()[A2].clone();
        let args_iter = FlattenedArgsReader::new(machine.memory_mut(), argc.clone(), argv);
        let buffer: Bytes = std::fs::read(path).unwrap().into();
        let machine_core_asm = AsmCoreMachine::new(ISA_IMC | ISA_B | ISA_MOP, VERSION2, u64::MAX);
        let machine_core = AsmDefaultMachineBuilder::new(machine_core_asm)
            .instruction_cycle_func(Box::new(constant_cycles))
            .syscall(Box::new(AsmSpawnSyscall {
                min_sp: self.min_sp.clone(),
            }))
            .build();
        let mut machine_child = AsmMachine::new(machine_core);
        machine_child.load_program(&buffer, args_iter).unwrap();
        let exit = machine_child.run().unwrap();
        machine.set_register(A0, Mac::REG::from_i8(exit));
        Ok(true)
    }
}

#[test]
pub fn test_spawn_int() {
    let buffer = std::fs::read("tests/programs/spawn").unwrap().into();
    let cur_sp = stack_depth();
    let min_sp = Arc::new(Mutex::new(u64::MAX));
    let machine_core = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
        ISA_IMC | ISA_B | ISA_MOP,
        VERSION2,
        u64::MAX,
    );
    let mut machine = TraceMachine::new(
        RustDefaultMachineBuilder::new(machine_core)
            .instruction_cycle_func(Box::new(constant_cycles))
            .syscall(Box::new(IntSpawnSyscall {
                min_sp: min_sp.clone(),
            }))
            .build(),
    );
    machine
        .load_program(&buffer, [Ok("main".into())].into_iter())
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert!(result.unwrap() == 0);
    // When the VM makes 64 recursive calls, make sure the stack is less than 1M.
    println!("stack size: {}", cur_sp - *min_sp.lock().unwrap());
    assert!((cur_sp - *min_sp.lock().unwrap()) < 1024 * 1024);
}

#[cfg(has_asm)]
#[test]
pub fn test_spawn_asm() {
    let buffer = std::fs::read("tests/programs/spawn").unwrap().into();
    let cur_sp = stack_depth();
    let min_sp = Arc::new(Mutex::new(u64::MAX));
    let machine_core_asm = AsmCoreMachine::new(ISA_IMC | ISA_B | ISA_MOP, VERSION2, u64::MAX);
    let machine_core = AsmDefaultMachineBuilder::new(machine_core_asm)
        .instruction_cycle_func(Box::new(constant_cycles))
        .syscall(Box::new(AsmSpawnSyscall {
            min_sp: min_sp.clone(),
        }))
        .build();
    let mut machine = AsmMachine::new(machine_core);
    machine
        .load_program(&buffer, [Ok("main".into())].into_iter())
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert!(result.unwrap() == 0);
    // When the VM makes 64 recursive calls, make sure the stack is less than 1M.
    println!("stack size: {}", cur_sp - *min_sp.lock().unwrap());
    assert!((cur_sp - *min_sp.lock().unwrap()) < 1024 * 1024);
}
