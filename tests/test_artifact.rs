use ckb_vm::{DefaultMachineRunner, ISA_A, ISA_B, ISA_IMC, ISA_MOP, machine::VERSION2};
use std::fs;
use std::path::Path;
pub mod machine_build;

#[test]
pub fn test_artifact() {
    if !Path::new("tests/artifact").exists() {
        println!("Skipping test: tests/artifact directory not found");
        return;
    }
    let mut case: Vec<fs::DirEntry> = Vec::new();
    for e in Path::new("tests/artifact/arch").read_dir().unwrap() {
        case.push(e.unwrap());
    }
    for e in Path::new("tests/artifact/cryptography").read_dir().unwrap() {
        case.push(e.unwrap());
    }
    for e in Path::new("tests/artifact/spec").read_dir().unwrap() {
        let e = e.unwrap();
        if e.file_name().to_string_lossy().starts_with("rv32") {
            continue;
        }
        case.push(e);
    }
    case.sort_by_key(|e| e.path().to_string_lossy().to_string());

    for e in &case {
        let mut machine = machine_build::int(
            e.path().to_str().unwrap(),
            vec![],
            VERSION2,
            ISA_IMC | ISA_B | ISA_A | ISA_MOP,
        );
        let result_int = machine.run();
        assert!(result_int.is_ok());
        assert_eq!(result_int.unwrap(), 0);
    }

    #[cfg(has_asm)]
    for e in &case {
        let mut machine = machine_build::asm(
            e.path().to_str().unwrap(),
            vec![],
            VERSION2,
            ISA_IMC | ISA_B | ISA_A | ISA_MOP,
        );
        let result_int = machine.run();
        assert!(result_int.is_ok());
        assert_eq!(result_int.unwrap(), 0);
    }
}
