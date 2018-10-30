use super::super::{Error, RISCV_MAX_MEMORY};
use super::Memory;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::cmp::min;
use std::io::{Cursor, Seek, SeekFrom};
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::rc::Rc;

pub struct FlatMemory {
    data: Vec<u8>,
}

impl Default for FlatMemory {
    fn default() -> Self {
        Self {
            data: vec![0; RISCV_MAX_MEMORY],
        }
    }
}

impl Deref for FlatMemory {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for FlatMemory {
    fn deref_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
}

/// A flat chunk of memory used for RISC-V machine, it lacks all the permission
/// checking logic.
impl Memory for FlatMemory {
    fn mmap(
        &mut self,
        addr: usize,
        size: usize,
        _prot: u32,
        source: Option<Rc<Box<[u8]>>>,
        offset: usize,
    ) -> Result<(), Error> {
        if addr + size > self.len() {
            return Err(Error::OutOfBound);
        }
        if let Some(source) = source {
            let real_size = min(size, source.len() - offset);
            let slice = &mut self[addr..addr + real_size];
            slice.copy_from_slice(&source[offset..offset + real_size]);
        }
        Ok(())
    }

    fn munmap(&mut self, addr: usize, size: usize) -> Result<(), Error> {
        if addr + size > self.len() {
            return Err(Error::OutOfBound);
        }
        // This is essentially memset call
        unsafe {
            let slice_ptr = self[..size].as_mut_ptr();
            ptr::write_bytes(slice_ptr, b'0', size);
        }
        Ok(())
    }

    fn execute_load16(&mut self, addr: usize) -> Result<u16, Error> {
        self.load16(addr)
    }

    fn load8(&mut self, addr: usize) -> Result<u8, Error> {
        if addr + 1 > self.len() {
            return Err(Error::OutOfBound);
        }
        let mut reader = Cursor::new(&self.data);
        reader.seek(SeekFrom::Start(addr as u64))?;
        Ok(reader.read_u8()?)
    }

    fn load16(&mut self, addr: usize) -> Result<u16, Error> {
        if addr + 2 > self.len() {
            return Err(Error::OutOfBound);
        }
        let mut reader = Cursor::new(&self.data);
        reader.seek(SeekFrom::Start(addr as u64))?;
        // NOTE: Base RISC-V ISA is defined as a little-endian memory system.
        Ok(reader.read_u16::<LittleEndian>()?)
    }

    fn load32(&mut self, addr: usize) -> Result<u32, Error> {
        if addr + 4 > self.len() {
            return Err(Error::OutOfBound);
        }
        let mut reader = Cursor::new(&self.data);
        reader.seek(SeekFrom::Start(addr as u64))?;
        // NOTE: Base RISC-V ISA is defined as a little-endian memory system.
        Ok(reader.read_u32::<LittleEndian>()?)
    }

    fn load64(&mut self, addr: usize) -> Result<u64, Error> {
        if addr + 8 > self.len() {
            return Err(Error::OutOfBound);
        }
        let mut reader = Cursor::new(&self.data);
        reader.seek(SeekFrom::Start(addr as u64))?;
        // NOTE: Base RISC-V ISA is defined as a little-endian memory system.
        Ok(reader.read_u64::<LittleEndian>()?)
    }

    fn store8(&mut self, addr: usize, value: u8) -> Result<(), Error> {
        if addr + 1 > self.len() {
            return Err(Error::OutOfBound);
        }
        let mut writer = Cursor::new(&mut self.data);
        writer.seek(SeekFrom::Start(addr as u64))?;
        writer.write_u8(value)?;
        Ok(())
    }

    fn store16(&mut self, addr: usize, value: u16) -> Result<(), Error> {
        if addr + 2 > self.len() {
            return Err(Error::OutOfBound);
        }
        let mut writer = Cursor::new(&mut self.data);
        writer.seek(SeekFrom::Start(addr as u64))?;
        writer.write_u16::<LittleEndian>(value)?;
        Ok(())
    }

    fn store32(&mut self, addr: usize, value: u32) -> Result<(), Error> {
        if addr + 4 > self.len() {
            return Err(Error::OutOfBound);
        }
        let mut writer = Cursor::new(&mut self.data);
        writer.seek(SeekFrom::Start(addr as u64))?;
        writer.write_u32::<LittleEndian>(value)?;
        Ok(())
    }

    fn store64(&mut self, addr: usize, value: u64) -> Result<(), Error> {
        if addr + 8 > self.len() {
            return Err(Error::OutOfBound);
        }
        let mut writer = Cursor::new(&mut self.data);
        writer.seek(SeekFrom::Start(addr as u64))?;
        writer.write_u64::<LittleEndian>(value)?;
        Ok(())
    }

    fn store_bytes(&mut self, addr: usize, value: &[u8]) -> Result<(), Error> {
        let size = value.len();
        if addr + size > self.len() {
            return Err(Error::OutOfBound);
        }
        let slice = &mut self[addr..addr + size];
        slice.copy_from_slice(value);
        Ok(())
    }

    fn store_byte(&mut self, addr: usize, size: usize, value: u8) -> Result<(), Error> {
        if addr + size > self.len() {
            return Err(Error::OutOfBound);
        }
        // This is essentially memset call
        unsafe {
            let slice_ptr = self[..size].as_mut_ptr();
            ptr::write_bytes(slice_ptr, value, size);
        }
        Ok(())
    }
}
