use super::Error;

use byteorder::{LittleEndian, ReadBytesExt};
use std::borrow::BorrowMut;
use std::io::{Cursor, Seek, SeekFrom};

/// Here we build a flat memory based Memory object as a starting point for fast
/// iteration. Later we might want to re-evaluate this to see if we need a real
/// MMU system.
/// Current system is lacking the following features needed in a real production
/// system:
///
/// * mmap should work on pages, not arbitrary memory segments
/// * disallow unaligned address on page boundary
/// * read/write/execute permission checking
pub trait Memory {
    fn mmap(
        &mut self,
        addr: usize,
        size: usize,
        source: &[u8],
        offset: usize,
    ) -> Result<usize, Error>;

    // TODO: maybe parameterize those?
    fn load16(&self, addr: usize) -> Result<u16, Error>;
    fn load32(&self, addr: usize) -> Result<u32, Error>;
}

impl<T> Memory for T
where
    T: BorrowMut<[u8]>,
{
    fn mmap(
        &mut self,
        addr: usize,
        size: usize,
        source: &[u8],
        offset: usize,
    ) -> Result<usize, Error> {
        let memory = self.borrow_mut();
        if addr + size > memory.len() || offset + size > source.len() {
            return Err(Error::OutOfBound);
        }
        let (_, right) = memory.split_at_mut(addr);
        let (slice, _) = right.split_at_mut(size);
        slice.copy_from_slice(&source[offset..offset + size]);
        Ok(addr)
    }

    fn load16(&self, addr: usize) -> Result<u16, Error> {
        let memory = self.borrow();
        if addr + 2 > memory.len() {
            return Err(Error::OutOfBound);
        }
        let mut reader = Cursor::new(memory);
        reader
            .seek(SeekFrom::Start(addr as u64))
            .map_err(Error::IO)?;
        // NOTE: Base RISC-V ISA is defined as a little-endian memory system.
        reader.read_u16::<LittleEndian>().map_err(Error::IO)
    }

    fn load32(&self, addr: usize) -> Result<u32, Error> {
        let memory = self.borrow();
        if addr + 4 > memory.len() {
            return Err(Error::OutOfBound);
        }
        let mut reader = Cursor::new(memory);
        reader
            .seek(SeekFrom::Start(addr as u64))
            .map_err(Error::IO)?;
        // NOTE: Base RISC-V ISA is defined as a little-endian memory system.
        reader.read_u32::<LittleEndian>().map_err(Error::IO)
    }
}
