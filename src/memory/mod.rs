use super::{Error, Register, RISCV_PAGESIZE};
use bytes::Bytes;

pub mod flat;
pub mod mmu;
pub mod sparse;

pub const PROT_READ: u32 = 0b0001;
pub const PROT_WRITE: u32 = 0b0010;
pub const PROT_EXEC: u32 = 0b0100;

#[inline(always)]
pub fn round_page(x: usize) -> usize {
    x & (!(RISCV_PAGESIZE - 1))
}

pub type Page = [u8; RISCV_PAGESIZE];

pub trait Memory<R: Register> {
    // Note this mmap only handles the very low level memory mapping logic.
    // It only takes an aligned address and size, then maps either existing
    // bytes or empty bytes to this range. It doesn't allocate addresses when
    // given 0 as address value. Instead, higher level machine should be leveraged
    // to manage code, heap(brk), mmap and stack regions.
    fn mmap(
        &mut self,
        addr: usize,
        size: usize,
        prot: u32,
        source: Option<Bytes>,
        offset: usize,
    ) -> Result<(), Error>;
    fn munmap(&mut self, addr: usize, size: usize) -> Result<(), Error>;
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
