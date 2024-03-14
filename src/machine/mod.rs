#[cfg(has_asm)]
pub mod asm;
pub mod trace;

use std::fmt::{self, Display};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

use bytes::Bytes;

use super::debugger::Debugger;
use super::decoder::{build_decoder, InstDecoder};
use super::elf::{parse_elf, LoadingAction, ProgramMetadata};
use super::instructions::{execute, Instruction, Register};
use super::memory::Memory;
use super::syscalls::Syscalls;
use super::{
    registers::{A0, A7, REGISTER_ABI_NAMES, SP},
    Error, ISA_MOP, RISCV_GENERAL_REGISTER_NUMBER,
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
pub const VERSION2: u32 = 2;

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
    fn set_max_cycles(&mut self, cycles: u64);

    fn running(&self) -> bool;
    fn set_running(&mut self, running: bool);

    // Erase all the states of the virtual machine.
    fn reset(&mut self, max_cycles: u64) -> Result<(), Error>;
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
        let metadata = parse_elf::<Self::REG>(program, version)?;
        self.load_binary(program, &metadata, update_pc)
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

    fn load_binary_inner(
        &mut self,
        program: &Bytes,
        metadata: &ProgramMetadata,
        update_pc: bool,
    ) -> Result<u64, Error> {
        let version = self.version();
        let mut bytes: u64 = 0;
        for action in &metadata.actions {
            let LoadingAction {
                addr,
                size,
                flags,
                source,
                offset_from_addr,
            } = action;

            self.memory_mut().init_pages(
                *addr,
                *size,
                *flags,
                Some(program.slice(source.start as usize..source.end as usize)),
                *offset_from_addr,
            )?;
            if version < VERSION1 {
                self.memory_mut().store_byte(*addr, *offset_from_addr, 0)?;
            }
            bytes = bytes
                .checked_add(source.end - source.start)
                .ok_or_else(|| {
                    Error::Unexpected(String::from("The bytes count overflowed on loading elf"))
                })?;
        }
        if update_pc {
            self.update_pc(Self::REG::from_u64(metadata.entry));
            self.commit_pc();
        }
        Ok(bytes)
    }

    fn load_binary(
        &mut self,
        program: &Bytes,
        metadata: &ProgramMetadata,
        update_pc: bool,
    ) -> Result<u64, Error> {
        // Similar to load_elf, this provides a way to adjust the behavior of load_binary_inner
        self.load_binary_inner(program, metadata, update_pc)
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

impl<R: Register, M: Memory<REG = R>> SupportMachine for DefaultCoreMachine<R, M> {
    fn cycles(&self) -> u64 {
        self.cycles
    }

    fn set_cycles(&mut self, cycles: u64) {
        self.cycles = cycles;
    }

    fn max_cycles(&self) -> u64 {
        self.max_cycles
    }

    fn set_max_cycles(&mut self, max_cycles: u64) {
        self.max_cycles = max_cycles;
    }

    fn reset(&mut self, max_cycles: u64) -> Result<(), Error> {
        self.registers = Default::default();
        self.pc = Default::default();
        self.memory.reset_memory()?;
        self.cycles = 0;
        self.max_cycles = max_cycles;
        self.reset_signal = true;
        self.memory_mut().set_lr(&R::from_u64(u64::MAX));
        Ok(())
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
        Self::new_with_memory(isa, version, max_cycles, M::default())
    }
}

impl<R: Register, M: Memory> DefaultCoreMachine<R, M> {
    pub fn new_with_memory(isa: u8, version: u32, max_cycles: u64, memory: M) -> Self {
        Self {
            registers: Default::default(),
            pc: Default::default(),
            next_pc: Default::default(),
            reset_signal: Default::default(),
            memory,
            cycles: Default::default(),
            max_cycles,
            running: Default::default(),
            isa,
            version,
            #[cfg(feature = "pprof")]
            code: Default::default(),
        }
    }

    pub fn set_max_cycles(&mut self, cycles: u64) {
        self.max_cycles = cycles;
    }

    pub fn take_memory(self) -> M {
        self.memory
    }
}

pub type InstructionCycleFunc = dyn Fn(Instruction) -> u64 + Send + Sync;

pub struct DefaultMachine<Inner> {
    inner: Inner,
    pause: Pause,

    // We have run benchmarks on secp256k1 verification, the performance
    // cost of the Box wrapper here is neglectable, hence we are sticking
    // with Box solution for simplicity now. Later if this becomes an issue,
    // we can change to static dispatch.
    instruction_cycle_func: Box<InstructionCycleFunc>,
    debugger: Option<Box<dyn Debugger<Inner>>>,
    syscalls: Vec<Box<dyn Syscalls<Inner>>>,
    exit_code: i8,
}

impl<Inner: CoreMachine> CoreMachine for DefaultMachine<Inner> {
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

impl<Inner: SupportMachine> SupportMachine for DefaultMachine<Inner> {
    fn cycles(&self) -> u64 {
        self.inner.cycles()
    }

    fn set_cycles(&mut self, cycles: u64) {
        self.inner.set_cycles(cycles)
    }

    fn max_cycles(&self) -> u64 {
        self.inner.max_cycles()
    }

    fn set_max_cycles(&mut self, max_cycles: u64) {
        self.inner.set_max_cycles(max_cycles)
    }

    fn reset(&mut self, max_cycles: u64) -> Result<(), Error> {
        self.inner_mut().reset(max_cycles)
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

impl<Inner: SupportMachine> Machine for DefaultMachine<Inner> {
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

impl<Inner: CoreMachine> Display for DefaultMachine<Inner> {
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

impl<Inner: SupportMachine> DefaultMachine<Inner> {
    pub fn load_program(&mut self, program: &Bytes, args: &[Bytes]) -> Result<u64, Error> {
        let elf_bytes = self.load_elf(program, true)?;
        let stack_bytes = self.initialize(args)?;
        let bytes = elf_bytes.checked_add(stack_bytes).ok_or_else(|| {
            Error::Unexpected(String::from(
                "The bytes count overflowed on loading program",
            ))
        })?;
        Ok(bytes)
    }

    pub fn load_program_with_metadata(
        &mut self,
        program: &Bytes,
        metadata: &ProgramMetadata,
        args: &[Bytes],
    ) -> Result<u64, Error> {
        let elf_bytes = self.load_binary(program, metadata, true)?;
        let stack_bytes = self.initialize(args)?;
        let bytes = elf_bytes.checked_add(stack_bytes).ok_or_else(|| {
            Error::Unexpected(String::from(
                "The bytes count overflowed on loading program",
            ))
        })?;
        Ok(bytes)
    }

    fn initialize(&mut self, args: &[Bytes]) -> Result<u64, Error> {
        for syscall in &mut self.syscalls {
            syscall.initialize(&mut self.inner)?;
        }
        if let Some(debugger) = &mut self.debugger {
            debugger.initialize(&mut self.inner)?;
        }
        let memory_size = self.memory().memory_size();
        let stack_size = memory_size / 4;
        let stack_bytes =
            self.initialize_stack(args, (memory_size - stack_size) as u64, stack_size as u64)?;
        // Make sure SP is 16 byte aligned
        if self.inner.version() >= VERSION1 {
            debug_assert!(self.registers()[SP].to_u64() % 16 == 0);
        }
        Ok(stack_bytes)
    }

    pub fn take_inner(self) -> Inner {
        self.inner
    }

    pub fn pause(&self) -> Pause {
        self.pause.clone()
    }

    pub fn set_pause(&mut self, pause: Pause) {
        self.pause = pause;
    }

    pub fn exit_code(&self) -> i8 {
        self.exit_code
    }

    pub fn instruction_cycle_func(&self) -> &InstructionCycleFunc {
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
        let mut decoder = build_decoder::<Inner::REG>(self.isa(), self.version());
        self.run_with_decoder(&mut decoder)
    }

    pub fn run_with_decoder<D: InstDecoder>(&mut self, decoder: &mut D) -> Result<i8, Error> {
        if self.isa() & ISA_MOP != 0 && self.version() == VERSION0 {
            return Err(Error::InvalidVersion);
        }
        self.set_running(true);
        while self.running() {
            if self.pause.has_interrupted() {
                self.pause.free();
                return Err(Error::Pause);
            }
            if self.reset_signal() {
                decoder.reset_instructions_cache()?;
            }
            self.step(decoder)?;
        }
        Ok(self.exit_code())
    }

    pub fn step<D: InstDecoder>(&mut self, decoder: &mut D) -> Result<(), Error> {
        let instruction = {
            let pc = self.pc().to_u64();
            let memory = self.memory_mut();
            decoder.decode(memory, pc)?
        };
        let cycles = self.instruction_cycle_func()(instruction);
        self.add_cycles(cycles)?;
        execute(instruction, self)
    }
}

pub struct DefaultMachineBuilder<Inner> {
    inner: Inner,
    instruction_cycle_func: Box<InstructionCycleFunc>,
    debugger: Option<Box<dyn Debugger<Inner>>>,
    syscalls: Vec<Box<dyn Syscalls<Inner>>>,
    pause: Pause,
}

impl<Inner> DefaultMachineBuilder<Inner> {
    pub fn new(inner: Inner) -> Self {
        Self {
            inner,
            instruction_cycle_func: Box::new(|_| 0),
            debugger: None,
            syscalls: vec![],
            pause: Pause::new(),
        }
    }

    pub fn instruction_cycle_func(
        mut self,
        instruction_cycle_func: Box<InstructionCycleFunc>,
    ) -> Self {
        self.instruction_cycle_func = instruction_cycle_func;
        self
    }

    pub fn syscall(mut self, syscall: Box<dyn Syscalls<Inner>>) -> Self {
        self.syscalls.push(syscall);
        self
    }

    pub fn pause(mut self, pause: Pause) -> Self {
        self.pause = pause;
        self
    }

    pub fn debugger(mut self, debugger: Box<dyn Debugger<Inner>>) -> Self {
        self.debugger = Some(debugger);
        self
    }

    pub fn build(self) -> DefaultMachine<Inner> {
        DefaultMachine {
            inner: self.inner,
            pause: self.pause,
            instruction_cycle_func: self.instruction_cycle_func,
            debugger: self.debugger,
            syscalls: self.syscalls,
            exit_code: 0,
        }
    }
}

#[derive(Clone, Default)]
pub struct Pause {
    s: Arc<AtomicU8>,
}

impl Pause {
    pub fn new() -> Self {
        Self {
            s: Arc::new(AtomicU8::new(0)),
        }
    }

    pub fn interrupt(&self) {
        self.s.store(1, Ordering::SeqCst);
    }

    pub fn has_interrupted(&self) -> bool {
        self.s.load(Ordering::SeqCst) != 0
    }

    pub fn get_raw_ptr(&self) -> *mut u8 {
        &*self.s as *const _ as *mut u8
    }

    pub fn free(&mut self) {
        self.s.store(0, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicU8;

    #[test]
    fn test_atomicu8() {
        // Assert AtomicU8 type has the same in-memory representation as u8.
        // This ensures that Pause::get_raw_ptr() works properly.
        assert_eq!(std::mem::size_of::<AtomicU8>(), 1);
    }
}
