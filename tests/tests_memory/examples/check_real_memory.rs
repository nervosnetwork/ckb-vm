use ckb_vm::{
    machine::{
        asm::{AsmCoreMachine, AsmMachine},
        DefaultMachineBuilder, VERSION0,
    },
    run, Bytes, FlatMemory, SparseMemory, ISA_IMC,
};
use lazy_static::lazy_static;
use std::process::{id, Command};

lazy_static! {
    pub static ref BIN_PATH_BUFFER: Bytes =
        Bytes::from(&include_bytes!("../../programs/alloc_many")[..]);
    pub static ref BIN_NAME: String = format!("alloc_many");
}

static G_CHECK_LOOP: usize = 100;

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

fn check_int_real_memory(memory_size: usize) {
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
}

fn check_falt_real_memory(memory_size: usize) {
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
}

fn check_asm_real_memory(memory_size: usize) {
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
}

fn check_real_memory(memory_size: usize) {
    check_int_real_memory(memory_size);
    check_falt_real_memory(memory_size);
    check_asm_real_memory(memory_size);
}

fn main() {
    let memory_size = 1024 * 1024 * 4;
    check_real_memory(memory_size);

    let memory_size = 1024 * 1024 * 2;
    check_real_memory(memory_size);
}
