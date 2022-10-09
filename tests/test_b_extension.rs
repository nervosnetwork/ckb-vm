pub mod machine_build;

#[test]
pub fn test_clzw_bug() {
    let mut machine = machine_build::int_v1_imcb("tests/programs/clzw_bug");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_imcb("tests/programs/clzw_bug");
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }

    #[cfg(has_aot)]
    {
        let code = machine_build::aot_v1_imcb_code("tests/programs/clzw_bug");
        let mut machine_aot = machine_build::aot_v1_imcb("tests/programs/clzw_bug", &code);
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
    }
}

#[test]
pub fn test_sbinvi_aot_load_imm_bug() {
    let mut machine = machine_build::int_v1_imcb("tests/programs/sbinvi_aot_load_imm_bug");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_imcb("tests/programs/sbinvi_aot_load_imm_bug");
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }

    #[cfg(has_aot)]
    {
        let code = machine_build::aot_v1_imcb_code("tests/programs/sbinvi_aot_load_imm_bug");
        let mut machine_aot =
            machine_build::aot_v1_imcb("tests/programs/sbinvi_aot_load_imm_bug", &code);
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
    }
}

#[test]
pub fn test_rorw_in_end_of_aot_block() {
    // The 1024th instruction will use one more temporary register than normal.
    let mut machine = machine_build::int_v1_imcb("tests/programs/rorw_in_end_of_aot_block");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_imcb("tests/programs/rorw_in_end_of_aot_block");
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }

    #[cfg(has_aot)]
    {
        let code = machine_build::aot_v1_imcb_code("tests/programs/rorw_in_end_of_aot_block");
        let mut machine_aot =
            machine_build::aot_v1_imcb("tests/programs/rorw_in_end_of_aot_block", &code);
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
    }
}

#[test]
pub fn test_pcnt() {
    let mut machine = machine_build::int_v1_imcb("tests/programs/pcnt");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_imcb("tests/programs/pcnt");
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }

    #[cfg(has_aot)]
    {
        let code = machine_build::aot_v1_imcb_code("tests/programs/pcnt");
        let mut machine_aot = machine_build::aot_v1_imcb("tests/programs/pcnt", &code);
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
    }
}

#[test]
pub fn test_clmul_bug() {
    let mut machine = machine_build::int_v1_imcb("tests/programs/clmul_bug");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_imcb("tests/programs/clmul_bug");
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }

    #[cfg(has_aot)]
    {
        let code = machine_build::aot_v1_imcb_code("tests/programs/clmul_bug");
        let mut machine_aot = machine_build::aot_v1_imcb("tests/programs/clmul_bug", &code);
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
    }
}

#[test]
pub fn test_orc_bug() {
    let mut machine = machine_build::int_v1_imcb("tests/programs/orc_bug");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_imcb("tests/programs/orc_bug");
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }

    #[cfg(has_aot)]
    {
        let code = machine_build::aot_v1_imcb_code("tests/programs/orc_bug");
        let mut machine_aot = machine_build::aot_v1_imcb("tests/programs/orc_bug", &code);
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
    }
}
