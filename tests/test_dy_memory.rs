#[cfg(has_asm)]
use ckb_vm::{
    DefaultMachineRunner, ISA_B, ISA_IMC, ISA_MOP, SupportMachine,
    machine::{
        VERSION0, VERSION2,
        asm::{AsmCoreMachine, AsmDefaultMachineBuilder, AsmMachine},
    },
};
use ckb_vm::{FlatMemory, SparseMemory, error::OutOfBoundKind, run_with_memory};
use std::fs;

fn run_memory_suc(memory_size: usize, bin_path: String, bin_name: String) {
    let buffer = fs::read(bin_path).unwrap().into();
    let result = run_with_memory::<u64, SparseMemory<u64>>(
        &buffer,
        &vec![bin_name.clone().into()],
        memory_size,
    );
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    let result = run_with_memory::<u64, FlatMemory<u64>>(
        &buffer,
        &vec![bin_name.clone().into()],
        memory_size,
    );
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let asm_core = AsmCoreMachine::new_with_memory(ISA_IMC, VERSION0, u64::MAX, memory_size);
        let core = AsmDefaultMachineBuilder::new(asm_core).build();
        let mut machine = AsmMachine::new(core);
        machine
            .load_program(&buffer, [Ok(bin_name.into())].into_iter())
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
    let result =
        run_with_memory::<u64, SparseMemory<u64>>(&buffer, &vec!["alloc_many".into()], memory_size);
    assert!(result.is_err());
    assert_eq!(
        ckb_vm::Error::MemOutOfBound(0xfffffffffff3ffb8, OutOfBoundKind::Memory),
        result.err().unwrap()
    );

    let result =
        run_with_memory::<u64, FlatMemory<u64>>(&buffer, &vec!["alloc_many".into()], memory_size);
    assert!(result.is_err());
    assert_eq!(
        ckb_vm::Error::MemOutOfBound(0xfffffffffff3ffb8, OutOfBoundKind::Memory),
        result.err().unwrap()
    );

    #[cfg(has_asm)]
    {
        let asm_core = AsmCoreMachine::new_with_memory(
            ISA_IMC | ISA_B | ISA_MOP,
            VERSION2,
            u64::MAX,
            memory_size,
        );
        let core = AsmDefaultMachineBuilder::new(asm_core).build();
        let mut machine = AsmMachine::new(core);
        machine
            .load_program(&buffer, [Ok("alloc_many".into())].into_iter())
            .unwrap();
        let result = machine.run();
        assert!(result.is_err());
        assert_eq!(
            ckb_vm::Error::MemOutOfBound(0xfffffffffff3ffb8, OutOfBoundKind::Memory),
            result.err().unwrap()
        );
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
