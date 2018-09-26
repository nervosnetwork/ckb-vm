extern crate ckb_vm;

use ckb_vm::run;
use std::fs::File;
use std::io::Read;

#[test]
pub fn test_andi() {
    let mut file = File::open("tests/programs/andi").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let result = run(&buffer, &vec!["andi".to_string()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_nop() {
    let mut file = File::open("tests/programs/nop").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let result = run(&buffer, &vec!["nop".to_string()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}
