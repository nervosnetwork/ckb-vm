use ckb_vm::{
    machine::{
        asm::{AsmCoreMachine, AsmMachine},
        DefaultMachineBuilder, VERSION0,
    },
    run, Bytes, FlatMemory, SparseMemory, ISA_IMC,
};
use jemalloc_ctl::{epoch, stats};
use jemallocator::Jemalloc;
use std::{fs, thread};

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn test_memory(memory_size: usize, bin_buffer: &Bytes, bin_name: &String) {
    let result =
        run::<u64, SparseMemory<u64>>(bin_buffer, &vec![bin_name.clone().into()], memory_size);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    let result =
        run::<u64, FlatMemory<u64>>(bin_buffer, &vec![bin_name.clone().into()], memory_size);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value(), memory_size);
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(bin_buffer, &vec![bin_name.clone().into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

fn test_thread(memory_size: usize, bin_buffer: &Bytes, bin_name: &String) {
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value(), memory_size);
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(bin_buffer, &vec![bin_name.clone().into()])
        .unwrap();
    let thread_join_handle = thread::spawn(move || {
        let result = machine.run();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    });
    thread_join_handle.join().unwrap();
}

fn main() {
    let bin_path = "../programs/alloc_many";
    let bin_name = format!("alloc_many");
    let memory_size = 1024 * 1024 * 2;
    let buffer: Bytes = fs::read(bin_path).unwrap().into();

    epoch::advance().unwrap();

    for _ in 0..10 {
        test_memory(memory_size, &buffer, &bin_name);
        test_thread(memory_size, &buffer, &bin_name);

        let allocated = stats::allocated::read().unwrap();
        let resident = stats::resident::read().unwrap();
        println!("{} bytes allocated/{} bytes resident", allocated, resident);
    }
}
