/// Example: Testing CFI Feature Detection
use bytes::Bytes;
use ckb_vm::elf::parse_elf;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let elf_path = &args[1];
    let program_data = fs::read(elf_path)?;
    let program = Bytes::from(program_data);
    let program_metadata = parse_elf::<u64>(&program, 3)?;
    println!("CFI_LP_UNLABELED: {}", program_metadata.cfi.lp_unlabeled);
    println!("CFI_SS:           {}", program_metadata.cfi.ss);
    println!("CFI_LP_FUNC_SIG:  {}", program_metadata.cfi.lp_func_sig);
    Ok(())
}
