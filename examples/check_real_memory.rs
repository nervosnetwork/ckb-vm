// This example is mainly to test whether there is memory overflow.

use ckb_vm::{run, Bytes, FlatMemory, SparseMemory};
use lazy_static::lazy_static;
use std::process::id;

#[cfg(has_asm)]
use ckb_vm::{
    machine::{
        asm::{AsmCoreMachine, AsmMachine},
        DefaultMachineBuilder, VERSION0,
    },
    ISA_IMC,
};

#[cfg(has_asm)]
use std::thread;

lazy_static! {
    pub static ref BIN_PATH_BUFFER: Bytes =
        Bytes::from(&include_bytes!("../tests/programs/alloc_many")[..]);
    pub static ref BIN_NAME: String = format!("alloc_many");
}

#[cfg(not(target_os = "windows"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

static G_CHECK_LOOP: usize = 2;

fn get_current_memory() -> usize {
    let process_info = psutil::process::Process::new(id()).expect("get process failed");
    let mem_info = process_info.memory_info().expect("get memory info failed");
    mem_info.rss() as usize
}

struct MemoryOverflow {
    base_allocated: usize,
    base_resident: usize,
}

impl MemoryOverflow {
    pub fn new() -> Self {
        jemalloc_ctl::epoch::advance().unwrap();

        Self {
            base_allocated: jemalloc_ctl::stats::allocated::read().unwrap(),
            base_resident: jemalloc_ctl::stats::resident::read().unwrap(),
        }
    }

    pub fn check(&self) {
        assert!(jemalloc_ctl::stats::allocated::read().unwrap() <= self.base_allocated);
        assert!(jemalloc_ctl::stats::resident::read().unwrap() <= self.base_resident);
    }
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

#[cfg(has_asm)]
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

#[cfg(has_asm)]
fn check_asm_in_thread(memory_size: usize) -> Result<(), ()> {
    println!(
        "Check asm in thread memory overflow, ckb-vm memory size: {}",
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
    if check_interpreter(memory_size).is_err() {
        return Err(());
    }
    if check_falt(memory_size).is_err() {
        return Err(());
    }

    #[cfg(has_asm)]
    if check_asm(memory_size).is_err() {
        return Err(());
    }

    #[cfg(has_asm)]
    if check_asm_in_thread(memory_size).is_err() {
        return Err(());
    }
    Ok(())
}

fn main() {
    #[cfg(not(target_os = "windows"))]
    let memory_overflow = MemoryOverflow::new();

    let memory_size = 1024 * 1024 * 4;
    if test_memory(memory_size).is_err() {
        panic!("run testcase failed");
    }

    #[cfg(not(target_os = "windows"))]
    memory_overflow.check();

    let memory_size = 1024 * 1024 * 2;
    if test_memory(memory_size).is_err() {
        panic!("run testcase failed");
    }

    #[cfg(not(target_os = "windows"))]
    memory_overflow.check();
}
