pub mod traces;

use byteorder::{ByteOrder, LittleEndian};
use bytes::Bytes;
pub use ckb_vm_definitions::asm::AsmCoreMachine;
use ckb_vm_definitions::{
    asm::{
        FixedTrace, InvokeData, RET_CYCLES_OVERFLOW, RET_DECODE_TRACE, RET_DYNAMIC_JUMP,
        RET_EBREAK, RET_ECALL, RET_INVALID_PERMISSION, RET_MAX_CYCLES_EXCEEDED, RET_OUT_OF_BOUND,
        RET_PAUSE, RET_SLOWPATH,
    },
    ISA_MOP, MEMORY_FRAME_PAGE_SHIFTS, RISCV_GENERAL_REGISTER_NUMBER, RISCV_PAGE_SHIFTS,
};
use rand::{prelude::RngCore, SeedableRng};
use std::os::raw::c_uchar;

use crate::{
    decoder::{build_decoder, InstDecoder},
    elf::ProgramMetadata,
    instructions::execute_instruction,
    machine::{
        asm::traces::{decode_fixed_trace, SimpleFixedTraceDecoder, TraceDecoder},
        VERSION0,
    },
    memory::{
        check_no_overflow, fill_page_data, get_page_indices, memset, round_page_down,
        round_page_up, FLAG_DIRTY, FLAG_EXECUTABLE, FLAG_FREEZED, FLAG_WRITABLE, FLAG_WXORX_BIT,
    },
    CoreMachine, DefaultMachine, Error, Machine, Memory, SupportMachine, MEMORY_FRAME_SHIFTS,
    RISCV_PAGESIZE,
};

impl CoreMachine for Box<AsmCoreMachine> {
    type REG = u64;
    type MEM = Self;

    fn pc(&self) -> &Self::REG {
        &self.pc
    }

    fn update_pc(&mut self, pc: Self::REG) {
        self.next_pc = pc;
    }

    fn commit_pc(&mut self) {
        self.pc = self.next_pc;
    }

    fn memory(&self) -> &Self {
        self
    }

    fn memory_mut(&mut self) -> &mut Self {
        self
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

// This function is exported for asm and aot machine.
// Note that the parameter `machine` is after parameter `frame_index`. Generally
// speaking, put `machine` in the first parameter is more human readable,
// but consider that in the asm machine, `frame_index` is stored in `rdi` and `machine`
// is stored in `rsi`, there is no need to exchange the values in the two registers
// in this way.
#[no_mangle]
pub extern "C" fn inited_memory(frame_index: u64, machine: &mut AsmCoreMachine) {
    let addr_from = (frame_index << MEMORY_FRAME_SHIFTS) as usize;
    let slice =
        machine.cast_ptr_to_slice_mut(machine.memory_ptr, addr_from, 1 << MEMORY_FRAME_SHIFTS);
    if machine.chaos_mode != 0 {
        let mut gen = rand::rngs::StdRng::seed_from_u64(machine.chaos_seed.into());
        gen.fill_bytes(slice);
        machine.chaos_seed = gen.next_u32();
    } else {
        memset(slice, 0);
    }
}

fn check_memory(machine: &mut AsmCoreMachine, page: u64) {
    let frame_index = page >> MEMORY_FRAME_PAGE_SHIFTS;
    unsafe {
        let frames = machine.frames_ptr as *mut u8;
        let frame_addr = frames.add(frame_index as usize);
        let frame_flag = frame_addr.read();
        if frame_flag == 0 {
            inited_memory(frame_index, machine);
            frame_addr.write(0x01);
        }
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
fn check_memory_writable(
    machine: &mut Box<AsmCoreMachine>,
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
fn check_memory_executable(
    machine: &mut Box<AsmCoreMachine>,
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
fn check_memory_inited(
    machine: &mut Box<AsmCoreMachine>,
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

// A newtype supporting fast store_byte / store_bytes without memory
// permission checking
struct FastMemory<'a>(&'a mut Box<AsmCoreMachine>);

impl<'a> FastMemory<'a> {
    fn prepare_memory(&mut self, addr: u64, size: u64) -> Result<(), Error> {
        check_no_overflow(addr, size, self.0.memory_size)?;
        let frame_start = addr >> MEMORY_FRAME_SHIFTS << MEMORY_FRAME_SHIFTS;
        // There is some memory space between the start of the first memory
        // frame touched, and the starting address of memory to be written. We
        // will need to initialize the last memory frame.
        if frame_start < addr {
            check_memory(self.0, addr >> RISCV_PAGE_SHIFTS);
        }
        let end = addr.wrapping_add(size);
        if end > 0 {
            let aligned_end = round_page_down(end);
            // Note that end is exclusive
            let frame_next_start = (((end - 1) >> MEMORY_FRAME_SHIFTS) + 1) << MEMORY_FRAME_SHIFTS;
            // There is some memory space between the ending address of memory to be
            // written, and the end of the last memory frame touched, we will need to
            // initialize the last memory frame.
            if (aligned_end + RISCV_PAGESIZE as u64) < frame_next_start {
                check_memory(self.0, aligned_end >> RISCV_PAGE_SHIFTS);
            }
        }
        let page_indices = get_page_indices(addr, size);
        for page in page_indices.0..=page_indices.1 {
            let frame_index = page >> MEMORY_FRAME_PAGE_SHIFTS;
            let slice = self
                .0
                .cast_ptr_to_slice_mut(self.0.frames_ptr, frame_index as usize, 1);
            slice[0] = 1;
            self.0.set_flag(page, FLAG_DIRTY)?;
        }
        Ok(())
    }
}

impl<'a> Memory for FastMemory<'a> {
    type REG = u64;

    fn store_bytes(&mut self, addr: u64, value: &[u8]) -> Result<(), Error> {
        if value.is_empty() {
            return Ok(());
        }
        self.prepare_memory(addr, value.len() as u64)?;
        let slice = self
            .0
            .cast_ptr_to_slice_mut(self.0.memory_ptr, addr as usize, value.len());
        slice.copy_from_slice(value);
        Ok(())
    }

    fn store_byte(&mut self, addr: u64, size: u64, value: u8) -> Result<(), Error> {
        if size == 0 {
            return Ok(());
        }
        self.prepare_memory(addr, size)?;
        let slice = self
            .0
            .cast_ptr_to_slice_mut(self.0.memory_ptr, addr as usize, size as usize);
        memset(slice, value);
        Ok(())
    }

    fn reset_memory(&mut self) -> Result<(), Error> {
        unreachable!()
    }

    fn init_pages(
        &mut self,
        _addr: u64,
        _size: u64,
        _flags: u8,
        _source: Option<Bytes>,
        _offset_from_addr: u64,
    ) -> Result<(), Error> {
        unreachable!()
    }

    fn fetch_flag(&mut self, _page: u64) -> Result<u8, Error> {
        unreachable!()
    }

    fn set_flag(&mut self, _page: u64, _flag: u8) -> Result<(), Error> {
        unreachable!()
    }

    fn clear_flag(&mut self, _page: u64, _flag: u8) -> Result<(), Error> {
        unreachable!()
    }

    fn memory_size(&self) -> usize {
        unreachable!()
    }

    fn load_bytes(&mut self, _addr: u64, _size: u64) -> Result<Bytes, Error> {
        unreachable!()
    }

    fn execute_load16(&mut self, _addr: u64) -> Result<u16, Error> {
        unreachable!()
    }

    fn execute_load32(&mut self, _addr: u64) -> Result<u32, Error> {
        unreachable!()
    }

    fn load8(&mut self, _addr: &Self::REG) -> Result<Self::REG, Error> {
        unreachable!()
    }

    fn load16(&mut self, _addr: &Self::REG) -> Result<Self::REG, Error> {
        unreachable!()
    }

    fn load32(&mut self, _addr: &Self::REG) -> Result<Self::REG, Error> {
        unreachable!()
    }

    fn load64(&mut self, _addr: &Self::REG) -> Result<Self::REG, Error> {
        unreachable!()
    }

    fn store8(&mut self, _addr: &Self::REG, _value: &Self::REG) -> Result<(), Error> {
        unreachable!()
    }

    fn store16(&mut self, _addr: &Self::REG, _value: &Self::REG) -> Result<(), Error> {
        unreachable!()
    }

    fn store32(&mut self, _addr: &Self::REG, _value: &Self::REG) -> Result<(), Error> {
        unreachable!()
    }

    fn store64(&mut self, _addr: &Self::REG, _value: &Self::REG) -> Result<(), Error> {
        unreachable!()
    }

    fn lr(&self) -> &Self::REG {
        unreachable!()
    }

    fn set_lr(&mut self, _value: &Self::REG) {
        unreachable!()
    }
}

impl Memory for Box<AsmCoreMachine> {
    type REG = u64;

    fn reset_memory(&mut self) -> Result<(), Error> {
        let slice = self.cast_ptr_to_slice_mut(self.flags_ptr, 0, self.flags_size as usize);
        memset(slice, 0);
        let slice = self.cast_ptr_to_slice_mut(self.frames_ptr, 0, self.frames_size as usize);
        memset(slice, 0);
        self.load_reservation_address = u64::MAX;
        self.last_read_frame = u64::max_value();
        self.last_write_page = u64::max_value();
        Ok(())
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
        fill_page_data(&mut FastMemory(self), addr, size, source, offset_from_addr)?;
        current_addr = addr;
        while current_addr < addr + size {
            let page = current_addr / RISCV_PAGESIZE as u64;
            self.set_flag(page, flags)?;
            current_addr += RISCV_PAGESIZE as u64;
        }
        // Clear last read/write page cache
        self.last_read_frame = u64::max_value();
        self.last_write_page = u64::max_value();
        Ok(())
    }

    fn fetch_flag(&mut self, page: u64) -> Result<u8, Error> {
        if page < self.memory_pages() as u64 {
            let slice = self.cast_ptr_to_slice(self.flags_ptr, page as usize, 1);
            Ok(slice[0])
        } else {
            Err(Error::MemOutOfBound)
        }
    }

    fn set_flag(&mut self, page: u64, flag: u8) -> Result<(), Error> {
        if page < self.memory_pages() as u64 {
            let slice = self.cast_ptr_to_slice_mut(self.flags_ptr, page as usize, 1);
            slice[0] |= flag;
            // Clear last write page cache
            self.last_write_page = u64::max_value();
            Ok(())
        } else {
            Err(Error::MemOutOfBound)
        }
    }

    fn clear_flag(&mut self, page: u64, flag: u8) -> Result<(), Error> {
        if page < self.memory_pages() as u64 {
            let slice = self.cast_ptr_to_slice_mut(self.flags_ptr, page as usize, 1);
            slice[0] &= !flag;
            // Clear last write page cache
            self.last_write_page = u64::max_value();
            Ok(())
        } else {
            Err(Error::MemOutOfBound)
        }
    }

    fn memory_size(&self) -> usize {
        self.memory_size as usize
    }

    fn store_bytes(&mut self, addr: u64, value: &[u8]) -> Result<(), Error> {
        if value.is_empty() {
            return Ok(());
        }
        check_no_overflow(addr, value.len() as u64, self.memory_size)?;
        let page_indices = get_page_indices(addr, value.len() as u64);
        for page in page_indices.0..=page_indices.1 {
            check_permission(self, page, FLAG_WRITABLE)?;
            check_memory(self, page);
            self.set_flag(page, FLAG_DIRTY)?;
        }
        let slice = self.cast_ptr_to_slice_mut(self.memory_ptr, addr as usize, value.len());
        slice.copy_from_slice(value);
        Ok(())
    }

    fn store_byte(&mut self, addr: u64, size: u64, value: u8) -> Result<(), Error> {
        if size == 0 {
            return Ok(());
        }
        check_no_overflow(addr, size, self.memory_size)?;
        let page_indices = get_page_indices(addr, size);
        for page in page_indices.0..=page_indices.1 {
            check_permission(self, page, FLAG_WRITABLE)?;
            check_memory(self, page);
            self.set_flag(page, FLAG_DIRTY)?;
        }
        let slice = self.cast_ptr_to_slice_mut(self.memory_ptr, addr as usize, size as usize);
        memset(slice, value);
        Ok(())
    }

    fn load_bytes(&mut self, addr: u64, size: u64) -> Result<Bytes, Error> {
        if size == 0 {
            return Ok(Bytes::new());
        }
        check_no_overflow(addr, size, self.memory_size)?;
        let page_indices = get_page_indices(addr, size);
        for page in page_indices.0..=page_indices.1 {
            check_memory(self, page);
        }
        let slice = unsafe {
            let memory = self.memory_ptr as *mut u8;
            let memory_from = memory.add(addr as usize);
            std::slice::from_raw_parts(memory_from, size as usize)
        };
        Ok(Bytes::from(slice))
    }

    fn execute_load16(&mut self, addr: u64) -> Result<u16, Error> {
        check_memory_executable(self, addr, 2)?;
        let slice = self.cast_ptr_to_slice(self.memory_ptr, addr as usize, 2);
        Ok(LittleEndian::read_u16(slice))
    }

    fn execute_load32(&mut self, addr: u64) -> Result<u32, Error> {
        check_memory_executable(self, addr, 4)?;
        let slice = self.cast_ptr_to_slice(self.memory_ptr, addr as usize, 4);
        Ok(LittleEndian::read_u32(slice))
    }

    fn load8(&mut self, addr: &u64) -> Result<u64, Error> {
        let addr = *addr;
        check_memory_inited(self, addr, 1)?;
        let slice = self.cast_ptr_to_slice(self.memory_ptr, addr as usize, 1);
        Ok(u64::from(slice[0]))
    }

    fn load16(&mut self, addr: &u64) -> Result<u64, Error> {
        let addr = *addr;
        check_memory_inited(self, addr, 2)?;
        let slice = self.cast_ptr_to_slice(self.memory_ptr, addr as usize, 2);
        Ok(u64::from(LittleEndian::read_u16(slice)))
    }

    fn load32(&mut self, addr: &u64) -> Result<u64, Error> {
        let addr = *addr;
        check_memory_inited(self, addr, 4)?;
        let slice = self.cast_ptr_to_slice(self.memory_ptr, addr as usize, 4);
        Ok(u64::from(LittleEndian::read_u32(slice)))
    }

    fn load64(&mut self, addr: &u64) -> Result<u64, Error> {
        let addr = *addr;
        check_memory_inited(self, addr, 8)?;
        let slice = self.cast_ptr_to_slice(self.memory_ptr, addr as usize, 8);
        Ok(LittleEndian::read_u64(slice))
    }

    fn store8(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        let addr = *addr;
        check_memory_writable(self, addr, 1)?;
        let slice = self.cast_ptr_to_slice_mut(self.memory_ptr, addr as usize, 1);
        slice[0] = *value as u8;
        Ok(())
    }

    fn store16(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        let addr = *addr;
        check_memory_writable(self, addr, 2)?;
        let slice = self.cast_ptr_to_slice_mut(self.memory_ptr, addr as usize, 2);
        LittleEndian::write_u16(slice, *value as u16);
        Ok(())
    }

    fn store32(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        let addr = *addr;
        check_memory_writable(self, addr, 4)?;
        let slice = self.cast_ptr_to_slice_mut(self.memory_ptr, addr as usize, 4);
        LittleEndian::write_u32(slice, *value as u32);
        Ok(())
    }

    fn store64(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        let addr = *addr;
        check_memory_writable(self, addr, 8)?;
        let slice = self.cast_ptr_to_slice_mut(self.memory_ptr, addr as usize, 8);
        LittleEndian::write_u64(slice, *value as u64);
        Ok(())
    }

    fn lr(&self) -> &Self::REG {
        &self.load_reservation_address
    }

    fn set_lr(&mut self, value: &Self::REG) {
        self.load_reservation_address = *value;
    }
}

impl SupportMachine for Box<AsmCoreMachine> {
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
        self.registers = [0; RISCV_GENERAL_REGISTER_NUMBER];
        self.pc = 0;
        self.cycles = 0;
        self.max_cycles = max_cycles;
        self.reset_signal = 1;
        self.reset_memory()
    }

    fn reset_signal(&mut self) -> bool {
        let ret = self.reset_signal != 0;
        self.reset_signal = 0;
        ret
    }

    fn running(&self) -> bool {
        self.running == 1
    }

    fn set_running(&mut self, running: bool) {
        self.running = if running { 1 } else { 0 }
    }

    #[cfg(feature = "pprof")]
    fn code(&self) -> &Bytes {
        unreachable!()
    }
}

extern "C" {
    pub fn ckb_vm_x64_execute(m: *mut AsmCoreMachine, d: *const InvokeData) -> c_uchar;
    // We are keeping this as a function here, but at the bottom level this really
    // just points to an array of assembly label offsets for each opcode.
    pub fn ckb_vm_asm_labels();
}

pub struct AsmMachine {
    pub machine: DefaultMachine<Box<AsmCoreMachine>>,
}

impl AsmMachine {
    pub fn new(machine: DefaultMachine<Box<AsmCoreMachine>>) -> Self {
        Self { machine }
    }

    pub fn set_max_cycles(&mut self, cycles: u64) {
        self.machine.inner.max_cycles = cycles;
    }

    pub fn load_program(&mut self, program: &Bytes, args: &[Bytes]) -> Result<u64, Error> {
        self.machine.load_program(program, args)
    }

    pub fn load_program_with_metadata(
        &mut self,
        program: &Bytes,
        metadata: &ProgramMetadata,
        args: &[Bytes],
    ) -> Result<u64, Error> {
        self.machine
            .load_program_with_metadata(program, metadata, args)
    }

    pub fn run(&mut self) -> Result<i8, Error> {
        let decoder = build_decoder::<u64>(self.machine.isa(), self.machine.version());
        let mut decoder = SimpleFixedTraceDecoder::new(decoder);
        self.run_with_decoder(&mut decoder)
    }

    pub fn run_with_decoder<D: TraceDecoder>(&mut self, decoder: &mut D) -> Result<i8, Error> {
        if self.machine.isa() & ISA_MOP != 0 && self.machine.version() == VERSION0 {
            return Err(Error::InvalidVersion);
        }
        self.machine.set_running(true);
        while self.machine.running() {
            if self.machine.reset_signal() {
                decoder.reset()?;
            }
            debug_assert!(decoder.fixed_trace_size().is_power_of_two());
            let result = unsafe {
                let data = InvokeData {
                    pause: self.machine.pause.get_raw_ptr(),
                    fixed_traces: decoder.fixed_traces(),
                    fixed_trace_mask: decoder.fixed_trace_size().wrapping_sub(1),
                };
                ckb_vm_x64_execute(&mut **self.machine.inner_mut(), &data as *const _)
            };
            match result {
                RET_DECODE_TRACE => decoder.prepare_traces(&mut self.machine)?,
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

    pub fn step<D: InstDecoder>(&mut self, decoder: &mut D) -> Result<(), Error> {
        // Decode only one instruction into a trace
        let (trace, _) = decode_fixed_trace(decoder, &mut self.machine, Some(1))?;

        let result = unsafe {
            let data = InvokeData {
                pause: self.machine.pause.get_raw_ptr(),
                fixed_traces: &trace as *const FixedTrace,
                fixed_trace_mask: 0,
            };
            ckb_vm_x64_execute(&mut **self.machine.inner_mut(), &data as *const _)
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
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use ckb_vm_definitions::asm::TRACE_ITEM_LENGTH;

    #[test]
    fn test_asm_constant_rules() {
        assert!(TRACE_ITEM_LENGTH * 4 < 256);
    }
}
