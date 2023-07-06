use bytes::Bytes;
use ckb_vm::cost_model::constant_cycles;
#[cfg(has_asm)]
use ckb_vm::machine::asm::{AsmCoreMachine, AsmMachine};
use ckb_vm::machine::{trace::TraceMachine, DefaultCoreMachine, VERSION1, VERSION2};
use ckb_vm::registers::{A0, A7};
use ckb_vm::{
    DefaultMachineBuilder, Error, ExecutionContext, Register, SparseMemory, SupportMachine,
    WXorXMemory, ISA_A, ISA_B, ISA_IMC, ISA_MOP,
};

pub struct SleepContext;

impl<Mac: SupportMachine> ExecutionContext<Mac> for SleepContext {
    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, Error> {
        let code = &machine.registers()[A7];
        if code.to_i32() != 1000 {
            return Ok(false);
        }
        let duration = machine.registers()[A0].to_u64();
        std::thread::sleep(std::time::Duration::from_millis(duration));

        machine.set_register(A0, Mac::REG::from_u8(0));
        Ok(true)
    }
    fn instruction_cycles(&self, inst: ckb_vm::Instruction) -> u64 {
        constant_cycles(inst)
    }
}

#[cfg(has_asm)]
pub fn asm_v1_imcb(path: &str) -> AsmMachine<SleepContext> {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_B, VERSION1, u64::max_value());
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
        .context(SleepContext)
        .build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec![Bytes::from("main")])
        .unwrap();
    machine
}

pub fn int_v1_imcb(
    path: &str,
) -> TraceMachine<DefaultCoreMachine<u64, WXorXMemory<SparseMemory<u64>>>, SleepContext> {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let core_machine = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
        ISA_IMC | ISA_B,
        VERSION1,
        u64::max_value(),
    );
    let mut machine = TraceMachine::new(
        DefaultMachineBuilder::new(core_machine)
            .context(SleepContext)
            .build(),
    );
    machine
        .load_program(&buffer, &vec![Bytes::from("main")])
        .unwrap();
    machine
}

#[cfg(has_asm)]
pub fn asm_v1_mop(path: &str, args: Vec<Bytes>) -> AsmMachine<SleepContext> {
    asm_mop(path, args, VERSION1)
}

#[cfg(has_asm)]
pub fn asm_mop(path: &str, args: Vec<Bytes>, version: u32) -> AsmMachine<SleepContext> {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_B | ISA_MOP, version, u64::max_value());
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
        .context(SleepContext)
        .build();
    let mut machine = AsmMachine::new(core);
    let mut argv = vec![Bytes::from("main")];
    argv.extend_from_slice(&args);
    machine.load_program(&buffer, &argv).unwrap();
    machine
}

pub fn int_v1_mop(
    path: &str,
    args: Vec<Bytes>,
) -> TraceMachine<DefaultCoreMachine<u64, WXorXMemory<SparseMemory<u64>>>, SleepContext> {
    int_mop(path, args, VERSION1)
}

pub fn int_mop(
    path: &str,
    args: Vec<Bytes>,
    version: u32,
) -> TraceMachine<DefaultCoreMachine<u64, WXorXMemory<SparseMemory<u64>>>, SleepContext> {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let core_machine = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
        ISA_IMC | ISA_B | ISA_MOP,
        version,
        u64::max_value(),
    );
    let mut machine = TraceMachine::new(
        DefaultMachineBuilder::new(core_machine)
            .context(SleepContext)
            .build(),
    );
    let mut argv = vec![Bytes::from("main")];
    argv.extend_from_slice(&args);
    machine.load_program(&buffer, &argv).unwrap();
    machine
}

#[cfg(has_asm)]
pub fn asm_v2_imacb(path: &str) -> AsmMachine<SleepContext> {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_A | ISA_B, VERSION2, u64::max_value());
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
        .context(SleepContext)
        .build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec![Bytes::from("main")])
        .unwrap();
    machine
}

pub fn int_v2_imacb(
    path: &str,
) -> TraceMachine<DefaultCoreMachine<u64, WXorXMemory<SparseMemory<u64>>>, SleepContext> {
    let buffer: Bytes = std::fs::read(path).unwrap().into();
    let core_machine = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
        ISA_IMC | ISA_A | ISA_B,
        VERSION2,
        u64::max_value(),
    );
    let mut machine = TraceMachine::new(
        DefaultMachineBuilder::new(core_machine)
            .context(SleepContext)
            .build(),
    );
    machine
        .load_program(&buffer, &vec![Bytes::from("main")])
        .unwrap();
    machine
}
