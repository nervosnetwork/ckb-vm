use super::super::{Error, Register, DEFAULT_MEMORY_SIZE, RISCV_PAGESIZE, RISCV_PAGE_SHIFTS};
use super::{fill_page_data, memset, round_page_down, Memory, Page, FLAG_DIRTY};

use bytes::Bytes;
use std::cmp::min;
use std::marker::PhantomData;

const INVALID_PAGE_INDEX: u16 = 0xFFFF;

/// A sparse flat memory implementation, it allocates pages only when requested,
/// but besides that, it does not permission checking.
pub struct SparseMemory<R> {
    // Stores the indices of each page in pages data structure, if a page hasn't
    // been initialized, the corresponding position will be filled with
    // INVALID_PAGE_INDEX. Considering u16 takes 2 bytes, this add an additional
    // of 64KB extra storage cost assuming we have 128MB memory.
    indices: Vec<u16>,
    pages: Vec<Page>,
    flags: Vec<u8>,
    memory_size: usize,
    riscv_pages: usize,
    load_reservation_address: R,
    _inner: PhantomData<R>,
}

impl<R: Register> SparseMemory<R> {
    fn fetch_page(&mut self, aligned_addr: u64) -> Result<&mut Page, Error> {
        let page = aligned_addr / RISCV_PAGESIZE as u64;
        if page >= self.riscv_pages as u64 {
            return Err(Error::MemOutOfBound);
        }
        let mut index = self.indices[page as usize];
        if index == INVALID_PAGE_INDEX {
            self.pages.push([0; RISCV_PAGESIZE]);
            index = (self.pages.len() - 1) as u16;
            self.indices[page as usize] = index;
        }
        Ok(&mut self.pages[index as usize])
    }

    fn load(&mut self, addr: u64, bytes: u64) -> Result<u64, Error> {
        debug_assert!(bytes == 1 || bytes == 2 || bytes == 4 || bytes == 8);
        let page_addr = round_page_down(addr);
        let first_page_bytes = min(bytes, RISCV_PAGESIZE as u64 - (addr - page_addr));
        let mut shift = 0;
        let mut value: u64 = 0;
        {
            let page = self.fetch_page(page_addr)?;
            for &byte in page
                .iter()
                .skip((addr - page_addr) as usize)
                .take(first_page_bytes as usize)
            {
                value |= u64::from(byte) << shift;
                shift += 8;
            }
        }
        let second_page_bytes = bytes - first_page_bytes;
        if second_page_bytes > 0 {
            let second_page = self.fetch_page(page_addr + RISCV_PAGESIZE as u64)?;
            for &byte in second_page.iter().take(second_page_bytes as usize) {
                value |= u64::from(byte) << shift;
                shift += 8;
            }
        }
        Ok(value)
    }

    pub fn new_with_memory(memory_size: usize) -> Self {
        assert!(memory_size % RISCV_PAGESIZE == 0);
        Self {
            indices: vec![INVALID_PAGE_INDEX; memory_size / RISCV_PAGESIZE],
            pages: Vec::new(),
            flags: vec![0; memory_size / RISCV_PAGESIZE],
            memory_size,
            riscv_pages: memory_size / RISCV_PAGESIZE,
            load_reservation_address: R::from_u64(u64::MAX),
            _inner: PhantomData,
        }
    }
}

impl<R: Register> Default for SparseMemory<R> {
    fn default() -> Self {
        Self::new_with_memory(DEFAULT_MEMORY_SIZE)
    }
}

impl<R: Register> Memory for SparseMemory<R> {
    type REG = R;

    fn reset_memory(&mut self) -> Result<(), Error> {
        self.indices = vec![INVALID_PAGE_INDEX; self.indices.len()];
        memset(&mut self.flags, 0);
        self.pages.clear();
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
            Err(Error::MemOutOfBound)
        }
    }

    fn set_flag(&mut self, page: u64, flag: u8) -> Result<(), Error> {
        if page < self.riscv_pages as u64 {
            self.flags[page as usize] |= flag;
            Ok(())
        } else {
            Err(Error::MemOutOfBound)
        }
    }

    fn clear_flag(&mut self, page: u64, flag: u8) -> Result<(), Error> {
        if page < self.riscv_pages as u64 {
            self.flags[page as usize] &= !flag;
            Ok(())
        } else {
            Err(Error::MemOutOfBound)
        }
    }

    fn memory_size(&self) -> usize {
        self.memory_size
    }

    fn load8(&mut self, addr: &Self::REG) -> Result<Self::REG, Error> {
        let v = self.load(addr.to_u64(), 1).map(|v| v as u8)?;
        Ok(Self::REG::from_u8(v))
    }

    fn load16(&mut self, addr: &Self::REG) -> Result<Self::REG, Error> {
        let v = self.load(addr.to_u64(), 2).map(|v| v as u16)?;
        Ok(Self::REG::from_u16(v))
    }

    fn load32(&mut self, addr: &Self::REG) -> Result<Self::REG, Error> {
        let v = self.load(addr.to_u64(), 4).map(|v| v as u32)?;
        Ok(Self::REG::from_u32(v))
    }

    fn load64(&mut self, addr: &Self::REG) -> Result<Self::REG, Error> {
        let v = self.load(addr.to_u64(), 8)?;
        Ok(Self::REG::from_u64(v))
    }

    fn execute_load16(&mut self, addr: u64) -> Result<u16, Error> {
        self.load(addr, 2).map(|v| v as u16)
    }

    fn execute_load32(&mut self, addr: u64) -> Result<u32, Error> {
        self.load(addr, 4).map(|v| v as u32)
    }

    fn store_bytes(&mut self, addr: u64, value: &[u8]) -> Result<(), Error> {
        let mut remaining_data = value;
        let mut current_page_addr = round_page_down(addr);
        let mut current_page_offset = addr - current_page_addr;
        while !remaining_data.is_empty() {
            let page = self.fetch_page(current_page_addr)?;
            let bytes = min(
                RISCV_PAGESIZE as u64 - current_page_offset,
                remaining_data.len() as u64,
            );
            let slice =
                &mut page[current_page_offset as usize..(current_page_offset + bytes) as usize];
            slice.copy_from_slice(&remaining_data[..bytes as usize]);
            self.set_flag(current_page_addr >> RISCV_PAGE_SHIFTS, FLAG_DIRTY)?;

            remaining_data = &remaining_data[bytes as usize..];
            current_page_addr += RISCV_PAGESIZE as u64;
            current_page_offset = 0;
        }
        Ok(())
    }

    fn store_byte(&mut self, addr: u64, size: u64, value: u8) -> Result<(), Error> {
        let mut current_page_addr = round_page_down(addr);
        let mut current_page_offset = addr - current_page_addr;
        let mut remaining_size = size;
        while remaining_size > 0 {
            let page = self.fetch_page(current_page_addr)?;
            let bytes = min(RISCV_PAGESIZE as u64 - current_page_offset, remaining_size);
            memset(
                &mut page[current_page_offset as usize..(current_page_offset + bytes) as usize],
                value,
            );
            self.set_flag(current_page_addr >> RISCV_PAGE_SHIFTS, FLAG_DIRTY)?;

            remaining_size -= bytes;
            current_page_addr += RISCV_PAGESIZE as u64;
            current_page_offset = 0;
        }
        Ok(())
    }

    fn load_bytes(&mut self, addr: u64, size: u64) -> Result<Bytes, Error> {
        if size == 0 {
            return Ok(Bytes::new());
        }
        if addr.checked_add(size).ok_or(Error::MemOutOfBound)? > self.memory_size() as u64 {
            return Err(Error::MemOutOfBound);
        }
        let mut current_page_addr = round_page_down(addr);
        let mut current_page_offset = addr - current_page_addr;
        let mut need_read_len = size;
        let mut out_value = Vec::<u8>::with_capacity(size as usize);
        while need_read_len != 0 {
            let page = self.fetch_page(current_page_addr)?;
            let bytes = min(RISCV_PAGESIZE as u64 - current_page_offset, need_read_len);
            out_value.extend(
                &page[current_page_offset as usize..(current_page_offset + bytes) as usize],
            );
            need_read_len -= bytes;
            current_page_addr += RISCV_PAGESIZE as u64;
            current_page_offset = 0;
        }
        Ok(Bytes::from(out_value))
    }

    fn store8(&mut self, addr: &Self::REG, value: &Self::REG) -> Result<(), Error> {
        self.store_bytes(addr.to_u64(), &[value.to_u8()])
    }

    fn store16(&mut self, addr: &Self::REG, value: &Self::REG) -> Result<(), Error> {
        let value = value.to_u16();
        // RISC-V is little-endian by specification
        self.store_bytes(addr.to_u64(), &[(value & 0xFF) as u8, (value >> 8) as u8])
    }

    fn store32(&mut self, addr: &Self::REG, value: &Self::REG) -> Result<(), Error> {
        let value = value.to_u32();
        // RISC-V is little-endian by specification
        self.store_bytes(
            addr.to_u64(),
            &[
                (value & 0xFF) as u8,
                ((value >> 8) & 0xFF) as u8,
                ((value >> 16) & 0xFF) as u8,
                ((value >> 24) & 0xFF) as u8,
            ],
        )
    }

    fn store64(&mut self, addr: &Self::REG, value: &Self::REG) -> Result<(), Error> {
        let value = value.to_u64();
        // RISC-V is little-endian by specification
        self.store_bytes(
            addr.to_u64(),
            &[
                (value & 0xFF) as u8,
                ((value >> 8) & 0xFF) as u8,
                ((value >> 16) & 0xFF) as u8,
                ((value >> 24) & 0xFF) as u8,
                ((value >> 32) & 0xFF) as u8,
                ((value >> 40) & 0xFF) as u8,
                ((value >> 48) & 0xFF) as u8,
                ((value >> 56) & 0xFF) as u8,
            ],
        )
    }

    fn lr(&self) -> &Self::REG {
        &self.load_reservation_address
    }

    fn set_lr(&mut self, value: &Self::REG) {
        self.load_reservation_address = value.clone();
    }
}
