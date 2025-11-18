use ckb_vm::machine::VERSION3;
use ckb_vm::{DefaultMachineRunner, Error, ISA_B, ISA_CFI, ISA_IMC, ISA_MOP};
pub mod machine_build;

fn run_int(path: &str) -> Result<i8, Error> {
    let mut machine =
        machine_build::int(path, vec![], VERSION3, ISA_IMC | ISA_B | ISA_MOP | ISA_CFI);
    machine.run()
}

#[cfg(has_asm)]
fn run_asm(path: &str) -> Result<i8, Error> {
    let mut machine =
        machine_build::asm(path, vec![], VERSION3, ISA_IMC | ISA_B | ISA_MOP | ISA_CFI);
    machine.run()
}

#[test]
pub fn test_simple_instructions_64() {
    let ret = run_int("tests/programs/simple64");
    assert!(ret.is_ok());

    #[cfg(has_asm)]
    {
        let ret_asm = run_asm("tests/programs/simple64");
        assert!(ret_asm.is_ok());
    }
}

#[test]
pub fn test_cfi_ss_success() {
    let ret = run_int("tests/programs/cfi_ss_success");
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let ret_asm = run_asm("tests/programs/cfi_ss_success");
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }
}

#[test]
pub fn test_cfi_ss_not_active() {
    let ret = run_int("tests/programs/cfi_ss_not_active");
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let ret_asm = run_asm("tests/programs/cfi_ss_not_active");
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }
}

#[test]
pub fn test_cfi_ss_stack_downto_zero() {
    let ret = run_int("tests/programs/cfi_ss_stack_downto_zero");
    assert!(ret.is_err());

    #[cfg(has_asm)]
    {
        let ret_asm = run_asm("tests/programs/cfi_ss_stack_downto_zero");
        assert!(ret_asm.is_err());
    }
}

#[test]
pub fn test_cfi_ss_stack_full() {
    let ret = run_int("tests/programs/cfi_ss_stack_full");
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let ret_asm = run_asm("tests/programs/cfi_ss_stack_full");
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }
}

#[test]
pub fn test_cfi_lpad_unlabeled() {
    let ret = run_int("tests/programs/cfi_lpad_unlabeled");
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let ret_asm = run_asm("tests/programs/cfi_lpad_unlabeled");
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }
}

#[test]
pub fn test_cfi_lpad_not_active() {
    let ret = run_int("tests/programs/cfi_lpad_not_active");
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), 0);

    #[cfg(has_asm)]
    {
        let ret_asm = run_asm("tests/programs/cfi_lpad_not_active");
        assert!(ret_asm.is_ok());
        assert_eq!(ret_asm.unwrap(), 0);
    }
}

#[test]
pub fn test_cfi_lpad_unlabeled_failed() {
    let ret = run_int("tests/programs/cfi_lpad_unlabeled_failed");
    assert!(ret.is_err());

    #[cfg(has_asm)]
    {
        let ret_asm = run_asm("tests/programs/cfi_lpad_unlabeled_failed");
        assert!(ret_asm.is_err());
    }
}

#[test]
pub fn test_cfi_lpad_func_sig() {
    let ret = run_int("tests/programs/cfi_lpad_func_sig");
    assert!(ret.is_ok());

    #[cfg(has_asm)]
    {
        let ret_asm = run_asm("tests/programs/cfi_lpad_func_sig");
        assert!(ret_asm.is_ok());
    }
}

#[test]
pub fn test_cfi_lpad_func_sig_zero() {
    let ret = run_int("tests/programs/cfi_lpad_func_sig_zero");
    assert!(ret.is_ok());

    #[cfg(has_asm)]
    {
        let ret_asm = run_asm("tests/programs/cfi_lpad_func_sig_zero");
        assert!(ret_asm.is_ok());
    }
}

#[test]
pub fn test_cfi_lpad_func_sig_failed() {
    let ret = run_int("tests/programs/cfi_lpad_func_sig_failed");
    assert!(ret.is_err());

    #[cfg(has_asm)]
    {
        let ret_asm = run_asm("tests/programs/cfi_lpad_func_sig_failed");
        assert!(ret_asm.is_err());
    }
}

#[test]
pub fn test_cfi_ss_only_pop() {
    let ret = run_int("tests/programs/cfi_ss_only_pop");
    assert!(ret.is_err());

    #[cfg(has_asm)]
    {
        let ret_asm = run_asm("tests/programs/cfi_ss_only_pop");
        assert!(ret_asm.is_err());
    }
}

#[test]
pub fn test_cfi_ss_popchk_failed() {
    let ret = run_int("tests/programs/cfi_ss_popchk_failed");
    assert!(ret.is_err());

    #[cfg(has_asm)]
    {
        let ret_asm = run_asm("tests/programs/cfi_ss_popchk_failed");
        assert!(ret_asm.is_err());
    }
}
