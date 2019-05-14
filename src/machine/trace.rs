use super::{
    super::{
        decoder::build_imac_decoder,
        instructions::{
            execute, instruction_length, is_basic_block_end_instruction, Instruction, Register,
        },
        memory::{wxorx::WXorXMemory, Memory},
        Error,
    },
    CoreMachine, DefaultMachine, Machine, SupportMachine,
};
use bytes::Bytes;

// The number of trace items to keep
const TRACE_SIZE: usize = 8192;
// Quick bit-mask to truncate a value in trace size range
const TRACE_MASK: usize = (TRACE_SIZE - 1);
// The maximum number of instructions to cache in a trace item
const TRACE_ITEM_LENGTH: usize = 16;
// Shifts to truncate a value so 2 traces has the minimal chance of sharing code.
const TRACE_ADDRESS_SHIFTS: usize = 5;

#[derive(Default)]
struct Trace {
    address: usize,
    length: usize,
    instruction_count: u8,
    instructions: [Instruction; TRACE_ITEM_LENGTH],
}

#[inline(always)]
fn calculate_slot(addr: usize) -> usize {
    (addr >> TRACE_ADDRESS_SHIFTS) & TRACE_MASK
}

pub struct TraceMachine<'a, Inner> {
    pub machine: DefaultMachine<'a, Inner>,

    traces: Vec<Trace>,
}

impl<R: Register, M: Memory<R>, Inner: SupportMachine<REG = R, MEM = WXorXMemory<R, M>>> CoreMachine
    for TraceMachine<'_, Inner>
{
    type REG = <Inner as CoreMachine>::REG;
    type MEM = <Inner as CoreMachine>::MEM;

    fn pc(&self) -> &Self::REG {
        &self.machine.pc()
    }

    fn set_pc(&mut self, next_pc: Self::REG) {
        self.machine.set_pc(next_pc)
    }

    fn memory(&self) -> &Self::MEM {
        self.machine.memory()
    }

    fn memory_mut(&mut self) -> &mut Self::MEM {
        self.machine.memory_mut()
    }

    fn registers(&self) -> &[Self::REG] {
        self.machine.registers()
    }

    fn set_register(&mut self, idx: usize, value: Self::REG) {
        self.machine.set_register(idx, value)
    }
}

impl<R: Register, M: Memory<R>, Inner: SupportMachine<REG = R, MEM = WXorXMemory<R, M>>> Machine
    for TraceMachine<'_, Inner>
{
    fn ecall(&mut self) -> Result<(), Error> {
        self.machine.ecall()
    }

    fn ebreak(&mut self) -> Result<(), Error> {
        self.machine.ebreak()
    }
}

impl<'a, R: Register, M: Memory<R>, Inner: SupportMachine<REG = R, MEM = WXorXMemory<R, M>>>
    TraceMachine<'a, Inner>
{
    pub fn new(machine: DefaultMachine<'a, Inner>) -> Self {
        Self {
            machine,
            traces: vec![],
        }
    }

    pub fn load_program(&mut self, program: &Bytes, args: &[Bytes]) -> Result<(), Error> {
        self.machine.load_program(program, args)?;
        Ok(())
    }

    pub fn run(&mut self) -> Result<i8, Error> {
        let decoder = build_imac_decoder::<Inner::REG>();
        self.machine.set_running(true);
        // For current trace size this is acceptable, however we might want
        // to tweak the code here if we choose to use a larger trace size or
        // larger trace item length.
        self.traces.resize_with(TRACE_SIZE, Trace::default);
        while self.machine.running() {
            let pc = self.machine.pc().to_usize();
            let slot = calculate_slot(pc);
            if pc != self.traces[slot].address || self.traces[slot].instruction_count == 0 {
                self.traces[slot] = Trace::default();
                let mut current_pc = pc;
                let mut i = 0;
                while i < TRACE_ITEM_LENGTH {
                    let instruction = decoder.decode(self.machine.memory_mut(), current_pc)?;
                    let end_instruction = is_basic_block_end_instruction(instruction);
                    current_pc += instruction_length(instruction);
                    self.traces[slot].instructions[i] = instruction;
                    i += 1;
                    if end_instruction {
                        break;
                    }
                }
                self.traces[slot].address = pc;
                self.traces[slot].length = current_pc - pc;
                self.traces[slot].instruction_count = i as u8;
            }
            for i in 0..self.traces[slot].instruction_count {
                let i = self.traces[slot].instructions[i as usize];
                execute(i, self)?;
                let cycles = self
                    .machine
                    .instruction_cycle_func()
                    .as_ref()
                    .map(|f| f(i))
                    .unwrap_or(0);
                self.machine.add_cycles(cycles)?;
            }
        }
        Ok(self.machine.exit_code())
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::bits::power_of_2;
    use super::*;

    #[test]
    fn test_trace_constant_rules() {
        assert!(power_of_2(TRACE_SIZE));
        assert_eq!(TRACE_MASK, TRACE_SIZE - 1);
        assert!(power_of_2(TRACE_ITEM_LENGTH));
        assert!(TRACE_ITEM_LENGTH <= 255);
    }
}
