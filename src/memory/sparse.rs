use super::super::{Error, Register, RISCV_PAGES, RISCV_PAGESIZE, RISCV_PAGE_SHIFTS};
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
    indices: [u16; RISCV_PAGES],
    pages: Vec<Page>,
    flags: Vec<u8>,
    _inner: PhantomData<R>,
}

impl<R> SparseMemory<R> {
    pub fn new() -> Self {
        debug_assert!(RISCV_PAGES < INVALID_PAGE_INDEX as usize);
        Self {
            indices: [INVALID_PAGE_INDEX; RISCV_PAGES],
            pages: Vec::new(),
            flags: vec![0; RISCV_PAGES],
            _inner: PhantomData,
        }
    }

    fn fetch_page(&mut self, aligned_addr: u64) -> Result<&mut Page, Error> {
        let page = aligned_addr / RISCV_PAGESIZE as u64;
        if page >= RISCV_PAGES as u64 {
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
}

impl<R: Register> Memory for SparseMemory<R> {
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

    fn load_bytes(&mut self, addr: u64, size: u64) -> Result<Vec<u8>, Error> {
        let mut current_addr = addr;
        let mut current_size = size;
        let mut r: Vec<u8> = Vec::new();
        loop {
            if current_size >= 8 {
                let e = self.load64(&Self::REG::from_u64(current_addr))?.to_u64();
                r.extend_from_slice(&e.to_le_bytes());
                current_addr += 8;
                current_size -= 8;
                continue;
            }
            if current_size >= 4 {
                let e = self.load32(&Self::REG::from_u64(current_addr))?.to_u32();
                r.extend_from_slice(&e.to_le_bytes());
                current_addr += 4;
                current_size -= 4;
                continue;
            }
            if current_size >= 2 {
                let e = self.load16(&Self::REG::from_u64(current_addr))?.to_u16();
                r.extend_from_slice(&e.to_le_bytes());
                current_addr += 2;
                current_size -= 2;
                continue;
            }
            if current_size >= 1 {
                let e = self.load8(&Self::REG::from_u64(current_addr))?.to_u8();
                r.push(e);
                current_addr += 1;
                current_size -= 1;
                continue;
            }
            break;
        }
        Ok(r)
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
}

impl<R> Default for SparseMemory<R> {
    fn default() -> Self {
        Self::new()
    }
}
