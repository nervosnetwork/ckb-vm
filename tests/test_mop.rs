pub mod machine_build;

use bytes::Bytes;

use ckb_vm::SupportMachine;

#[test]
#[cfg_attr(miri, ignore)]
pub fn test_mop_wide_multiply() {
    let mut machine = machine_build::int_v1_imcb("tests/programs/mop_wide_multiply");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    let mut machine = machine_build::int_v1_mop("tests/programs/mop_wide_multiply", vec![]);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 9192427);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_mop("tests/programs/mop_wide_multiply", vec![]);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine.machine.cycles(), 9192427);

        let code = machine_build::aot_v1_mop_code("tests/programs/mop_wide_multiply");
        let mut machine_aot =
            machine_build::aot_v1_mop("tests/programs/mop_wide_multiply", vec![], &code);
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
        assert_eq!(machine.machine.cycles(), 9192427);
    }
}

#[test]
#[cfg_attr(miri, ignore)]
pub fn test_mop_wide_divide() {
    let mut machine = machine_build::int_v1_imcb("tests/programs/mop_wide_divide");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    let mut machine = machine_build::int_v1_mop("tests/programs/mop_wide_divide", vec![]);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 6106583);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_mop("tests/programs/mop_wide_divide", vec![]);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine.machine.cycles(), 6106583);

        let code = machine_build::aot_v1_mop_code("tests/programs/mop_wide_divide");
        let mut machine_aot =
            machine_build::aot_v1_mop("tests/programs/mop_wide_divide", vec![], &code);
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
        assert_eq!(machine.machine.cycles(), 6106583);
    }
}

#[test]
pub fn test_mop_far_jump() {
    let mut machine = machine_build::int_v1_imcb("tests/programs/mop_far_jump");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    let mut machine = machine_build::int_v1_mop("tests/programs/mop_far_jump", vec![]);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 5);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_mop("tests/programs/mop_far_jump", vec![]);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine.machine.cycles(), 5);

        let code = machine_build::aot_v1_mop_code("tests/programs/mop_far_jump");
        let mut machine_aot =
            machine_build::aot_v1_mop("tests/programs/mop_far_jump", vec![], &code);
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
        assert_eq!(machine.machine.cycles(), 5);
    }
}

#[test]
#[cfg_attr(miri, ignore)]
pub fn test_mop_ld_32_constants() {
    let mut machine = machine_build::int_v1_imcb("tests/programs/mop_ld_signextend_32");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    let mut machine = machine_build::int_v1_mop("tests/programs/mop_ld_signextend_32", vec![]);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm =
            machine_build::asm_v1_mop("tests/programs/mop_ld_signextend_32", vec![]);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);

        let code = machine_build::aot_v1_mop_code("tests/programs/mop_ld_signextend_32");
        let mut machine_aot =
            machine_build::aot_v1_mop("tests/programs/mop_ld_signextend_32", vec![], &code);
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
    }
}

#[test]
#[cfg_attr(miri, ignore)]
pub fn test_mop_secp256k1() {
    let args = vec![
        Bytes::from("033f8cf9c4d51a33206a6c1c6b27d2cc5129daa19dbd1fc148d395284f6b26411f"),
        Bytes::from("304402203679d909f43f073c7c1dcf8468a485090589079ee834e6eed92fea9b09b06a2402201e46f1075afa18f306715e7db87493e7b7e779569aa13c64ab3d09980b3560a3"),
        Bytes::from("foo"),
        Bytes::from("bar"),
    ];

    let mut machine = machine_build::int_v1_mop("benches/data/secp256k1_bench", args.clone());
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 611871);

    #[cfg(has_asm)]
    {
        let mut machine_asm =
            machine_build::asm_v1_mop("benches/data/secp256k1_bench", args.clone());
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine.machine.cycles(), 611871);

        let code = machine_build::aot_v1_mop_code("benches/data/secp256k1_bench");
        let mut machine_aot =
            machine_build::aot_v1_mop("benches/data/secp256k1_bench", args.clone(), &code);
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
        assert_eq!(machine.machine.cycles(), 611871);
    }
}

#[test]
pub fn test_mop_adc() {
    let mut machine = machine_build::int_v1_imcb("tests/programs/mop_adc");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 73);

    let mut machine = machine_build::int_v1_mop("tests/programs/mop_adc", vec![]);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 61);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_mop("tests/programs/mop_adc", vec![]);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine.machine.cycles(), 61);

        let code = machine_build::aot_v1_mop_code("tests/programs/mop_adc");
        let mut machine_aot = machine_build::aot_v1_mop("tests/programs/mop_adc", vec![], &code);
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
        assert_eq!(machine.machine.cycles(), 61);
    }
}

#[test]
pub fn test_mop_sbb() {
    let mut machine = machine_build::int_v1_imcb("tests/programs/mop_sbb");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 35);

    let mut machine = machine_build::int_v1_mop("tests/programs/mop_sbb", vec![]);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 27);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_mop("tests/programs/mop_sbb", vec![]);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine.machine.cycles(), 27);

        let code = machine_build::aot_v1_mop_code("tests/programs/mop_sbb");
        let mut machine_aot = machine_build::aot_v1_mop("tests/programs/mop_sbb", vec![], &code);
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
        assert_eq!(machine.machine.cycles(), 27);
    }
}

#[test]
pub fn test_mop_random_adc_sbb() {
    let mut machine = machine_build::int_v1_imcb("tests/programs/mop_random_adc_sbb");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 9458);

    let mut machine = machine_build::int_v1_mop("tests/programs/mop_random_adc_sbb", vec![]);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 6755);

    #[cfg(has_asm)]
    {
        let mut machine_asm =
            machine_build::asm_v1_mop("tests/programs/mop_random_adc_sbb", vec![]);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine.machine.cycles(), 6755);

        let code = machine_build::aot_v1_mop_code("tests/programs/mop_random_adc_sbb");
        let mut machine_aot =
            machine_build::aot_v1_mop("tests/programs/mop_random_adc_sbb", vec![], &code);
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
        assert_eq!(machine.machine.cycles(), 6755);
    }
}

#[test]
pub fn test_mop_ld_signextend_32_overflow_bug() {
    let mut machine =
        machine_build::int_v1_mop("tests/programs/mop_ld_signextend_32_overflow_bug", vec![]);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm =
            machine_build::asm_v1_mop("tests/programs/mop_ld_signextend_32_overflow_bug", vec![]);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);

        let code =
            machine_build::aot_v1_mop_code("tests/programs/mop_ld_signextend_32_overflow_bug");
        let mut machine_aot = machine_build::aot_v1_mop(
            "tests/programs/mop_ld_signextend_32_overflow_bug",
            vec![],
            &code,
        );
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
    }
}

#[test]
pub fn test_mop_wide_mul_zero() {
    let mut machine = machine_build::int_v1_mop("tests/programs/mop_wide_mul_zero", vec![]);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_mop("tests/programs/mop_wide_mul_zero", vec![]);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);

        let code = machine_build::aot_v1_mop_code("tests/programs/mop_wide_mul_zero");
        let mut machine_aot =
            machine_build::aot_v1_mop("tests/programs/mop_wide_mul_zero", vec![], &code);
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
    }
}

#[test]
pub fn test_mop_wide_div_zero() {
    let mut machine = machine_build::int_v1_mop("tests/programs/mop_wide_div_zero", vec![]);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_mop("tests/programs/mop_wide_div_zero", vec![]);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);

        let code = machine_build::aot_v1_mop_code("tests/programs/mop_wide_div_zero");
        let mut machine_aot =
            machine_build::aot_v1_mop("tests/programs/mop_wide_div_zero", vec![], &code);
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
    }
}
