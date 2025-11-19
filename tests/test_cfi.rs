use ckb_vm::machine::VERSION3;
use ckb_vm::{DefaultMachineRunner, Error, ISA_B, ISA_CFI, ISA_IMC, ISA_MOP};
pub mod machine_build;

fn run(path: &str) -> Result<i8, Error> {
    let mut machine =
        machine_build::int(path, vec![], VERSION3, ISA_IMC | ISA_B | ISA_MOP | ISA_CFI);
    let ret = machine.run();

    #[cfg(has_asm)]
    {
        let mut machine_asm =
            machine_build::asm(path, vec![], VERSION3, ISA_IMC | ISA_B | ISA_MOP | ISA_CFI);
        let ret_asm = machine_asm.run();
        assert_eq!(ret, ret_asm, "Interpreter and ASM results should match");
    }

    ret
}

#[test]
pub fn test_simple_instructions_64() {
    let ret = run("tests/programs/simple64");
    assert!(ret.is_ok());
}

#[test]
pub fn test_cfi_ss_success() {
    let ret = run("tests/programs/cfi_ss_success");
    assert_eq!(ret, Ok(0));
}

#[test]
pub fn test_cfi_ss_not_active() {
    let ret = run("tests/programs/cfi_ss_not_active");
    assert_eq!(ret, Ok(0));
}

#[test]
pub fn test_cfi_ss_not_active_amo() {
    let ret = run("tests/programs/cfi_ss_not_active_amo");
    assert!(matches!(
        ret,
        Err(Error::InvalidInstruction {
            pc: 69922,
            instruction: 1208135855
        })
    ));
}

#[test]
pub fn test_cfi_ss_stack_downto_zero() {
    let ret = run("tests/programs/cfi_ss_stack_downto_zero");
    assert!(matches!(ret, Err(Error::CFIShadowStackOutOfStack)));
}

#[test]
pub fn test_cfi_ss_stack_full() {
    let ret = run("tests/programs/cfi_ss_stack_full");
    assert_eq!(ret, Ok(0));
}

#[test]
pub fn test_cfi_lpad_unlabeled() {
    let ret = run("tests/programs/cfi_lpad_unlabeled");
    assert_eq!(ret, Ok(0));
}

#[test]
pub fn test_cfi_lpad_not_active() {
    let ret = run("tests/programs/cfi_lpad_not_active");
    assert_eq!(ret, Ok(0));
}

#[test]
pub fn test_cfi_lpad_unlabeled_failed() {
    let ret = run("tests/programs/cfi_lpad_unlabeled_failed");
    assert!(matches!(ret, Err(Error::CFILpadNotFound)));
}

#[test]
pub fn test_cfi_lpad_func_sig() {
    let ret = run("tests/programs/cfi_lpad_func_sig");
    assert!(ret.is_ok());
}

#[test]
pub fn test_cfi_lpad_func_sig_zero() {
    let ret = run("tests/programs/cfi_lpad_func_sig_zero");
    assert!(ret.is_ok());
}

#[test]
pub fn test_cfi_lpad_func_sig_failed() {
    let ret = run("tests/programs/cfi_lpad_func_sig_failed");
    assert!(matches!(ret, Err(Error::CFILpadLabelMismatched)));
}

#[test]
pub fn test_cfi_ss_only_pop() {
    let ret = run("tests/programs/cfi_ss_only_pop");
    assert!(matches!(ret, Err(Error::CFIShadowStackOutOfStack)));
}

#[test]
pub fn test_cfi_ss_popchk_failed() {
    let ret = run("tests/programs/cfi_ss_popchk_failed");
    assert!(matches!(ret, Err(Error::CFIShadowStackValueFault)));
}

#[test]
pub fn test_cfi_lpad_align_failed() {
    let ret = run("tests/programs/cfi_lpad_align_failed");
    assert!(matches!(ret, Err(Error::CFILpadNot4ByteAligned)));
}
