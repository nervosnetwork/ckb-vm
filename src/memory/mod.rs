use super::{
    bits::{rounddown, roundup},
    Error, Register, RISCV_PAGESIZE,
};
use bytes::Bytes;
use std::cmp::min;
use std::ptr;

pub mod flat;
pub mod sparse;
pub mod wxorx;

pub use ckb_vm_definitions::{
    memory::{FLAG_DIRTY, FLAG_EXECUTABLE, FLAG_FREEZED, FLAG_WRITABLE, FLAG_WXORX_BIT},
    MEMORY_FRAME_PAGE_SHIFTS, RISCV_MAX_MEMORY, RISCV_PAGE_SHIFTS,
};

#[inline(always)]
pub fn round_page_down(x: u64) -> u64 {
    rounddown(x, RISCV_PAGESIZE as u64)
}

#[inline(always)]
pub fn round_page_up(x: u64) -> u64 {
    roundup(x, RISCV_PAGESIZE as u64)
}

pub type Page = [u8; RISCV_PAGESIZE];

pub trait Memory {
    type REG: Register;

    fn init_pages(
        &mut self,
        addr: u64,
        size: u64,
        flags: u8,
        source: Option<Bytes>,
        offset_from_addr: u64,
    ) -> Result<(), Error>;
    fn fetch_flag(&mut self, page: u64) -> Result<u8, Error>;
    fn set_flag(&mut self, page: u64, flag: u8) -> Result<(), Error>;
    fn clear_flag(&mut self, page: u64, flag: u8) -> Result<(), Error>;

    // This is in fact just memset
    fn store_byte(&mut self, addr: u64, size: u64, value: u8) -> Result<(), Error>;
    fn store_bytes(&mut self, addr: u64, value: &[u8]) -> Result<(), Error>;
    fn execute_load16(&mut self, addr: u64) -> Result<u16, Error>;
    fn execute_load32(&mut self, addr: u64) -> Result<u32, Error>;
    fn load_bytes(&mut self, addr: u64, size: u64) -> Result<Vec<u8>, Error>;

    // Methods below are used to implement RISC-V instructions, to make JIT
    // possible, we need to use register type here so as to pass enough
    // information around.
    fn load8(&mut self, addr: &Self::REG) -> Result<Self::REG, Error>;
    fn load16(&mut self, addr: &Self::REG) -> Result<Self::REG, Error>;
    fn load32(&mut self, addr: &Self::REG) -> Result<Self::REG, Error>;
    fn load64(&mut self, addr: &Self::REG) -> Result<Self::REG, Error>;

    fn store8(&mut self, addr: &Self::REG, value: &Self::REG) -> Result<(), Error>;
    fn store16(&mut self, addr: &Self::REG, value: &Self::REG) -> Result<(), Error>;
    fn store32(&mut self, addr: &Self::REG, value: &Self::REG) -> Result<(), Error>;
    fn store64(&mut self, addr: &Self::REG, value: &Self::REG) -> Result<(), Error>;
}

#[inline(always)]
pub(crate) fn fill_page_data<M: Memory>(
    memory: &mut M,
    addr: u64,
    size: u64,
    source: Option<Bytes>,
    offset_from_addr: u64,
) -> Result<(), Error> {
    let mut written_size = 0;
    if offset_from_addr > 0 {
        let real_size = min(size, offset_from_addr);
        memory.store_byte(addr, real_size, 0)?;
        written_size += real_size;
    }
    if let Some(source) = source {
        let real_size = min(size - written_size, source.len() as u64);
        if real_size > 0 {
            memory.store_bytes(addr + written_size, &source[0..real_size as usize])?;
            written_size += real_size;
        }
    }
    if written_size < size {
        memory.store_byte(addr + written_size, size - written_size, 0)?;
    }
    Ok(())
}

// `size` should be none zero u64
pub fn get_page_indices(addr: u64, size: u64) -> Result<(u64, u64), Error> {
    let (addr_end, overflowed) = addr.overflowing_add(size);
    if overflowed {
        return Err(Error::MemOutOfBound);
    }
    if addr_end > RISCV_MAX_MEMORY as u64 {
        return Err(Error::MemOutOfBound);
    }
    let page = addr >> RISCV_PAGE_SHIFTS;
    let page_end = (addr_end - 1) >> RISCV_PAGE_SHIFTS;
    Ok((page, page_end))
}

pub fn check_permission<M: Memory>(
    memory: &mut M,
    page_indices: &(u64, u64),
    flag: u8,
) -> Result<(), Error> {
    for page in page_indices.0..=page_indices.1 {
        let page_flag = memory.fetch_flag(page)?;
        if (page_flag & FLAG_WXORX_BIT) != (flag & FLAG_WXORX_BIT) {
            return Err(Error::MemWriteOnExecutablePage);
        }
    }
    Ok(())
}

pub fn set_dirty<M: Memory>(memory: &mut M, page_indices: &(u64, u64)) -> Result<(), Error> {
    for page in page_indices.0..=page_indices.1 {
        memory.set_flag(page, FLAG_DIRTY)?
    }
    Ok(())
}

// Keep this in a central place to allow for future optimization
#[inline(always)]
pub fn memset(slice: &mut [u8], value: u8) {
    let p = slice.as_mut_ptr();
    unsafe {
        ptr::write_bytes(p, value, slice.len());
    }
}
