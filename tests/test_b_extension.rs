use ckb_vm::{ISA_B, ISA_IMC, machine::VERSION1};

pub mod machine_build;

use ckb_vm::DefaultMachineRunner;

#[test]
pub fn test_clzw_bug() {
    let mut machine =
        machine_build::int("tests/programs/clzw_bug", vec![], VERSION1, ISA_IMC | ISA_B);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm =
            machine_build::asm("tests/programs/clzw_bug", vec![], VERSION1, ISA_IMC | ISA_B);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }
}

#[test]
pub fn test_sbinvi_aot_load_imm_bug() {
    let mut machine = machine_build::int(
        "tests/programs/sbinvi_aot_load_imm_bug",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/sbinvi_aot_load_imm_bug",
            vec![],
            VERSION1,
            ISA_IMC | ISA_B,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }
}

#[test]
pub fn test_rorw_in_end_of_aot_block() {
    // The 1024th instruction will use one more temporary register than normal.
    let mut machine = machine_build::int(
        "tests/programs/rorw_in_end_of_aot_block",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/rorw_in_end_of_aot_block",
            vec![],
            VERSION1,
            ISA_IMC | ISA_B,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }
}

#[test]
pub fn test_pcnt() {
    let mut machine = machine_build::int("tests/programs/pcnt", vec![], VERSION1, ISA_IMC | ISA_B);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm =
            machine_build::asm("tests/programs/pcnt", vec![], VERSION1, ISA_IMC | ISA_B);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }
}

#[test]
pub fn test_clmul_bug() {
    let mut machine = machine_build::int(
        "tests/programs/clmul_bug",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/clmul_bug",
            vec![],
            VERSION1,
            ISA_IMC | ISA_B,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }
}

#[test]
pub fn test_orc_bug() {
    let mut machine =
        machine_build::int("tests/programs/orc_bug", vec![], VERSION1, ISA_IMC | ISA_B);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm =
            machine_build::asm("tests/programs/orc_bug", vec![], VERSION1, ISA_IMC | ISA_B);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }
}
