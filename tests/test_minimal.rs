use ckb_vm::{machine::VERSION0, run, SparseMemory, ISA_IMC};

#[test]
pub fn test_minimal_with_no_args() {
    let path = "tests/programs/minimal";
    let code = std::fs::read(path).unwrap().into();
    let result = run::<u32, SparseMemory<u32>>(&code, &vec!["minimal".into()], ISA_IMC, VERSION0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[test]
pub fn test_minimal_with_a() {
    let path = "tests/programs/minimal";
    let code = std::fs::read(path).unwrap().into();
    let result = run::<u32, SparseMemory<u32>>(
        &code,
        &vec!["minimal".into(), "a".into()],
        ISA_IMC,
        VERSION0,
    );
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 2);
}

#[test]
pub fn test_minimal_with_b() {
    let path = "tests/programs/minimal";
    let code = std::fs::read(path).unwrap().into();
    let result =
        run::<u32, SparseMemory<u32>>(&code, &vec!["minimal".into(), "".into()], ISA_IMC, VERSION0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}
