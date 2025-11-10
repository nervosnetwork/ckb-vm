use bytes::Bytes;
use ckb_vm::{
    machine::asm::{AsmCoreMachine, AsmMachine},
    DefaultMachineBuilder,
};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("tests/programs/simple64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let mut machine = AsmMachine::default();

    println!("Loading program...");
    match machine.load_program(&buffer, &vec!["simple".into()]) {
        Ok(size) => println!("Program loaded successfully, size: {}", size),
        Err(e) => {
            println!("Failed to load program: {:?}", e);
            return;
        }
    }

    println!("Running program...");
    let result = machine.run();

    match result {
        Ok(code) => println!("Success! Exit code: {}", code),
        Err(e) => println!("Error: {:?}", e),
    }
}
