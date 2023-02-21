pub mod machine_build;
use bytes::Bytes;
use ckb_vm::{registers::A0, CoreMachine, Error, SupportMachine};

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
        assert_eq!(machine_asm.machine.cycles(), 9192427);
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
        assert_eq!(machine_asm.machine.cycles(), 6106583);
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
        assert_eq!(machine_asm.machine.cycles(), 5);
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

    let mut machine = machine_build::int_mop("benches/data/secp256k1_bench", args.clone(), 2);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 576608);

    #[cfg(has_asm)]
    {
        let mut machine_asm =
            machine_build::asm_v1_mop("benches/data/secp256k1_bench", args.clone());
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.cycles(), 611871);

        let mut machine_asm =
            machine_build::asm_mop("benches/data/secp256k1_bench", args.clone(), 2);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.cycles(), 576608);
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
        assert_eq!(machine_asm.machine.cycles(), 61);
    }
}

#[test]
pub fn test_mop_adcs() {
    let mut machine = machine_build::int_v1_imcb("tests/programs/mop_adcs");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 53);

    let mut machine = machine_build::int_v1_mop("tests/programs/mop_adcs", vec![]);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 53);

    let mut machine = machine_build::int_mop("tests/programs/mop_adcs", vec![], 2);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 47);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_mop("tests/programs/mop_adcs", vec![]);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.cycles(), 53);

        let mut machine_asm = machine_build::asm_mop("tests/programs/mop_adcs", vec![], 2);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.cycles(), 47);
    }
}

#[test]
pub fn test_mop_add3() {
    let mut machine = machine_build::int_v1_imcb("tests/programs/mop_add3");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0, "Machine state: {}", machine.machine);
    assert_eq!(machine.machine.cycles(), 1047);

    let mut machine = machine_build::int_v1_mop("tests/programs/mop_add3", vec![]);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 939);

    let mut machine = machine_build::int_mop("tests/programs/mop_add3", vec![], 2);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 903);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_mop("tests/programs/mop_add3", vec![]);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.cycles(), 939);

        let mut machine_asm = machine_build::asm_mop("tests/programs/mop_add3", vec![], 2);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.cycles(), 903);
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
        assert_eq!(machine_asm.machine.cycles(), 27);
    }
}

#[test]
pub fn test_mop_sbbs() {
    let mut machine = machine_build::int_v1_imcb("tests/programs/mop_sbbs");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0, "Machine state: {}", machine.machine);
    assert_eq!(machine.machine.cycles(), 87);

    let mut machine = machine_build::int_v1_mop("tests/programs/mop_sbbs", vec![]);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 81);

    let mut machine = machine_build::int_mop("tests/programs/mop_sbbs", vec![], 2);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 76);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_mop("tests/programs/mop_sbbs", vec![]);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.cycles(), 81);

        let mut machine_asm = machine_build::asm_mop("tests/programs/mop_sbbs", vec![], 2);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.cycles(), 76);
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

    let mut machine = machine_build::int_mop("tests/programs/mop_random_adc_sbb", vec![], 2);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 6561);

    #[cfg(has_asm)]
    {
        let mut machine_asm =
            machine_build::asm_v1_mop("tests/programs/mop_random_adc_sbb", vec![]);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.cycles(), 6755);

        let mut machine_asm =
            machine_build::asm_mop("tests/programs/mop_random_adc_sbb", vec![], 2);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.cycles(), 6561);
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
    }
}

#[test]
pub fn test_mop_jump_rel_version1_bug() {
    let mut machine = machine_build::int_v1_imcb("tests/programs/mop_jump_rel_version1_bug");
    let ret = machine.run();
    assert_eq!(ret, Err(Error::MemOutOfBound));
    assert_eq!(*machine.pc(), 0xffffffff8000f878);

    let mut machine = machine_build::int_v1_mop("tests/programs/mop_jump_rel_version1_bug", vec![]);
    let ret = machine.run();
    assert_eq!(ret, Err(Error::MemOutOfBound));
    assert_eq!(*machine.pc(), 0x8000f878);

    let mut machine = machine_build::int_mop("tests/programs/mop_jump_rel_version1_bug", vec![], 2);
    let ret = machine.run();
    assert_eq!(ret, Err(Error::MemOutOfBound));
    assert_eq!(*machine.pc(), 0xffffffff8000f878);

    #[cfg(has_asm)]
    {
        let mut machine_asm =
            machine_build::asm_v1_mop("tests/programs/mop_jump_rel_version1_bug", vec![]);
        let ret_asm = machine_asm.run();
        assert_eq!(ret_asm, Err(Error::MemOutOfBound));
        assert_eq!(*machine_asm.machine.pc(), 0x8000f878);

        let mut machine_asm =
            machine_build::asm_mop("tests/programs/mop_jump_rel_version1_bug", vec![], 2);
        let ret_asm = machine_asm.run();
        assert_eq!(ret_asm, Err(Error::MemOutOfBound));
        assert_eq!(*machine_asm.machine.pc(), 0xffffffff8000f878);
    }
}

#[test]
pub fn test_mop_jump_rel_version1_reg_not_updated_bug() {
    let mut machine =
        machine_build::int_v1_imcb("tests/programs/mop_jump_rel_version1_reg_not_updated_bug");
    let ret = machine.run();
    assert_eq!(ret, Err(Error::MemOutOfBound));
    assert_eq!(machine.registers()[A0], 67174520);

    let mut machine = machine_build::int_v1_mop(
        "tests/programs/mop_jump_rel_version1_reg_not_updated_bug",
        vec![],
    );
    let ret = machine.run();
    assert_eq!(ret, Err(Error::MemOutOfBound));
    assert_eq!(machine.registers()[A0], 0);

    let mut machine = machine_build::int_mop(
        "tests/programs/mop_jump_rel_version1_reg_not_updated_bug",
        vec![],
        2,
    );
    let ret = machine.run();
    assert_eq!(ret, Err(Error::MemOutOfBound));
    assert_eq!(machine.registers()[A0], 67174520);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_mop(
            "tests/programs/mop_jump_rel_version1_reg_not_updated_bug",
            vec![],
        );
        let ret_asm = machine_asm.run();
        assert_eq!(ret_asm, Err(Error::MemOutOfBound));
        assert_eq!(machine_asm.machine.registers()[A0], 0);

        let mut machine_asm = machine_build::asm_mop(
            "tests/programs/mop_jump_rel_version1_reg_not_updated_bug",
            vec![],
            2,
        );
        let ret_asm = machine_asm.run();
        assert_eq!(ret_asm, Err(Error::MemOutOfBound));
        assert_eq!(machine_asm.machine.registers()[A0], 67174520);
    }
}

#[test]
pub fn test_mop_jump_abs_version1_reg_not_updated_bug() {
    let mut machine =
        machine_build::int_v1_imcb("tests/programs/mop_jump_abs_version1_reg_not_updated_bug");
    let ret = machine.run();
    assert_eq!(ret, Err(Error::MemOutOfBound));
    assert_eq!(machine.registers()[A0], 67108864);

    let mut machine = machine_build::int_v1_mop(
        "tests/programs/mop_jump_abs_version1_reg_not_updated_bug",
        vec![],
    );
    let ret = machine.run();
    assert_eq!(ret, Err(Error::MemOutOfBound));
    assert_eq!(machine.registers()[A0], 0);

    let mut machine = machine_build::int_mop(
        "tests/programs/mop_jump_abs_version1_reg_not_updated_bug",
        vec![],
        2,
    );
    let ret = machine.run();
    assert_eq!(ret, Err(Error::MemOutOfBound));
    assert_eq!(machine.registers()[A0], 67108864);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_mop(
            "tests/programs/mop_jump_abs_version1_reg_not_updated_bug",
            vec![],
        );
        let ret_asm = machine_asm.run();
        assert_eq!(ret_asm, Err(Error::MemOutOfBound));
        assert_eq!(machine_asm.machine.registers()[A0], 0);

        let mut machine_asm = machine_build::asm_mop(
            "tests/programs/mop_jump_abs_version1_reg_not_updated_bug",
            vec![],
            2,
        );
        let ret_asm = machine_asm.run();
        assert_eq!(ret_asm, Err(Error::MemOutOfBound));
        assert_eq!(machine_asm.machine.registers()[A0], 67108864);
    }
}
