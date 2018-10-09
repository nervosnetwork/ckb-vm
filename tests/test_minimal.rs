extern crate ckb_vm;

use ckb_vm::{run, SparseMemory};
use std::fs::File;
use std::io::Read;

#[test]
pub fn test_minimal_with_no_args() {
    let mut file = File::open("tests/programs/minimal").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let result = run::<u32, SparseMemory>(&buffer, &vec![b"minimal".to_vec()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[test]
pub fn test_minimal_with_a() {
    let mut file = File::open("tests/programs/minimal").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let result = run::<u32, SparseMemory>(&buffer, &vec![b"minimal".to_vec(), b"a".to_vec()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 2);
}

#[test]
pub fn test_minimal_with_b() {
    let mut file = File::open("tests/programs/minimal").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let result = run::<u32, SparseMemory>(&buffer, &vec![b"minimal".to_vec(), b"b".to_vec()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}
