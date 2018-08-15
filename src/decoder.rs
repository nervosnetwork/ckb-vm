use super::instructions::{rv32i, Instruction, InstructionFactory};
use super::machine::Machine;
use super::Error;

pub struct Decoder {
    factories: Vec<InstructionFactory>,
}

impl Decoder {
    pub fn new() -> Decoder {
        Decoder {
            factories: Vec::new(),
        }
    }

    pub fn add_instruction_factory(&mut self, factory: InstructionFactory) {
        self.factories.push(factory);
    }

    pub fn decode(&self, machine: &Machine) -> Result<Instruction, Error> {
        let mut instruction: u32 = u32::from(machine.memory.load16(machine.pc as usize)?);
        if instruction & 0x3 == 0x3 {
            instruction |= u32::from(machine.memory.load16(machine.pc as usize + 2)?) << 16;
        }
        for factory in &self.factories {
            if let Some(instruction) = factory(instruction) {
                return Ok(instruction);
            }
        }
        Err(Error::InvalidInstruction(instruction))
    }
}

pub fn build_rv32imac_decoder() -> Decoder {
    let mut decoder = Decoder::new();
    decoder.add_instruction_factory(rv32i::factory);
    decoder
}
