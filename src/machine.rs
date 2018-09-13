use super::bits::{rounddown, roundup};
use super::decoder::{build_rv32imac_decoder, Decoder};
use super::memory::mmu::Mmu;
use super::memory::{Memory, PROT_EXEC, PROT_READ, PROT_WRITE};
use super::{
    Error, A0, A7, REGISTER_ABI_NAMES, RISCV_GENERAL_REGISTER_NUMBER, RISCV_MAX_MEMORY,
    RISCV_PAGESIZE, SP,
};
use goblin::elf::program_header::PT_LOAD;
use goblin::elf::Elf;
use std::fmt::{self, Display};
use std::rc::Rc;

pub trait Machine<W, M: Memory> {
    fn pc(&self) -> W;
    fn set_pc(&mut self, next_pc: W);
    fn memory(&self) -> &M;
    fn memory_mut(&mut self) -> &mut M;
    fn registers(&self) -> &[W];
    fn registers_mut(&mut self) -> &mut [W];

    fn ecall(&mut self) -> Result<(), Error>;
    fn ebreak(&mut self) -> Result<(), Error>;
}

const DEFAULT_STACK_SIZE: usize = 8 * 1024 * 1024;

pub struct DefaultMachine<M: Memory> {
    // TODO: while CKB doesn't need it, other environment could benefit from
    // parameterized register size.
    pub registers: [u32; RISCV_GENERAL_REGISTER_NUMBER],
    pub pc: u32,
    pub memory: M,

    decoder: Decoder,
    running: bool,
    exit_code: u8,
}

impl<M: Memory> Machine<u32, M> for DefaultMachine<M> {
    fn pc(&self) -> u32 {
        self.pc
    }

    fn set_pc(&mut self, next_pc: u32) {
        self.pc = next_pc;
    }

    fn memory(&self) -> &M {
        &self.memory
    }

    fn memory_mut(&mut self) -> &mut M {
        &mut self.memory
    }

    fn registers(&self) -> &[u32] {
        &self.registers
    }

    fn registers_mut(&mut self) -> &mut [u32] {
        &mut self.registers
    }

    fn ecall(&mut self) -> Result<(), Error> {
        match self.registers[A7] {
            93 => {
                // exit
                self.exit_code = self.registers[A0] as u8;
                self.running = false;
                Ok(())
            }
            _ => Err(Error::InvalidEcall(self.registers[A7])),
        }
    }

    // TODO
    fn ebreak(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl<M> Display for DefaultMachine<M>
where
    M: Memory,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "pc  : 0x{:08X}", self.pc)?;
        for (i, name) in REGISTER_ABI_NAMES.iter().enumerate() {
            write!(f, "{:4}: 0x{:08X}", name, self.registers[i])?;
            if (i + 1) % 4 == 0 {
                writeln!(f)?;
            } else {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}

impl<M> DefaultMachine<M>
where
    M: Memory,
{
    // By default, we are using a default machine with proper MMU implementation now
    pub fn default() -> DefaultMachine<Mmu> {
        // While a real machine might use whatever random data left in the memory(or
        // random scrubbed data for security), we are initializing everything to 0 here
        // for deterministic behavior.
        DefaultMachine {
            registers: [0; RISCV_GENERAL_REGISTER_NUMBER],
            pc: 0,
            memory: Mmu::default(),
            decoder: build_rv32imac_decoder(),
            running: false,
            exit_code: 0,
        }
    }

    pub fn load(&mut self, program: &[u8]) -> Result<(), Error> {
        let elf = Elf::parse(program).map_err(|_e| Error::ParseError)?;
        for program_header in &elf.program_headers {
            if program_header.p_type == PT_LOAD {
                let aligned_start = rounddown(program_header.p_vaddr as usize, RISCV_PAGESIZE);
                let padding_start = program_header.p_vaddr as usize - aligned_start;
                let aligned_size = roundup(
                    program_header.p_filesz as usize + padding_start,
                    RISCV_PAGESIZE,
                );
                self.memory.mmap(
                    aligned_start,
                    aligned_size,
                    // TODO: do we need to distinguish between code pages and bss pages,
                    // then mark code pages as readonly?
                    PROT_READ | PROT_WRITE | PROT_EXEC,
                    Some(Rc::new(program.to_vec().into_boxed_slice())),
                    program_header.p_offset as usize - padding_start,
                )?;
            }
        }
        self.pc = elf.header.e_entry as u32;
        Ok(())
    }

    pub fn run(&mut self, args: &[String]) -> Result<u8, Error> {
        self.running = true;
        self.initialize_stack(args)?;
        while self.running {
            let instruction = {
                let memory = &mut self.memory;
                let pc = self.pc as usize;
                self.decoder.decode(memory, pc)?
            };
            instruction.execute(self)?;
        }
        Ok(self.exit_code)
    }

    fn initialize_stack(&mut self, args: &[String]) -> Result<(), Error> {
        // Initialize stack space
        self.memory.mmap(
            RISCV_MAX_MEMORY - DEFAULT_STACK_SIZE,
            DEFAULT_STACK_SIZE,
            PROT_READ | PROT_WRITE,
            None,
            0,
        )?;
        self.registers[SP] = RISCV_MAX_MEMORY as u32;
        // First value in this array is argc, then it contains the address(pointer)
        // of each argv object.
        let mut values = vec![args.len() as u32];
        for arg in args {
            let bytes = arg.as_bytes();
            let len = bytes.len() as u32 + 1;
            let address = self.registers[SP] - len;

            self.memory.store_bytes(address as usize, bytes)?;
            self.memory.store8(address as usize + bytes.len(), 0)?;

            values.push(address);
            self.registers[SP] = address;
        }

        // Since we are dealing with a stack, we need to push items in reversed
        // order
        for value in values.iter().rev() {
            let address = self.registers[SP] - 4;
            self.memory.store32(address as usize, *value)?;
            self.registers[SP] = address;
        }
        Ok(())
    }
}
