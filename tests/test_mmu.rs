extern crate ckb_vm;

use bytes::Bytes;
use ckb_vm::{run, Mmu};
use std::fs::File;
use std::io::Read;

#[test]
pub fn test_invalid_write() {
    let mut file = File::open("tests/programs/invalidwrite").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u32, Mmu<u32>>(&buffer, &["invalidwrite".into()]);
    assert!(result.is_err());
}

#[test]
pub fn test_invalid_exec() {
    let mut file = File::open("tests/programs/invalidexec").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let result = run::<u32, Mmu<u32>>(&buffer, &["invalidexec".into()]);
    assert!(result.is_err());
}
