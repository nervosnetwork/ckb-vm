mod emitter;

use super::super::{
    decoder::build_imac_decoder,
    instructions::{
        ast::Value, execute, instruction_length, is_basic_block_end_instruction, Instruction,
    },
    CoreMachine, DefaultCoreMachine, Error, FlatMemory, InstructionCycleFunc, Machine, Memory,
    Register, SupportMachine, RISCV_MAX_MEMORY,
};
use bytes::Bytes;
use emitter::Emitter;
use goblin::elf::{section_header::SHF_EXECINSTR, Elf};
use memmap::{Mmap, MmapMut};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

const MAXIMUM_INSTRUCTIONS_PER_BLOCK: usize = 1024;
const MAXIMUM_LABELS: usize = 65535;
const MAXIMUM_SECTIONS: usize = 1024;
const MAXIMUM_DUMMY_SECTIONS: usize = 64;

const ADDRESS_WRITE_ONLY_FLAG: u64 = 0x8000_0000_0000_0000;
const ADDRESS_LABEL_FLAG: u64 = 0x4000_0000_0000_0000;
const MAXIMUM_ENCODED_ADDRESS: u64 = 0x8000_0000;

#[derive(Debug, Clone)]
pub enum Write {
    Memory {
        address: Value,
        size: u8,
        value: Value,
    },
    Register {
        index: usize,
        value: Value,
    },
    Pc {
        value: Value,
    },
    Ecall,
    Ebreak,
}

fn init_registers() -> [Value; 32] {
    [
        Value::Imm(0),
        Value::Register(1),
        Value::Register(2),
        Value::Register(3),
        Value::Register(4),
        Value::Register(5),
        Value::Register(6),
        Value::Register(7),
        Value::Register(8),
        Value::Register(9),
        Value::Register(10),
        Value::Register(11),
        Value::Register(12),
        Value::Register(13),
        Value::Register(14),
        Value::Register(15),
        Value::Register(16),
        Value::Register(17),
        Value::Register(18),
        Value::Register(19),
        Value::Register(20),
        Value::Register(21),
        Value::Register(22),
        Value::Register(23),
        Value::Register(24),
        Value::Register(25),
        Value::Register(26),
        Value::Register(27),
        Value::Register(28),
        Value::Register(29),
        Value::Register(30),
        Value::Register(31),
    ]
}

struct LabelGatheringMachine {
    registers: [Value; 32],
    pc: Value,
    labels_to_test: Vec<u64>,
    version: u32,

    // A memory segment which contains code loaded from ELF
    memory: FlatMemory<u64>,
    labels: HashSet<u64>,
    sections: Vec<(u64, u64)>,
    dummy_sections: HashMap<u64, u64>,
}

impl LabelGatheringMachine {
    pub fn load(program: &Bytes, version: u32) -> Result<Self, Error> {
        let elf = Elf::parse(program).map_err(|_e| Error::ParseError)?;
        if elf.section_headers.len() > MAXIMUM_SECTIONS {
            return Err(Error::LimitReached);
        }
        let mut sections: Vec<(u64, u64)> = elf
            .section_headers
            .iter()
            .filter_map(|section_header| {
                if section_header.sh_flags & u64::from(SHF_EXECINSTR) != 0 {
                    Some((
                        section_header.sh_addr,
                        section_header.sh_addr.wrapping_add(section_header.sh_size),
                    ))
                } else {
                    None
                }
            })
            .rev()
            .collect();
        // Test there's no empty section
        if sections.iter().any(|(s, e)| s >= e) {
            return Err(Error::OutOfBound);
        }
        // Test no section overlaps with one another. We first sort section
        // list by start, then we test if each end is equal or less than
        // the next start.
        sections.sort_by_key(|section| section.0);
        if sections.windows(2).any(|w| w[0].1 > w[1].0) {
            return Err(Error::OutOfBound);
        }
        // DefaultCoreMachine is only used here for loading ELF binaries
        // into memory.
        let mut inner = DefaultCoreMachine::default();
        inner.load_elf(&program, false)?;

        Ok(Self {
            version,
            registers: init_registers(),
            pc: Value::from_u64(0),
            labels: HashSet::default(),
            labels_to_test: Vec::new(),
            memory: inner.take_memory(),
            sections,
            dummy_sections: HashMap::default(),
        })
    }

    fn read_pc(&self) -> Result<u64, Error> {
        match &self.pc {
            Value::Imm(pc) => Ok(*pc),
            _ => Err(Error::Unexpected),
        }
    }

    pub fn gather(&mut self) -> Result<(), Error> {
        let decoder = build_imac_decoder::<u64>(self.version);
        for i in 0..self.sections.len() {
            let (section_start, section_end) = self.sections[i];
            self.pc = Value::from_u64(section_start);
            let mut start_of_basic_block = true;
            while self.read_pc()? < section_end {
                let pc = self.read_pc()?;
                match decoder.decode(&mut self.memory, pc) {
                    Ok(instruction) => {
                        if start_of_basic_block {
                            self.labels.insert(pc);
                        }
                        start_of_basic_block = is_basic_block_end_instruction(instruction);
                        let next_pc = pc + u64::from(instruction_length(instruction));
                        execute(instruction, self)?;
                        for label in self.labels_to_test.drain(..) {
                            if label != next_pc && label < section_end && label >= section_start {
                                self.labels.insert(label);
                            }
                        }
                        if self.labels.len() > MAXIMUM_LABELS {
                            return Err(Error::LimitReached);
                        }
                        self.pc = Value::from_u64(next_pc);
                    }
                    Err(Error::InvalidInstruction(i)) if i == 0 => {
                        // Due to alignment or other reasons, the code might
                        // certain invalid instructions in the executable
                        // sections, for a normal VM instance that's executing
                        // instructions, this is usually fine since the invalid
                        // regions might never be touched. But for an AOT
                        // solution, this won't work since we have to
                        // pre-process the whole text section without knowing
                        // which part would be touched. The solution here, is
                        // to skip sections containing invalid instructions,
                        // keep a note of them, and ignore them in the code
                        // generation phase. One caveat here,
                        // is that a malicious program might choose to include
                        // invalid instructions everywhere, hence creating
                        // numerous sections hoping to bring the program down.
                        // To tackle that, we will add an upper bound on the
                        // number of dummy sections allowed here. That
                        // allow us to signal correct error and revert back
                        // to assembly VM for those quirky programs.
                        if !start_of_basic_block {
                            return Err(Error::OutOfBound);
                        }
                        let mut dummy_end = pc + 2;
                        while dummy_end < section_end && self.memory.execute_load16(dummy_end)? == 0
                        {
                            dummy_end += 2;
                        }
                        // We checked no sections are overlapped, so dummy
                        // sections won't overlap with each other as well.
                        self.dummy_sections.insert(pc, dummy_end);
                        if self.dummy_sections.len() > MAXIMUM_DUMMY_SECTIONS {
                            return Err(Error::LimitReached);
                        }
                        self.pc = Value::from_u64(dummy_end);
                    }
                    Err(e) => return Err(e),
                }
            }
            // A section must end a basic block, otherwise we would run into
            // out of bound error;
            if !start_of_basic_block {
                return Err(Error::OutOfBound);
            }
            debug_assert!(!self.labels.contains(&section_end));
        }
        // Remove all labels pointed to dummy sections, since we won't generate
        // code for dummy sections
        for (dummy_start, dummy_end) in &self.dummy_sections {
            self.labels
                .retain(|label| *label < *dummy_start || *label >= *dummy_end);
        }
        Ok(())
    }
}

impl CoreMachine for LabelGatheringMachine {
    type REG = Value;
    type MEM = Self;

    fn pc(&self) -> &Value {
        &self.pc
    }

    fn set_pc(&mut self, next_pc: Value) {
        match next_pc {
            Value::Imm(pc) => self.labels_to_test.push(pc),
            Value::Cond(_, t, f) => {
                if let (Value::Imm(t), Value::Imm(f)) = (&*t, &*f) {
                    self.labels_to_test.push(*t);
                    self.labels_to_test.push(*f);
                }
            }
            _ => (),
        }
    }

    fn memory(&self) -> &Self {
        &self
    }

    fn memory_mut(&mut self) -> &mut Self {
        self
    }

    fn registers(&self) -> &[Value] {
        &self.registers
    }

    fn set_register(&mut self, _idx: usize, _value: Value) {
        // This is a NOP since we only care about PC writes
    }

    fn version(&self) -> u32 {
        self.version
    }
}

impl Machine for LabelGatheringMachine {
    fn ecall(&mut self) -> Result<(), Error> {
        // This is a basic block end instruction, main loop will record the
        // address after this instruction.
        Ok(())
    }

    fn ebreak(&mut self) -> Result<(), Error> {
        // This is a basic block end instruction, main loop will record the
        // address after this instruction.
        Ok(())
    }
}

impl Memory<Value> for LabelGatheringMachine {
    fn init_pages(
        &mut self,
        _addr: u64,
        _size: u64,
        _flags: u8,
        _source: Option<Bytes>,
        _offset_from_addr: u64,
    ) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn fetch_flag(&mut self, _page: u64) -> Result<u8, Error> {
        Err(Error::Unimplemented)
    }

    fn store_byte(&mut self, _addr: u64, _size: u64, _value: u8) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn store_bytes(&mut self, _addr: u64, _value: &[u8]) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn execute_load16(&mut self, _addr: u64) -> Result<u16, Error> {
        Err(Error::Unimplemented)
    }

    fn load8(&mut self, addr: &Value) -> Result<Value, Error> {
        Ok(Value::Load(Rc::new(addr.clone()), 1))
    }

    fn load16(&mut self, addr: &Value) -> Result<Value, Error> {
        Ok(Value::Load(Rc::new(addr.clone()), 2))
    }

    fn load32(&mut self, addr: &Value) -> Result<Value, Error> {
        Ok(Value::Load(Rc::new(addr.clone()), 4))
    }

    fn load64(&mut self, addr: &Value) -> Result<Value, Error> {
        Ok(Value::Load(Rc::new(addr.clone()), 8))
    }

    fn store8(&mut self, _addr: &Value, _value: &Value) -> Result<(), Error> {
        Ok(())
    }

    fn store16(&mut self, _addr: &Value, _value: &Value) -> Result<(), Error> {
        Ok(())
    }

    fn store32(&mut self, _addr: &Value, _value: &Value) -> Result<(), Error> {
        Ok(())
    }

    fn store64(&mut self, _addr: &Value, _value: &Value) -> Result<(), Error> {
        Ok(())
    }
}

pub struct AotCode {
    pub code: Mmap,
    /// Labels that map RISC-V addresses to offsets into the compiled x86_64
    /// assembly code. This can be used as entrypoints to start executing in
    /// AOT code.
    pub labels: HashMap<u64, u32>,
}

impl AotCode {
    pub fn base_address(&self) -> u64 {
        self.code.as_ptr() as u64
    }
}

pub struct AotCompilingMachine {
    version: u32,
    registers: [Value; 32],
    pc: Value,
    emitter: Emitter,
    memory: FlatMemory<u64>,
    sections: Vec<(u64, u64)>,
    dummy_sections: HashMap<u64, u64>,
    addresses_to_labels: HashMap<u64, u32>,
    writes: Vec<Write>,
    next_pc_write: Option<Value>,
    instruction_cycle_func: Option<Box<InstructionCycleFunc>>,
}

impl AotCompilingMachine {
    pub fn load(
        program: &Bytes,
        instruction_cycle_func: Option<Box<InstructionCycleFunc>>,
        version: u32,
    ) -> Result<Self, Error> {
        // First we need to gather labels
        let mut label_gathering_machine = LabelGatheringMachine::load(&program, version)?;
        label_gathering_machine.gather()?;

        let mut labels: Vec<u64> = label_gathering_machine.labels.iter().cloned().collect();
        labels.sort_unstable();
        let addresses_to_labels = labels
            .iter()
            .enumerate()
            .map(|(i, address)| (*address, i as u32))
            .collect();

        Ok(Self {
            version,
            registers: init_registers(),
            pc: Value::from_u64(0),
            emitter: Emitter::new(labels.len())?,
            addresses_to_labels,
            memory: label_gathering_machine.memory,
            sections: label_gathering_machine.sections,
            dummy_sections: label_gathering_machine.dummy_sections,
            writes: vec![],
            next_pc_write: None,
            instruction_cycle_func,
        })
    }

    fn read_pc(&self) -> Result<u64, Error> {
        match &self.pc {
            Value::Imm(pc) => Ok(*pc),
            _ => Err(Error::Unexpected),
        }
    }

    fn emit_block(&mut self, instructions: &[Instruction]) -> Result<(), Error> {
        let mut cycles = 0;
        for i in instructions {
            let pc = self.read_pc()?;
            let length = instruction_length(*i);
            cycles += self
                .instruction_cycle_func
                .as_ref()
                .map(|f| f(*i))
                .unwrap_or(0);
            execute(*i, self)?;
            self.pc = Value::from_u64(pc + u64::from(length));
        }
        let pc = self.read_pc()?;
        self.emitter.emit_add_cycles(cycles)?;
        // Emit succeeding PC write only
        if pc >= RISCV_MAX_MEMORY as u64 {
            return Err(Error::OutOfBound);
        }
        self.emitter.emit(&Write::Pc {
            value: Value::Imm(pc | ADDRESS_WRITE_ONLY_FLAG),
        })?;
        for write in &self.writes {
            self.emitter.emit(&write)?;
        }
        self.writes.clear();
        if let Some(value) = self.next_pc_write.take() {
            self.emitter.emit(&Write::Pc {
                value: self.optimize_pc_value(value)?,
            })?;
        }
        Ok(())
    }

    pub fn compile(&mut self) -> Result<AotCode, Error> {
        let decoder = build_imac_decoder::<u64>(self.version);
        let mut instructions = [Instruction::default(); MAXIMUM_INSTRUCTIONS_PER_BLOCK];
        for i in 0..self.sections.len() {
            let (section_start, section_end) = self.sections[i];
            self.pc = Value::from_u64(section_start);
            loop {
                let pc = self.read_pc()?;
                if pc >= section_end {
                    break;
                }
                if let Some(dummy_end) = self.dummy_sections.get(&pc) {
                    self.pc = Value::from_u64(*dummy_end);
                    continue;
                }
                if let Some(label) = self.addresses_to_labels.get(&pc) {
                    self.emitter.emit_label(*label)?;
                }
                let mut count = 0;
                let mut current_pc = pc;
                while count < MAXIMUM_INSTRUCTIONS_PER_BLOCK && current_pc < section_end {
                    let instruction = decoder.decode(&mut self.memory, current_pc)?;
                    instructions[count] = instruction;
                    count += 1;
                    current_pc += u64::from(instruction_length(instruction));
                    if is_basic_block_end_instruction(instruction)
                        || self.addresses_to_labels.contains_key(&current_pc)
                    {
                        break;
                    }
                }
                self.emit_block(&instructions[0..count])?;
            }
        }
        let encoded_size = self.emitter.link()?;
        let mut buffer_mut = MmapMut::map_anon(encoded_size)?;
        self.emitter.encode(&mut buffer_mut[..])?;
        let code = buffer_mut.make_exec()?;
        let mut labels = HashMap::default();
        for (address, label) in &self.addresses_to_labels {
            let offset = self.emitter.get_label_offset(*label)?;
            labels.insert(*address, offset);
        }
        Ok(AotCode { code, labels })
    }

    // This method inspects PC value, and if any immediate encoded in the PC
    // Value matches a label, we will encode the real label directly in the
    // address for fast path jumps.
    fn optimize_pc_value(&self, value: Value) -> Result<Value, Error> {
        match value {
            Value::Imm(v) => Ok(Value::Imm(self.optimize_pc(v)?)),
            Value::Cond(c, t, f) => Ok(match (&*t, &*f) {
                (Value::Imm(t), Value::Imm(f)) => Value::Cond(
                    c,
                    Rc::new(Value::Imm(self.optimize_pc(*t)?)),
                    Rc::new(Value::Imm(self.optimize_pc(*f)?)),
                ),
                _ => Value::Cond(c, t, f),
            }),
            _ => Ok(value),
        }
    }

    fn optimize_pc(&self, pc: u64) -> Result<u64, Error> {
        if pc >= RISCV_MAX_MEMORY as u64 {
            return Err(Error::OutOfBound);
        }
        if pc < MAXIMUM_ENCODED_ADDRESS {
            if let Some(label) = self.addresses_to_labels.get(&pc) {
                return Ok(pc | (u64::from(*label) << 32) | ADDRESS_LABEL_FLAG);
            }
        }
        Ok(pc)
    }
}

impl CoreMachine for AotCompilingMachine {
    type REG = Value;
    type MEM = Self;

    fn pc(&self) -> &Value {
        &self.pc
    }

    fn set_pc(&mut self, next_pc: Value) {
        self.next_pc_write = Some(next_pc);
    }

    fn memory(&self) -> &Self {
        &self
    }

    fn memory_mut(&mut self) -> &mut Self {
        self
    }

    fn registers(&self) -> &[Value] {
        &self.registers
    }

    fn set_register(&mut self, index: usize, value: Value) {
        self.writes.push(Write::Register { index, value });
    }

    fn version(&self) -> u32 {
        self.version
    }
}

impl Machine for AotCompilingMachine {
    fn ecall(&mut self) -> Result<(), Error> {
        self.writes.push(Write::Ecall);
        Ok(())
    }

    fn ebreak(&mut self) -> Result<(), Error> {
        self.writes.push(Write::Ebreak);
        Ok(())
    }
}

impl Memory<Value> for AotCompilingMachine {
    fn init_pages(
        &mut self,
        _addr: u64,
        _size: u64,
        _flags: u8,
        _source: Option<Bytes>,
        _offset_from_addr: u64,
    ) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn fetch_flag(&mut self, _page: u64) -> Result<u8, Error> {
        Err(Error::Unimplemented)
    }

    fn store_byte(&mut self, _addr: u64, _size: u64, _value: u8) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn store_bytes(&mut self, _addr: u64, _value: &[u8]) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn execute_load16(&mut self, _addr: u64) -> Result<u16, Error> {
        Err(Error::Unimplemented)
    }

    fn load8(&mut self, addr: &Value) -> Result<Value, Error> {
        Ok(Value::Load(Rc::new(addr.clone()), 1))
    }

    fn load16(&mut self, addr: &Value) -> Result<Value, Error> {
        Ok(Value::Load(Rc::new(addr.clone()), 2))
    }

    fn load32(&mut self, addr: &Value) -> Result<Value, Error> {
        Ok(Value::Load(Rc::new(addr.clone()), 4))
    }

    fn load64(&mut self, addr: &Value) -> Result<Value, Error> {
        Ok(Value::Load(Rc::new(addr.clone()), 8))
    }

    fn store8(&mut self, addr: &Value, value: &Value) -> Result<(), Error> {
        self.writes.push(Write::Memory {
            address: addr.clone(),
            size: 1,
            value: value.clone(),
        });
        Ok(())
    }

    fn store16(&mut self, addr: &Value, value: &Value) -> Result<(), Error> {
        self.writes.push(Write::Memory {
            address: addr.clone(),
            size: 2,
            value: value.clone(),
        });
        Ok(())
    }

    fn store32(&mut self, addr: &Value, value: &Value) -> Result<(), Error> {
        self.writes.push(Write::Memory {
            address: addr.clone(),
            size: 4,
            value: value.clone(),
        });
        Ok(())
    }

    fn store64(&mut self, addr: &Value, value: &Value) -> Result<(), Error> {
        self.writes.push(Write::Memory {
            address: addr.clone(),
            size: 8,
            value: value.clone(),
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_memory_should_not_overlap_with_flags() {
        assert_eq!((RISCV_MAX_MEMORY as u64) & ADDRESS_WRITE_ONLY_FLAG, 0);
        assert_eq!((RISCV_MAX_MEMORY as u64) & ADDRESS_LABEL_FLAG, 0);
    }

    #[test]
    fn test_label_number_should_fit_in_24_bit() {
        assert_eq!(((MAXIMUM_LABELS as u64) << 32) & ADDRESS_WRITE_ONLY_FLAG, 0);
        assert_eq!(((MAXIMUM_LABELS as u64) << 32) & ADDRESS_LABEL_FLAG, 0);
    }

    #[test]
    fn test_maximum_encoded_address() {
        assert!(MAXIMUM_ENCODED_ADDRESS.is_power_of_two());
        assert!((RISCV_MAX_MEMORY as u64) < MAXIMUM_ENCODED_ADDRESS);
    }
}
