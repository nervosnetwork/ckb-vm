use ckb_vm::decoder::{build_decoder, Decoder, InstDecoder};
use ckb_vm::instructions::{
    extract_opcode, instruction_length, set_instruction_length_n, Instruction, Utype,
};
#[cfg(has_asm)]
use ckb_vm::machine::asm::{traces::SimpleFixedTraceDecoder, AsmCoreMachine, AsmMachine};
use ckb_vm::machine::VERSION1;
use ckb_vm::{
    CoreMachine, DefaultCoreMachine, DefaultMachineBuilder, Error, Memory, SparseMemory, ISA_IMC,
};
use ckb_vm_definitions::instructions as insts;
use std::fs;

// This is simplified from https://github.com/xxuejie/ckb-vm-contrib/blob/main/src/decoder.rs
pub struct AuxDecoder {
    inner: Decoder,
}

impl AuxDecoder {
    pub fn new(inner: Decoder) -> Self {
        Self { inner }
    }
}

impl InstDecoder for AuxDecoder {
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
    let mut machine = DefaultMachineBuilder::new(core_machine).build();
    machine
        .load_program(&buffer, &vec!["auipc_no_sign_extend".into()])
        .unwrap();

    let mut decoder = AuxDecoder::new(build_decoder::<u64>(machine.isa(), machine.version()));
    let result = machine.run_with_decoder(&mut decoder).unwrap();
    assert_eq!(result, 0);
}

#[cfg(has_asm)]
#[test]
pub fn test_asm_auipc_fusion() {
    let buffer = fs::read("tests/programs/auipc_no_sign_extend")
        .unwrap()
        .into();

    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION1, u64::MAX);
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core).build();
    let mut machine = AsmMachine::new(core);
    machine
        .load_program(&buffer, &vec!["auipc_no_sign_extend".into()])
        .unwrap();

    let decoder = AuxDecoder::new(build_decoder::<u64>(
        machine.machine.isa(),
        machine.machine.version(),
    ));
    let mut decoder = SimpleFixedTraceDecoder::new(decoder);

    let result = machine.run_with_decoder(&mut decoder).expect("run");
    assert_eq!(result, 0);
}
