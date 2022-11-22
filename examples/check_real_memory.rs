use ckb_vm::{
    machine::{
        asm::{AsmCoreMachine, AsmMachine},
        DefaultMachineBuilder, VERSION0,
    },
    run, Bytes, FlatMemory, SparseMemory, ISA_IMC,
};
use jemalloc_ctl::{epoch, stats};
use jemallocator::Jemalloc;
use lazy_static::lazy_static;
use std::{
    process::{id, Command},
    thread,
};

lazy_static! {
    pub static ref BIN_PATH_BUFFER: Bytes =
        Bytes::from(&include_bytes!("../tests/programs/alloc_many")[..]);
    pub static ref BIN_NAME: String = format!("alloc_many");
}

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

static G_CHECK_LOOP: usize = 2;

fn get_current_memory() -> usize {
    let pid = format!("{}", id());
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

fn check_interpreter(memory_size: usize) -> Result<(), ()> {
    println!(
        "Check interpreter memory overflow, ckb-vm memory size: {}",
        memory_size
    );
    println!("Base memory: {}", get_current_memory());
    for _ in 0..G_CHECK_LOOP {
        let result = run::<u64, SparseMemory<u64>>(
            &BIN_PATH_BUFFER,
            &vec![BIN_NAME.clone().into()],
            memory_size,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
        println!("Current memory: {}", get_current_memory());
    }
    println!("End of check");
    Ok(())
}

fn check_falt(memory_size: usize) -> Result<(), ()> {
    println!(
        "Check falt memory overflow, ckb-vm memory size: {}",
        memory_size
    );
    println!("Base memory: {}", get_current_memory());
    for _ in 0..G_CHECK_LOOP {
        let result = run::<u64, FlatMemory<u64>>(
            &BIN_PATH_BUFFER,
            &vec![BIN_NAME.clone().into()],
            memory_size,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
        println!("Current memory: {}", get_current_memory());
    }
    println!("End of check");
    Ok(())
}

fn check_asm(memory_size: usize) -> Result<(), ()> {
    println!(
        "Check asm memory overflow, ckb-vm memory size: {}",
        memory_size
    );
    println!("Base memory: {}", get_current_memory());
    for _ in 0..G_CHECK_LOOP {
        let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value(), memory_size);
        let core = DefaultMachineBuilder::new(asm_core).build();
        let mut machine = AsmMachine::new(core);
        machine
            .load_program(&BIN_PATH_BUFFER, &vec![BIN_NAME.clone().into()])
            .unwrap();
        let result = machine.run();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        println!("Current memory: {}", get_current_memory());
    }
    println!("End of check");
    Ok(())
}

fn check_asm_in_thread(memory_size: usize) -> Result<(), ()> {
    println!(
        "Check asm in thread memory overflow, ckb-vm memory size: {}",
        memory_size
    );
    println!("Base memory: {}", get_current_memory());
    for _ in 0..10 {
        let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value(), memory_size);
        let core = DefaultMachineBuilder::new(asm_core).build();
        let mut machine = AsmMachine::new(core);
        machine
            .load_program(&BIN_PATH_BUFFER, &vec![BIN_NAME.clone().into()])
            .unwrap();
        let thread_join_handle = thread::spawn(move || {
            let result = machine.run();
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 0);
        });
        thread_join_handle.join().unwrap();
        println!("Current memory: {}", get_current_memory());
    }
    println!("End of check");
    Ok(())
}

fn test_memory(memory_size: usize) -> Result<(), ()> {
    if check_int_real_memory(memory_size).is_err() {
        return Err(());
    }
    if check_falt_real_memory(memory_size).is_err() {
        return Err(());
    }
    if check_asm_real_memory(memory_size).is_err() {
        return Err(());
    }
    if check_asm_in_thread(memory_size).is_err() {
        return Err(());
    }
    Ok(())
}

fn main() {
    epoch::advance().unwrap();
    let base_allocated = stats::allocated::read().unwrap() as f64 * 1.02f64;
    let base_resident = stats::resident::read().unwrap() as f64 * 1.02f64;

    let memory_size = 1024 * 1024 * 4;
    if test_memory(memory_size).is_err() {
        panic!("run testcase failed");
    }

    assert!((stats::allocated::read().unwrap() as f64) < base_allocated);
    assert!((stats::resident::read().unwrap() as f64) < base_resident);

    let memory_size = 1024 * 1024 * 2;
    if test_memory(memory_size).is_err() {
        panic!("run testcase failed");
    }

    assert!((stats::allocated::read().unwrap() as f64) < base_allocated);
    assert!((stats::resident::read().unwrap() as f64) < base_resident);
}
