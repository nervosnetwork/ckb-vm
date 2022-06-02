#[cfg(has_aot)]
pub mod aot;
#[cfg(has_asm)]
pub mod asm;
pub mod elf_adaptor;
pub mod trace;

use std::fmt::{self, Display};

use bytes::Bytes;
use scroll::Pread;

use super::debugger::Debugger;
use super::decoder::{build_decoder, Decoder};
use super::instructions::{execute, Instruction, Register};
use super::memory::{round_page_down, round_page_up, Memory};
use super::syscalls::Syscalls;
use super::{
    registers::{A0, A7, REGISTER_ABI_NAMES, SP},
    Error, DEFAULT_STACK_SIZE, ISA_MOP, RISCV_GENERAL_REGISTER_NUMBER, RISCV_MAX_MEMORY,
};

// Version 0 is the initial launched CKB VM, it is used in CKB Lina mainnet
pub const VERSION0: u32 = 0;
// Version 1 fixes known bugs discovered in version 0:
// * It's not possible to read the last byte in the VM memory;
// * https://github.com/nervosnetwork/ckb-vm/issues/92
// * https://github.com/nervosnetwork/ckb-vm/issues/97
// * https://github.com/nervosnetwork/ckb-vm/issues/98
// * https://github.com/nervosnetwork/ckb-vm/issues/106
pub const VERSION1: u32 = 1;

/// This is the core part of RISC-V that only deals with data part, it
/// is extracted from Machine so we can handle lifetime logic in dynamic
/// syscall support.
pub trait CoreMachine {
    type REG: Register;
    type MEM: Memory<REG = Self::REG>;

    fn pc(&self) -> &Self::REG;
    fn update_pc(&mut self, pc: Self::REG);
    fn commit_pc(&mut self);
    fn memory(&self) -> &Self::MEM;
    fn memory_mut(&mut self) -> &mut Self::MEM;
    fn registers(&self) -> &[Self::REG];
    fn set_register(&mut self, idx: usize, value: Self::REG);

    // Current running machine version, used to support compatible behavior
    // in case of bug fixes.
    fn version(&self) -> u32;
    fn isa(&self) -> u8;
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
    fn max_cycles(&self) -> u64;

    fn running(&self) -> bool;
    fn set_running(&mut self, running: bool);

    // Erase all the states of the virtual machine.
    fn reset(&mut self, max_cycles: u64);
    fn reset_signal(&mut self) -> bool;

    fn add_cycles(&mut self, cycles: u64) -> Result<(), Error> {
        let new_cycles = self
            .cycles()
            .checked_add(cycles)
            .ok_or(Error::CyclesOverflow)?;
        if new_cycles > self.max_cycles() {
            return Err(Error::CyclesExceeded);
        }
        self.set_cycles(new_cycles);
        Ok(())
    }

    fn add_cycles_no_checking(&mut self, cycles: u64) -> Result<(), Error> {
        let new_cycles = self
            .cycles()
            .checked_add(cycles)
            .ok_or(Error::CyclesOverflow)?;
        self.set_cycles(new_cycles);
        Ok(())
    }

    fn load_elf_inner(&mut self, program: &Bytes, update_pc: bool) -> Result<u64, Error> {
        let version = self.version();
        // We did not use Elf::parse here to avoid triggering potential bugs in goblin.
        // * https://github.com/nervosnetwork/ckb-vm/issues/143
        let (e_entry, program_headers): (u64, Vec<elf_adaptor::ProgramHeader>) =
            if version < VERSION1 {
                use goblin_v023::container::Ctx;
                use goblin_v023::elf::{program_header::ProgramHeader, Header};
                let header = program.pread::<Header>(0)?;
                let container = header.container().map_err(|_e| Error::ElfBits)?;
                let endianness = header.endianness().map_err(|_e| Error::ElfBits)?;
                if Self::REG::BITS != if container.is_big() { 64 } else { 32 } {
                    return Err(Error::ElfBits);
                }
                let ctx = Ctx::new(container, endianness);
                let program_headers = ProgramHeader::parse(
                    program,
                    header.e_phoff as usize,
                    header.e_phnum as usize,
                    ctx,
                )?
                .iter()
                .map(elf_adaptor::ProgramHeader::from_v0)
                .collect();
                (header.e_entry, program_headers)
            } else {
                use goblin_v040::container::Ctx;
                use goblin_v040::elf::{program_header::ProgramHeader, Header};
                let header = program.pread::<Header>(0)?;
                let container = header.container().map_err(|_e| Error::ElfBits)?;
                let endianness = header.endianness().map_err(|_e| Error::ElfBits)?;
                if Self::REG::BITS != if container.is_big() { 64 } else { 32 } {
                    return Err(Error::ElfBits);
                }
                let ctx = Ctx::new(container, endianness);
                let program_headers = ProgramHeader::parse(
                    program,
                    header.e_phoff as usize,
                    header.e_phnum as usize,
                    ctx,
                )?
                .iter()
                .map(elf_adaptor::ProgramHeader::from_v1)
                .collect();
                (header.e_entry, program_headers)
            };
        let mut bytes: u64 = 0;
        for program_header in program_headers {
            if program_header.p_type == elf_adaptor::PT_LOAD {
                let aligned_start = round_page_down(program_header.p_vaddr);
                let padding_start = program_header.p_vaddr.wrapping_sub(aligned_start);
                let size = round_page_up(program_header.p_memsz.wrapping_add(padding_start));
                let slice_start = program_header.p_offset;
                let slice_end = program_header
                    .p_offset
                    .wrapping_add(program_header.p_filesz);
                if slice_start > slice_end || slice_end > program.len() as u64 {
                    return Err(Error::ElfSegmentAddrOrSizeError);
                }
                self.memory_mut().init_pages(
                    aligned_start,
                    size,
                    elf_adaptor::convert_flags(program_header.p_flags, version < VERSION1)?,
                    Some(program.slice(slice_start as usize..slice_end as usize)),
                    padding_start,
                )?;
                if version < VERSION1 {
                    self.memory_mut()
                        .store_byte(aligned_start, padding_start, 0)?;
                }
                bytes = bytes.checked_add(slice_end - slice_start).ok_or_else(|| {
                    Error::Unexpected(String::from("The bytes count overflowed on loading elf"))
                })?;
            }
        }
        if update_pc {
            self.update_pc(Self::REG::from_u64(e_entry));
            self.commit_pc();
        }
        Ok(bytes)
    }

    fn load_elf(&mut self, program: &Bytes, update_pc: bool) -> Result<u64, Error> {
        // Allows to override load_elf by writing the real function body in load_elf_inner.
        //
        // impl SupportMachine for Somebody {
        //     fn load_elf(&mut self, program: &Bytes, update_pc: bool) -> Result<u64, Error> {
        //         // Do something before load_elf
        //         let r = self.load_elf_inner(program, update_pc);
        //         // Do something after
        //         return r;
        //     }
        // }
        self.load_elf_inner(program, update_pc)
    }

    fn initialize_stack(
        &mut self,
        args: &[Bytes],
        stack_start: u64,
        stack_size: u64,
    ) -> Result<u64, Error> {
        // When we re-ordered the sections of a program, writing data in high memory
        // will cause unnecessary changes. At the same time, for ckb, argc is always 0
        // and the memory is initialized to 0, so memory writing can be safely skipped.
        //
        // It should be noted that when "chaos_mode" enabled and "argv" is empty,
        // reading "argc" will return an unexpected data. This situation is not very common.
        //
        // See https://github.com/nervosnetwork/ckb-vm/issues/106 for more details.
        if self.version() >= VERSION1 && args.is_empty() {
            let argc_size = u64::from(Self::REG::BITS / 8);
            let origin_sp = stack_start + stack_size;
            let unaligned_sp_address = origin_sp - argc_size;
            let aligned_sp_address = unaligned_sp_address & (!15);
            let used_bytes = origin_sp - aligned_sp_address;
            self.set_register(SP, Self::REG::from_u64(aligned_sp_address));
            return Ok(used_bytes);
        }

        // We are enforcing WXorX now, there's no need to call init_pages here
        // since all the required bits are already set.
        self.set_register(SP, Self::REG::from_u64(stack_start + stack_size));
        // First value in this array is argc, then it contains the address(pointer)
        // of each argv object.
        let mut values = vec![Self::REG::from_u64(args.len() as u64)];
        for arg in args {
            let len = Self::REG::from_u64(arg.len() as u64 + 1);
            let address = self.registers()[SP].overflowing_sub(&len);

            self.memory_mut().store_bytes(address.to_u64(), arg)?;
            self.memory_mut()
                .store_byte(address.to_u64() + arg.len() as u64, 1, 0)?;

            values.push(address.clone());
            self.set_register(SP, address);
        }
        if self.version() >= VERSION1 {
            // There are 2 standard requirements of the initialized stack:
            // 1. argv[argc] should contain a null pointer here, hence we are
            // pushing another 0 to the values array;
            values.push(Self::REG::zero());
            // 2. SP must be aligned to 16-byte boundary, also considering _start
            // will read argc from SP and argv from SP + 8, we have to factor in
            // alignment here first, then push the values.
            let values_bytes =
                Self::REG::from_u64(Self::REG::BITS as u64 / 8 * values.len() as u64);
            let unaligned_sp_address = self.registers()[SP].overflowing_sub(&values_bytes).to_u64();
            // Perform alignment at 16-byte boundary towards lower address
            let aligned_sp_address = unaligned_sp_address & (!15);
            let aligned_bytes = unaligned_sp_address - aligned_sp_address;
            self.set_register(
                SP,
                self.registers()[SP].overflowing_sub(&Self::REG::from_u64(aligned_bytes)),
            );
        }
        // Since we are dealing with a stack, we need to push items in reversed
        // order
        for value in values.iter().rev() {
            let address =
                self.registers()[SP].overflowing_sub(&Self::REG::from_u8(Self::REG::BITS / 8));
            if self.version() >= VERSION1 {
                if Self::REG::BITS == 64 {
                    self.memory_mut().store64(&address, value)?;
                } else {
                    self.memory_mut().store32(&address, value)?;
                }
            } else {
                self.memory_mut().store32(&address, value)?;
            }
            self.set_register(SP, address);
        }
        if self.registers()[SP].to_u64() < stack_start {
            // args exceed stack size
            return Err(Error::MemOutOfStack);
        }
        Ok(stack_start + stack_size - self.registers()[SP].to_u64())
    }

    #[cfg(feature = "pprof")]
    fn code(&self) -> &Bytes;
}

#[derive(Default)]
pub struct DefaultCoreMachine<R, M> {
    registers: [R; RISCV_GENERAL_REGISTER_NUMBER],
    pc: R,
    next_pc: R,
    reset_signal: bool,
    memory: M,
    cycles: u64,
    max_cycles: u64,
    running: bool,
    isa: u8,
    version: u32,
    #[cfg(feature = "pprof")]
    code: Bytes,
}

impl<R: Register, M: Memory<REG = R>> CoreMachine for DefaultCoreMachine<R, M> {
    type REG = R;
    type MEM = M;
    fn pc(&self) -> &Self::REG {
        &self.pc
    }

    fn update_pc(&mut self, pc: Self::REG) {
        self.next_pc = pc;
    }

    fn commit_pc(&mut self) {
        self.pc = self.next_pc.clone();
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

    fn isa(&self) -> u8 {
        self.isa
    }

    fn version(&self) -> u32 {
        self.version
    }
}

impl<R: Register, M: Memory<REG = R> + Default> SupportMachine for DefaultCoreMachine<R, M> {
    fn cycles(&self) -> u64 {
        self.cycles
    }

    fn set_cycles(&mut self, cycles: u64) {
        self.cycles = cycles;
    }

    fn max_cycles(&self) -> u64 {
        self.max_cycles
    }

    fn reset(&mut self, max_cycles: u64) {
        self.registers = Default::default();
        self.pc = Default::default();
        self.memory = Default::default();
        self.cycles = 0;
        self.max_cycles = max_cycles;
        self.reset_signal = true;
    }

    fn reset_signal(&mut self) -> bool {
        let ret = self.reset_signal;
        self.reset_signal = false;
        ret
    }

    fn running(&self) -> bool {
        self.running
    }

    fn set_running(&mut self, running: bool) {
        self.running = running;
    }

    fn load_elf(&mut self, program: &Bytes, update_pc: bool) -> Result<u64, Error> {
        #[cfg(feature = "pprof")]
        {
            self.code = program.clone();
        }
        self.load_elf_inner(program, update_pc)
    }

    #[cfg(feature = "pprof")]
    fn code(&self) -> &Bytes {
        &self.code
    }
}

impl<R: Register, M: Memory + Default> DefaultCoreMachine<R, M> {
    pub fn new(isa: u8, version: u32, max_cycles: u64) -> Self {
        Self {
            isa,
            version,
            max_cycles,
            ..Default::default()
        }
    }

    pub fn set_max_cycles(&mut self, cycles: u64) {
        self.max_cycles = cycles;
    }

    pub fn take_memory(self) -> M {
        self.memory
    }
}

pub type InstructionCycleFunc = dyn Fn(Instruction) -> u64;

#[derive(Default)]
pub struct DefaultMachine<'a, Inner> {
    inner: Inner,

    // We have run benchmarks on secp256k1 verification, the performance
    // cost of the Box wrapper here is neglectable, hence we are sticking
    // with Box solution for simplicity now. Later if this becomes an issue,
    // we can change to static dispatch.
    instruction_cycle_func: Option<Box<InstructionCycleFunc>>,
    debugger: Option<Box<dyn Debugger<Inner> + 'a>>,
    syscalls: Vec<Box<dyn Syscalls<Inner> + 'a>>,
    exit_code: i8,
}

impl<Inner: CoreMachine> CoreMachine for DefaultMachine<'_, Inner> {
    type REG = <Inner as CoreMachine>::REG;
    type MEM = <Inner as CoreMachine>::MEM;

    fn pc(&self) -> &Self::REG {
        self.inner.pc()
    }

    fn update_pc(&mut self, pc: Self::REG) {
        self.inner.update_pc(pc);
    }

    fn commit_pc(&mut self) {
        self.inner.commit_pc();
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

    fn isa(&self) -> u8 {
        self.inner.isa()
    }

    fn version(&self) -> u32 {
        self.inner.version()
    }
}

impl<Inner: SupportMachine> SupportMachine for DefaultMachine<'_, Inner> {
    fn cycles(&self) -> u64 {
        self.inner.cycles()
    }

    fn set_cycles(&mut self, cycles: u64) {
        self.inner.set_cycles(cycles)
    }

    fn max_cycles(&self) -> u64 {
        self.inner.max_cycles()
    }

    fn reset(&mut self, max_cycles: u64) {
        self.inner_mut().reset(max_cycles);
    }

    fn reset_signal(&mut self) -> bool {
        self.inner_mut().reset_signal()
    }

    fn running(&self) -> bool {
        self.inner.running()
    }

    fn set_running(&mut self, running: bool) {
        self.inner.set_running(running);
    }

    #[cfg(feature = "pprof")]
    fn code(&self) -> &Bytes {
        self.inner.code()
    }
}

impl<Inner: SupportMachine> Machine for DefaultMachine<'_, Inner> {
    fn ecall(&mut self) -> Result<(), Error> {
        let code = self.registers()[A7].to_u64();
        match code {
            93 => {
                // exit
                self.exit_code = self.registers()[A0].to_i8();
                self.set_running(false);
                Ok(())
            }
            _ => {
                for syscall in &mut self.syscalls {
                    let processed = syscall.ecall(&mut self.inner)?;
                    if processed {
                        if self.cycles() > self.max_cycles() {
                            return Err(Error::CyclesExceeded);
                        }
                        return Ok(());
                    }
                }
                Err(Error::InvalidEcall(code))
            }
        }
    }

    fn ebreak(&mut self) -> Result<(), Error> {
        if let Some(debugger) = &mut self.debugger {
            debugger.ebreak(&mut self.inner)
        } else {
            // Unlike ecall, the default behavior of an EBREAK operation is
            // a dummy one.
            Ok(())
        }
    }
}

impl<Inner: CoreMachine> Display for DefaultMachine<'_, Inner> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "pc  : 0x{:16X}", self.pc().to_u64())?;
        for (i, name) in REGISTER_ABI_NAMES.iter().enumerate() {
            write!(f, "{:4}: 0x{:16X}", name, self.registers()[i].to_u64())?;
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
    pub fn load_program(&mut self, program: &Bytes, args: &[Bytes]) -> Result<u64, Error> {
        let elf_bytes = self.load_elf(program, true)?;
        for syscall in &mut self.syscalls {
            syscall.initialize(&mut self.inner)?;
        }
        if let Some(debugger) = &mut self.debugger {
            debugger.initialize(&mut self.inner)?;
        }
        let stack_bytes = self.initialize_stack(
            args,
            (RISCV_MAX_MEMORY - DEFAULT_STACK_SIZE) as u64,
            DEFAULT_STACK_SIZE as u64,
        )?;
        // Make sure SP is 16 byte aligned
        if self.inner.version() >= VERSION1 {
            debug_assert!(self.registers()[SP].to_u64() % 16 == 0);
        }
        let bytes = elf_bytes.checked_add(stack_bytes).ok_or_else(|| {
            Error::Unexpected(String::from(
                "The bytes count overflowed on loading program",
            ))
        })?;
        Ok(bytes)
    }

    pub fn take_inner(self) -> Inner {
        self.inner
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
        if self.isa() & ISA_MOP != 0 && self.version() == VERSION0 {
            return Err(Error::InvalidVersion);
        }
        let mut decoder = build_decoder::<Inner::REG>(self.isa(), self.version());
        self.set_running(true);
        while self.running() {
            if self.reset_signal() {
                decoder.reset_instructions_cache();
            }
            self.step(&mut decoder)?;
        }
        Ok(self.exit_code())
    }

    pub fn step(&mut self, decoder: &mut Decoder) -> Result<(), Error> {
        let instruction = {
            let pc = self.pc().to_u64();
            let memory = self.memory_mut();
            decoder.decode(memory, pc)?
        };
        let cycles = self
            .instruction_cycle_func()
            .as_ref()
            .map(|f| f(instruction))
            .unwrap_or(0);
        self.add_cycles(cycles)?;
        execute(instruction, self)
    }
}

#[derive(Default)]
pub struct DefaultMachineBuilder<'a, Inner> {
    inner: Inner,
    instruction_cycle_func: Option<Box<InstructionCycleFunc>>,
    debugger: Option<Box<dyn Debugger<Inner> + 'a>>,
    syscalls: Vec<Box<dyn Syscalls<Inner> + 'a>>,
}

impl<'a, Inner> DefaultMachineBuilder<'a, Inner> {
    pub fn new(inner: Inner) -> Self {
        Self {
            inner,
            instruction_cycle_func: None,
            debugger: None,
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

    pub fn debugger(mut self, debugger: Box<dyn Debugger<Inner> + 'a>) -> Self {
        self.debugger = Some(debugger);
        self
    }

    pub fn build(self) -> DefaultMachine<'a, Inner> {
        DefaultMachine {
            inner: self.inner,
            instruction_cycle_func: self.instruction_cycle_func,
            debugger: self.debugger,
            syscalls: self.syscalls,
            exit_code: 0,
        }
    }
}
