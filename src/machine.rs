use super::bits::rounddown;
use super::instructions::{Instruction, Register};
use super::memory::{Memory, PROT_EXEC, PROT_READ, PROT_WRITE};
use super::syscalls::Syscalls;
use super::{
    Error, A0, A7, DEFAULT_STACK_SIZE, REGISTER_ABI_NAMES, RISCV_GENERAL_REGISTER_NUMBER,
    RISCV_MAX_MEMORY, RISCV_PAGESIZE, SP,
};
use goblin::elf::program_header::{PF_R, PF_W, PF_X, PT_LOAD};
use goblin::elf::{Elf, Header};
use std::cmp::max;
use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

fn elf_bits(header: &Header) -> Option<usize> {
    // This is documented in ELF specification, we are exacting ELF file
    // class part here.
    // Right now we are only supporting 32 and 64 bits, in the future we
    // might add 128 bits support.
    match header.e_ident[4] {
        1 => Some(32),
        2 => Some(64),
        _ => None,
    }
}

// Converts goblin's ELF flags into RISC-V flags
fn convert_flags(p_flags: u32) -> u32 {
    let mut flags = 0;
    if p_flags & PF_R != 0 {
        flags |= PROT_READ;
    }
    if p_flags & PF_W != 0 {
        flags |= PROT_WRITE;
    }
    if p_flags & PF_X != 0 {
        flags |= PROT_EXEC;
    }
    flags
}

/// This is the core part of RISC-V that only deals with data part, it
/// is extracted from Machine so we can handle lifetime logic in dynamic
/// syscall support.
pub trait CoreMachine {
    type REG: Register;
    type MEM: Memory;
    fn pc(&self) -> &Self::REG;
    fn set_pc(&mut self, next_pc: Self::REG);
    fn memory(&self) -> &Self::MEM;
    fn memory_mut(&mut self) -> &mut Self::MEM;
    fn registers(&self) -> &[Self::REG];
    fn set_register(&mut self, idx: usize, value: Self::REG);
}

/// This is the core trait describing a full RISC-V machine. Instruction
/// package only needs to deal with the functions in this trait.
pub trait Machine: CoreMachine {
    fn ecall(&mut self) -> Result<(), Error>;
    fn ebreak(&mut self) -> Result<(), Error>;
}

/// This traits extend on top of CoreMachine by adding additional support
/// such as ELF range, cycles which might be needed on Rust side of the logic,
/// such as runner or syscall implementations.
pub trait SupportMachine: CoreMachine {
    // End address of elf segment
    fn elf_end(&self) -> usize;
    fn set_elf_end(&mut self, elf_end: usize);
    // Current execution cycles, it's up to the actual implementation to
    // call add_cycles for each instruction/operation to provide cycles.
    // The implementation might also choose not to do this to ignore this
    // feature.
    fn cycles(&self) -> u64;
    fn set_cycles(&mut self, cycles: u64);
    fn max_cycles(&self) -> Option<u64>;

    fn add_cycles(&mut self, cycles: u64) -> Result<(), Error> {
        let new_cycles = self
            .cycles()
            .checked_add(cycles)
            .ok_or(Error::InvalidCycles)?;
        if let Some(max_cycles) = self.max_cycles() {
            if new_cycles > max_cycles {
                return Err(Error::InvalidCycles);
            }
        }
        self.set_cycles(new_cycles);
        Ok(())
    }

    fn load_elf(&mut self, program: &[u8]) -> Result<(), Error> {
        let elf = Elf::parse(program).map_err(|_e| Error::ParseError)?;
        let bits = elf_bits(&elf.header).ok_or(Error::InvalidElfBits)?;
        if bits != Self::REG::BITS {
            return Err(Error::InvalidElfBits);
        }
        let program_slice = Rc::new(program.to_vec().into_boxed_slice());
        for program_header in &elf.program_headers {
            if program_header.p_type == PT_LOAD {
                let aligned_start = rounddown(program_header.p_vaddr as usize, RISCV_PAGESIZE);
                let padding_start = program_header.p_vaddr as usize - aligned_start;
                // Like a normal mmap, we will align size to pages internally
                let size = program_header.p_filesz as usize + padding_start;
                let current_elf_end = self.elf_end();
                self.set_elf_end(max(aligned_start + size, current_elf_end));
                self.memory_mut().mmap(
                    aligned_start,
                    size,
                    convert_flags(program_header.p_flags),
                    Some(Rc::clone(&program_slice)),
                    program_header.p_offset as usize - padding_start,
                )?;
                self.memory_mut()
                    .store_byte(aligned_start, padding_start, 0)?;
            }
        }
        self.set_pc(Self::REG::from_u64(elf.header.e_entry));
        Ok(())
    }

    fn initialize_stack(
        &mut self,
        args: &[Vec<u8>],
        stack_start: usize,
        stack_size: usize,
    ) -> Result<(), Error> {
        self.memory_mut()
            .mmap(stack_start, stack_size, PROT_READ | PROT_WRITE, None, 0)?;
        self.set_register(SP, Self::REG::from_usize(stack_start + stack_size));
        // First value in this array is argc, then it contains the address(pointer)
        // of each argv object.
        let mut values = vec![Self::REG::from_usize(args.len())];
        for arg in args {
            let bytes = arg.as_slice();
            let len = Self::REG::from_usize(bytes.len() + 1);
            let address = self.registers()[SP].overflowing_sub(&len);

            self.memory_mut().store_bytes(address.to_usize(), bytes)?;
            self.memory_mut()
                .store8(address.to_usize() + bytes.len(), 0)?;

            values.push(address.clone());
            self.set_register(SP, address);
        }
        // Since we are dealing with a stack, we need to push items in reversed
        // order
        for value in values.iter().rev() {
            let address =
                self.registers()[SP].overflowing_sub(&Self::REG::from_usize(Self::REG::BITS / 8));

            self.memory_mut()
                .store32(address.to_usize(), value.to_u32())?;
            self.set_register(SP, address);
        }
        if self.registers()[SP].to_usize() < stack_start {
            // args exceed stack size
            self.memory_mut().munmap(stack_start, stack_size)?;
            return Err(Error::OutOfBound);
        }
        Ok(())
    }
}

pub struct DefaultCoreMachine<R, M> {
    registers: [R; RISCV_GENERAL_REGISTER_NUMBER],
    pc: R,
    memory: M,
    elf_end: usize,
    cycles: u64,
    max_cycles: Option<u64>,
}

impl<R: Register, M: Memory> CoreMachine for DefaultCoreMachine<R, M> {
    type REG = R;
    type MEM = M;
    fn pc(&self) -> &R {
        &self.pc
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

    fn set_register(&mut self, idx: usize, value: R) {
        self.registers[idx] = value;
    }
}

impl<R: Register, M: Memory> SupportMachine for DefaultCoreMachine<R, M> {
    fn elf_end(&self) -> usize {
        self.elf_end
    }

    fn set_elf_end(&mut self, elf_end: usize) {
        self.elf_end = elf_end;
    }

    fn cycles(&self) -> u64 {
        self.cycles
    }

    fn set_cycles(&mut self, cycles: u64) {
        self.cycles = cycles;
    }

    fn max_cycles(&self) -> Option<u64> {
        self.max_cycles
    }
}

impl<R, M> Default for DefaultCoreMachine<R, M>
where
    R: Register,
    M: Default,
{
    fn default() -> DefaultCoreMachine<R, M> {
        // While a real machine might use whatever random data left in the memory(or
        // random scrubbed data for security), we are initializing everything to 0 here
        // for deterministic behavior.
        DefaultCoreMachine {
            registers: Default::default(),
            pc: R::zero(),
            memory: M::default(),
            elf_end: 0,
            cycles: 0,
            max_cycles: None,
        }
    }
}

impl<R, M> DefaultCoreMachine<R, M>
where
    R: Register,
    M: Default,
{
    pub fn new_with_max_cycles(max_cycles: u64) -> DefaultCoreMachine<R, M> {
        Self {
            max_cycles: Some(max_cycles),
            ..Self::default()
        }
    }
}

pub type InstructionCycleFunc = Fn(&Instruction) -> u64;

pub struct DefaultMachine<'a, R: Register, M: Memory> {
    core: DefaultCoreMachine<R, M>,

    // We have run benchmarks on secp256k1 verification, the performance
    // cost of the Box wrapper here is neglectable, hence we are sticking
    // with Box solution for simplicity now. Later if this becomes an issue,
    // we can change to static dispatch.
    instruction_cycle_func: Option<Box<InstructionCycleFunc>>,
    syscalls: Vec<Box<dyn Syscalls<R, M> + 'a>>,
    running: bool,
    exit_code: u8,
}

impl<'a, R: Register, M: Memory> Deref for DefaultMachine<'a, R, M> {
    type Target = DefaultCoreMachine<R, M>;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

impl<'a, R: Register, M: Memory> DerefMut for DefaultMachine<'a, R, M> {
    fn deref_mut(&mut self) -> &mut DefaultCoreMachine<R, M> {
        &mut self.core
    }
}

impl<'a, R: Register, M: Memory> CoreMachine for DefaultMachine<'a, R, M> {
    type REG = R;
    type MEM = M;
    fn pc(&self) -> &R {
        &self.pc
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

    fn set_register(&mut self, idx: usize, value: R) {
        self.registers[idx] = value;
    }
}

impl<'a, R: Register, M: Memory> SupportMachine for DefaultMachine<'a, R, M> {
    fn elf_end(&self) -> usize {
        self.elf_end
    }

    fn set_elf_end(&mut self, elf_end: usize) {
        self.elf_end = elf_end;
    }

    fn cycles(&self) -> u64 {
        self.cycles
    }

    fn set_cycles(&mut self, cycles: u64) {
        self.cycles = cycles;
    }

    fn max_cycles(&self) -> Option<u64> {
        self.max_cycles
    }
}

impl<'a, R: Register, M: Memory> Machine for DefaultMachine<'a, R, M> {
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

impl<'a, R, M> Display for DefaultMachine<'a, R, M>
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

impl<'a, R, M> Default for DefaultMachine<'a, R, M>
where
    R: Register,
    M: Memory + Default,
{
    fn default() -> DefaultMachine<'a, R, M> {
        DefaultMachine {
            instruction_cycle_func: None,
            core: DefaultCoreMachine::default(),
            syscalls: vec![],
            running: false,
            exit_code: 0,
        }
    }
}

impl<'a, R, M> DefaultMachine<'a, R, M>
where
    R: Register,
    M: Memory + Default,
{
    pub fn new_with_cost_model(
        instruction_cycle_func: Box<InstructionCycleFunc>,
        max_cycles: u64,
    ) -> DefaultMachine<'a, R, M> {
        Self {
            core: DefaultCoreMachine::new_with_max_cycles(max_cycles),
            instruction_cycle_func: Some(instruction_cycle_func),
            ..Self::default()
        }
    }
}

impl<'a, R, M> DefaultMachine<'a, R, M>
where
    R: Register,
    M: Memory,
{
    pub fn add_syscall_module(&mut self, syscall: Box<dyn Syscalls<R, M> + 'a>) {
        self.syscalls.push(syscall);
    }

    pub fn load_program(mut self, program: &[u8], args: &[Vec<u8>]) -> Result<Self, Error> {
        self.load_elf(program)?;
        for syscall in &mut self.syscalls {
            syscall.initialize(&mut self.core)?;
        }
        self.initialize_stack(
            args,
            RISCV_MAX_MEMORY - DEFAULT_STACK_SIZE,
            DEFAULT_STACK_SIZE,
        )?;
        Ok(self)
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn set_running(&mut self, running: bool) {
        self.running = running;
    }

    pub fn exit_code(&self) -> u8 {
        self.exit_code
    }

    pub fn instruction_cycle_func(&self) -> &Option<Box<InstructionCycleFunc>> {
        &self.instruction_cycle_func
    }
}
