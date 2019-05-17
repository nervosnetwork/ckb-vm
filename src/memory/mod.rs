use super::{bits::rounddown, Error, Register, RISCV_PAGESIZE};
use bytes::Bytes;
use std::cmp::min;
use std::ptr;

pub mod flat;
pub mod sparse;
pub mod wxorx;

pub use ckb_vm_definitions::memory::{
    FLAG_EXECUTABLE, FLAG_FREEZED, FLAG_WRITABLE, FLAG_WXORX_BIT,
};

#[inline(always)]
pub fn round_page(x: usize) -> usize {
    x & (!(RISCV_PAGESIZE - 1))
}

pub type Page = [u8; RISCV_PAGESIZE];

pub trait Memory<R: Register> {
    fn init_pages(
        &mut self,
        addr: usize,
        size: usize,
        flags: u8,
        source: Option<Bytes>,
        offset_from_addr: usize,
    ) -> Result<(), Error>;
    fn fetch_flag(&mut self, page: usize) -> Result<u8, Error>;
    // This is in fact just memset
    fn store_byte(&mut self, addr: usize, size: usize, value: u8) -> Result<(), Error>;
    fn store_bytes(&mut self, addr: usize, value: &[u8]) -> Result<(), Error>;
    fn execute_load16(&mut self, addr: usize) -> Result<u16, Error>;

    // Methods below are used to implement RISC-V instructions, to make JIT
    // possible, we need to use register type here so as to pass enough
    // information around.
    fn load8(&mut self, addr: &R) -> Result<R, Error>;
    fn load16(&mut self, addr: &R) -> Result<R, Error>;
    fn load32(&mut self, addr: &R) -> Result<R, Error>;
    fn load64(&mut self, addr: &R) -> Result<R, Error>;

    fn store8(&mut self, addr: &R, value: &R) -> Result<(), Error>;
    fn store16(&mut self, addr: &R, value: &R) -> Result<(), Error>;
    fn store32(&mut self, addr: &R, value: &R) -> Result<(), Error>;
    fn store64(&mut self, addr: &R, value: &R) -> Result<(), Error>;
}

#[inline(always)]
pub(crate) fn fill_page_data<R: Register>(
    memory: &mut Memory<R>,
    addr: usize,
    size: usize,
    source: Option<Bytes>,
    offset_from_addr: usize,
) -> Result<(), Error> {
    let mut written_size = 0;
    if offset_from_addr > 0 {
        let real_size = min(size, offset_from_addr);
        memory.store_byte(addr, real_size, 0)?;
        written_size += real_size;
    }
    if let Some(source) = source {
        let real_size = min(size - written_size, source.len());
        if real_size > 0 {
            memory.store_bytes(addr + written_size, &source[0..real_size])?;
            written_size += real_size;
        }
    }
    if written_size < size {
        memory.store_byte(addr + written_size, size - written_size, 0)?;
    }
    Ok(())
}

pub fn check_permission<R: Register>(
    memory: &mut Memory<R>,
    addr: usize,
    size: usize,
    flag: u8,
) -> Result<(), Error> {
    let e = addr + size;
    let mut current_addr = rounddown(addr, RISCV_PAGESIZE);
    while current_addr < e {
        let page = current_addr / RISCV_PAGESIZE;
        let page_flag = memory.fetch_flag(page)?;
        if (page_flag & FLAG_WXORX_BIT) != (flag & FLAG_WXORX_BIT) {
            return Err(Error::InvalidPermission);
        }
        current_addr += RISCV_PAGESIZE;
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
