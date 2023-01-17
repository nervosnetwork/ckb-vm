use ckb_vm::{run, FlatMemory, SparseMemory};
use std::fs;

#[cfg(has_asm)]
use ckb_vm::{
    machine::{
        asm::{AsmCoreMachine, AsmMachine},
        DefaultMachineBuilder, VERSION0,
    },
    ISA_IMC,
};

fn run_memory_suc(memory_size: usize, bin_path: String, bin_name: String) {
    let buffer = fs::read(bin_path).unwrap().into();
    let result =
        run::<u64, SparseMemory<u64>>(&buffer, &vec![bin_name.clone().into()], memory_size);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    let result = run::<u64, FlatMemory<u64>>(&buffer, &vec![bin_name.clone().into()], memory_size);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let asm_core = AsmCoreMachine::new_with_memory(ISA_IMC, VERSION0, u64::max_value(), memory_size);
        let core = DefaultMachineBuilder::new(asm_core).build();
        let mut machine = AsmMachine::new(core);
        machine
            .load_program(&buffer, &vec![bin_name.into()])
            .unwrap();
        let result = machine.run();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }
}

#[test]
fn test_dy_memory() {
    run_memory_suc(
        1024 * 1024 * 2,
        format!("tests/programs/alloc_many"),
        format!("alloc_many"),
    );
}

#[test]
fn test_memory_out_of_bounds() {
    let memory_size = 1024 * 256;
    let buffer = fs::read("tests/programs/alloc_many").unwrap().into();
    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec!["alloc_many".into()], memory_size);
    assert!(result.is_err());
    assert_eq!(ckb_vm::Error::MemOutOfBound, result.err().unwrap());

    let result = run::<u64, FlatMemory<u64>>(&buffer, &vec!["alloc_many".into()], memory_size);
    assert!(result.is_err());
    assert_eq!(ckb_vm::Error::MemOutOfBound, result.err().unwrap());

    #[cfg(has_asm)]
    {
        let asm_core = AsmCoreMachine::new_with_memory(ISA_IMC, VERSION0, u64::max_value(), memory_size);
        let core = DefaultMachineBuilder::new(asm_core).build();
        let mut machine = AsmMachine::new(core);
        machine
            .load_program(&buffer, &vec!["alloc_many".into()])
            .unwrap();
        let result = machine.run();
        assert!(result.is_err());
        assert_eq!(ckb_vm::Error::MemOutOfBound, result.err().unwrap());
    }
}

#[test]
fn test_memory_min_size() {
    run_memory_suc(
        1024 * 256,
        format!("tests/programs/mulw64"),
        format!("mulw64"),
    );
}

#[test]
fn test_memory_thread_safe() {}
