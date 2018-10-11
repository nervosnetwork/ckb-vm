use super::bits::{rounddown, roundup};
use super::decoder::build_imac_decoder;
use super::instructions::Register;
use super::memory::{Memory, PROT_EXEC, PROT_READ, PROT_WRITE};
use super::syscalls::Syscalls;
use super::{
    Error, A0, A7, REGISTER_ABI_NAMES, RISCV_GENERAL_REGISTER_NUMBER, RISCV_MAX_MEMORY,
    RISCV_PAGESIZE, SP,
};
use goblin::elf::program_header::PT_LOAD;
use goblin::elf::Elf;
use std::cmp::max;
use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

const DEFAULT_STACK_SIZE: usize = 8 * 1024 * 1024;

// This is the core part of RISC-V that only deals with data part, it
// is extracted from Machine so we can handle lifetime logic in dynamic
// syscall support.
pub trait CoreMachine<R: Register, M: Memory> {
    fn pc(&self) -> R;
    fn set_pc(&mut self, next_pc: R);
    fn memory(&self) -> &M;
    fn memory_mut(&mut self) -> &mut M;
    fn registers(&self) -> &[R];
    fn registers_mut(&mut self) -> &mut [R];
    // End address of elf segment
    fn elf_end(&self) -> usize;
    fn set_elf_end(&mut self, elf_end: usize);
}

pub trait Machine<R: Register, M: Memory>: CoreMachine<R, M> {
    fn ecall(&mut self) -> Result<(), Error>;
    fn ebreak(&mut self) -> Result<(), Error>;
}

pub struct DefaultCoreMachine<R: Register, M: Memory> {
    pub registers: [R; RISCV_GENERAL_REGISTER_NUMBER],
    pub pc: R,
    pub memory: M,
    pub elf_end: usize,
}

impl<R: Register, M: Memory> CoreMachine<R, M> for DefaultCoreMachine<R, M> {
    fn pc(&self) -> R {
        self.pc
    }

    fn set_pc(&mut self, next_pc: R) {
        self.pc = next_pc;
    }

    fn memory(&self) -> &M {
        &self.memory
    }

    fn memory_mut(&mut self) -> &mut M {
        &mut self.memory
    }

    fn registers(&self) -> &[R] {
        &self.registers
    }

    fn registers_mut(&mut self) -> &mut [R] {
        &mut self.registers
    }

    fn elf_end(&self) -> usize {
        self.elf_end
    }

    fn set_elf_end(&mut self, elf_end: usize) {
        self.elf_end = elf_end;
    }
}

impl<R, M> Default for DefaultCoreMachine<R, M>
where
    R: Register,
    M: Memory + Default,
{
    fn default() -> DefaultCoreMachine<R, M> {
        // While a real machine might use whatever random data left in the memory(or
        // random scrubbed data for security), we are initializing everything to 0 here
        // for deterministic behavior.
        DefaultCoreMachine {
            registers: [R::zero(); RISCV_GENERAL_REGISTER_NUMBER],
            pc: R::zero(),
            memory: M::default(),
            elf_end: 0,
        }
    }
}

pub struct DefaultMachine<R: Register, M: Memory> {
    core: DefaultCoreMachine<R, M>,

    syscalls: Vec<Box<Syscalls<R, M>>>,
    running: bool,
    exit_code: u8,
}

impl<R: Register, M: Memory> Deref for DefaultMachine<R, M> {
    type Target = DefaultCoreMachine<R, M>;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

impl<R: Register, M: Memory> DerefMut for DefaultMachine<R, M> {
    fn deref_mut(&mut self) -> &mut DefaultCoreMachine<R, M> {
        &mut self.core
    }
}

impl<R: Register, M: Memory> CoreMachine<R, M> for DefaultMachine<R, M> {
    fn pc(&self) -> R {
        self.pc
    }

    fn set_pc(&mut self, next_pc: R) {
        self.pc = next_pc;
    }

    fn memory(&self) -> &M {
        &self.memory
    }

    fn memory_mut(&mut self) -> &mut M {
        &mut self.memory
    }

    fn registers(&self) -> &[R] {
        &self.registers
    }

    fn registers_mut(&mut self) -> &mut [R] {
        &mut self.registers
    }

    fn elf_end(&self) -> usize {
        self.elf_end
    }

    fn set_elf_end(&mut self, elf_end: usize) {
        self.elf_end = elf_end;
    }
}

impl<R: Register, M: Memory> Machine<R, M> for DefaultMachine<R, M> {
    fn ecall(&mut self) -> Result<(), Error> {
        let code = self.registers[A7].to_u64();
        match code {
            93 => {
                // exit
                self.exit_code = self.registers[A0].to_u8();
                self.running = false;
                Ok(())
            }
            _ => {
                for syscall in &mut self.syscalls {
                    let processed = syscall.ecall(&mut self.core)?;
                    if processed {
                        return Ok(());
                    }
                }
                Err(Error::InvalidEcall(code))
            }
        }
    }

    // TODO
    fn ebreak(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl<R, M> Display for DefaultMachine<R, M>
where
    R: Register,
    M: Memory,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "pc  : 0x{:16X}", self.pc.to_usize())?;
        for (i, name) in REGISTER_ABI_NAMES.iter().enumerate() {
            write!(f, "{:4}: 0x{:16X}", name, self.registers[i].to_usize())?;
            if (i + 1) % 4 == 0 {
                writeln!(f)?;
            } else {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}

impl<R, M> Default for DefaultMachine<R, M>
where
    R: Register,
    M: Memory + Default,
{
    fn default() -> DefaultMachine<R, M> {
        DefaultMachine {
            core: DefaultCoreMachine::default(),
            syscalls: vec![],
            running: false,
            exit_code: 0,
        }
    }
}

impl<R, M> DefaultMachine<R, M>
where
    R: Register,
    M: Memory + Default,
{
    pub fn add_syscall_module(&mut self, syscall: Box<Syscalls<R, M>>) {
        self.syscalls.push(syscall);
    }

    pub fn load(&mut self, program: &[u8]) -> Result<(), Error> {
        let elf = Elf::parse(program).map_err(|_e| Error::ParseError)?;
        let program_slice = Rc::new(program.to_vec().into_boxed_slice());
        for program_header in &elf.program_headers {
            if program_header.p_type == PT_LOAD {
                let aligned_start = rounddown(program_header.p_vaddr as usize, RISCV_PAGESIZE);
                let padding_start = program_header.p_vaddr as usize - aligned_start;
                let aligned_size = roundup(
                    program_header.p_filesz as usize + padding_start,
                    RISCV_PAGESIZE,
                );
                let current_elf_end = self.elf_end();
                self.set_elf_end(max(aligned_start + aligned_size, current_elf_end));
                self.memory.mmap(
                    aligned_start,
                    aligned_size,
                    // TODO: do we need to distinguish between code pages and bss pages,
                    // then mark code pages as readonly?
                    PROT_READ | PROT_WRITE | PROT_EXEC,
                    Some(Rc::clone(&program_slice)),
                    program_header.p_offset as usize - padding_start,
                )?;
            }
        }
        self.set_pc(R::from_u64(elf.header.e_entry));
        Ok(())
    }

    pub fn run(&mut self, args: &[Vec<u8>]) -> Result<u8, Error> {
        self.running = true;
        for syscall in &mut self.syscalls {
            syscall.initialize(&mut self.core)?;
        }
        self.initialize_stack(args)?;
        let decoder = build_imac_decoder::<R>();
        while self.running {
            let instruction = {
                let pc = self.pc().to_usize();
                let memory = self.memory_mut();
                decoder.decode(memory, pc)?
            };
            instruction.execute(self)?;
        }
        Ok(self.exit_code)
    }

    fn initialize_stack(&mut self, args: &[Vec<u8>]) -> Result<(), Error> {
        // Initialize stack space
        self.memory.mmap(
            RISCV_MAX_MEMORY - DEFAULT_STACK_SIZE,
            DEFAULT_STACK_SIZE,
            PROT_READ | PROT_WRITE,
            None,
            0,
        )?;
        self.registers[SP] = R::from_usize(RISCV_MAX_MEMORY);
        // First value in this array is argc, then it contains the address(pointer)
        // of each argv object.
        let mut values = vec![R::from_usize(args.len())];
        for arg in args {
            let bytes = arg.as_slice();
            let len = R::from_usize(bytes.len() + 1);
            let address = self.registers[SP].overflowing_sub(len).0;

            self.memory.store_bytes(address.to_usize(), bytes)?;
            self.memory.store8(address.to_usize() + bytes.len(), 0)?;

            values.push(address);
            self.registers[SP] = address;
        }

        // Since we are dealing with a stack, we need to push items in reversed
        // order
        for value in values.iter().rev() {
            let address = self.registers[SP]
                .overflowing_sub(R::from_usize(R::BITS / 8))
                .0;
            self.memory.store32(address.to_usize(), value.to_u32())?;
            self.registers[SP] = address;
        }
        Ok(())
    }
}
