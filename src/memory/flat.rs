use super::super::Error;
use super::Memory;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::borrow::BorrowMut;
use std::cmp::min;
use std::io::{Cursor, Seek, SeekFrom};
use std::ptr;
use std::rc::Rc;

/// Here we build a flat memory based Memory object as a starting point for fast
/// iteration. Later we might want to re-evaluate this to see if we need a real
/// MMU system.
/// Current system is lacking the following features needed in a real production
/// system:
///
/// * mmap should work on pages, not arbitrary memory segments
/// * disallow unaligned address on page boundary
/// * read/write/execute permission checking
impl<T> Memory for T
where
    T: BorrowMut<[u8]>,
{
    fn mmap(
        &mut self,
        addr: usize,
        size: usize,
        _prot: u32,
        source: Option<Rc<Box<[u8]>>>,
        offset: usize,
    ) -> Result<(), Error> {
        let memory = self.borrow_mut();
        if addr + size > memory.len() {
            return Err(Error::OutOfBound);
        }
        if let Some(source) = source {
            let real_size = min(size, source.len() - offset);
            let slice = &mut memory[addr..addr + real_size];
            slice.copy_from_slice(&source[offset..offset + real_size]);
        }
        Ok(())
    }

    fn munmap(&mut self, addr: usize, size: usize) -> Result<(), Error> {
        let memory = self.borrow_mut();
        if addr + size > memory.len() {
            return Err(Error::OutOfBound);
        }
        // This is essentially memset call
        unsafe {
            let slice_ptr = memory[..size].as_mut_ptr();
            ptr::write_bytes(slice_ptr, b'0', size);
        }
        Ok(())
    }

    fn execute_load16(&mut self, addr: usize) -> Result<u16, Error> {
        self.load16(addr)
    }

    fn load8(&mut self, addr: usize) -> Result<u8, Error> {
        let memory = self.borrow();
        if addr + 1 > memory.len() {
            return Err(Error::OutOfBound);
        }
        let mut reader = Cursor::new(memory);
        reader
            .seek(SeekFrom::Start(addr as u64))
            .map_err(Error::IO)?;
        reader.read_u8().map_err(Error::IO)
    }

    fn load16(&mut self, addr: usize) -> Result<u16, Error> {
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

    fn load32(&mut self, addr: usize) -> Result<u32, Error> {
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

    fn load64(&mut self, addr: usize) -> Result<u64, Error> {
        let memory = self.borrow();
        if addr + 8 > memory.len() {
            return Err(Error::OutOfBound);
        }
        let mut reader = Cursor::new(memory);
        reader
            .seek(SeekFrom::Start(addr as u64))
            .map_err(Error::IO)?;
        // NOTE: Base RISC-V ISA is defined as a little-endian memory system.
        reader.read_u64::<LittleEndian>().map_err(Error::IO)
    }

    fn store8(&mut self, addr: usize, value: u8) -> Result<(), Error> {
        let memory = self.borrow_mut();
        if addr + 1 > memory.len() {
            return Err(Error::OutOfBound);
        }
        let mut writer = Cursor::new(memory);
        writer
            .seek(SeekFrom::Start(addr as u64))
            .map_err(Error::IO)?;
        writer.write_u8(value).map_err(Error::IO)
    }

    fn store16(&mut self, addr: usize, value: u16) -> Result<(), Error> {
        let memory = self.borrow_mut();
        if addr + 2 > memory.len() {
            return Err(Error::OutOfBound);
        }
        let mut writer = Cursor::new(memory);
        writer
            .seek(SeekFrom::Start(addr as u64))
            .map_err(Error::IO)?;
        writer.write_u16::<LittleEndian>(value).map_err(Error::IO)
    }

    fn store32(&mut self, addr: usize, value: u32) -> Result<(), Error> {
        let memory = self.borrow_mut();
        if addr + 4 > memory.len() {
            return Err(Error::OutOfBound);
        }
        let mut writer = Cursor::new(memory);
        writer
            .seek(SeekFrom::Start(addr as u64))
            .map_err(Error::IO)?;
        writer.write_u32::<LittleEndian>(value).map_err(Error::IO)
    }

    fn store64(&mut self, addr: usize, value: u64) -> Result<(), Error> {
        let memory = self.borrow_mut();
        if addr + 8 > memory.len() {
            return Err(Error::OutOfBound);
        }
        let mut writer = Cursor::new(memory);
        writer
            .seek(SeekFrom::Start(addr as u64))
            .map_err(Error::IO)?;
        writer.write_u64::<LittleEndian>(value).map_err(Error::IO)
    }

    fn store_bytes(&mut self, addr: usize, value: &[u8]) -> Result<(), Error> {
        let size = value.len();
        let memory = self.borrow_mut();
        if addr + size > memory.len() {
            return Err(Error::OutOfBound);
        }
        let slice = &mut memory[addr..addr + size];
        slice.copy_from_slice(value);
        Ok(())
    }
}
