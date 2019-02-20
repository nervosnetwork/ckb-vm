use ckb_vm::{default_jit_machine, BaselineJitMachine, TcgTracer};
use std::fs::File;
use std::io::Read;

#[test]
pub fn test_tcg_simple64() {
    let mut file = File::open("tests/programs/simple64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let machine = BaselineJitMachine::new(&buffer, Box::new(TcgTracer::default()));
    let result = machine.run(&vec![b"simple".to_vec()]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().0, 0);
}

#[test]
pub fn test_jit_simple64() {
    let mut file = File::open("tests/programs/simple64").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let mut pair = (255, default_jit_machine(&buffer));

    // Run the program 20 times to make sure JIT is triggered.
    for _ in 1..20 {
        pair = pair.1.run(&vec![b"simple".to_vec()]).unwrap();
        assert_eq!(pair.0, 0);
        pair.0 = 255;
    }
}
