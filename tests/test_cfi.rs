use ckb_vm::machine::VERSION3;
use ckb_vm::{DefaultMachineRunner, Error, ISA_B, ISA_CFI, ISA_IMC, ISA_MOP};
pub mod machine_build;

fn run(path: &str) -> Result<i8, Error> {
    let mut machine =
        machine_build::int(path, vec![], VERSION3, ISA_IMC | ISA_B | ISA_MOP | ISA_CFI);
    machine.run()
}

#[test]
pub fn test_simple_instructions_64() {
    let ret = run("tests/programs/simple64");
    assert!(ret.is_ok());

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/simple64",
            vec![],
            VERSION3,
            ISA_IMC | ISA_B | ISA_MOP | ISA_CFI,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
    }
}

#[test]
pub fn test_cfi_success() {
    let ret = run("tests/programs/cfi_success");
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
}

#[test]
pub fn test_cfi_ss_not_active() {
    let ret = run("tests/programs/cfi_ss_not_active");
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
}
