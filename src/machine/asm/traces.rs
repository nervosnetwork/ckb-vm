use crate::{
    ckb_vm_definitions::{
        asm::{calculate_slot, FixedTrace, TRACE_ITEM_LENGTH, TRACE_SIZE},
        instructions::{Instruction, OP_CUSTOM_TRACE_END},
    },
    decoder::InstDecoder,
    error::Error,
    instructions::{
        blank_instruction, extract_opcode, instruction_length, is_basic_block_end_instruction,
    },
    machine::{
        asm::{ckb_vm_asm_labels, AsmCoreMachine},
        CoreMachine, DefaultMachine,
    },
    memory::Memory,
};
use std::alloc::{alloc, alloc_zeroed, Layout};
use std::collections::HashMap;

pub trait TraceDecoder: InstDecoder {
    fn fixed_traces(&self) -> *const FixedTrace;
    fn fixed_trace_size(&self) -> u64;
    fn prepare_traces(
        &mut self,
        machine: &mut DefaultMachine<Box<AsmCoreMachine>>,
    ) -> Result<(), Error>;
    fn reset(&mut self) -> Result<(), Error>;
}

pub fn decode_fixed_trace<D: InstDecoder>(
    decoder: &mut D,
    machine: &mut DefaultMachine<Box<AsmCoreMachine>>,
    maximum_insts: Option<usize>,
) -> Result<(FixedTrace, bool), Error> {
    let pc = *machine.pc();

    let mut trace = FixedTrace::default();
    let mut current_pc = pc;
    let mut i = 0;
    let mut basic_block_end = false;

    let size = match maximum_insts {
        Some(items) => std::cmp::min(items, TRACE_ITEM_LENGTH),
        None => TRACE_ITEM_LENGTH,
    };
    while i < size {
        let instruction = decoder.decode(machine.memory_mut(), current_pc)?;
        let end_instruction = is_basic_block_end_instruction(instruction);
        current_pc += u64::from(instruction_length(instruction));
        trace.cycles += machine.instruction_cycle_func()(instruction);
        let opcode = extract_opcode(instruction);
        // Here we are calculating the absolute address used in direct threading
        // from label offsets.
        trace.set_thread(i, instruction, unsafe {
            u64::from(*(ckb_vm_asm_labels as *const u32).offset(opcode as u8 as isize))
                + (ckb_vm_asm_labels as *const u32 as u64)
        });
        i += 1;
        if end_instruction {
            basic_block_end = true;
            break;
        }
    }
    trace.set_thread(i, blank_instruction(OP_CUSTOM_TRACE_END), unsafe {
        u64::from(*(ckb_vm_asm_labels as *const u32).offset(OP_CUSTOM_TRACE_END as isize))
            + (ckb_vm_asm_labels as *const u32 as u64)
    });
    trace.address = pc;
    trace.length = (current_pc - pc) as u32;
    Ok((trace, basic_block_end))
}

/// A simple and naive trace decoder that only works with 8192 fixed traces.
/// It serves as the default implementation.
pub struct SimpleFixedTraceDecoder<D: InstDecoder> {
    traces: Box<[FixedTrace; TRACE_SIZE]>,
    decoder: D,
}

impl<D: InstDecoder> SimpleFixedTraceDecoder<D> {
    pub fn new(decoder: D) -> Self {
        let traces = unsafe {
            let layout = Layout::array::<FixedTrace>(TRACE_SIZE).unwrap();
            let raw_allocation = alloc_zeroed(layout) as *mut _;
            Box::from_raw(raw_allocation)
        };
        Self { decoder, traces }
    }
}

impl<D: InstDecoder> TraceDecoder for SimpleFixedTraceDecoder<D> {
    fn fixed_traces(&self) -> *const FixedTrace {
        self.traces.as_ptr()
    }

    fn fixed_trace_size(&self) -> u64 {
        TRACE_SIZE as u64
    }

    fn prepare_traces(
        &mut self,
        machine: &mut DefaultMachine<Box<AsmCoreMachine>>,
    ) -> Result<(), Error> {
        let (trace, _) = decode_fixed_trace(&mut self.decoder, machine, None)?;
        let slot = calculate_slot(*machine.pc());
        self.traces[slot] = trace;
        Ok(())
    }

    fn reset(&mut self) -> Result<(), Error> {
        for i in 0..TRACE_SIZE {
            self.traces[i] = FixedTrace::default();
        }
        self.decoder.reset_instructions_cache()
    }
}

impl<D: InstDecoder> InstDecoder for SimpleFixedTraceDecoder<D> {
    fn decode<M: Memory>(&mut self, memory: &mut M, pc: u64) -> Result<Instruction, Error> {
        self.decoder.decode(memory, pc)
    }

    fn reset_instructions_cache(&mut self) -> Result<(), Error> {
        self.decoder.reset_instructions_cache()
    }
}

/// A fixed trace decoder that memorizes all traces after the initial decoding
pub struct MemoizedFixedTraceDecoder<D: InstDecoder> {
    inner: SimpleFixedTraceDecoder<D>,
    cache: HashMap<u64, FixedTrace>,
}

impl<D: InstDecoder> MemoizedFixedTraceDecoder<D> {
    pub fn new(decoder: D) -> Self {
        Self {
            inner: SimpleFixedTraceDecoder::new(decoder),
            cache: HashMap::default(),
        }
    }
}

impl<D: InstDecoder> TraceDecoder for MemoizedFixedTraceDecoder<D> {
    fn fixed_traces(&self) -> *const FixedTrace {
        self.inner.fixed_traces()
    }

    fn fixed_trace_size(&self) -> u64 {
        self.inner.fixed_trace_size()
    }

    fn prepare_traces(
        &mut self,
        machine: &mut DefaultMachine<Box<AsmCoreMachine>>,
    ) -> Result<(), Error> {
        let pc = *machine.pc();
        let slot = calculate_slot(pc);
        let trace = match self.cache.get(&pc) {
            Some(trace) => trace.clone(),
            None => {
                let (trace, _) = decode_fixed_trace(&mut self.inner.decoder, machine, None)?;
                self.cache.insert(pc, trace.clone());
                trace
            }
        };
        self.inner.traces[slot] = trace;
        Ok(())
    }

    fn reset(&mut self) -> Result<(), Error> {
        self.cache.clear();
        self.inner.reset()
    }
}

impl<D: InstDecoder> InstDecoder for MemoizedFixedTraceDecoder<D> {
    fn decode<M: Memory>(&mut self, memory: &mut M, pc: u64) -> Result<Instruction, Error> {
        self.inner.decode(memory, pc)
    }

    fn reset_instructions_cache(&mut self) -> Result<(), Error> {
        self.inner.reset_instructions_cache()
    }
}

