#[cfg(feature = "asm")]
pub mod asm;
pub mod trace;

use super::bits::{rounddown, roundup};
use super::decoder::build_imac_decoder;
use super::instructions::{execute, Instruction, Register};
use super::memory::{Memory, FLAG_EXECUTABLE, FLAG_FREEZED};
use super::syscalls::Syscalls;
use super::{
    registers::{A0, A7, REGISTER_ABI_NAMES, SP},
    Error, DEFAULT_STACK_SIZE, RISCV_GENERAL_REGISTER_NUMBER, RISCV_MAX_MEMORY, RISCV_PAGESIZE,
};
use bytes::Bytes;
use goblin::elf::program_header::{PF_R, PF_W, PF_X, PT_LOAD};
use goblin::elf::{Elf, Header};
use std::fmt::{self, Display};

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
fn convert_flags(p_flags: u32) -> Result<u8, Error> {
    let readable = p_flags & PF_R != 0;
    let writable = p_flags & PF_W != 0;
    let executable = p_flags & PF_X != 0;
    if (!readable) || (writable && executable) {
        return Err(Error::InvalidPermission);
    }
    if executable {
        Ok(FLAG_EXECUTABLE | FLAG_FREEZED)
    } else {
        Ok(FLAG_FREEZED)
    }
}

/// This is the core part of RISC-V that only deals with data part, it
/// is extracted from Machine so we can handle lifetime logic in dynamic
/// syscall support.
pub trait CoreMachine {
    type REG: Register;
    type MEM: Memory<Self::REG>;

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

    fn load_elf(&mut self, program: &Bytes) -> Result<(), Error> {
        let elf = Elf::parse(program).map_err(|_e| Error::ParseError)?;
        let bits = elf_bits(&elf.header).ok_or(Error::InvalidElfBits)?;
        if bits != Self::REG::BITS {
            return Err(Error::InvalidElfBits);
        }
        for program_header in &elf.program_headers {
            if program_header.p_type == PT_LOAD {
                let aligned_start = rounddown(program_header.p_vaddr as usize, RISCV_PAGESIZE);
                let padding_start = program_header.p_vaddr as usize - aligned_start;
                let size = roundup(
                    program_header.p_memsz as usize + padding_start,
                    RISCV_PAGESIZE,
                );
                self.memory_mut().init_pages(
                    aligned_start,
                    size,
                    convert_flags(program_header.p_flags)?,
                    Some(program.slice(
                        program_header.p_offset as usize,
                        (program_header.p_offset + program_header.p_filesz) as usize,
                    )),
                    padding_start,
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
        args: &[Bytes],
        stack_start: usize,
        stack_size: usize,
    ) -> Result<(), Error> {
        // We are enforcing WXorX now, there's no need to call init_pages here
        // since all the required bits are already set.
        self.set_register(SP, Self::REG::from_usize(stack_start + stack_size));
        // First value in this array is argc, then it contains the address(pointer)
        // of each argv object.
        let mut values = vec![Self::REG::from_usize(args.len())];
        for arg in args {
            let len = Self::REG::from_usize(arg.len() + 1);
            let address = self.registers()[SP].overflowing_sub(&len);

            self.memory_mut().store_bytes(address.to_usize(), arg)?;
            self.memory_mut()
                .store_byte(address.to_usize() + arg.len(), 1, 0)?;

            values.push(address.clone());
            self.set_register(SP, address);
        }
        // Since we are dealing with a stack, we need to push items in reversed
        // order
        for value in values.iter().rev() {
            let address =
                self.registers()[SP].overflowing_sub(&Self::REG::from_usize(Self::REG::BITS / 8));

            self.memory_mut().store32(&address, value)?;
            self.set_register(SP, address);
        }
        if self.registers()[SP].to_usize() < stack_start {
            // args exceed stack size
            return Err(Error::OutOfBound);
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct DefaultCoreMachine<R, M> {
    registers: [R; RISCV_GENERAL_REGISTER_NUMBER],
    pc: R,
    memory: M,
    cycles: u64,
    max_cycles: Option<u64>,
}

impl<R: Register, M: Memory<R>> CoreMachine for DefaultCoreMachine<R, M> {
    type REG = R;
    type MEM = M;
    fn pc(&self) -> &Self::REG {
        &self.pc
    }

    fn set_pc(&mut self, next_pc: Self::REG) {
        self.pc = next_pc;
    }

    fn memory(&self) -> &Self::MEM {
        &self.memory
    }

    fn memory_mut(&mut self) -> &mut Self::MEM {
        &mut self.memory
    }

    fn registers(&self) -> &[Self::REG] {
        &self.registers
    }

    fn set_register(&mut self, idx: usize, value: Self::REG) {
        self.registers[idx] = value;
    }
}

impl<R: Register, M: Memory<R>> SupportMachine for DefaultCoreMachine<R, M> {
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

impl<R: Register, M: Memory<R> + Default> DefaultCoreMachine<R, M> {
    pub fn new_with_max_cycles(max_cycles: u64) -> Self {
        Self {
            max_cycles: Some(max_cycles),
            ..Default::default()
        }
    }
}

pub type InstructionCycleFunc = Fn(Instruction) -> u64;

#[derive(Default)]
pub struct DefaultMachine<'a, Inner> {
    inner: Inner,

    // We have run benchmarks on secp256k1 verification, the performance
    // cost of the Box wrapper here is neglectable, hence we are sticking
    // with Box solution for simplicity now. Later if this becomes an issue,
    // we can change to static dispatch.
    instruction_cycle_func: Option<Box<InstructionCycleFunc>>,
    syscalls: Vec<Box<dyn Syscalls<Inner> + 'a>>,
    running: bool,
    exit_code: i8,
}

impl<Inner: CoreMachine> CoreMachine for DefaultMachine<'_, Inner> {
    type REG = <Inner as CoreMachine>::REG;
    type MEM = <Inner as CoreMachine>::MEM;

    fn pc(&self) -> &Self::REG {
        &self.inner.pc()
    }

    fn set_pc(&mut self, next_pc: Self::REG) {
        self.inner.set_pc(next_pc)
    }

    fn memory(&self) -> &Self::MEM {
        self.inner.memory()
    }

    fn memory_mut(&mut self) -> &mut Self::MEM {
        self.inner.memory_mut()
    }

    fn registers(&self) -> &[Self::REG] {
        self.inner.registers()
    }

    fn set_register(&mut self, idx: usize, value: Self::REG) {
        self.inner.set_register(idx, value)
    }
}

impl<Inner: SupportMachine> SupportMachine for DefaultMachine<'_, Inner> {
    fn cycles(&self) -> u64 {
        self.inner.cycles()
    }

    fn set_cycles(&mut self, cycles: u64) {
        self.inner.set_cycles(cycles)
    }

    fn max_cycles(&self) -> Option<u64> {
        self.inner.max_cycles()
    }
}

impl<Inner: SupportMachine> Machine for DefaultMachine<'_, Inner> {
    fn ecall(&mut self) -> Result<(), Error> {
        let code = self.registers()[A7].to_u64();
        match code {
            93 => {
                // exit
                self.exit_code = self.registers()[A0].to_i8();
                self.running = false;
                Ok(())
            }
            _ => {
                for syscall in &mut self.syscalls {
                    let processed = syscall.ecall(&mut self.inner)?;
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

impl<Inner: CoreMachine> Display for DefaultMachine<'_, Inner> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "pc  : 0x{:16X}", self.pc().to_usize())?;
        for (i, name) in REGISTER_ABI_NAMES.iter().enumerate() {
            write!(f, "{:4}: 0x{:16X}", name, self.registers()[i].to_usize())?;
            if (i + 1) % 4 == 0 {
                writeln!(f)?;
            } else {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}

impl<'a, Inner: SupportMachine> DefaultMachine<'a, Inner> {
    pub fn load_program(&mut self, program: &Bytes, args: &[Bytes]) -> Result<(), Error> {
        self.load_elf(program)?;
        for syscall in &mut self.syscalls {
            syscall.initialize(&mut self.inner)?;
        }
        self.initialize_stack(
            args,
            RISCV_MAX_MEMORY - DEFAULT_STACK_SIZE,
            DEFAULT_STACK_SIZE,
        )?;
        Ok(())
    }

    pub fn take_inner(self) -> Inner {
        self.inner
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn set_running(&mut self, running: bool) {
        self.running = running;
    }

    pub fn exit_code(&self) -> i8 {
        self.exit_code
    }

    pub fn instruction_cycle_func(&self) -> &Option<Box<InstructionCycleFunc>> {
        &self.instruction_cycle_func
    }

    pub fn inner_mut(&mut self) -> &mut Inner {
        &mut self.inner
    }

    // This is the most naive way of running the VM, it only decodes each
    // instruction and run it, no optimization is performed here. It might
    // not be practical in production, but it serves as a baseline and
    // reference implementation
    pub fn run(&mut self) -> Result<i8, Error> {
        let decoder = build_imac_decoder::<Inner::REG>();
        self.set_running(true);
        while self.running() {
            let instruction = {
                let pc = self.pc().to_usize();
                let memory = self.memory_mut();
                decoder.decode(memory, pc)?
            };
            execute(instruction, self)?;
            let cycles = self
                .instruction_cycle_func()
                .as_ref()
                .map(|f| f(instruction))
                .unwrap_or(0);
            self.add_cycles(cycles)?;
        }
        Ok(self.exit_code())
    }
}

#[derive(Default)]
pub struct DefaultMachineBuilder<'a, Inner> {
    inner: Inner,
    instruction_cycle_func: Option<Box<InstructionCycleFunc>>,
    syscalls: Vec<Box<dyn Syscalls<Inner> + 'a>>,
}

impl<'a, Inner> DefaultMachineBuilder<'a, Inner> {
    pub fn new(inner: Inner) -> Self {
        Self {
            inner,
            instruction_cycle_func: None,
            syscalls: vec![],
        }
    }

    pub fn instruction_cycle_func(
        mut self,
        instruction_cycle_func: Box<InstructionCycleFunc>,
    ) -> Self {
        self.instruction_cycle_func = Some(instruction_cycle_func);
        self
    }

    pub fn syscall(mut self, syscall: Box<dyn Syscalls<Inner> + 'a>) -> Self {
        self.syscalls.push(syscall);
        self
    }

    pub fn build(self) -> DefaultMachine<'a, Inner> {
        DefaultMachine {
            inner: self.inner,
            instruction_cycle_func: self.instruction_cycle_func,
            syscalls: self.syscalls,
            running: false,
            exit_code: 0,
        }
    }
}
