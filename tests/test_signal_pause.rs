#[cfg(has_asm)]
use ckb_vm::machine::asm::{AsmCoreMachine, AsmMachine};
use ckb_vm::{Error, SupportMachine};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
pub mod machine_build;

#[cfg(has_asm)]
#[test]
pub fn test_asm_pause() {
    let expect_cycles = {
        let mut machine = machine_build::asm_v2_imacb("tests/programs/fib35");
        machine.run().unwrap();
        machine.machine.cycles()
    };

    let mut machine = machine_build::asm_v2_imacb("tests/programs/fib35");
    let branch_pause_cnt = Arc::new(AtomicU32::new(0));
    let branch_pause_cnt_jh = branch_pause_cnt.clone();

    let signal = machine.machine.pause();
    let jh = std::thread::spawn(move || loop {
        let result = machine.run();
        if result == Err(Error::Pause) {
            branch_pause_cnt_jh.fetch_add(1, Ordering::SeqCst);
            continue;
        } else {
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 0);
            assert_eq!(machine.machine.cycles(), expect_cycles);
            break;
        }
    });
    for _ in 0..10 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        signal.interrupt()
    }
    jh.join().unwrap();
    assert_eq!(branch_pause_cnt.load(Ordering::SeqCst), 10);
}

#[test]
pub fn test_int_pause() {
    let expect_cycles = {
        let mut machine = machine_build::int_v2_imacb("tests/programs/fib30");
        machine.run().unwrap();
        machine.machine.cycles()
    };

    let mut machine = machine_build::int_v2_imacb("tests/programs/fib30");
    let branch_pause_cnt = Arc::new(AtomicU32::new(0));
    let branch_pause_cnt_jh = branch_pause_cnt.clone();
    let signal = machine.machine.pause();
    let jh = std::thread::spawn(move || loop {
        let result = machine.run();
        if result == Err(Error::Pause) {
            branch_pause_cnt_jh.fetch_add(1, Ordering::SeqCst);
            continue;
        } else {
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 0);
            assert_eq!(machine.machine.cycles(), expect_cycles);
            break;
        }
    });
    for _ in 0..10 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        signal.interrupt()
    }
    jh.join().unwrap();
    assert_eq!(branch_pause_cnt.load(Ordering::SeqCst), 10);
}
