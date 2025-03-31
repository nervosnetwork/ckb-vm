use ckb_vm::decoder::{DefaultDecoder, InstDecoder};
use ckb_vm::instructions::{
    Instruction, Utype, extract_opcode, instruction_length, set_instruction_length_n,
};
use ckb_vm::machine::VERSION1;
#[cfg(has_asm)]
use ckb_vm::machine::asm::{AbstractAsmMachine, AsmCoreMachine, traces::SimpleFixedTraceDecoder};
use ckb_vm::{
    CoreMachine, DefaultCoreMachine, DefaultMachineRunner, Error, ISA_IMC, Memory, Register,
    SparseMemory, SupportMachine, machine::AbstractDefaultMachineBuilder,
};
use ckb_vm_definitions::instructions as insts;
use std::fs;

// This is simplified from https://github.com/xxuejie/ckb-vm-contrib/blob/main/src/decoder.rs
pub struct AuxDecoder {
    inner: DefaultDecoder,
}

impl InstDecoder for AuxDecoder {
    fn new<R: Register>(isa: u8, version: u32) -> Self {
        Self {
            inner: DefaultDecoder::new::<R>(isa, version),
        }
    }

    fn decode<M: Memory>(&mut self, memory: &mut M, pc: u64) -> Result<Instruction, Error> {
        let head_inst = self.inner.decode(memory, pc)?;
        match extract_opcode(head_inst) {
            insts::OP_AUIPC => {
                let i = Utype(head_inst);
                let head_len = instruction_length(head_inst);
                let value = pc.wrapping_add(i64::from(i.immediate_s()) as u64);
                if let Ok(value) = value.try_into() {
                    return Ok(set_instruction_length_n(
                        Utype::new(insts::OP_CUSTOM_LOAD_UIMM, i.rd(), value).0,
                        head_len,
                    ));
                }
            }
            _ => (),
        };

        Ok(head_inst)
    }

    fn reset_instructions_cache(&mut self) -> Result<(), Error> {
        self.inner.reset_instructions_cache()
    }
}

#[test]
pub fn test_rust_auipc_fusion() {
    let buffer = fs::read("tests/programs/auipc_no_sign_extend")
        .unwrap()
        .into();

    let core_machine =
        DefaultCoreMachine::<u64, SparseMemory<u64>>::new(ISA_IMC, VERSION1, u64::MAX);
    let mut machine = AbstractDefaultMachineBuilder::<_, AuxDecoder>::new(core_machine).build();
    machine
        .load_program(&buffer, [Ok("auipc_no_sign_extend".into())].into_iter())
        .unwrap();

    let mut decoder = AuxDecoder::new::<u64>(machine.isa(), machine.version());
    let result = machine.run_with_decoder(&mut decoder).unwrap();
    assert_eq!(result, 0);
}

#[cfg(has_asm)]
#[test]
pub fn test_asm_auipc_fusion() {
    let buffer = fs::read("tests/programs/auipc_no_sign_extend")
        .unwrap()
        .into();

    let asm_core = <AsmCoreMachine as SupportMachine>::new(ISA_IMC, VERSION1, u64::MAX);
    let core =
        AbstractDefaultMachineBuilder::<_, SimpleFixedTraceDecoder<AuxDecoder>>::new(asm_core)
            .build();
    let mut machine = AbstractAsmMachine::new(core);
    machine
        .load_program(&buffer, [Ok("auipc_no_sign_extend".into())].into_iter())
        .unwrap();

    let mut decoder = SimpleFixedTraceDecoder::<AuxDecoder>::new::<u64>(
        machine.machine.isa(),
        machine.machine.version(),
    );

    let result = machine.run_with_decoder(&mut decoder).expect("run");
    assert_eq!(result, 0);
}
