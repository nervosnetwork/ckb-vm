use super::{
    super::{
        decoder::build_decoder,
        instructions::{
            execute, instruction_length, is_basic_block_end_instruction, Instruction, Register,
        },
        Error,
    },
    CoreMachine, DefaultMachine, Machine, SupportMachine,
};
use bytes::Bytes;

// The number of trace items to keep
const TRACE_SIZE: usize = 8192;
// Quick bit-mask to truncate a value in trace size range
const TRACE_MASK: usize = TRACE_SIZE - 1;
// The maximum number of instructions to cache in a trace item
const TRACE_ITEM_LENGTH: usize = 16;
// Shifts to truncate a value so 2 traces has the minimal chance of sharing code.
const TRACE_ADDRESS_SHIFTS: usize = 2;

#[derive(Default)]
struct Trace {
    address: u64,
    length: usize,
    instruction_count: u8,
    instructions: [Instruction; TRACE_ITEM_LENGTH],
}

#[inline(always)]
fn calculate_slot(addr: u64) -> usize {
    (addr as usize >> TRACE_ADDRESS_SHIFTS) & TRACE_MASK
}

pub struct TraceMachine<'a, Inner> {
    pub machine: DefaultMachine<'a, Inner>,

    traces: Vec<Trace>,
}

impl<Inner: SupportMachine> CoreMachine for TraceMachine<'_, Inner> {
    type REG = <Inner as CoreMachine>::REG;
    type MEM = <Inner as CoreMachine>::MEM;

    fn pc(&self) -> &Self::REG {
        self.machine.pc()
    }

    fn update_pc(&mut self, pc: Self::REG) {
        self.machine.update_pc(pc);
    }

    fn commit_pc(&mut self) {
        self.machine.commit_pc();
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

    fn isa(&self) -> u8 {
        self.machine.isa()
    }

    fn version(&self) -> u32 {
        self.machine.version()
    }
}

impl<Inner: SupportMachine> Machine for TraceMachine<'_, Inner> {
    fn ecall(&mut self) -> Result<(), Error> {
        self.machine.ecall()
    }

    fn ebreak(&mut self) -> Result<(), Error> {
        self.machine.ebreak()
    }
}

impl<'a, Inner: SupportMachine> TraceMachine<'a, Inner> {
    pub fn new(machine: DefaultMachine<'a, Inner>) -> Self {
        Self {
            machine,
            traces: vec![],
        }
    }

    pub fn load_program(&mut self, program: &Bytes, args: &[Bytes]) -> Result<u64, Error> {
        self.machine.load_program(program, args)
    }

    pub fn run(&mut self) -> Result<i8, Error> {
        let mut decoder = build_decoder::<Inner::REG>(self.isa(), self.version());
        self.machine.set_running(true);
        // For current trace size this is acceptable, however we might want
        // to tweak the code here if we choose to use a larger trace size or
        // larger trace item length.
        self.traces.resize_with(TRACE_SIZE, Trace::default);
        while self.machine.running() {
            if self.machine.reset_signal() {
                decoder.reset_instructions_cache();
                for i in self.traces.iter_mut() {
                    *i = Trace::default()
                }
            }
            let pc = self.machine.pc().to_u64();
            let slot = calculate_slot(pc);
            if pc != self.traces[slot].address || self.traces[slot].instruction_count == 0 {
                self.traces[slot] = Trace::default();
                let mut current_pc = pc;
                let mut i = 0;
                while i < TRACE_ITEM_LENGTH {
                    let instruction = decoder.decode(self.machine.memory_mut(), current_pc)?;
                    let end_instruction = is_basic_block_end_instruction(instruction);
                    current_pc += u64::from(instruction_length(instruction));
                    self.traces[slot].instructions[i] = instruction;
                    i += 1;
                    if end_instruction {
                        break;
                    }
                }
                self.traces[slot].address = pc;
                self.traces[slot].length = (current_pc - pc) as usize;
                self.traces[slot].instruction_count = i as u8;
            }
            for i in 0..self.traces[slot].instruction_count {
                let i = self.traces[slot].instructions[i as usize];
                let cycles = self
                    .machine
                    .instruction_cycle_func()
                    .as_ref()
                    .map(|f| f(i))
                    .unwrap_or(0);
                self.machine.add_cycles(cycles)?;
                execute(i, self)?;
            }
        }
        Ok(self.machine.exit_code())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_constant_rules() {
        assert!(TRACE_SIZE.is_power_of_two());
        assert_eq!(TRACE_MASK, TRACE_SIZE - 1);
        assert!(TRACE_ITEM_LENGTH.is_power_of_two());
        assert!(TRACE_ITEM_LENGTH <= 255);
    }
}
