use super::super::{Error, Register, RISCV_MAX_MEMORY, RISCV_PAGES};
use super::{fill_page_data, get_page_indices, memset, set_dirty, Memory};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use bytes::Bytes;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub struct FlatMemory<R> {
    data: Vec<u8>,
    flags: Vec<u8>,
    _inner: PhantomData<R>,
}

impl<R> Default for FlatMemory<R> {
    fn default() -> Self {
        Self {
            data: vec![0; RISCV_MAX_MEMORY],
            flags: vec![0; RISCV_PAGES],
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
impl<R: Register> Memory for FlatMemory<R> {
    type REG = R;

    fn init_pages(
        &mut self,
        addr: u64,
        size: u64,
        _flags: u8,
        source: Option<Bytes>,
        offset_from_addr: u64,
    ) -> Result<(), Error> {
        fill_page_data(self, addr, size, source, offset_from_addr)
    }

    fn fetch_flag(&mut self, page: u64) -> Result<u8, Error> {
        if page < RISCV_PAGES as u64 {
            Ok(self.flags[page as usize])
        } else {
            Err(Error::MemOutOfBound)
        }
    }

    fn set_flag(&mut self, page: u64, flag: u8) -> Result<(), Error> {
        if page < RISCV_PAGES as u64 {
            self.flags[page as usize] |= flag;
            Ok(())
        } else {
            Err(Error::MemOutOfBound)
        }
    }

    fn clear_flag(&mut self, page: u64, flag: u8) -> Result<(), Error> {
        if page < RISCV_PAGES as u64 {
            self.flags[page as usize] &= !flag;
            Ok(())
        } else {
            Err(Error::MemOutOfBound)
        }
    }

    fn execute_load16(&mut self, addr: u64) -> Result<u16, Error> {
        self.load16(&Self::REG::from_u64(addr)).map(|v| v.to_u16())
    }

    fn execute_load32(&mut self, addr: u64) -> Result<u32, Error> {
        self.load32(&R::from_u64(addr)).map(|v| v.to_u32())
    }

    fn load8(&mut self, addr: &Self::REG) -> Result<Self::REG, Error> {
        let addr = addr.to_u64();
        if addr.checked_add(1).ok_or(Error::MemOutOfBound)? > self.len() as u64 {
            return Err(Error::MemOutOfBound);
        }
        let mut reader = Cursor::new(&self.data);
        reader.seek(SeekFrom::Start(addr as u64))?;
        let v = reader.read_u8()?;
        Ok(Self::REG::from_u8(v))
    }

    fn load16(&mut self, addr: &Self::REG) -> Result<Self::REG, Error> {
        let addr = addr.to_u64();
        if addr.checked_add(2).ok_or(Error::MemOutOfBound)? > self.len() as u64 {
            return Err(Error::MemOutOfBound);
        }
        let mut reader = Cursor::new(&self.data);
        reader.seek(SeekFrom::Start(addr as u64))?;
        // NOTE: Base RISC-V ISA is defined as a little-endian memory system.
        let v = reader.read_u16::<LittleEndian>()?;
        Ok(Self::REG::from_u16(v))
    }

    fn load32(&mut self, addr: &Self::REG) -> Result<Self::REG, Error> {
        let addr = addr.to_u64();
        if addr.checked_add(4).ok_or(Error::MemOutOfBound)? > self.len() as u64 {
            return Err(Error::MemOutOfBound);
        }
        let mut reader = Cursor::new(&self.data);
        reader.seek(SeekFrom::Start(addr as u64))?;
        // NOTE: Base RISC-V ISA is defined as a little-endian memory system.
        let v = reader.read_u32::<LittleEndian>()?;
        Ok(Self::REG::from_u32(v))
    }

    fn load64(&mut self, addr: &Self::REG) -> Result<Self::REG, Error> {
        let addr = addr.to_u64();
        if addr.checked_add(8).ok_or(Error::MemOutOfBound)? > self.len() as u64 {
            return Err(Error::MemOutOfBound);
        }
        let mut reader = Cursor::new(&self.data);
        reader.seek(SeekFrom::Start(addr as u64))?;
        // NOTE: Base RISC-V ISA is defined as a little-endian memory system.
        let v = reader.read_u64::<LittleEndian>()?;
        Ok(Self::REG::from_u64(v))
    }

    fn store8(&mut self, addr: &Self::REG, value: &Self::REG) -> Result<(), Error> {
        let addr = addr.to_u64();
        let page_indices = get_page_indices(addr.to_u64(), 1)?;
        set_dirty(self, &page_indices)?;
        let mut writer = Cursor::new(&mut self.data);
        writer.seek(SeekFrom::Start(addr as u64))?;
        writer.write_u8(value.to_u8())?;
        Ok(())
    }

    fn store16(&mut self, addr: &Self::REG, value: &Self::REG) -> Result<(), Error> {
        let addr = addr.to_u64();
        let page_indices = get_page_indices(addr.to_u64(), 2)?;
        set_dirty(self, &page_indices)?;
        let mut writer = Cursor::new(&mut self.data);
        writer.seek(SeekFrom::Start(addr as u64))?;
        writer.write_u16::<LittleEndian>(value.to_u16())?;
        Ok(())
    }

    fn store32(&mut self, addr: &Self::REG, value: &Self::REG) -> Result<(), Error> {
        let addr = addr.to_u64();
        let page_indices = get_page_indices(addr.to_u64(), 4)?;
        set_dirty(self, &page_indices)?;
        let mut writer = Cursor::new(&mut self.data);
        writer.seek(SeekFrom::Start(addr as u64))?;
        writer.write_u32::<LittleEndian>(value.to_u32())?;
        Ok(())
    }

    fn store64(&mut self, addr: &Self::REG, value: &Self::REG) -> Result<(), Error> {
        let addr = addr.to_u64();
        let page_indices = get_page_indices(addr.to_u64(), 8)?;
        set_dirty(self, &page_indices)?;
        let mut writer = Cursor::new(&mut self.data);
        writer.seek(SeekFrom::Start(addr as u64))?;
        writer.write_u64::<LittleEndian>(value.to_u64())?;
        Ok(())
    }

    fn store_bytes(&mut self, addr: u64, value: &[u8]) -> Result<(), Error> {
        let size = value.len() as u64;
        if size == 0 {
            return Ok(());
        }
        let page_indices = get_page_indices(addr.to_u64(), size)?;
        set_dirty(self, &page_indices)?;
        let slice = &mut self[addr as usize..(addr + size) as usize];
        slice.copy_from_slice(value);
        Ok(())
    }

    fn store_byte(&mut self, addr: u64, size: u64, value: u8) -> Result<(), Error> {
        if size == 0 {
            return Ok(());
        }
        let page_indices = get_page_indices(addr.to_u64(), size)?;
        set_dirty(self, &page_indices)?;
        memset(&mut self[addr as usize..(addr + size) as usize], value);
        Ok(())
    }

    fn load_bytes(&mut self, addr: u64, size: u64) -> Result<Vec<u8>, Error> {
        if addr.checked_add(size).ok_or(Error::MemOutOfBound)? > self.len() as u64 {
            return Err(Error::MemOutOfBound);
        }
        let mut reader = Cursor::new(&self.data);
        reader.seek(SeekFrom::Start(addr))?;
        let mut v = Vec::with_capacity(size as usize);
        reader.read_exact(&mut v)?;
        Ok(v)
    }
}
