use super::{Error, Register, RISCV_PAGESIZE};
use bytes::Bytes;
use std::cmp::min;

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
pub fn fill_page_data<R: Register>(
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
