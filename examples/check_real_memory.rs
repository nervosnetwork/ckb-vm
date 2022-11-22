// This example is mainly to test whether there is memory overflow.
// Under linux, we choose to use smem, which can monitor memory changes more accurately

use ckb_vm::{run, Bytes, FlatMemory, SparseMemory};
use lazy_static::lazy_static;
use std::process::{id, Command};

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

static G_CHECK_LOOP: usize = 10;

fn get_current_memory_linux() -> usize {
    let output = String::from_utf8(
        Command::new("smem")
            .arg("-P")
            .arg("check_real_memory")   // current process name
            .arg("-c")
            .arg("pid uss")
            .output()
            .expect("run ps failed")
            .stdout,
    )
    .unwrap();

    let outputs = output.split("\n").collect::<Vec<&str>>();
    for i in 1..outputs.len() {
        let mut has_pid = false;
        let mut memory_size: u32 = 0;
        for d in outputs[i].split(" ").collect::<Vec<&str>>() {
            if d == " " || d == "" {
                continue;
            }
            let val: u32 = d.parse().unwrap();
            if !has_pid {
                if val != id() {
                    continue;
                }
                has_pid = true;
            } else {
                memory_size = val;
            }
        }
        if memory_size != 0 {
            return memory_size as usize;
        }
    }

    0
}

fn get_current_memory() -> usize {
    if !cfg!(linux) {
        get_current_memory_linux()
    } else {
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
}

struct MemoryOverflow {
    #[cfg(not(target_os = "windows"))]
    base_allocated: usize,

    #[cfg(not(target_os = "windows"))]
    base_resident: usize,
}

impl MemoryOverflow {
    pub fn new() -> Self {
        Self {
            #[cfg(not(target_os = "windows"))]
            base_allocated: jemalloc_ctl::stats::allocated::read().unwrap(),

            #[cfg(not(target_os = "windows"))]
            base_resident: jemalloc_ctl::stats::resident::read().unwrap(),
        }
    }

    pub fn check(&self) {
        #[cfg(not(target_os = "windows"))]
        assert!(jemalloc_ctl::stats::allocated::read().unwrap() <= self.base_allocated);

        #[cfg(not(target_os = "windows"))]
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
    jemalloc_ctl::epoch::advance().unwrap();

    let memory_overflow = MemoryOverflow::new();

    let memory_size = 1024 * 1024 * 4;
    if test_memory(memory_size).is_err() {
        panic!("run testcase failed");
    }

    memory_overflow.check();

    let memory_size = 1024 * 1024 * 2;
    if test_memory(memory_size).is_err() {
        panic!("run testcase failed");
    }

    memory_overflow.check();
}
