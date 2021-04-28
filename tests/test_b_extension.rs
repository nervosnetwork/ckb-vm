pub mod machine_build;

#[test]
#[cfg_attr(miri, ignore)] // takes at least 9 hours
pub fn test_b_extension() {
    let mut machine = machine_build::int_v1_imcb("tests/programs/b_extension");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_imcb("tests/programs/b_extension");
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);

        let code = machine_build::aot_v1_imcb_code("tests/programs/b_extension");
        let mut machine_aot = machine_build::aot_v1_imcb("tests/programs/b_extension", &code);
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
    }
}

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

        let code = machine_build::aot_v1_imcb_code("tests/programs/clzw_bug");
        let mut machine_aot = machine_build::aot_v1_imcb("tests/programs/clzw_bug", &code);
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
    }
}

#[test]
pub fn test_packw_signextend() {
    let mut machine = machine_build::int_v1_imcb("tests/programs/packw_signextend");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_imcb("tests/programs/packw_signextend");
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);

        let code = machine_build::aot_v1_imcb_code("tests/programs/packw_signextend");
        let mut machine_aot = machine_build::aot_v1_imcb("tests/programs/packw_signextend", &code);
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
    }
}

#[test]
pub fn test_single_bit_signextend() {
    let mut machine = machine_build::int_v1_imcb("tests/programs/single_bit_signextend");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_imcb("tests/programs/single_bit_signextend");
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);

        let code = machine_build::aot_v1_imcb_code("tests/programs/single_bit_signextend");
        let mut machine_aot =
            machine_build::aot_v1_imcb("tests/programs/single_bit_signextend", &code);
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

        let code = machine_build::aot_v1_imcb_code("tests/programs/rorw_in_end_of_aot_block");
        let mut machine_aot =
            machine_build::aot_v1_imcb("tests/programs/rorw_in_end_of_aot_block", &code);
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
    }
}

#[test]
pub fn test_fsri_decode_bug() {
    let mut machine = machine_build::int_v1_imcb("tests/programs/fsri_decode_bug");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_imcb("tests/programs/fsri_decode_bug");
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);

        let code = machine_build::aot_v1_imcb_code("tests/programs/fsri_decode_bug");
        let mut machine_aot = machine_build::aot_v1_imcb("tests/programs/fsri_decode_bug", &code);
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
    }
}

#[test]
pub fn test_pcnt() {
    machine_run::int_v1_imcb("tests/programs/pcnt");
    #[cfg(has_asm)]
    machine_run::asm_v1_imcb("tests/programs/pcnt");
    #[cfg(has_asm)]
    machine_run::aot_v1_imcb("tests/programs/pcnt");
}
