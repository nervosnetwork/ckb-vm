use super::instructions::is_jitable_instruction;
use crate::{Error, Instruction};
use fnv::FnvHashMap;
use std::collections::hash_map::Entry::Occupied;

pub trait Tracer {
    fn trace(&mut self, pc: usize) -> Result<(), Error>;
    fn should_jit(
        &mut self,
        pc: usize,
        block_length: usize,
        instructions: &[Instruction],
    ) -> Result<bool, Error>;
    fn clear(&mut self, pc: usize) -> Result<(), Error>;
}

// Default JIT tracer with a basic block execution counter
#[derive(Default)]
pub struct DefaultTracer {
    // Fast path
    last_trace_pc: usize,
    last_trace_measure: usize,
    measures: FnvHashMap<usize, usize>,
}

impl Tracer for DefaultTracer {
    fn trace(&mut self, pc: usize) -> Result<(), Error> {
        let entry = self.measures.entry(pc).or_insert(0);
        *entry += 1;
        self.last_trace_pc = pc;
        self.last_trace_measure = *entry;
        Ok(())
    }

    fn should_jit(
        &mut self,
        pc: usize,
        _block_length: usize,
        instructions: &[Instruction],
    ) -> Result<bool, Error> {
        let measure = if self.last_trace_pc == pc {
            self.last_trace_measure
        } else {
            *self.measures.get(&pc).unwrap_or(&0)
        };
        // NOTE: this is just made-up numbers now, for achieving maximum
        // performance, we should properly profile and benchmark those numbers.
        Ok(measure >= 10 && instructions.len() > 10)
    }

    fn clear(&mut self, pc: usize) -> Result<(), Error> {
        if let Occupied(mut e) = self.measures.entry(pc) {
            *e.get_mut() = 0;
        }
        if self.last_trace_pc == pc {
            self.last_trace_measure = 0;
        }
        Ok(())
    }
}

// This tracer works like QEMU's TCG, it will JIT every basic block. This
// can be quite handy in testing JIT infrastructure.
#[derive(Default)]
pub struct TcgTracer {}

impl Tracer for TcgTracer {
    fn trace(&mut self, _pc: usize) -> Result<(), Error> {
        Ok(())
    }

    fn should_jit(
        &mut self,
        _pc: usize,
        _block_length: usize,
        instructions: &[Instruction],
    ) -> Result<bool, Error> {
        Ok(instructions.iter().all(|i| is_jitable_instruction(*i)))
    }

    fn clear(&mut self, _pc: usize) -> Result<(), Error> {
        Ok(())
    }
}
