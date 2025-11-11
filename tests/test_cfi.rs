use ckb_vm::machine::VERSION3;
use ckb_vm::{DefaultMachineRunner, ISA_B, ISA_CFI, ISA_IMC, ISA_MOP};
pub mod machine_build;

#[test]
pub fn test_simple_instructions_64() {
    let mut machine = machine_build::int(
        "tests/programs/simple64",
        vec![],
        VERSION3,
        ISA_IMC | ISA_B | ISA_MOP | ISA_CFI,
    );
    let ret = machine.run();
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
