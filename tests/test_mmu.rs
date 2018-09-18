extern crate ckb_riscv;

use ckb_riscv::run;
use std::fs::File;
use std::io::Read;

#[test]
pub fn test_invalid_write() {
    let mut file = File::open("tests/programs/invalidwrite").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let result = run(&buffer, &["invalidwrite".to_string()]);
    assert!(result.is_err());
}

#[test]
pub fn test_invalid_exec() {
    let mut file = File::open("tests/programs/invalidexec").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let result = run(&buffer, &["invalidexec".to_string()]);
    assert!(result.is_err());
}
