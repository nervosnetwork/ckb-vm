use ckb_vm::{
    machine::{
        asm::{AsmCoreMachine, AsmMachine},
        DefaultMachineBuilder, VERSION0,
    },
    run, Bytes, FlatMemory, SparseMemory, ISA_IMC,
};
use jemalloc_ctl::{epoch, stats};
use jemallocator::Jemalloc;
use std::{
    fs,
    process::{self, Command},
    thread,
};

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn get_current_memory() -> usize {
    let pid = format!("{}", process::id());
    let output = String::from_utf8(
        Command::new("ps")
            .arg("-p")
            .arg(pid)
            .arg("-o")
            .arg("rss")
            .output()
            .expect("run ps failed")
            .stdout,
    )
    .unwrap();

    let output = output.split("\n").collect::<Vec<&str>>();

    let memory_size = output[1].replace(" ", "");
    memory_size.parse().unwrap()
}

#[test]
fn test_memory() {
    let bin_path = "../programs/alloc_many";
    let bin_name = format!("alloc_many");
    let memory_size = 1024 * 1024 * 2;
    let buffer: Bytes = fs::read(bin_path).unwrap().into();

    epoch::advance().unwrap();

    let base_allocated = stats::allocated::read().unwrap() as f64 * 1.02f64;
    let base_resident = stats::resident::read().unwrap() as f64 * 1.02f64;

    for _ in 0..100 {
        let result =
            run::<u64, SparseMemory<u64>>(&buffer, &vec![bin_name.clone().into()], memory_size);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        let result =
            run::<u64, FlatMemory<u64>>(&buffer, &vec![bin_name.clone().into()], memory_size);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value(), memory_size);
        let core = DefaultMachineBuilder::new(asm_core).build();
        let mut machine = AsmMachine::new(core);
        machine
            .load_program(&buffer, &vec![bin_name.clone().into()])
            .unwrap();
        let result = machine.run();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        assert!((stats::allocated::read().unwrap() as f64) < base_allocated);
        assert!((stats::resident::read().unwrap() as f64) < base_resident);
        println!("--{}", get_current_memory());
    }
}

#[test]
fn test_thread_safe() {
    let bin_path = "../programs/alloc_many";
    let bin_name = format!("alloc_many");
    let memory_size = 1024 * 1024 * 2;
    let buffer: Bytes = fs::read(bin_path).unwrap().into();

    epoch::advance().unwrap();

    let base_allocated = stats::allocated::read().unwrap() as f64 * 1.02f64;
    let base_resident = stats::resident::read().unwrap() as f64 * 1.02f64;

    for _ in 0..10 {
        let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value(), memory_size);
        let core = DefaultMachineBuilder::new(asm_core).build();
        let mut machine = AsmMachine::new(core);
        machine
            .load_program(&buffer, &vec![bin_name.clone().into()])
            .unwrap();
        let thread_join_handle = thread::spawn(move || {
            let result = machine.run();
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 0);
        });
        thread_join_handle.join().unwrap();

        assert!((stats::allocated::read().unwrap() as f64) < base_allocated);
        assert!((stats::resident::read().unwrap() as f64) < base_resident);

        println!("--{}", get_current_memory());
    }
}
