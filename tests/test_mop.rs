pub mod machine_build;
use ckb_vm::error::OutOfBoundKind;
use ckb_vm::machine::{VERSION1, VERSION2};
use ckb_vm::{
    CoreMachine, DefaultMachineRunner, Error, ISA_B, ISA_IMC, ISA_MOP, SupportMachine,
    registers::A0,
};

#[test]
#[cfg_attr(miri, ignore)]
pub fn test_mop_wide_multiply() {
    let mut machine = machine_build::int(
        "tests/programs/mop_wide_multiply",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    let mut machine = machine_build::int(
        "tests/programs/mop_wide_multiply",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B | ISA_MOP,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 9192427);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/mop_wide_multiply",
            vec![],
            VERSION1,
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.cycles(), 9192427);
    }
}

#[test]
#[cfg_attr(miri, ignore)]
pub fn test_mop_wide_divide() {
    let mut machine = machine_build::int(
        "tests/programs/mop_wide_divide",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    let mut machine = machine_build::int(
        "tests/programs/mop_wide_divide",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B | ISA_MOP,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 6106583);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/mop_wide_divide",
            vec![],
            VERSION1,
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.cycles(), 6106583);
    }
}

#[test]
pub fn test_mop_far_jump() {
    let mut machine = machine_build::int(
        "tests/programs/mop_far_jump",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    let mut machine = machine_build::int(
        "tests/programs/mop_far_jump",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B | ISA_MOP,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 5);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/mop_far_jump",
            vec![],
            VERSION1,
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.cycles(), 5);
    }
}

#[test]
#[cfg_attr(miri, ignore)]
pub fn test_mop_ld_32_constants() {
    let mut machine = machine_build::int(
        "tests/programs/mop_ld_signextend_32",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    let mut machine = machine_build::int(
        "tests/programs/mop_ld_signextend_32",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B | ISA_MOP,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/mop_ld_signextend_32",
            vec![],
            VERSION1,
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }
}

#[test]
pub fn test_mop_adc() {
    let mut machine =
        machine_build::int("tests/programs/mop_adc", vec![], VERSION1, ISA_IMC | ISA_B);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 73);

    let mut machine = machine_build::int(
        "tests/programs/mop_adc",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B | ISA_MOP,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 61);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/mop_adc",
            vec![],
            VERSION1,
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.cycles(), 61);
    }
}

#[test]
pub fn test_mop_adcs() {
    let mut machine =
        machine_build::int("tests/programs/mop_adcs", vec![], VERSION1, ISA_IMC | ISA_B);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 53);

    let mut machine = machine_build::int(
        "tests/programs/mop_adcs",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B | ISA_MOP,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 53);

    let mut machine = machine_build::int(
        "tests/programs/mop_adcs",
        vec![],
        VERSION2,
        ISA_IMC | ISA_B | ISA_MOP,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 47);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/mop_adcs",
            vec![],
            VERSION1,
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.cycles(), 53);

        let mut machine_asm = machine_build::asm(
            "tests/programs/mop_adcs",
            vec![],
            VERSION2,
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.cycles(), 47);
    }
}

#[test]
pub fn test_mop_add3() {
    let mut machine =
        machine_build::int("tests/programs/mop_add3", vec![], VERSION1, ISA_IMC | ISA_B);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0, "Machine state: {}", machine.machine);
    assert_eq!(machine.machine.cycles(), 1047);

    let mut machine = machine_build::int(
        "tests/programs/mop_add3",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B | ISA_MOP,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 939);

    let mut machine = machine_build::int(
        "tests/programs/mop_add3",
        vec![],
        VERSION2,
        ISA_IMC | ISA_B | ISA_MOP,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 903);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/mop_add3",
            vec![],
            VERSION1,
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.cycles(), 939);

        let mut machine_asm = machine_build::asm(
            "tests/programs/mop_add3",
            vec![],
            VERSION2,
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.cycles(), 903);
    }
}

#[test]
pub fn test_mop_sbb() {
    let mut machine =
        machine_build::int("tests/programs/mop_sbb", vec![], VERSION1, ISA_IMC | ISA_B);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 35);

    let mut machine = machine_build::int(
        "tests/programs/mop_sbb",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B | ISA_MOP,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 27);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/mop_sbb",
            vec![],
            VERSION1,
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.cycles(), 27);
    }
}

#[test]
pub fn test_mop_sbbs() {
    let mut machine =
        machine_build::int("tests/programs/mop_sbbs", vec![], VERSION1, ISA_IMC | ISA_B);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0, "Machine state: {}", machine.machine);
    assert_eq!(machine.machine.cycles(), 87);

    let mut machine = machine_build::int(
        "tests/programs/mop_sbbs",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B | ISA_MOP,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 81);

    let mut machine = machine_build::int(
        "tests/programs/mop_sbbs",
        vec![],
        VERSION2,
        ISA_IMC | ISA_B | ISA_MOP,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 76);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/mop_sbbs",
            vec![],
            VERSION1,
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.cycles(), 81);

        let mut machine_asm = machine_build::asm(
            "tests/programs/mop_sbbs",
            vec![],
            VERSION2,
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.cycles(), 76);
    }
}

#[test]
pub fn test_mop_random_adc_sbb() {
    let mut machine = machine_build::int(
        "tests/programs/mop_random_adc_sbb",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 9458);

    let mut machine = machine_build::int(
        "tests/programs/mop_random_adc_sbb",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B | ISA_MOP,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 6755);

    let mut machine = machine_build::int(
        "tests/programs/mop_random_adc_sbb",
        vec![],
        VERSION2,
        ISA_IMC | ISA_B | ISA_MOP,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 6561);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/mop_random_adc_sbb",
            vec![],
            VERSION1,
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.cycles(), 6755);

        let mut machine_asm = machine_build::asm(
            "tests/programs/mop_random_adc_sbb",
            vec![],
            VERSION2,
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.cycles(), 6561);
    }
}

#[test]
pub fn test_mop_ld_signextend_32_overflow_bug() {
    let mut machine = machine_build::int(
        "tests/programs/mop_ld_signextend_32_overflow_bug",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B | ISA_MOP,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/mop_ld_signextend_32_overflow_bug",
            vec![],
            VERSION1,
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }
}

#[test]
pub fn test_mop_wide_mul_zero() {
    let mut machine = machine_build::int(
        "tests/programs/mop_wide_mul_zero",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B | ISA_MOP,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/mop_wide_mul_zero",
            vec![],
            VERSION1,
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }
}

#[test]
pub fn test_mop_wide_div_zero() {
    let mut machine = machine_build::int(
        "tests/programs/mop_wide_div_zero",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B | ISA_MOP,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/mop_wide_div_zero",
            vec![],
            VERSION1,
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }
}

#[test]
pub fn test_mop_jump_rel_version1_bug() {
    let mut machine = machine_build::int(
        "tests/programs/mop_jump_rel_version1_bug",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B,
    );
    let ret = machine.run();
    assert_eq!(
        ret,
        Err(Error::MemOutOfBound(
            0xffffffff8000f878,
            OutOfBoundKind::Memory
        ))
    );
    assert_eq!(*machine.pc(), 0xffffffff8000f878);

    let mut machine = machine_build::int(
        "tests/programs/mop_jump_rel_version1_bug",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B | ISA_MOP,
    );
    let ret = machine.run();
    assert_eq!(
        ret,
        Err(Error::MemOutOfBound(0x8000f878, OutOfBoundKind::Memory))
    );
    assert_eq!(*machine.pc(), 0x8000f878);

    let mut machine = machine_build::int(
        "tests/programs/mop_jump_rel_version1_bug",
        vec![],
        VERSION2,
        ISA_IMC | ISA_B | ISA_MOP,
    );
    let ret = machine.run();
    assert_eq!(
        ret,
        Err(Error::MemOutOfBound(
            0xffffffff8000f878,
            OutOfBoundKind::Memory
        ))
    );
    assert_eq!(*machine.pc(), 0xffffffff8000f878);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/mop_jump_rel_version1_bug",
            vec![],
            VERSION1,
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let ret_asm = machine_asm.run();
        assert_eq!(
            ret_asm,
            Err(Error::MemOutOfBound(0x8000f878, OutOfBoundKind::Memory))
        );
        assert_eq!(*machine_asm.machine.pc(), 0x8000f878);

        let mut machine_asm = machine_build::asm(
            "tests/programs/mop_jump_rel_version1_bug",
            vec![],
            VERSION2,
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let ret_asm = machine_asm.run();
        assert_eq!(
            ret_asm,
            Err(Error::MemOutOfBound(
                0xffffffff8000f878,
                OutOfBoundKind::Memory
            ))
        );
        assert_eq!(*machine_asm.machine.pc(), 0xffffffff8000f878);
    }
}

#[test]
pub fn test_mop_jump_rel_version1_reg_not_updated_bug() {
    let mut machine = machine_build::int(
        "tests/programs/mop_jump_rel_version1_reg_not_updated_bug",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B,
    );
    let ret = machine.run();
    assert_eq!(
        ret,
        Err(Error::MemOutOfBound(0x401054a, OutOfBoundKind::Memory))
    );
    assert_eq!(machine.registers()[A0], 67174520);

    let mut machine = machine_build::int(
        "tests/programs/mop_jump_rel_version1_reg_not_updated_bug",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B | ISA_MOP,
    );
    let ret = machine.run();
    assert_eq!(
        ret,
        Err(Error::MemOutOfBound(0x401054a, OutOfBoundKind::Memory))
    );
    assert_eq!(machine.registers()[A0], 0);

    let mut machine = machine_build::int(
        "tests/programs/mop_jump_rel_version1_reg_not_updated_bug",
        vec![],
        VERSION2,
        ISA_IMC | ISA_B | ISA_MOP,
    );
    let ret = machine.run();
    assert_eq!(
        ret,
        Err(Error::MemOutOfBound(0x401054a, OutOfBoundKind::Memory))
    );
    assert_eq!(machine.registers()[A0], 67174520);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/mop_jump_rel_version1_reg_not_updated_bug",
            vec![],
            VERSION1,
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let ret_asm = machine_asm.run();
        assert_eq!(
            ret_asm,
            Err(Error::MemOutOfBound(0x401054a, OutOfBoundKind::Memory))
        );
        assert_eq!(machine_asm.machine.registers()[A0], 0);

        let mut machine_asm = machine_build::asm(
            "tests/programs/mop_jump_rel_version1_reg_not_updated_bug",
            vec![],
            VERSION2,
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let ret_asm = machine_asm.run();
        assert_eq!(
            ret_asm,
            Err(Error::MemOutOfBound(0x401054a, OutOfBoundKind::Memory))
        );
        assert_eq!(machine_asm.machine.registers()[A0], 67174520);
    }
}

#[test]
pub fn test_mop_jump_abs_version1_reg_not_updated_bug() {
    let mut machine = machine_build::int(
        "tests/programs/mop_jump_abs_version1_reg_not_updated_bug",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B,
    );
    let ret = machine.run();
    assert_eq!(
        ret,
        Err(Error::MemOutOfBound(0x40004d2, OutOfBoundKind::Memory))
    );
    assert_eq!(machine.registers()[A0], 67108864);

    let mut machine = machine_build::int(
        "tests/programs/mop_jump_abs_version1_reg_not_updated_bug",
        vec![],
        VERSION1,
        ISA_IMC | ISA_B | ISA_MOP,
    );
    let ret = machine.run();
    assert_eq!(
        ret,
        Err(Error::MemOutOfBound(0x40004d2, OutOfBoundKind::Memory))
    );
    assert_eq!(machine.registers()[A0], 0);

    let mut machine = machine_build::int(
        "tests/programs/mop_jump_abs_version1_reg_not_updated_bug",
        vec![],
        VERSION2,
        ISA_IMC | ISA_B | ISA_MOP,
    );
    let ret = machine.run();
    assert_eq!(
        ret,
        Err(Error::MemOutOfBound(0x40004d2, OutOfBoundKind::Memory))
    );
    assert_eq!(machine.registers()[A0], 67108864);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/mop_jump_abs_version1_reg_not_updated_bug",
            vec![],
            VERSION1,
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let ret_asm = machine_asm.run();
        assert_eq!(
            ret_asm,
            Err(Error::MemOutOfBound(0x40004d2, OutOfBoundKind::Memory))
        );
        assert_eq!(machine_asm.machine.registers()[A0], 0);

        let mut machine_asm = machine_build::asm(
            "tests/programs/mop_jump_abs_version1_reg_not_updated_bug",
            vec![],
            VERSION2,
            ISA_IMC | ISA_B | ISA_MOP,
        );
        let ret_asm = machine_asm.run();
        assert_eq!(
            ret_asm,
            Err(Error::MemOutOfBound(0x40004d2, OutOfBoundKind::Memory))
        );
        assert_eq!(machine_asm.machine.registers()[A0], 67108864);
    }
}
