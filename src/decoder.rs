use super::instructions::{i, m, rvc, Instruction, InstructionFactory, Register};
use super::memory::Memory;
use super::Error;

#[derive(Default)]
pub struct Decoder {
    version: u32,
    factories: Vec<InstructionFactory>,
}

impl Decoder {
    pub fn new(version: u32) -> Decoder {
        Decoder {
            version,
            factories: vec![],
        }
    }

    pub fn add_instruction_factory(&mut self, factory: InstructionFactory) {
        self.factories.push(factory);
    }

    // This method is used to decode instruction raw bits from memory pointed
    // by current PC. Right now we support 32-bit instructions and RVC compressed
    // instructions. In future version we might add support for longer instructions.
    //
    // This decode method actually leverages a trick from little endian encoding:
    // the format for a full 32 bit RISC-V instruction is as follows:
    //
    // WWWWWWWWZZZZZZZZYYYYYYYYXXXXXX11
    //
    // While the format for a 16 bit RVC RIST-V instruction is one of the following 3:
    //
    // YYYYYYYYXXXXXX00
    // YYYYYYYYXXXXXX01
    // YYYYYYYYXXXXXX10
    //
    // Here X, Y, Z and W stands for arbitrary bits.
    // However the above is the representation in a 16-bit or 32-bit integer, since
    // we are using little endian, in memory it's actually in following reversed order:
    //
    // XXXXXX11 YYYYYYYY ZZZZZZZZ WWWWWWWW
    // XXXXXX00 YYYYYYYY
    // XXXXXX01 YYYYYYYY
    // XXXXXX10 YYYYYYYY
    //
    // One observation here, is the first byte in memory is always the least
    // significant byte whether we load a 32-bit or 16-bit integer.
    // So when we are decoding an instruction, we can first load 2 bytes forming
    // a 16-bit integer, then we check the 2 least significant bits, if the 2 bitss
    // are 0b11, we know this is a 32-bit instruction, we should load another 2 bytes
    // from memory and concat the 2 16-bit integers into a full 32-bit integers.
    // Otherwise, we know we are loading a RVC integer, and we are done here.
    // Also, due to RISC-V encoding behavior, it's totally okay when we cast a 16-bit
    // RVC instruction into a 32-bit instruction, the meaning of the instruction stays
    // unchanged in the cast conversion.
    fn decode_bits<R: Register, M: Memory<R>>(
        &self,
        memory: &mut M,
        pc: u64,
    ) -> Result<u32, Error> {
        let mut instruction_bits = u32::from(memory.execute_load16(pc)?);
        if instruction_bits & 0x3 == 0x3 {
            instruction_bits |= u32::from(memory.execute_load16(pc + 2)?) << 16;
        }
        Ok(instruction_bits)
    }

    pub fn decode<R: Register, M: Memory<R>>(
        &self,
        memory: &mut M,
        pc: u64,
    ) -> Result<Instruction, Error> {
        let instruction_bits = self.decode_bits(memory, pc)?;
        for factory in &self.factories {
            if let Some(instruction) = factory(instruction_bits, self.version) {
                return Ok(instruction);
            }
        }
        Err(Error::InvalidInstruction(instruction_bits))
    }
}

pub fn build_imac_decoder<R: Register>(version: u32) -> Decoder {
    let mut decoder = Decoder::new(version);
    decoder.add_instruction_factory(rvc::factory::<R>);
    decoder.add_instruction_factory(i::factory::<R>);
    decoder.add_instruction_factory(m::factory::<R>);
    decoder
}
