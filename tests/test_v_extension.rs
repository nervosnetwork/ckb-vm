pub mod machine_build;
use ckb_vm::SupportMachine;

#[test]
pub fn test_v_vadd_vv_32() {
    let mut machine = machine_build::int_v1_imcv("tests/programs/vadd_vv_32");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_imcv("tests/programs/vadd_vv_32");
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }

    #[cfg(has_aot)]
    {
        let code = machine_build::aot_v1_imcv_code("tests/programs/vadd_vv_32");
        let mut machine_aot = machine_build::aot_v1_imcv("tests/programs/vadd_vv_32", code);
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
    }
}

#[test]
pub fn test_v_blst() {
    let mut machine = machine_build::int_v1_imcv("tests/programs/blst_rvv");
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    assert_eq!(machine.machine.cycles(), 16065831);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm_v1_imcv("tests/programs/blst_rvv");
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine.machine.cycles(), 16065831);
    }

    #[cfg(has_aot)]
    {
        let code = machine_build::aot_v1_imcv_code("tests/programs/blst_rvv");
        let mut machine_aot = machine_build::aot_v1_imcv("tests/programs/blst_rvv", code);
        let ret_aot = machine_aot.run();
        assert!(ret_aot.is_ok());
        assert_eq!(ret_aot.unwrap(), 0);
        assert_eq!(machine.machine.cycles(), 16065831);
    }
}
