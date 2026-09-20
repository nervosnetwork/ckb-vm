use ckb_vm::{DefaultMachineRunner, ISA_B, ISA_IMC, ISA_MOP, SupportMachine, machine::VERSION2};
use std::fs;
use std::path::Path;
pub mod machine_build;

#[test]
pub fn test_artifact() {
    let mut case: Vec<fs::DirEntry> = Vec::new();
    for e in Path::new("tests/artifact/arch").read_dir().unwrap() {
        case.push(e.unwrap());
    }
    for e in Path::new("tests/artifact/cryptography").read_dir().unwrap() {
        case.push(e.unwrap());
    }
    for e in Path::new("tests/artifact/spec").read_dir().unwrap() {
        case.push(e.unwrap());
    }
    case.sort_by_key(|e| e.path().to_string_lossy().to_string());

    for e in &case {
        let mut machine = machine_build::int(
            e.path().to_str().unwrap(),
            vec![],
            VERSION2,
            ISA_IMC | ISA_B | ISA_MOP,
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
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let result_asm = machine.run();
        assert!(result_asm.is_ok());
        assert_eq!(result_asm.unwrap(), 0);
    }
}

#[test]
pub fn test_artifact_cycles_sample() {
    let cases = [
        ("tests/artifact/arch/add-01.elf", 12352),
        ("tests/artifact/arch/cadd-01.elf", 12313),
        ("tests/artifact/arch/div-01.elf", 12145),
        ("tests/artifact/spec/rv64ui-u-add", 420),
        ("tests/artifact/spec/rv64ui-u-lw", 192),
        ("tests/artifact/spec/rv64um-u-mul", 412),
    ];

    for (path, cycles) in cases {
        let mut machine = machine_build::int(path, vec![], VERSION2, ISA_IMC | ISA_B | ISA_MOP);
        assert_eq!(machine.run().unwrap(), 0);
        assert_eq!(machine.machine.cycles(), cycles, "{path}");

        #[cfg(has_asm)]
        {
            let mut machine = machine_build::asm(path, vec![], VERSION2, ISA_IMC | ISA_B | ISA_MOP);
            assert_eq!(machine.run().unwrap(), 0);
            assert_eq!(machine.machine.cycles(), cycles, "{path}");
        }
    }
}
