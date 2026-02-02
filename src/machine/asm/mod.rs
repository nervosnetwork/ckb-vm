pub mod traces;

use byteorder::{ByteOrder, LittleEndian};
use bytes::Bytes;
pub use ckb_vm_definitions::asm::AsmCoreMachine;
use ckb_vm_definitions::{
    ISA_MOP, MEMORY_FRAME_PAGE_SHIFTS, MEMORY_FRAMESIZE, RISCV_GENERAL_REGISTER_NUMBER,
    RISCV_PAGE_SHIFTS,
    asm::{
        FixedTrace, InvokeData, RET_CYCLES_OVERFLOW, RET_DECODE_TRACE, RET_DYNAMIC_JUMP,
        RET_EBREAK, RET_ECALL, RET_INVALID_PERMISSION, RET_MAX_CYCLES_EXCEEDED, RET_OUT_OF_BOUND,
        RET_PAUSE, RET_SLOWPATH,
    },
};
use std::alloc::{Layout, alloc, alloc_zeroed};
use std::mem::MaybeUninit;
use std::os::raw::c_uchar;

use crate::{
    CoreMachine, DefaultMachine, DefaultMachineRunner, Error, MEMORY_FRAME_SHIFTS, Machine, Memory,
    RISCV_PAGESIZE, SupportMachine,
    elf::ProgramMetadata,
    error::OutOfBoundKind,
    instructions::execute_instruction,
    machine::{
        AbstractDefaultMachineBuilder, VERSION0,
        asm::traces::{SimpleFixedTraceDecoder, TraceDecoder, decode_fixed_trace},
    },
    memory::{
        FLAG_DIRTY, FLAG_EXECUTABLE, FLAG_FREEZED, FLAG_WRITABLE, FLAG_WXORX_BIT,
        check_no_overflow, fill_page_data, get_page_indices, memset, round_page_down,
        round_page_up,
    },
    rng,
};

pub trait AsmCoreMachineRevealer: AsRef<AsmCoreMachine> + AsMut<AsmCoreMachine> {
    fn new(isa: u8, version: u32, max_cycles: u64, memory_size: usize) -> Self;
}

impl AsmCoreMachineRevealer for AsmCoreMachine {
    fn new(isa: u8, version: u32, max_cycles: u64, memory_size: usize) -> Self {
        assert_ne!(memory_size, 0);
        assert_eq!(memory_size % RISCV_PAGESIZE, 0);
        assert_eq!(memory_size % (1 << MEMORY_FRAME_SHIFTS), 0);

        let mut machine: AsmCoreMachine = unsafe { MaybeUninit::zeroed().assume_init() };

        machine.max_cycles = max_cycles;
        if cfg!(feature = "enable-chaos-mode-by-default") {
            machine.chaos_mode = 1;
        }
        machine.load_reservation_address = u64::MAX;
        machine.version = version;
        machine.isa = isa;

        machine.memory_size = memory_size as u64;
        machine.frames_size = (memory_size / MEMORY_FRAMESIZE) as u64;
        machine.flags_size = (memory_size / RISCV_PAGESIZE) as u64;

        machine.last_read_frame = u64::MAX;
        machine.last_write_page = u64::MAX;

        let memory_layout = Layout::array::<u8>(machine.memory_size as usize).unwrap();
        machine.memory_ptr = unsafe { alloc(memory_layout) } as u64;
        let flags_layout = Layout::array::<u8>(machine.flags_size as usize).unwrap();
        machine.flags_ptr = unsafe { alloc_zeroed(flags_layout) } as u64;
        let frames_layout = Layout::array::<u8>(machine.frames_size as usize).unwrap();
        machine.frames_ptr = unsafe { alloc_zeroed(frames_layout) } as u64;

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
#[unsafe(no_mangle)]
pub extern "C" fn inited_memory(frame_index: u64, machine: &mut AsmCoreMachine) {
    let addr_from = (frame_index << MEMORY_FRAME_SHIFTS) as usize;
    let is_chaos_mode = machine.chaos_mode != 0;
    let chaos_seed: u64 = machine.chaos_seed.into();

    let slice = cast_ptr_to_slice_mut(
        machine,
        machine.memory_ptr,
        addr_from,
        1 << MEMORY_FRAME_SHIFTS,
    );
    if is_chaos_mode {
        machine.chaos_seed = rng::fill(chaos_seed, slice) as u32;
    } else {
        memset(slice, 0);
    }
}

fn check_memory<R: AsmCoreMachineRevealer>(machine: &mut R, page: u64) {
    let frame_index = page >> MEMORY_FRAME_PAGE_SHIFTS;
    unsafe {
        let frames = machine.as_mut().frames_ptr as *mut u8;
        let frame_addr = frames.add(frame_index as usize);
        let frame_flag = frame_addr.read();
        if frame_flag == 0 {
            inited_memory(frame_index, machine.as_mut());
            frame_addr.write(0x01);
        }
    }
}

fn check_permission<M: Memory>(memory: &mut M, page: u64, flag: u8) -> Result<(), Error> {
    let page_flag = memory.fetch_flag(page)?;
    if (page_flag & FLAG_WXORX_BIT) != (flag & FLAG_WXORX_BIT) {
        return Err(Error::MemWriteOnExecutablePage(page));
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
        return Err(Error::MemOutOfBound(addr, OutOfBoundKind::Memory));
    }
    check_permission(machine, page, FLAG_WRITABLE)?;
    check_memory(machine, page);
    machine.set_flag(page, FLAG_DIRTY)?;

    // check next page if necessary
    let page_offset = addr as usize % RISCV_PAGESIZE;
    if page_offset + size > RISCV_PAGESIZE {
        let page = page + 1;
        if page as usize >= machine.memory_pages() {
            return Err(Error::MemOutOfBound(
                addr.wrapping_add(size as u64),
                OutOfBoundKind::Memory,
            ));
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
        return Err(Error::MemOutOfBound(addr, OutOfBoundKind::Memory));
    }
    check_permission(machine, page, FLAG_EXECUTABLE)?;
    check_memory(machine, page);

    // check next page if necessary
    let page_offset = addr as usize % RISCV_PAGESIZE;
    if page_offset + size > RISCV_PAGESIZE {
        let page = page + 1;
        if page as usize >= machine.memory_pages() {
            return Err(Error::MemOutOfBound(
                addr.wrapping_add(size as u64),
                OutOfBoundKind::Memory,
            ));
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
        return Err(Error::MemOutOfBound(addr, OutOfBoundKind::Memory));
    }
    check_memory(machine, page);

    // check next page if necessary
    let page_offset = addr as usize % RISCV_PAGESIZE;
    if page_offset + size > RISCV_PAGESIZE {
        let page = page + 1;
        if page as usize >= machine.memory_pages() {
            return Err(Error::MemOutOfBound(
                addr.wrapping_add(size as u64),
                OutOfBoundKind::Memory,
            ));
        } else {
            check_memory(machine, page);
        }
    }
    Ok(())
}

// A newtype supporting fast store_byte / store_bytes without memory
// permission checking
struct FastMemory<'a>(&'a mut AsmCoreMachine);

impl FastMemory<'_> {
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
            let slice = cast_ptr_to_slice_mut(self.0, self.0.frames_ptr, frame_index as usize, 1);
            slice[0] = 1;
            self.0.set_flag(page, FLAG_DIRTY)?;
        }
        Ok(())
    }
}

impl Memory for FastMemory<'_> {
    type REG = u64;

    fn new(_memory_size: usize) -> Self {
        unreachable!()
    }

    fn store_bytes(&mut self, addr: u64, value: &[u8]) -> Result<(), Error> {
        if value.is_empty() {
            return Ok(());
        }
        self.prepare_memory(addr, value.len() as u64)?;
        let slice = cast_ptr_to_slice_mut(self.0, self.0.memory_ptr, addr as usize, value.len());
        slice.copy_from_slice(value);
        Ok(())
    }

    fn store_byte(&mut self, addr: u64, size: u64, value: u8) -> Result<(), Error> {
        if size == 0 {
            return Ok(());
        }
        self.prepare_memory(addr, size)?;
        let slice = cast_ptr_to_slice_mut(self.0, self.0.memory_ptr, addr as usize, size as usize);
        memset(slice, value);
        Ok(())
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

impl<R> Memory for R
where
    R: AsmCoreMachineRevealer,
{
    type REG = u64;

    fn new(_memory_size: usize) -> Self {
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
        if round_page_down(addr) != addr {
            return Err(Error::MemPageUnalignedAccess(addr));
        }
        if round_page_up(size) != size {
            return Err(Error::MemPageUnalignedAccess(addr.wrapping_add(size)));
        }

        if addr > self.memory_size() as u64 {
            return Err(Error::MemOutOfBound(addr, OutOfBoundKind::Memory));
        }
        if size > self.memory_size() as u64 || addr + size > self.memory_size() as u64 {
            return Err(Error::MemOutOfBound(
                addr.wrapping_add(size),
                OutOfBoundKind::Memory,
            ));
        }
        if offset_from_addr > size {
            return Err(Error::MemOutOfBound(
                offset_from_addr,
                OutOfBoundKind::ExternalData,
            ));
        }

        // We benchmarked the code piece here, using while loop this way is
        // actually faster than a for..in solution. The difference is roughly
        // 3% so we are keeping this version.
        let mut current_addr = addr;
        while current_addr < addr + size {
            let page = current_addr / RISCV_PAGESIZE as u64;
            if self.fetch_flag(page)? & FLAG_FREEZED != 0 {
                return Err(Error::MemWriteOnFreezedPage(page));
            }
            current_addr += RISCV_PAGESIZE as u64;
        }
        fill_page_data(
            &mut FastMemory(self.as_mut()),
            addr,
            size,
            source,
            offset_from_addr,
        )?;
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
            let slice = cast_ptr_to_slice(self, self.as_ref().flags_ptr, page as usize, 1);
            Ok(slice[0])
        } else {
            Err(Error::MemOutOfBound(
                page << RISCV_PAGE_SHIFTS,
                OutOfBoundKind::Memory,
            ))
        }
    }

    fn set_flag(&mut self, page: u64, flag: u8) -> Result<(), Error> {
        if page < self.memory_pages() as u64 {
            let slice = cast_ptr_to_slice_mut(self, self.as_ref().flags_ptr, page as usize, 1);
            slice[0] |= flag;
            // Clear last write page cache
            self.as_mut().last_write_page = u64::MAX;
            Ok(())
        } else {
            Err(Error::MemOutOfBound(
                page << RISCV_PAGE_SHIFTS,
                OutOfBoundKind::Memory,
            ))
        }
    }

    fn clear_flag(&mut self, page: u64, flag: u8) -> Result<(), Error> {
        if page < self.memory_pages() as u64 {
            let slice = cast_ptr_to_slice_mut(self, self.as_ref().flags_ptr, page as usize, 1);
            slice[0] &= !flag;
            // Clear last write page cache
            self.as_mut().last_write_page = u64::MAX;
            Ok(())
        } else {
            Err(Error::MemOutOfBound(
                page << RISCV_PAGE_SHIFTS,
                OutOfBoundKind::Memory,
            ))
        }
    }

    fn memory_size(&self) -> usize {
        self.as_ref().memory_size as usize
    }

    fn store_bytes(&mut self, addr: u64, value: &[u8]) -> Result<(), Error> {
        if value.is_empty() {
            return Ok(());
        }
        check_no_overflow(addr, value.len() as u64, self.as_ref().memory_size)?;
        let page_indices = get_page_indices(addr, value.len() as u64);
        for page in page_indices.0..=page_indices.1 {
            check_permission(self, page, FLAG_WRITABLE)?;
            check_memory(self, page);
            self.set_flag(page, FLAG_DIRTY)?;
        }
        let slice =
            cast_ptr_to_slice_mut(self, self.as_ref().memory_ptr, addr as usize, value.len());
        slice.copy_from_slice(value);
        Ok(())
    }

    fn store_byte(&mut self, addr: u64, size: u64, value: u8) -> Result<(), Error> {
        if size == 0 {
            return Ok(());
        }
        check_no_overflow(addr, size, self.as_ref().memory_size)?;
        let page_indices = get_page_indices(addr, size);
        for page in page_indices.0..=page_indices.1 {
            check_permission(self, page, FLAG_WRITABLE)?;
            check_memory(self, page);
            self.set_flag(page, FLAG_DIRTY)?;
        }
        let slice =
            cast_ptr_to_slice_mut(self, self.as_ref().memory_ptr, addr as usize, size as usize);
        memset(slice, value);
        Ok(())
    }

    fn load_bytes(&mut self, addr: u64, size: u64) -> Result<Bytes, Error> {
        if size == 0 {
            return Ok(Bytes::new());
        }
        check_no_overflow(addr, size, self.as_ref().memory_size)?;
        let page_indices = get_page_indices(addr, size);
        for page in page_indices.0..=page_indices.1 {
            check_memory(self, page);
        }
        let slice = unsafe {
            let memory = self.as_ref().memory_ptr as *const u8;
            let memory_from = memory.add(addr as usize);
            std::slice::from_raw_parts(memory_from, size as usize)
        };
        Ok(Bytes::from(slice))
    }

    fn execute_load16(&mut self, addr: u64) -> Result<u16, Error> {
        check_memory_executable(self, addr, 2)?;
        let slice = cast_ptr_to_slice(self, self.as_ref().memory_ptr, addr as usize, 2);
        Ok(LittleEndian::read_u16(slice))
    }

    fn execute_load32(&mut self, addr: u64) -> Result<u32, Error> {
        check_memory_executable(self, addr, 4)?;
        let slice = cast_ptr_to_slice(self, self.as_ref().memory_ptr, addr as usize, 4);
        Ok(LittleEndian::read_u32(slice))
    }

    fn load8(&mut self, addr: &u64) -> Result<u64, Error> {
        let addr = *addr;
        check_memory_inited(self, addr, 1)?;
        let slice = cast_ptr_to_slice(self, self.as_ref().memory_ptr, addr as usize, 1);
        Ok(u64::from(slice[0]))
    }

    fn load16(&mut self, addr: &u64) -> Result<u64, Error> {
        let addr = *addr;
        check_memory_inited(self, addr, 2)?;
        let slice = cast_ptr_to_slice(self, self.as_ref().memory_ptr, addr as usize, 2);
        Ok(u64::from(LittleEndian::read_u16(slice)))
    }

    fn load32(&mut self, addr: &u64) -> Result<u64, Error> {
        let addr = *addr;
        check_memory_inited(self, addr, 4)?;
        let slice = cast_ptr_to_slice(self, self.as_ref().memory_ptr, addr as usize, 4);
        Ok(u64::from(LittleEndian::read_u32(slice)))
    }

    fn load64(&mut self, addr: &u64) -> Result<u64, Error> {
        let addr = *addr;
        check_memory_inited(self, addr, 8)?;
        let slice = cast_ptr_to_slice(self, self.as_ref().memory_ptr, addr as usize, 8);
        Ok(LittleEndian::read_u64(slice))
    }

    fn store8(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        let addr = *addr;
        check_memory_writable(self, addr, 1)?;
        let slice = cast_ptr_to_slice_mut(self, self.as_ref().memory_ptr, addr as usize, 1);
        slice[0] = *value as u8;
        Ok(())
    }

    fn store16(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        let addr = *addr;
        check_memory_writable(self, addr, 2)?;
        let slice = cast_ptr_to_slice_mut(self, self.as_ref().memory_ptr, addr as usize, 2);
        LittleEndian::write_u16(slice, *value as u16);
        Ok(())
    }

    fn store32(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        let addr = *addr;
        check_memory_writable(self, addr, 4)?;
        let slice = cast_ptr_to_slice_mut(self, self.as_ref().memory_ptr, addr as usize, 4);
        LittleEndian::write_u32(slice, *value as u32);
        Ok(())
    }

    fn store64(&mut self, addr: &u64, value: &u64) -> Result<(), Error> {
        let addr = *addr;
        check_memory_writable(self, addr, 8)?;
        let slice = cast_ptr_to_slice_mut(self, self.as_ref().memory_ptr, addr as usize, 8);
        LittleEndian::write_u64(slice, *value as u64);
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

    fn reset(&mut self, max_cycles: u64) -> Result<(), Error> {
        {
            let m = self.as_mut();

            m.registers = [0; RISCV_GENERAL_REGISTER_NUMBER];
            m.pc = 0;
            m.cycles = 0;
            m.max_cycles = max_cycles;
            m.reset_signal = 1;
            m.load_reservation_address = u64::MAX;
            m.last_read_frame = u64::MAX;
            m.last_write_page = u64::MAX;
        }

        // Reset memory
        let flags_ptr = self.as_ref().flags_ptr;
        let flags_size = self.as_ref().flags_size as usize;
        let slice = cast_ptr_to_slice_mut(self, flags_ptr, 0, flags_size);
        memset(slice, 0);

        let frames_ptr = self.as_ref().frames_ptr;
        let frames_size = self.as_ref().frames_size as usize;
        let slice = cast_ptr_to_slice_mut(self, frames_ptr, 0, frames_size);
        memset(slice, 0);

        Ok(())
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

unsafe extern "C" {
    pub fn ckb_vm_x64_execute(m: *mut AsmCoreMachine, d: *const InvokeData) -> c_uchar;
    // We are keeping this as a function here, but at the bottom level this really
    // just points to an array of assembly label offsets for each opcode.
    pub fn ckb_vm_asm_labels();
}

/// This builder only works with assembly VMs
pub type AsmDefaultMachineBuilder =
    AbstractDefaultMachineBuilder<AsmCoreMachine, SimpleFixedTraceDecoder>;
pub type AsmDefaultMachine = DefaultMachine<AsmCoreMachine, SimpleFixedTraceDecoder>;

pub type AsmMachine = AbstractAsmMachine<AsmCoreMachine, SimpleFixedTraceDecoder>;

pub struct AbstractAsmMachine<R: AsmCoreMachineRevealer, D: TraceDecoder> {
    pub machine: DefaultMachine<R, D>,
}

impl<R: AsmCoreMachineRevealer, D: TraceDecoder> DefaultMachineRunner for AbstractAsmMachine<R, D> {
    type Inner = R;
    type Decoder = D;

    fn new(machine: DefaultMachine<R, D>) -> Self {
        Self { machine }
    }

    fn machine(&self) -> &DefaultMachine<R, D> {
        &self.machine
    }

    fn machine_mut(&mut self) -> &mut DefaultMachine<R, D> {
        &mut self.machine
    }

    fn run_with_decoder(&mut self, decoder: &mut D) -> Result<i8, Error> {
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
                ckb_vm_x64_execute(&mut *self.machine.inner_mut().as_mut(), &data as *const _)
            };
            match result {
                RET_DECODE_TRACE => decoder.prepare_traces(&mut self.machine)?,
                RET_ECALL => self.machine.ecall()?,
                RET_EBREAK => self.machine.ebreak()?,
                RET_DYNAMIC_JUMP => (),
                RET_MAX_CYCLES_EXCEEDED => return Err(Error::CyclesExceeded),
                RET_CYCLES_OVERFLOW => return Err(Error::CyclesOverflow),
                RET_OUT_OF_BOUND => {
                    return Err(Error::MemOutOfBound(
                        self.machine.inner.as_ref().error_arg0,
                        OutOfBoundKind::Memory,
                    ));
                }
                RET_INVALID_PERMISSION => {
                    return Err(Error::MemWriteOnExecutablePage(
                        self.machine.inner.as_ref().error_arg0,
                    ));
                }
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

impl<R: AsmCoreMachineRevealer, D: TraceDecoder> AbstractAsmMachine<R, D> {
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

    pub fn step(&mut self, decoder: &mut D) -> Result<(), Error> {
        // Decode only one instruction into a trace
        let (trace, _) = decode_fixed_trace(decoder, &mut self.machine, Some(1))?;

        let result = unsafe {
            let data = InvokeData {
                pause: self.machine.pause.get_raw_ptr(),
                fixed_traces: &trace as *const FixedTrace,
                fixed_trace_mask: 0,
            };
            ckb_vm_x64_execute(&mut *self.machine.inner_mut().as_mut(), &data as *const _)
        };
        match result {
            RET_DECODE_TRACE => (),
            RET_ECALL => self.machine.ecall()?,
            RET_EBREAK => self.machine.ebreak()?,
            RET_MAX_CYCLES_EXCEEDED => return Err(Error::CyclesExceeded),
            RET_OUT_OF_BOUND => {
                return Err(Error::MemOutOfBound(
                    self.machine.inner.as_ref().error_arg0,
                    OutOfBoundKind::Memory,
                ));
            }
            RET_INVALID_PERMISSION => {
                return Err(Error::MemWriteOnExecutablePage(
                    self.machine.inner.as_ref().error_arg0,
                ));
            }
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

// Casts a raw pointer with an offset and size to a byte slice.
// We need machine here for the lifetime.
fn cast_ptr_to_slice<R>(_machine: &R, ptr: u64, offset: usize, size: usize) -> &[u8] {
    unsafe {
        let ptr = ptr as *const u8;
        let ptr = ptr.add(offset);
        std::slice::from_raw_parts(ptr, size)
    }
}

// Provides similar functionality to `cast_ptr_to_slice` but returns mut slice.
fn cast_ptr_to_slice_mut<R>(_machine: &mut R, ptr: u64, offset: usize, size: usize) -> &mut [u8] {
    unsafe {
        let ptr = ptr as *mut u8;
        let ptr = ptr.add(offset);
        std::slice::from_raw_parts_mut(ptr, size)
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
