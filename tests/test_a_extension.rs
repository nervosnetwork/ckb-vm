#[cfg(has_asm)]
use ckb_vm::{CoreMachine, Memory};
use ckb_vm::{DefaultMachineRunner, Error, ISA_A, ISA_IMC, machine::VERSION2};
pub mod machine_build;

#[test]
pub fn test_write_permission_bug() {
    let mut machine = machine_build::int(
        "tests/programs/amo_write_permission",
        vec![],
        VERSION2,
        ISA_IMC | ISA_A,
    );
    let ret = machine.run();
    assert!(ret.is_err());
    assert_eq!(ret.err(), Some(Error::MemWriteOnExecutablePage(16)));

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/amo_write_permission",
            vec![],
            VERSION2,
            ISA_IMC | ISA_A,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_err());
        assert_eq!(ret_asm.err(), Some(Error::MemWriteOnExecutablePage(16)));
    }
}

#[test]
pub fn test_sc_after_sc() {
    let mut machine = machine_build::int(
        "tests/programs/sc_after_sc",
        vec![],
        VERSION2,
        ISA_IMC | ISA_A,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/sc_after_sc",
            vec![],
            VERSION2,
            ISA_IMC | ISA_A,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }
}

#[test]
pub fn test_sc_only() {
    let mut machine =
        machine_build::int("tests/programs/sc_only", vec![], VERSION2, ISA_IMC | ISA_A);
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm =
            machine_build::asm("tests/programs/sc_only", vec![], VERSION2, ISA_IMC | ISA_A);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }
}

#[test]
pub fn test_amo_compare() {
    let mut machine = machine_build::int(
        "tests/programs/amo_compare",
        vec![],
        VERSION2,
        ISA_IMC | ISA_A,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/amo_compare",
            vec![],
            VERSION2,
            ISA_IMC | ISA_A,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }
}

#[test]
pub fn test_amo_check_write() {
    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/amo_check_write",
            vec![],
            VERSION2,
            ISA_IMC | ISA_A,
        );
        let page_a = 0;
        let page_b = 17;
        let flag_a = machine_asm.machine.memory_mut().fetch_flag(page_a).unwrap();
        assert_eq!(flag_a, 0);
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        assert_eq!(machine_asm.machine.inner_mut().last_write_page, page_b);
        let flag_a = machine_asm.machine.memory_mut().fetch_flag(page_a).unwrap();
        assert_eq!(flag_a, 0);
        let flag_b = machine_asm.machine.memory_mut().fetch_flag(page_b).unwrap();
        assert_eq!(flag_b, 4);
    }
}

#[test]
pub fn test_sc_failed_no_write() {
    // Regression test: a failed SC.W / SC.D must not modify memory.
    const SC_FAILED_VAL_W_ADDR: u64 = 0x11000;
    const SC_FAILED_VAL_D_ADDR: u64 = 0x11004;
    const SC_FAILED_SENTINEL: u64 = 0x12345678;

    let mut machine = machine_build::int(
        "tests/programs/sc_failed_no_write",
        vec![],
        VERSION2,
        ISA_IMC | ISA_A,
    );
    let ret = machine.run();
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);
    // Belt-and-suspenders: assert the sentinel words were not overwritten.
    let val_w = machine
        .machine
        .memory_mut()
        .load32(&SC_FAILED_VAL_W_ADDR)
        .unwrap();
    assert_eq!(
        val_w, SC_FAILED_SENTINEL,
        "failed SC.W must not write memory"
    );
    let val_d = machine
        .machine
        .memory_mut()
        .load64(&SC_FAILED_VAL_D_ADDR)
        .unwrap();
    assert_eq!(
        val_d, SC_FAILED_SENTINEL,
        "failed SC.D must not write memory"
    );

    #[cfg(has_asm)]
    {
        let mut machine_asm = machine_build::asm(
            "tests/programs/sc_failed_no_write",
            vec![],
            VERSION2,
            ISA_IMC | ISA_A,
        );
        let ret_asm = machine_asm.run();
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
        let val_w = machine_asm
            .machine
            .memory_mut()
            .load32(&SC_FAILED_VAL_W_ADDR)
            .unwrap();
        assert_eq!(
            val_w, SC_FAILED_SENTINEL,
            "failed SC.W must not write memory"
        );
        let val_d = machine_asm
            .machine
            .memory_mut()
            .load64(&SC_FAILED_VAL_D_ADDR)
            .unwrap();
        assert_eq!(
            val_d, SC_FAILED_SENTINEL,
            "failed SC.D must not write memory"
        );
    }
}
