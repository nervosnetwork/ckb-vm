use bytes::Bytes;
use ckb_vm::cost_model::constant_cycles;
#[cfg(has_asm)]
use ckb_vm::machine::asm::{AsmCoreMachine, AsmMachine};
use ckb_vm::machine::{trace::TraceMachine, DefaultCoreMachine, VERSION2};
use ckb_vm::registers::{A0, A1, A2, A7};
use ckb_vm::{
    DefaultMachineBuilder, Error, Memory, Register, SparseMemory, SupportMachine, Syscalls,
    WXorXMemory, ISA_A, ISA_B, ISA_IMC, ISA_MOP,
};
use std::sync::{Arc, Mutex};

// There is a spawn system call in ckb, we must ensure that in the worst case, ckb will not crashed by stack overflow.

pub fn load_c_string<Mac: SupportMachine>(machine: &mut Mac, addr: u64) -> Result<Bytes, Error> {
    let mut buffer = Vec::new();
    let mut addr = addr;

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

    Ok(Bytes::from(buffer))
}

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

        let addr = &machine.registers()[A0];
        let path_byte = load_c_string(machine, addr.to_u64()).unwrap();
        let path = std::str::from_utf8(&path_byte).unwrap();
        let argc = &machine.registers()[A1];
        let argv = &machine.registers()[A2];
        let mut addr = argv.to_u64();
        let mut argv_vec = Vec::new();
        for _ in 0..argc.to_u64() {
            let target_addr = machine
                .memory_mut()
                .load64(&Mac::REG::from_u64(addr))?
                .to_u64();
            let cstr = load_c_string(machine, target_addr)?;
            argv_vec.push(cstr);
            addr += 8;
        }
        let buffer: Bytes = std::fs::read(path).unwrap().into();
        let machine_core = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
            ISA_IMC | ISA_A | ISA_B | ISA_MOP,
            VERSION2,
            u64::max_value(),
        );
        let mut machine_child = TraceMachine::new(
            DefaultMachineBuilder::new(machine_core)
                .instruction_cycle_func(Box::new(constant_cycles))
                .syscall(Box::new(IntSpawnSyscall {
                    min_sp: self.min_sp.clone(),
                }))
                .build(),
        );
        machine_child.load_program(&buffer, &argv_vec).unwrap();
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

        let addr = &machine.registers()[A0];
        let path_byte = load_c_string(machine, addr.to_u64()).unwrap();
        let path = std::str::from_utf8(&path_byte).unwrap();
        let argc = &machine.registers()[A1];
        let argv = &machine.registers()[A2];
        let mut addr = argv.to_u64();
        let mut argv_vec = Vec::new();
        for _ in 0..argc.to_u64() {
            let target_addr = machine
                .memory_mut()
                .load64(&Mac::REG::from_u64(addr))?
                .to_u64();
            let cstr = load_c_string(machine, target_addr)?;
            argv_vec.push(cstr);
            addr += 8;
        }
        let buffer: Bytes = std::fs::read(path).unwrap().into();
        let machine_core_asm = AsmCoreMachine::new(
            ISA_IMC | ISA_A | ISA_B | ISA_MOP,
            VERSION2,
            u64::max_value(),
        );
        let machine_core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(machine_core_asm)
            .instruction_cycle_func(Box::new(constant_cycles))
            .syscall(Box::new(AsmSpawnSyscall {
                min_sp: self.min_sp.clone(),
            }))
            .build();
        let mut machine_child = AsmMachine::new(machine_core);
        machine_child.load_program(&buffer, &argv_vec).unwrap();
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
        ISA_IMC | ISA_A | ISA_B | ISA_MOP,
        VERSION2,
        u64::max_value(),
    );
    let mut machine = TraceMachine::new(
        DefaultMachineBuilder::new(machine_core)
            .instruction_cycle_func(Box::new(constant_cycles))
            .syscall(Box::new(IntSpawnSyscall {
                min_sp: min_sp.clone(),
            }))
            .build(),
    );
    machine.load_program(&buffer, &["main".into()]).unwrap();
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
    let machine_core_asm = AsmCoreMachine::new(
        ISA_IMC | ISA_A | ISA_B | ISA_MOP,
        VERSION2,
        u64::max_value(),
    );
    let machine_core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(machine_core_asm)
        .instruction_cycle_func(Box::new(constant_cycles))
        .syscall(Box::new(AsmSpawnSyscall {
            min_sp: min_sp.clone(),
        }))
        .build();
    let mut machine = AsmMachine::new(machine_core);
    machine.load_program(&buffer, &["main".into()]).unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert!(result.unwrap() == 0);
    // When the VM makes 64 recursive calls, make sure the stack is less than 1M.
    println!("stack size: {}", cur_sp - *min_sp.lock().unwrap());
    assert!((cur_sp - *min_sp.lock().unwrap()) < 1024 * 1024);
}
