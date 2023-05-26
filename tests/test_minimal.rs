use ckb_vm::{run, SparseMemory};
use std::fs;

#[test]
pub fn test_minimal_with_no_args() {
    let buffer = fs::read("tests/programs/minimal").unwrap().into();
    let result = run::<u32, SparseMemory<u32>>(&buffer, &vec!["minimal".into()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[test]
pub fn test_minimal_with_a() {
    let buffer = fs::read("tests/programs/minimal").unwrap().into();
    let result = run::<u32, SparseMemory<u32>>(&buffer, &vec!["minimal".into(), "a".into()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 2);
}

#[test]
pub fn test_minimal_with_b() {
    let buffer = fs::read("tests/programs/minimal").unwrap().into();
    let result = run::<u32, SparseMemory<u32>>(&buffer, &vec!["minimal".into(), "".into()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}
