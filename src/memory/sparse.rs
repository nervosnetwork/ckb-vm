use super::super::{Error, Register, RISCV_PAGES, RISCV_PAGESIZE};
use super::{fill_page_data, memset, round_page, Memory, Page};

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
    _inner: PhantomData<R>,
}

impl<R> SparseMemory<R> {
    pub fn new() -> Self {
        debug_assert!(RISCV_PAGES < INVALID_PAGE_INDEX as usize);
        Self {
            indices: [INVALID_PAGE_INDEX; RISCV_PAGES],
            pages: Vec::new(),
            _inner: PhantomData,
        }
    }

    fn fetch_page(&mut self, aligned_addr: usize) -> Result<&mut Page, Error> {
        let page = aligned_addr / RISCV_PAGESIZE;
        let mut index = *(self.indices.get(page).ok_or(Error::OutOfBound)?);
        if index == INVALID_PAGE_INDEX {
            self.pages.push([0; RISCV_PAGESIZE]);
            index = (self.pages.len() - 1) as u16;
            self.indices[page] = index;
        }
        Ok(&mut self.pages[index as usize])
    }

    fn load(&mut self, addr: usize, bytes: usize) -> Result<u64, Error> {
        debug_assert!(bytes == 1 || bytes == 2 || bytes == 4 || bytes == 8);
        let page_addr = round_page(addr);
        let first_page_bytes = min(bytes, RISCV_PAGESIZE - (addr - page_addr));
        let mut shift = 0;
        let mut value: u64 = 0;
        {
            let page = self.fetch_page(page_addr)?;
            for &byte in page.iter().skip(addr - page_addr).take(first_page_bytes) {
                value |= u64::from(byte) << shift;
                shift += 8;
            }
        }
        let second_page_bytes = bytes - first_page_bytes;
        if second_page_bytes > 0 {
            let second_page = self.fetch_page(page_addr + RISCV_PAGESIZE)?;
            for &byte in second_page.iter().take(second_page_bytes) {
                value |= u64::from(byte) << shift;
                shift += 8;
            }
        }
        Ok(value)
    }
}

impl<R: Register> Memory<R> for SparseMemory<R> {
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

    fn load8(&mut self, addr: &R) -> Result<R, Error> {
        let v = self.load(addr.to_usize(), 1).map(|v| v as u8)?;
        Ok(R::from_u8(v))
    }

    fn load16(&mut self, addr: &R) -> Result<R, Error> {
        let v = self.load(addr.to_usize(), 2).map(|v| v as u16)?;
        Ok(R::from_u16(v))
    }

    fn load32(&mut self, addr: &R) -> Result<R, Error> {
        let v = self.load(addr.to_usize(), 4).map(|v| v as u32)?;
        Ok(R::from_u32(v))
    }

    fn load64(&mut self, addr: &R) -> Result<R, Error> {
        let v = self.load(addr.to_usize(), 8)?;
        Ok(R::from_u64(v))
    }

    fn execute_load16(&mut self, addr: usize) -> Result<u16, Error> {
        self.load(addr, 2).map(|v| v as u16)
    }

    fn store_bytes(&mut self, addr: usize, value: &[u8]) -> Result<(), Error> {
        let mut remaining_data = value;
        let mut current_page_addr = round_page(addr);
        let mut current_page_offset = addr - current_page_addr;
        while !remaining_data.is_empty() {
            let page = self.fetch_page(current_page_addr)?;
            let bytes = min(RISCV_PAGESIZE - current_page_offset, remaining_data.len());
            let slice = &mut page[current_page_offset..current_page_offset + bytes];
            slice.copy_from_slice(&remaining_data[..bytes]);

            remaining_data = &remaining_data[bytes..];
            current_page_addr += RISCV_PAGESIZE;
            current_page_offset = 0;
        }
        Ok(())
    }

    fn store_byte(&mut self, addr: usize, size: usize, value: u8) -> Result<(), Error> {
        let mut current_page_addr = round_page(addr);
        let mut current_page_offset = addr - current_page_addr;
        let mut remaining_size = size;
        while remaining_size > 0 {
            let page = self.fetch_page(current_page_addr)?;
            let bytes = min(RISCV_PAGESIZE - current_page_offset, remaining_size);
            memset(
                &mut page[current_page_offset..current_page_offset + bytes],
                value,
            );
            remaining_size -= bytes;
            current_page_addr += RISCV_PAGESIZE;
            current_page_offset = 0;
        }
        Ok(())
    }

    fn store8(&mut self, addr: &R, value: &R) -> Result<(), Error> {
        self.store_bytes(addr.to_usize(), &[value.to_u8()])
    }

    fn store16(&mut self, addr: &R, value: &R) -> Result<(), Error> {
        let value = value.to_u16();
        // RISC-V is little-endian by specification
        self.store_bytes(addr.to_usize(), &[(value & 0xFF) as u8, (value >> 8) as u8])
    }

    fn store32(&mut self, addr: &R, value: &R) -> Result<(), Error> {
        let value = value.to_u32();
        // RISC-V is little-endian by specification
        self.store_bytes(
            addr.to_usize(),
            &[
                (value & 0xFF) as u8,
                ((value >> 8) & 0xFF) as u8,
                ((value >> 16) & 0xFF) as u8,
                ((value >> 24) & 0xFF) as u8,
            ],
        )
    }

    fn store64(&mut self, addr: &R, value: &R) -> Result<(), Error> {
        let value = value.to_u64();
        // RISC-V is little-endian by specification
        self.store_bytes(
            addr.to_usize(),
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
