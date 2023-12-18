use super::super::{
    error::OutOfBoundKind, Error, Register, DEFAULT_MEMORY_SIZE, RISCV_PAGESIZE, RISCV_PAGE_SHIFTS,
};
use super::{check_no_overflow, fill_page_data, get_page_indices, memset, set_dirty, Memory};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use bytes::Bytes;
use std::io::{Cursor, Seek, SeekFrom};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub struct FlatMemory<R> {
    data: Vec<u8>,
    flags: Vec<u8>,
    memory_size: usize,
    riscv_pages: usize,
    load_reservation_address: R,
    _inner: PhantomData<R>,
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

impl<R: Register> FlatMemory<R> {
    pub fn new_with_memory(memory_size: usize) -> Self {
        assert!(memory_size % RISCV_PAGESIZE == 0);
        Self {
            data: vec![0; memory_size],
            flags: vec![0; memory_size / RISCV_PAGESIZE],
            memory_size,
            riscv_pages: memory_size / RISCV_PAGESIZE,
            load_reservation_address: R::from_u64(u64::MAX),
            _inner: PhantomData,
        }
    }
}

impl<R: Register> Default for FlatMemory<R> {
    fn default() -> Self {
        Self::new_with_memory(DEFAULT_MEMORY_SIZE)
    }
}

/// A flat chunk of memory used for RISC-V machine, it lacks all the permission
/// checking logic.
impl<R: Register> Memory for FlatMemory<R> {
    type REG = R;

    fn reset_memory(&mut self) -> Result<(), Error> {
        memset(&mut self.data, 0);
        memset(&mut self.flags, 0);
        self.load_reservation_address = R::from_u64(u64::MAX);
        Ok(())
    }

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
        if page < self.riscv_pages as u64 {
            Ok(self.flags[page as usize])
        } else {
            Err(Error::MemOutOfBound(
                page << RISCV_PAGE_SHIFTS,
                OutOfBoundKind::Memory,
            ))
        }
    }

    fn set_flag(&mut self, page: u64, flag: u8) -> Result<(), Error> {
        if page < self.riscv_pages as u64 {
            self.flags[page as usize] |= flag;
            Ok(())
        } else {
            Err(Error::MemOutOfBound(
                page << RISCV_PAGE_SHIFTS,
                OutOfBoundKind::Memory,
            ))
        }
    }

    fn clear_flag(&mut self, page: u64, flag: u8) -> Result<(), Error> {
        if page < self.riscv_pages as u64 {
            self.flags[page as usize] &= !flag;
            Ok(())
        } else {
            Err(Error::MemOutOfBound(
                page << RISCV_PAGE_SHIFTS,
                OutOfBoundKind::Memory,
            ))
        }
    }

    fn memory_size(&self) -> usize {
        self.memory_size
    }

    fn execute_load16(&mut self, addr: u64) -> Result<u16, Error> {
        self.load16(&Self::REG::from_u64(addr)).map(|v| v.to_u16())
    }

    fn execute_load32(&mut self, addr: u64) -> Result<u32, Error> {
        self.load32(&R::from_u64(addr)).map(|v| v.to_u32())
    }

    fn load8(&mut self, addr: &Self::REG) -> Result<Self::REG, Error> {
        let addr = addr.to_u64();
        check_no_overflow(addr, 1, self.memory_size as u64)?;
        let mut reader = Cursor::new(&self.data);
        reader.seek(SeekFrom::Start(addr as u64))?;
        let v = reader.read_u8()?;
        Ok(Self::REG::from_u8(v))
    }

    fn load16(&mut self, addr: &Self::REG) -> Result<Self::REG, Error> {
        let addr = addr.to_u64();
        check_no_overflow(addr, 2, self.memory_size as u64)?;
        let mut reader = Cursor::new(&self.data);
        reader.seek(SeekFrom::Start(addr as u64))?;
        // NOTE: Base RISC-V ISA is defined as a little-endian memory system.
        let v = reader.read_u16::<LittleEndian>()?;
        Ok(Self::REG::from_u16(v))
    }

    fn load32(&mut self, addr: &Self::REG) -> Result<Self::REG, Error> {
        let addr = addr.to_u64();
        check_no_overflow(addr, 4, self.memory_size as u64)?;
        let mut reader = Cursor::new(&self.data);
        reader.seek(SeekFrom::Start(addr as u64))?;
        // NOTE: Base RISC-V ISA is defined as a little-endian memory system.
        let v = reader.read_u32::<LittleEndian>()?;
        Ok(Self::REG::from_u32(v))
    }

    fn load64(&mut self, addr: &Self::REG) -> Result<Self::REG, Error> {
        let addr = addr.to_u64();
        check_no_overflow(addr, 8, self.memory_size as u64)?;
        let mut reader = Cursor::new(&self.data);
        reader.seek(SeekFrom::Start(addr as u64))?;
        // NOTE: Base RISC-V ISA is defined as a little-endian memory system.
        let v = reader.read_u64::<LittleEndian>()?;
        Ok(Self::REG::from_u64(v))
    }

    fn store8(&mut self, addr: &Self::REG, value: &Self::REG) -> Result<(), Error> {
        let addr = addr.to_u64();
        check_no_overflow(addr, 1, self.memory_size as u64)?;
        let page_indices = get_page_indices(addr, 1);
        set_dirty(self, &page_indices)?;
        let mut writer = Cursor::new(&mut self.data);
        writer.seek(SeekFrom::Start(addr as u64))?;
        writer.write_u8(value.to_u8())?;
        Ok(())
    }

    fn store16(&mut self, addr: &Self::REG, value: &Self::REG) -> Result<(), Error> {
        let addr = addr.to_u64();
        check_no_overflow(addr, 2, self.memory_size as u64)?;
        let page_indices = get_page_indices(addr, 2);
        set_dirty(self, &page_indices)?;
        let mut writer = Cursor::new(&mut self.data);
        writer.seek(SeekFrom::Start(addr as u64))?;
        writer.write_u16::<LittleEndian>(value.to_u16())?;
        Ok(())
    }

    fn store32(&mut self, addr: &Self::REG, value: &Self::REG) -> Result<(), Error> {
        let addr = addr.to_u64();
        check_no_overflow(addr, 4, self.memory_size as u64)?;
        let page_indices = get_page_indices(addr, 4);
        set_dirty(self, &page_indices)?;
        let mut writer = Cursor::new(&mut self.data);
        writer.seek(SeekFrom::Start(addr as u64))?;
        writer.write_u32::<LittleEndian>(value.to_u32())?;
        Ok(())
    }

    fn store64(&mut self, addr: &Self::REG, value: &Self::REG) -> Result<(), Error> {
        let addr = addr.to_u64();
        check_no_overflow(addr, 8, self.memory_size as u64)?;
        let page_indices = get_page_indices(addr, 8);
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
        check_no_overflow(addr, size, self.memory_size as u64)?;
        let page_indices = get_page_indices(addr, size);
        set_dirty(self, &page_indices)?;
        let slice = &mut self[addr as usize..(addr + size) as usize];
        slice.copy_from_slice(value);
        Ok(())
    }

    fn store_byte(&mut self, addr: u64, size: u64, value: u8) -> Result<(), Error> {
        if size == 0 {
            return Ok(());
        }
        check_no_overflow(addr, size, self.memory_size as u64)?;
        let page_indices = get_page_indices(addr, size);
        set_dirty(self, &page_indices)?;
        memset(&mut self[addr as usize..(addr + size) as usize], value);
        Ok(())
    }

    fn load_bytes(&mut self, addr: u64, size: u64) -> Result<Bytes, Error> {
        if size == 0 {
            return Ok(Bytes::new());
        }
        check_no_overflow(addr, size, self.memory_size as u64)?;
        Ok(Bytes::from(
            self[addr as usize..(addr + size) as usize].to_vec(),
        ))
    }

    fn lr(&self) -> &Self::REG {
        &self.load_reservation_address
    }

    fn set_lr(&mut self, value: &Self::REG) {
        self.load_reservation_address = value.clone();
    }
}
