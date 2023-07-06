use crate::{
    ckb_vm_definitions::{
        asm::{calculate_slot, FixedTrace, TRACE_ITEM_LENGTH, TRACE_SIZE},
        instructions::{
            Instruction, InstructionOpcode, OP_CUSTOM_ASM_TRACE_JUMP, OP_CUSTOM_TRACE_END,
        },
    },
    decoder::InstDecoder,
    error::Error,
    instructions::{
        blank_instruction, extract_opcode, instruction_length, is_basic_block_end_instruction,
        is_slowpath_instruction,
    },
    machine::{
        asm::{ckb_vm_asm_labels, AsmCoreMachine},
        CoreMachine, DefaultMachine,
    },
    memory::Memory,
    ExecutionContext,
};
use std::alloc::{alloc, alloc_zeroed, Layout};
use std::collections::HashMap;

pub trait TraceDecoder: InstDecoder {
    fn fixed_traces(&self) -> *const FixedTrace;
    fn fixed_trace_size(&self) -> u64;
    fn prepare_traces<Ctx: ExecutionContext<Box<AsmCoreMachine>>>(
        &mut self,
        machine: &mut DefaultMachine<Box<AsmCoreMachine>, Ctx>,
    ) -> Result<(), Error>;
    fn reset(&mut self) -> Result<(), Error>;
}

pub fn label_from_fastpath_opcode(opcode: InstructionOpcode) -> u64 {
    debug_assert!(!is_slowpath_instruction(blank_instruction(opcode)));
    unsafe {
        u64::from(*(ckb_vm_asm_labels as *const u32).offset(opcode as u8 as isize))
            + (ckb_vm_asm_labels as *const u32 as u64)
    }
}

pub fn decode_fixed_trace<D: InstDecoder, Ctx: ExecutionContext<Box<AsmCoreMachine>>>(
    decoder: &mut D,
    machine: &mut DefaultMachine<Box<AsmCoreMachine>, Ctx>,
    maximum_insts: Option<usize>,
) -> Result<(FixedTrace, usize), Error> {
    let pc = *machine.pc();

    let mut trace = FixedTrace::default();
    let mut current_pc = pc;
    let mut i = 0;

    let size = match maximum_insts {
        Some(items) => std::cmp::min(items, TRACE_ITEM_LENGTH),
        None => TRACE_ITEM_LENGTH,
    };
    while i < size {
        let instruction = decoder.decode(machine.memory_mut(), current_pc)?;
        let end_instruction = is_basic_block_end_instruction(instruction);
        current_pc += u64::from(instruction_length(instruction));
        trace.cycles += machine.context().instruction_cycles(instruction);
        let opcode = extract_opcode(instruction);
        // Here we are calculating the absolute address used in direct threading
        // from label offsets.
        trace.set_thread(i, instruction, label_from_fastpath_opcode(opcode));
        i += 1;
        if end_instruction {
            break;
        }
    }
    trace.set_thread(
        i,
        blank_instruction(OP_CUSTOM_TRACE_END),
        label_from_fastpath_opcode(OP_CUSTOM_TRACE_END),
    );
    i += 1;
    trace.address = pc;
    trace.length = (current_pc - pc) as u32;
    Ok((trace, i))
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

    pub fn clear_traces(&mut self) {
        for i in 0..TRACE_SIZE {
            self.traces[i] = FixedTrace::default();
        }
    }
}

impl<D: InstDecoder> TraceDecoder for SimpleFixedTraceDecoder<D> {
    fn fixed_traces(&self) -> *const FixedTrace {
        self.traces.as_ptr()
    }

    fn fixed_trace_size(&self) -> u64 {
        TRACE_SIZE as u64
    }

    fn prepare_traces<Ctx: ExecutionContext<Box<AsmCoreMachine>>>(
        &mut self,
        machine: &mut DefaultMachine<Box<AsmCoreMachine>, Ctx>,
    ) -> Result<(), Error> {
        let (trace, _) = decode_fixed_trace(&mut self.decoder, machine, None)?;
        let slot = calculate_slot(*machine.pc());
        self.traces[slot] = trace;
        Ok(())
    }

    fn reset(&mut self) -> Result<(), Error> {
        self.clear_traces();
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

    pub fn clear_traces(&mut self) {
        self.inner.clear_traces();
    }
}

impl<D: InstDecoder> TraceDecoder for MemoizedFixedTraceDecoder<D> {
    fn fixed_traces(&self) -> *const FixedTrace {
        self.inner.fixed_traces()
    }

    fn fixed_trace_size(&self) -> u64 {
        self.inner.fixed_trace_size()
    }

    fn prepare_traces<Ctx: ExecutionContext<Box<AsmCoreMachine>>>(
        &mut self,
        machine: &mut DefaultMachine<Box<AsmCoreMachine>, Ctx>,
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

/// This is similar to FixedTrace, except that it uses a special pattern
/// named [flexible array member](https://en.wikipedia.org/wiki/Flexible_array_member).
/// The individual fields in this data structure, albeit similar to FixedTrace,
/// are marked as privates so one cannot directly instantiate a struct of
/// DynamicTrace. Instead, all constructions of DynamicTrace are done via
/// DynamicTraceBuilder, which builds `Box<DynamicTrace>`. This is due to the
/// reasons that we might have a variable number of `threads` allocated at the
/// end of this data structure. So on the surface, it might appear that a
/// struct of DynamicTrace is 24 bytes. In reality it is bigger than this:
/// a struct of DynamicTrace containing 30 opcodes could be
/// 24 + 30 * 16 + 16 = 520 bytes long(the final 16 bytes are for OP_CUSTOM_TRACE_END).
/// Using `Box<DynamicTrace>`, together with `DynamicTraceBuilder`, helps us to
/// abstract this details away when we are constructing the types. The underlying
/// assembly code cannot tell the difference between DynamicTrace and FixedTrace.
#[repr(C)]
pub struct DynamicTrace {
    address: u64,
    length: u32,
    cycles: u64,
}

pub struct DynamicTraceBuilder {
    start_address: u64,
    length: u32,
    cycles: u64,
    insts: Vec<(Instruction, u64)>,
}

impl DynamicTraceBuilder {
    pub fn new(start_address: u64) -> Self {
        Self {
            start_address,
            length: 0,
            cycles: 0,
            insts: vec![],
        }
    }

    pub fn next_pc(&self) -> u64 {
        self.start_address + self.length as u64
    }

    pub fn push(&mut self, inst: Instruction, cycles: u64) {
        let opcode = extract_opcode(inst);
        // Here we are calculating the absolute address used in direct threading
        // from label offsets.
        let label = label_from_fastpath_opcode(opcode);
        self.length += u32::from(instruction_length(inst));
        self.cycles += cycles;
        self.insts.push((inst, label));
    }

    pub fn build(mut self) -> Box<DynamicTrace> {
        self.insts.push((
            blank_instruction(OP_CUSTOM_TRACE_END),
            label_from_fastpath_opcode(OP_CUSTOM_TRACE_END),
        ));
        let fixed_size = std::mem::size_of::<DynamicTrace>();
        let total_size = fixed_size + self.insts.len() * 16;
        let p = unsafe {
            let layout = Layout::array::<u8>(total_size).unwrap();
            alloc(layout)
        };
        let threads = unsafe { p.add(fixed_size) } as *mut u64;
        for (i, (inst, label)) in self.insts.iter().enumerate() {
            unsafe {
                threads.offset(i as isize * 2).write(*label);
                threads.offset(i as isize * 2 + 1).write(*inst);
            }
        }
        let mut trace = unsafe { Box::from_raw(p as *mut DynamicTrace) };
        trace.address = self.start_address;
        trace.length = self.length;
        trace.cycles = self.cycles;
        trace
    }
}

/// A memoized trace decoder that also generates DynamicTrace for longer
/// sequential code.
pub struct MemoizedDynamicTraceDecoder<D: InstDecoder> {
    inner: SimpleFixedTraceDecoder<D>,
    fixed_cache: HashMap<u64, FixedTrace>,
    dynamic_cache: HashMap<u64, Box<DynamicTrace>>,
}

impl<D: InstDecoder> MemoizedDynamicTraceDecoder<D> {
    pub fn new(decoder: D) -> Self {
        Self {
            inner: SimpleFixedTraceDecoder::new(decoder),
            fixed_cache: HashMap::default(),
            dynamic_cache: HashMap::default(),
        }
    }

    pub fn clear_traces(&mut self) {
        self.inner.clear_traces();
    }

    fn find_or_build_dynamic_trace<Ctx: ExecutionContext<Box<AsmCoreMachine>>>(
        &mut self,
        pc: u64,
        machine: &mut DefaultMachine<Box<AsmCoreMachine>, Ctx>,
    ) -> Result<*const DynamicTrace, Error> {
        if let Some(trace) = self.dynamic_cache.get(&pc) {
            return Ok(trace.as_ref() as *const DynamicTrace);
        }
        let mut builder = DynamicTraceBuilder::new(pc);
        loop {
            let instruction = self.decode(machine.memory_mut(), builder.next_pc())?;
            let end_instruction = is_basic_block_end_instruction(instruction);
            let cycles = machine.context().instruction_cycles(instruction);
            builder.push(instruction, cycles);
            if end_instruction {
                break;
            }
        }
        let dynamic_trace = builder.build();
        let p = dynamic_trace.as_ref() as *const DynamicTrace;
        self.dynamic_cache.insert(pc, dynamic_trace);
        Ok(p)
    }
}

impl<D: InstDecoder> TraceDecoder for MemoizedDynamicTraceDecoder<D> {
    fn fixed_traces(&self) -> *const FixedTrace {
        self.inner.fixed_traces()
    }

    fn fixed_trace_size(&self) -> u64 {
        self.inner.fixed_trace_size()
    }

    fn prepare_traces<Ctx: ExecutionContext<Box<AsmCoreMachine>>>(
        &mut self,
        machine: &mut DefaultMachine<Box<AsmCoreMachine>, Ctx>,
    ) -> Result<(), Error> {
        let pc = *machine.pc();
        let slot = calculate_slot(pc);
        let trace = match self.fixed_cache.get(&pc) {
            Some(trace) => trace.clone(),
            None => {
                let (mut trace, count) =
                    decode_fixed_trace(&mut self.inner.decoder, machine, None)?;
                if let Some((inst, _)) = trace.thread(count.wrapping_sub(2)) {
                    if !is_basic_block_end_instruction(inst) {
                        // Decoded trace is not yet a full basic block, there
                        // are still sequential code left. We can build a dynamic
                        // trace here covering the remaining of the sequential code
                        // to speed up processing
                        let dynamic_trace =
                            self.find_or_build_dynamic_trace(pc + trace.length as u64, machine)?;
                        trace.set_thread(
                            count.wrapping_sub(1),
                            dynamic_trace as u64,
                            label_from_fastpath_opcode(OP_CUSTOM_ASM_TRACE_JUMP),
                        );
                    }
                }
                self.fixed_cache.insert(pc, trace.clone());
                trace
            }
        };
        self.inner.traces[slot] = trace;
        Ok(())
    }

    fn reset(&mut self) -> Result<(), Error> {
        self.fixed_cache.clear();
        self.dynamic_cache.clear();
        self.inner.reset()
    }
}

impl<D: InstDecoder> InstDecoder for MemoizedDynamicTraceDecoder<D> {
    fn decode<M: Memory>(&mut self, memory: &mut M, pc: u64) -> Result<Instruction, Error> {
        self.inner.decode(memory, pc)
    }

    fn reset_instructions_cache(&mut self) -> Result<(), Error> {
        self.inner.reset_instructions_cache()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{size_of, zeroed};

    #[test]
    fn test_dynamic_trace_has_the_same_layout_as_fixed_trace() {
        let f: FixedTrace = unsafe { zeroed() };
        let f_address = &f as *const FixedTrace as usize;

        let d: DynamicTrace = unsafe { zeroed() };
        let d_address = &d as *const DynamicTrace as usize;

        assert_eq!(
            (&f.address as *const _ as usize) - f_address,
            (&d.address as *const _ as usize) - d_address,
        );
        assert_eq!(
            (&f.length as *const _ as usize) - f_address,
            (&d.length as *const _ as usize) - d_address,
        );
        assert_eq!(
            (&f.cycles as *const _ as usize) - f_address,
            (&d.cycles as *const _ as usize) - d_address,
        );
        assert_eq!(
            (&f._threads as *const _ as usize) - f_address,
            size_of::<DynamicTrace>(),
        );
    }
}
