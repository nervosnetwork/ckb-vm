use super::super::{Error, Register, RISCV_MAX_MEMORY, RISCV_PAGES};
use super::{fill_page_data, memset, Memory};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use bytes::Bytes;
use std::io::{Cursor, Seek, SeekFrom};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub struct FlatMemory<R> {
    data: Vec<u8>,
    _inner: PhantomData<R>,
}

impl<R> Default for FlatMemory<R> {
    fn default() -> Self {
        Self {
            data: vec![0; RISCV_MAX_MEMORY],
            _inner: PhantomData,
        }
    }
}

impl<R> Deref for FlatMemory<R> {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<R> DerefMut for FlatMemory<R> {
    fn deref_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
}

/// A flat chunk of memory used for RISC-V machine, it lacks all the permission
/// checking logic.
impl<R: Register> Memory<R> for FlatMemory<R> {
    fn init_pages(
        &mut self,
        addr: usize,
        size: usize,
        _flags: u8,
        source: Option<Bytes>,
        offset_from_addr: usize,
    ) -> Result<(), Error> {
        fill_page_data(self, addr, size, source, offset_from_addr)
    }

    fn fetch_flag(&mut self, page: usize) -> Result<u8, Error> {
        if page < RISCV_PAGES {
            Ok(0)
        } else {
            Err(Error::OutOfBound)
        }
    }

    fn execute_load16(&mut self, addr: usize) -> Result<u16, Error> {
        self.load16(&R::from_usize(addr)).map(|v| v.to_u16())
    }

    fn load8(&mut self, addr: &R) -> Result<R, Error> {
        let addr = addr.to_usize();
        if addr + 1 > self.len() {
            return Err(Error::OutOfBound);
        }
        let mut reader = Cursor::new(&self.data);
        reader.seek(SeekFrom::Start(addr as u64))?;
        let v = reader.read_u8()?;
        Ok(R::from_u8(v))
    }

    fn load16(&mut self, addr: &R) -> Result<R, Error> {
        let addr = addr.to_usize();
        if addr + 2 > self.len() {
            return Err(Error::OutOfBound);
        }
        let mut reader = Cursor::new(&self.data);
        reader.seek(SeekFrom::Start(addr as u64))?;
        // NOTE: Base RISC-V ISA is defined as a little-endian memory system.
        let v = reader.read_u16::<LittleEndian>()?;
        Ok(R::from_u16(v))
    }

    fn load32(&mut self, addr: &R) -> Result<R, Error> {
        let addr = addr.to_usize();
        if addr + 4 > self.len() {
            return Err(Error::OutOfBound);
        }
        let mut reader = Cursor::new(&self.data);
        reader.seek(SeekFrom::Start(addr as u64))?;
        // NOTE: Base RISC-V ISA is defined as a little-endian memory system.
        let v = reader.read_u32::<LittleEndian>()?;
        Ok(R::from_u32(v))
    }

    fn load64(&mut self, addr: &R) -> Result<R, Error> {
        let addr = addr.to_usize();
        if addr + 8 > self.len() {
            return Err(Error::OutOfBound);
        }
        let mut reader = Cursor::new(&self.data);
        reader.seek(SeekFrom::Start(addr as u64))?;
        // NOTE: Base RISC-V ISA is defined as a little-endian memory system.
        let v = reader.read_u64::<LittleEndian>()?;
        Ok(R::from_u64(v))
    }

    fn store8(&mut self, addr: &R, value: &R) -> Result<(), Error> {
        let addr = addr.to_usize();
        if addr + 1 > self.len() {
            return Err(Error::OutOfBound);
        }
        let mut writer = Cursor::new(&mut self.data);
        writer.seek(SeekFrom::Start(addr as u64))?;
        writer.write_u8(value.to_u8())?;
        Ok(())
    }

    fn store16(&mut self, addr: &R, value: &R) -> Result<(), Error> {
        let addr = addr.to_usize();
        if addr + 2 > self.len() {
            return Err(Error::OutOfBound);
        }
        let mut writer = Cursor::new(&mut self.data);
        writer.seek(SeekFrom::Start(addr as u64))?;
        writer.write_u16::<LittleEndian>(value.to_u16())?;
        Ok(())
    }

    fn store32(&mut self, addr: &R, value: &R) -> Result<(), Error> {
        let addr = addr.to_usize();
        if addr + 4 > self.len() {
            return Err(Error::OutOfBound);
        }
        let mut writer = Cursor::new(&mut self.data);
        writer.seek(SeekFrom::Start(addr as u64))?;
        writer.write_u32::<LittleEndian>(value.to_u32())?;
        Ok(())
    }

    fn store64(&mut self, addr: &R, value: &R) -> Result<(), Error> {
        let addr = addr.to_usize();
        if addr + 8 > self.len() {
            return Err(Error::OutOfBound);
        }
        let mut writer = Cursor::new(&mut self.data);
        writer.seek(SeekFrom::Start(addr as u64))?;
        writer.write_u64::<LittleEndian>(value.to_u64())?;
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
        memset(&mut self[addr..addr + size], value);
        Ok(())
    }
}
