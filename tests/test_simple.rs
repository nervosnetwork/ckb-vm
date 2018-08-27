extern crate ckb_riscv;

use ckb_riscv::run;
use std::fs::File;
use std::io::Read;

#[test]
pub fn test_simple_instructions() {
    let mut file = File::open("tests/programs/simple").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let result = run(&buffer, &vec!["simple".to_string()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}
