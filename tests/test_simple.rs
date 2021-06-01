use ckb_vm::{
    machine::{trace::TraceMachine, VERSION0},
    memory::wxorx::WXorXMemory,
    DefaultCoreMachine, DefaultMachineBuilder, Error, FlatMemory, SparseMemory, SupportMachine,
    ISA_IMC,
};

#[test]
pub fn test_simple_instructions() {
    let path = "tests/programs/simple";
    let code = std::fs::read(path).unwrap().into();
    let core_machine = DefaultCoreMachine::<u32, WXorXMemory<SparseMemory<u32>>>::new(
        ISA_IMC,
        VERSION0,
        u64::max_value(),
    );
    let mut machine = TraceMachine::new(DefaultMachineBuilder::new(core_machine).build());
    machine.load_program(&code, &vec!["simple".into()]).unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_simple_instructions_64() {
    let path = "tests/programs/simple64";
    let code = std::fs::read(path).unwrap().into();
    let core_machine = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
        ISA_IMC,
        VERSION0,
        u64::max_value(),
    );
    let mut machine = TraceMachine::new(DefaultMachineBuilder::new(core_machine).build());
    machine
        .load_program(&code, &vec!["simple64".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_simple_instructions_flatmemory() {
    let path = "tests/programs/simple";
    let code = std::fs::read(path).unwrap().into();
    let core_machine = DefaultCoreMachine::<u32, WXorXMemory<FlatMemory<u32>>>::new(
        ISA_IMC,
        VERSION0,
        u64::max_value(),
    );
    let mut machine = TraceMachine::new(DefaultMachineBuilder::new(core_machine).build());
    machine.load_program(&code, &vec!["simple".into()]).unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
pub fn test_simple_cycles() {
    let path = "tests/programs/simple64";
    let code = std::fs::read(path).unwrap().into();
    let core_machine = DefaultCoreMachine::<u64, SparseMemory<u64>>::new(ISA_IMC, VERSION0, 517);
    let mut machine =
        DefaultMachineBuilder::<DefaultCoreMachine<u64, SparseMemory<u64>>>::new(core_machine)
            .instruction_cycle_func(Box::new(|_| 1))
            .build();
    machine
        .load_program(&code, &vec!["simple64".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
    assert_eq!(SupportMachine::cycles(&machine), 517);
}

#[test]
pub fn test_simple_max_cycles_reached() {
    let path = "tests/programs/simple64";
    let code = std::fs::read(path).unwrap().into();
    // Running simple64 should consume 517 cycles using dummy cycle func
    let core_machine = DefaultCoreMachine::<u64, SparseMemory<u64>>::new(ISA_IMC, VERSION0, 500);
    let mut machine =
        DefaultMachineBuilder::<DefaultCoreMachine<u64, SparseMemory<u64>>>::new(core_machine)
            .instruction_cycle_func(Box::new(|_| 1))
            .build();
    machine
        .load_program(&code, &vec!["simple64".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidCycles);
}

#[test]
pub fn test_simple_invalid_bits() {
    let path = "tests/programs/simple";
    let code = std::fs::read(path).unwrap().into();
    let core_machine = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
        ISA_IMC,
        VERSION0,
        u64::max_value(),
    );
    let mut machine = TraceMachine::new(DefaultMachineBuilder::new(core_machine).build());
    let result = machine.load_program(&code, &vec!["simple".into()]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidElfBits);
}

#[test]
pub fn test_simple_loaded_bytes() {
    let path = "tests/programs/simple64";
    let code = std::fs::read(path).unwrap().into();
    let core_machine =
        DefaultCoreMachine::<u64, SparseMemory<u64>>::new(ISA_IMC, VERSION0, u64::max_value());
    let mut machine = DefaultMachineBuilder::new(core_machine).build();
    let bytes = machine
        .load_program(&code, &vec!["simple64".into()])
        .unwrap();
    assert_eq!(bytes, 4057);
}
