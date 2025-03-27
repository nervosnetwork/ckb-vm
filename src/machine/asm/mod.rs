use byteorder::{ByteOrder, LittleEndian};
use bytes::Bytes;
pub use ckb_vm_definitions::asm::AsmCoreMachine;
use ckb_vm_definitions::{
    asm::{
        calculate_slot, Trace, RET_CYCLES_OVERFLOW, RET_DECODE_TRACE, RET_DYNAMIC_JUMP, RET_EBREAK,
        RET_ECALL, RET_INVALID_PERMISSION, RET_MAX_CYCLES_EXCEEDED, RET_OUT_OF_BOUND, RET_PAUSE,
        RET_SLOWPATH, TRACE_ITEM_LENGTH, TRACE_SIZE,
    },
    instructions::OP_CUSTOM_TRACE_END,
    ISA_MOP, MEMORY_FRAMES, MEMORY_FRAMESIZE, MEMORY_FRAME_PAGE_SHIFTS,
    RISCV_GENERAL_REGISTER_NUMBER, RISCV_MAX_MEMORY, RISCV_PAGE_SHIFTS,
};
use rand::{prelude::RngCore, SeedableRng};
use std::alloc::{alloc, Layout};
use std::os::raw::c_uchar;

use crate::{
    decoder::{build_decoder, Decoder},
    elf::ProgramMetadata,
    instructions::{
        blank_instruction, execute_instruction, extract_opcode, instruction_length,
        is_basic_block_end_instruction,
    },
    machine::VERSION0,
    memory::{
        fill_page_data, get_page_indices, memset, round_page_down, round_page_up, FLAG_DIRTY,
        FLAG_EXECUTABLE, FLAG_FREEZED, FLAG_WRITABLE, FLAG_WXORX_BIT,
    },
    CoreMachine, DefaultMachine, DefaultMachineRunner, Error, Machine, Memory, SupportMachine,
    MEMORY_FRAME_SHIFTS, RISCV_PAGES, RISCV_PAGESIZE,
};

pub trait AsmCoreMachineRevealer: AsRef<Box<AsmCoreMachine>> + AsMut<Box<AsmCoreMachine>> {
    fn new(isa: u8, version: u32, max_cycles: u64, memory_size: usize) -> Self;
}

impl AsmCoreMachineRevealer for Box<AsmCoreMachine> {
    fn new(isa: u8, version: u32, max_cycles: u64, memory_size: usize) -> Self {
        assert_ne!(memory_size, 0);
        assert_eq!(memory_size % RISCV_PAGESIZE, 0);
        assert_eq!(memory_size % (1 << MEMORY_FRAME_SHIFTS), 0);

        let mut machine = unsafe {
            let machine_size =
                std::mem::size_of::<AsmCoreMachine>() - RISCV_MAX_MEMORY + memory_size;

            let layout = Layout::array::<u8>(machine_size).unwrap();
            let raw_allocation = alloc(layout) as *mut AsmCoreMachine;
            Box::from_raw(raw_allocation)
        };
        machine.registers = [0; RISCV_GENERAL_REGISTER_NUMBER];
        machine.pc = 0;
        machine.next_pc = 0;
        machine.running = 0;
        machine.cycles = 0;
        machine.max_cycles = max_cycles;
        if cfg!(feature = "enable-chaos-mode-by-default") {
            machine.chaos_mode = 1;
        } else {
            machine.chaos_mode = 0;
        }
        machine.chaos_seed = 0;
        machine.load_reservation_address = u64::MAX;
        machine.reset_signal = 0;
        machine.version = version;
        machine.isa = isa;
        machine.flags = [0; RISCV_PAGES];
        for i in 0..TRACE_SIZE {
            machine.traces[i] = Trace::default();
        }
        machine.frames = [0; MEMORY_FRAMES];

        machine.memory_size = memory_size as u64;
        machine.frames_size = (memory_size / MEMORY_FRAMESIZE) as u64;
        machine.flags_size = (memory_size / RISCV_PAGESIZE) as u64;

        machine.last_read_frame = u64::MAX;
        machine.last_write_page = u64::MAX;

        machine
    }
}

impl<R> CoreMachine for R
where
    R: AsmCoreMachineRevealer,
{
    type REG = u64;
    type MEM = Self;

    fn pc(&self) -> &Self::REG {
        &self.as_ref().pc
    }

    fn update_pc(&mut self, pc: Self::REG) {
        self.as_mut().next_pc = pc;
    }

    fn commit_pc(&mut self) {
        self.as_mut().pc = self.as_ref().next_pc;
    }

    fn memory(&self) -> &Self {
        self
    }

    fn memory_mut(&mut self) -> &mut Self {
        self
    }

    fn registers(&self) -> &[Self::REG] {
        &self.as_ref().registers
    }

    fn set_register(&mut self, idx: usize, value: Self::REG) {
        self.as_mut().registers[idx] = value;
    }

    fn isa(&self) -> u8 {
        self.as_ref().isa
    }

    fn version(&self) -> u32 {
        self.as_ref().version
    }
}

// This function is exported for asm and aot machine.
// Note that the parameter `machine` is after parameter `frame_index`. Generally
// speaking, put `machine` in the first parameter is more human readable,
// but consider that in the asm machine, `frame_index` is stored in `rdi` and `machine`
// is stored in `rsi`, there is no need to exchange the values in the two registers
// in this way.
#[no_mangle]
pub extern "C" fn inited_memory(frame_index: u64, machine: &mut AsmCoreMachine) {
    let addr_from = (frame_index << MEMORY_FRAME_SHIFTS) as usize;
    let addr_to = ((frame_index + 1) << MEMORY_FRAME_SHIFTS) as usize;
    if machine.chaos_mode != 0 {
        let mut gen = rand::rngs::StdRng::seed_from_u64(machine.chaos_seed.into());
        gen.fill_bytes(&mut machine.memory[addr_from..addr_to]);
        machine.chaos_seed = gen.next_u32();
    } else {
        memset(&mut machine.memory[addr_from..addr_to], 0);
    }
}

fn check_memory<R: AsmCoreMachineRevealer>(machine: &mut R, page: u64) {
    let frame = page >> MEMORY_FRAME_PAGE_SHIFTS;
    if machine.as_ref().frames[frame as usize] == 0 {
        inited_memory(frame, machine.as_mut());
        machine.as_mut().frames[frame as usize] = 1;
    }
}

fn check_permission<M: Memory>(memory: &mut M, page: u64, flag: u8) -> Result<(), Error> {
    let page_flag = memory.fetch_flag(page)?;
    if (page_flag & FLAG_WXORX_BIT) != (flag & FLAG_WXORX_BIT) {
        return Err(Error::MemWriteOnExecutablePage);
    }
    Ok(())
}

// check whether a memory address is writable or not and mark it as dirty, `size` should be 1, 2, 4 or 8
fn check_memory_writable<R: AsmCoreMachineRevealer>(
    machine: &mut R,
    addr: u64,
    size: usize,
) -> Result<(), Error> {
    debug_assert!(size == 1 || size == 2 || size == 4 || size == 8);
    let page = addr >> RISCV_PAGE_SHIFTS;
    if page as usize >= machine.memory_pages() {
        return Err(Error::MemOutOfBound);
    }
    check_permission(machine, page, FLAG_WRITABLE)?;
    check_memory(machine, page);
    machine.set_flag(page, FLAG_DIRTY)?;

    // check next page if neccessary
    let page_offset = addr as usize % RISCV_PAGESIZE;
    if page_offset + size > RISCV_PAGESIZE {
        let page = page + 1;
        if page as usize >= machine.memory_pages() {
            return Err(Error::MemOutOfBound);
        } else {
            check_permission(machine, page, FLAG_WRITABLE)?;
            check_memory(machine, page);
            machine.set_flag(page, FLAG_DIRTY)?
        }
    }
    Ok(())
}

// check whether a memory address is executable, `size` should be 2 or 4
fn check_memory_executable<R: AsmCoreMachineRevealer>(
    machine: &mut R,
    addr: u64,
    size: usize,
) -> Result<(), Error> {
    debug_assert!(size == 2 || size == 4);

    let page = addr >> RISCV_PAGE_SHIFTS;
    if page as usize >= machine.memory_pages() {
        return Err(Error::MemOutOfBound);
    }
    check_permission(machine, page, FLAG_EXECUTABLE)?;
    check_memory(machine, page);

    // check next page if neccessary
    let page_offset = addr as usize % RISCV_PAGESIZE;
    if page_offset + size > RISCV_PAGESIZE {
        let page = page + 1;
        if page as usize >= machine.memory_pages() {
            return Err(Error::MemOutOfBound);
        } else {
            check_permission(machine, page, FLAG_EXECUTABLE)?;
            check_memory(machine, page);
        }
    }
    Ok(())
}

// check whether a memory address is initialized, `size` should be 1, 2, 4 or 8
fn check_memory_inited<R: AsmCoreMachineRevealer>(
    machine: &mut R,
    addr: u64,
    size: usize,
) -> Result<(), Error> {
    debug_assert!(size == 1 || size == 2 || size == 4 || size == 8);
    let page = addr >> RISCV_PAGE_SHIFTS;
    if page as usize >= machine.memory_pages() {
        return Err(Error::MemOutOfBound);
    }
    check_memory(machine, page);

    // check next page if neccessary
    let page_offset = addr as usize % RISCV_PAGESIZE;
    if page_offset + size > RISCV_PAGESIZE {
        let page = page + 1;
        if page as usize >= machine.memory_pages() {
            return Err(Error::MemOutOfBound);
        } else {
            check_memory(machine, page);
        }
    }
    Ok(())
}

impl<R> Memory for R
where
    R: AsmCoreMachineRevealer,
{
    type REG = u64;

    fn new() -> Self {
        unreachable!()
    }

    fn new_with_memory(_: usize) -> Self {
        unreachable!()
    }

    fn init_pages(
        &mut self,
        addr: u64,
        size: u64,
        flags: u8,
        source: Option<Bytes>,
        offset_from_addr: u64,
    ) -> Result<(), Error> {
        if round_page_down(addr) != addr || round_page_up(size) != size {
            return Err(Error::MemPageUnalignedAccess);
        }
        let memory_size = self.memory_size() as u64;
        if addr > memory_size
            || size > memory_size as u64
            || addr + size > memory_size as u64
            || offset_from_addr > size
        {
            return Err(Error::MemOutOfBound);
        }
        // We benchmarked the code piece here, using while loop this way is
        // actually faster than a for..in solution. The difference is roughly
        // 3% so we are keeping this version.
        let mut current_addr = addr;
        while current_addr < addr + size {
            let page = current_addr / RISCV_PAGESIZE as u64;
            if self.fetch_flag(page)? & FLAG_FREEZED != 0 {
                return Err(Error::MemWriteOnFreezedPage);
            }
            current_addr += RISCV_PAGESIZE as u64;
        }
        fill_page_data(self, addr, size, source, offset_from_addr)?;
        current_addr = addr;
        while current_addr < addr + size {
            let page = current_addr / RISCV_PAGESIZE as u64;
            self.set_flag(page, flags)?;
            current_addr += RISCV_PAGESIZE as u64;
        }
        // Clear last read/write page cache
        self.as_mut().last_read_frame = u64::MAX;
        self.as_mut().last_write_page = u64::MAX;
        Ok(())
    }

    fn fetch_flag(&mut self, page: u64) -> Result<u8, Error> {
        if page < self.memory_pages() as u64 {
            Ok(self.as_mut().flags[page as usize])
        } else {
            Err(Error::MemOutOfBound)
        }
    }

    fn set_flag(&mut self, page: u64, flag: u8) -> Result<(), Error> {
        if page < self.memory_pages() as u64 {
            self.as_mut().flags[page as usize] |= flag;
            // Clear last write page cache
            self.as_mut().last_write_page = u64::MAX;
            Ok(())
        } else {
            Err(Error::MemOutOfBound)
        }
    }

    fn clear_flag(&mut self, page: u64, flag: u8) -> Result<(), Error> {
        if page < self.memory_pages() as u64 {
            self.as_mut().flags[page as usize] &= !flag;
            // Clear last write page cache
            self.as_mut().last_write_page = u64::MAX;
            Ok(())
        } else {
            Err(Error::MemOutOfBound)
        }
    }

    fn memory_size(&self) -> usize {
        self.as_ref().memory_size as usize
    }

    fn store_bytes(&mut self, addr: u64, value: &[u8]) -> Result<(), Error> {
        if value.is_empty() {
            return Ok(());
        }
        let page_indices = get_page_indices(addr, value.len() as u64)?;
        for page in page_indices.0..=page_indices.1 {
            check_permission(self, page, FLAG_WRITABLE)?;
            check_memory(self, page);
            self.set_flag(page, FLAG_DIRTY)?;
        }
        let slice = &mut self.as_mut().memory[addr as usize..addr as usize + value.len()];
        slice.copy_from_slice(value);
        Ok(())
    }

    fn store_byte(&mut self, addr: u64, size: u64, value: u8) -> Result<(), Error> {
        if size == 0 {
            return Ok(());
        }
        let page_indices = get_page_indices(addr, size)?;
        for page in page_indices.0..=page_indices.1 {
            check_permission(self, page, FLAG_WRITABLE)?;
            check_memory(self, page);
            self.set_flag(page, FLAG_DIRTY)?;
        }
        memset(
            &mut self.as_mut().memory[addr as usize..(addr + size) as usize],
            value,
        );
        Ok(())
    }

    fn load_bytes(&mut self, addr: u64, size: u64) -> Result<Bytes, Error> {
        if size == 0 {
            return Ok(Bytes::new());
        }
        let page_indices = get_page_indices(addr, size)?;
        for page in page_indices.0..=page_indices.1 {
            check_memory(self, page);
        }
        Ok(Bytes::from(
            self.as_ref().memory[addr as usize..(addr + size) as usize].to_vec(),
        ))
    }

    fn execute_load16(&mut self, addr: u64) -> Result<u16, Error> {
        check_memory_executable(self, addr, 2)?;
        Ok(LittleEndian::read_u16(
            &self.as_ref().memory[addr as usize..addr as usize + 2],
        ))
    }

    fn execute_load32(&mut self, addr: u64) -> Result<u32, Error> {
        check_memory_executable(self, addr, 4)?;
        Ok(LittleEndian::read_u32(
            &self.as_ref().memory[addr as usize..addr as usize + 4],
        ))
    }

    fn load8(&mut self, addr: &u64) -> Result<u64, Error> {
        let addr = *addr;
        check_memory_inited(self, addr, 1)?;
        Ok(u64::from(self.as_ref().memory[addr as usize]))
    }

    fn load16(&mut self, addr: &u64) -> Result<u64, Error> {
        let addr = *addr;
        check_memory_inited(self, addr, 2)?;
        Ok(u64::from(LittleEndian::read_u16(
            &self.as_ref().memory[addr as usize..addr as usize + 2],
        )))
    }

    fn load32(&mut self, addr: &u64) -> Result<u64, Error> {
        let addr = *addr;
        check_memory_inited(self, addr, 4)?;
        Ok(u64::from(LittleEndian::read_u32(
            &self.as_ref().memory[addr as usize..addr as usize + 4],
        )))
    }

    fn load64(&mut self, addr: &u64) -> Result<u64, Error> {
        let addr = *addr;
        check_memory_inited(self, addr, 8)?;
        Ok(LittleEndian::read_u64(
            &self.as_ref().memory[addr as usize..addr as usize + 8],
        ))
    }

    fn store8(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        let addr = *addr;
        check_memory_writable(self, addr, 1)?;
        self.as_mut().memory[addr as usize] = (*value) as u8;
        Ok(())
    }

    fn store16(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        let addr = *addr;
        check_memory_writable(self, addr, 2)?;
        LittleEndian::write_u16(
            &mut self.as_mut().memory[addr as usize..(addr + 2) as usize],
            *value as u16,
        );
        Ok(())
    }

    fn store32(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        let addr = *addr;
        check_memory_writable(self, addr, 4)?;
        LittleEndian::write_u32(
            &mut self.as_mut().memory[addr as usize..(addr + 4) as usize],
            *value as u32,
        );
        Ok(())
    }

    fn store64(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        let addr = *addr;
        check_memory_writable(self, addr, 8)?;
        LittleEndian::write_u64(
            &mut self.as_mut().memory[addr as usize..(addr + 8) as usize],
            *value,
        );
        Ok(())
    }

    fn lr(&self) -> &Self::REG {
        &self.as_ref().load_reservation_address
    }

    fn set_lr(&mut self, value: &Self::REG) {
        self.as_mut().load_reservation_address = *value;
    }
}

impl<R> SupportMachine for R
where
    R: AsmCoreMachineRevealer,
{
    fn new_with_memory(isa: u8, version: u32, max_cycles: u64, memory_size: usize) -> R {
        R::new(isa, version, max_cycles, memory_size)
    }

    fn cycles(&self) -> u64 {
        self.as_ref().cycles
    }

    fn set_cycles(&mut self, cycles: u64) {
        self.as_mut().cycles = cycles;
    }

    fn max_cycles(&self) -> u64 {
        self.as_ref().max_cycles
    }

    fn set_max_cycles(&mut self, max_cycles: u64) {
        self.as_mut().max_cycles = max_cycles;
    }

    fn reset(&mut self, max_cycles: u64) {
        let m = self.as_mut();

        m.registers = [0; RISCV_GENERAL_REGISTER_NUMBER];
        m.pc = 0;
        m.flags = [0; RISCV_PAGES];
        for i in 0..TRACE_SIZE {
            m.traces[i] = Trace::default();
        }
        m.frames = [0; MEMORY_FRAMES];
        m.cycles = 0;
        m.max_cycles = max_cycles;
        m.reset_signal = 1;
        m.load_reservation_address = u64::MAX;
    }

    fn reset_signal(&mut self) -> bool {
        let ret = self.as_ref().reset_signal != 0;
        self.as_mut().reset_signal = 0;
        ret
    }

    fn running(&self) -> bool {
        self.as_ref().running == 1
    }

    fn set_running(&mut self, running: bool) {
        self.as_mut().running = if running { 1 } else { 0 }
    }

    #[cfg(feature = "pprof")]
    fn code(&self) -> &Bytes {
        unreachable!()
    }
}

extern "C" {
    pub fn ckb_vm_x64_execute(m: *mut AsmCoreMachine, s: *mut u8) -> c_uchar;
    // We are keeping this as a function here, but at the bottom level this really
    // just points to an array of assembly label offsets for each opcode.
    pub fn ckb_vm_asm_labels();
}

pub struct AsmMachine<R = Box<AsmCoreMachine>> {
    pub machine: DefaultMachine<R>,
}

impl<R> DefaultMachineRunner for AsmMachine<R>
where
    R: AsmCoreMachineRevealer,
{
    type Inner = R;

    fn new(machine: DefaultMachine<R>) -> Self {
        Self { machine }
    }

    fn machine(&self) -> &DefaultMachine<R> {
        &self.machine
    }

    fn machine_mut(&mut self) -> &mut DefaultMachine<R> {
        &mut self.machine
    }

    fn run(&mut self) -> Result<i8, Error> {
        if self.machine.isa() & ISA_MOP != 0 && self.machine.version() == VERSION0 {
            return Err(Error::InvalidVersion);
        }
        let mut decoder = build_decoder::<u64>(self.machine.isa(), self.machine.version());
        self.machine.set_running(true);
        while self.machine.running() {
            if self.machine.reset_signal() {
                decoder.reset_instructions_cache();
            }
            let result = unsafe {
                ckb_vm_x64_execute(
                    &mut **self.machine.inner_mut().as_mut(),
                    self.machine.pause.get_raw_ptr(),
                )
            };
            match result {
                RET_DECODE_TRACE => {
                    let pc = *self.machine.pc();
                    let slot = calculate_slot(pc);
                    let mut trace = Trace::default();
                    let mut current_pc = pc;
                    let mut i = 0;
                    while i < TRACE_ITEM_LENGTH {
                        let instruction = decoder.decode(self.machine.memory_mut(), current_pc)?;
                        let end_instruction = is_basic_block_end_instruction(instruction);
                        current_pc += u64::from(instruction_length(instruction));
                        trace.instructions[i] = instruction;
                        trace.cycles += self.machine.instruction_cycle_func()(instruction);
                        let opcode = extract_opcode(instruction);
                        // Here we are calculating the absolute address used in direct threading
                        // from label offsets.
                        trace.thread[i] = unsafe {
                            u64::from(
                                *(ckb_vm_asm_labels as *const u32).offset(opcode as u8 as isize),
                            ) + (ckb_vm_asm_labels as *const u32 as u64)
                        };
                        i += 1;
                        if end_instruction {
                            break;
                        }
                    }
                    trace.instructions[i] = blank_instruction(OP_CUSTOM_TRACE_END);
                    trace.thread[i] = unsafe {
                        u64::from(
                            *(ckb_vm_asm_labels as *const u32).offset(OP_CUSTOM_TRACE_END as isize),
                        ) + (ckb_vm_asm_labels as *const u32 as u64)
                    };
                    trace.address = pc;
                    trace.length = (current_pc - pc) as u8;
                    self.machine.inner_mut().as_mut().traces[slot] = trace;
                }
                RET_ECALL => self.machine.ecall()?,
                RET_EBREAK => self.machine.ebreak()?,
                RET_DYNAMIC_JUMP => (),
                RET_MAX_CYCLES_EXCEEDED => return Err(Error::CyclesExceeded),
                RET_CYCLES_OVERFLOW => return Err(Error::CyclesOverflow),
                RET_OUT_OF_BOUND => return Err(Error::MemOutOfBound),
                RET_INVALID_PERMISSION => return Err(Error::MemWriteOnExecutablePage),
                RET_SLOWPATH => {
                    let pc = *self.machine.pc() - 4;
                    let instruction = decoder.decode(self.machine.memory_mut(), pc)?;
                    execute_instruction(instruction, &mut self.machine)?;
                }
                RET_PAUSE => {
                    self.machine.pause.free();
                    return Err(Error::Pause);
                }
                _ => return Err(Error::Asm(result)),
            }
        }
        Ok(self.machine.exit_code())
    }
}

impl<R> AsmMachine<R>
where
    R: AsmCoreMachineRevealer,
{
    pub fn set_max_cycles(&mut self, cycles: u64) {
        self.machine.inner.as_mut().max_cycles = cycles;
    }

    pub fn load_program(
        &mut self,
        program: &Bytes,
        args: impl ExactSizeIterator<Item = Result<Bytes, Error>>,
    ) -> Result<u64, Error> {
        self.machine.load_program(program, args)
    }

    pub fn load_program_with_metadata(
        &mut self,
        program: &Bytes,
        metadata: &ProgramMetadata,
        args: impl ExactSizeIterator<Item = Result<Bytes, Error>>,
    ) -> Result<u64, Error> {
        self.machine
            .load_program_with_metadata(program, metadata, args)
    }

    pub fn step(&mut self, decoder: &mut Decoder) -> Result<(), Error> {
        // Decode only one instruction into a trace
        let pc = *self.machine.pc();
        let slot = calculate_slot(pc);
        let mut trace = Trace::default();
        let instruction = decoder.decode(self.machine.memory_mut(), pc)?;
        let len = instruction_length(instruction) as u8;
        trace.instructions[0] = instruction;
        trace.cycles += self.machine.instruction_cycle_func()(instruction);
        let opcode = extract_opcode(instruction);
        trace.thread[0] = unsafe {
            u64::from(*(ckb_vm_asm_labels as *const u32).offset(opcode as isize))
                + (ckb_vm_asm_labels as *const u32 as u64)
        };
        trace.instructions[1] = blank_instruction(OP_CUSTOM_TRACE_END);
        trace.thread[1] = unsafe {
            u64::from(*(ckb_vm_asm_labels as *const u32).offset(OP_CUSTOM_TRACE_END as isize))
                + (ckb_vm_asm_labels as *const u32 as u64)
        };
        trace.address = pc;
        trace.length = len;
        self.machine.inner_mut().as_mut().traces[slot] = trace;

        let result = unsafe {
            ckb_vm_x64_execute(
                &mut (**self.machine.inner_mut().as_mut()),
                self.machine.pause.get_raw_ptr(),
            )
        };
        match result {
            RET_DECODE_TRACE => (),
            RET_ECALL => self.machine.ecall()?,
            RET_EBREAK => self.machine.ebreak()?,
            RET_MAX_CYCLES_EXCEEDED => return Err(Error::CyclesExceeded),
            RET_OUT_OF_BOUND => return Err(Error::MemOutOfBound),
            RET_INVALID_PERMISSION => return Err(Error::MemWriteOnExecutablePage),
            RET_SLOWPATH => {
                let pc = *self.machine.pc() - 4;
                let instruction = decoder.decode(self.machine.memory_mut(), pc)?;
                execute_instruction(instruction, &mut self.machine)?;
            }
            _ => return Err(Error::Asm(result)),
        }
        self.machine.inner_mut().as_mut().traces[slot] = Trace::default();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asm_constant_rules() {
        assert!(TRACE_ITEM_LENGTH * 4 < 256);
    }
}
