use ckb_vm::Error;
pub mod machine_build;

#[test]
pub fn test_write_permission_bug() {
    let mut machine = machine_build::int_v2_imacb("tests/programs/amo_write_permission");
    let ret = machine.run();
    assert!(ret.is_err());
    assert_eq!(ret.err(), Some(Error::MemWriteOnExecutablePage));

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v2_imacb("tests/programs/amo_write_permission");
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_err());
        assert_eq!(ret_asm.err(), Some(Error::MemWriteOnExecutablePage));
    }
}

#[test]
pub fn test_sc_after_sc() {
    let mut machine = machine_build::int_v2_imacb("tests/programs/sc_after_sc");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v2_imacb("tests/programs/sc_after_sc");
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }
}

#[test]
pub fn test_amo_compare() {
    let mut machine = machine_build::int_v2_imacb("tests/programs/amo_compare");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v2_imacb("tests/programs/amo_compare");
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }
}
