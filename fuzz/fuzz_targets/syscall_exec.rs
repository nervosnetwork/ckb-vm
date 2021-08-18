#![no_main]
use arbitrary::Arbitrary;
use ckb_vm::machine::{DefaultCoreMachine, DefaultMachineBuilder, SupportMachine, VERSION1};
use ckb_vm::registers::A7;
use ckb_vm::{Bytes, Error};
use ckb_vm::{
    Register, SparseMemory, Syscalls, WXorXMemory, DEFAULT_STACK_SIZE, ISA_B, ISA_IMC, ISA_MOP,
    RISCV_MAX_MEMORY,
};
use lazy_static::lazy_static;
use libfuzzer_sys::fuzz_target;

// Function signature of exec syscall:
//
// int ckb_exec_cell(const uint8_t* code_hash, uint8_t hash_type, uint32_t offset,
//                   uint32_t length, int argc, const char* argv[]);

static CALLER: &[u8] = include_bytes!("../programs/exec_caller");
static CANDIDATE_CALLEE: &'static [&[u8]] = &[include_bytes!("../programs/exec_callee_1")];

lazy_static! {}

pub struct CustomSyscall {
    argv: Vec<Vec<u8>>,
    opt_candidate_callee: u32,
}

impl<Mac: SupportMachine> Syscalls<Mac> for CustomSyscall {
    fn initialize(&mut self, _: &mut Mac) -> Result<(), Error> {
        Ok(())
    }

    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, Error> {
        let code = &machine.registers()[A7];
        if code.to_i32() != 1111 {
            return Ok(false);
        }
        let cycles = machine.cycles();
        let max_cycles = machine.max_cycles();
        machine.reset(max_cycles);
        machine.set_cycles(cycles);

        let callee_idx = self.opt_candidate_callee as usize % CANDIDATE_CALLEE.len();
        let callee = CANDIDATE_CALLEE[callee_idx];
        let code = Bytes::from(callee);
        let argv: Vec<Bytes> = self.argv.iter().map(|s| Bytes::from(s.clone())).collect();
        machine.load_elf(&code, true).unwrap();
        machine.initialize_stack(
            &argv,
            (RISCV_MAX_MEMORY - DEFAULT_STACK_SIZE) as u64,
            DEFAULT_STACK_SIZE as u64,
        )?;
        Ok(true)
    }
}

#[derive(Arbitrary, Debug)]
pub struct FuzzData {
    argv: Vec<Vec<u8>>,
    opt_candidate_callee: u32,
}

fuzz_target!(|data: FuzzData| {
    let code = Bytes::from(CALLER);

    let core_machine = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
        ISA_IMC | ISA_B | ISA_MOP,
        VERSION1,
        u64::max_value(),
    );
    let mut machine = DefaultMachineBuilder::new(core_machine)
        .instruction_cycle_func(Box::new(|_| 1))
        .syscall(Box::new(CustomSyscall {
            argv: data.argv,
            opt_candidate_callee: data.opt_candidate_callee,
        }))
        .build();
    machine.load_program(&code, &vec![]).unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert!(result.unwrap() != 0);
});
