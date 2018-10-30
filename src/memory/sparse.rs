use super::super::{Error, RISCV_MAX_MEMORY, RISCV_PAGESIZE};
use super::{round_page, Memory, Page};

use std::cmp::min;
use std::ptr;
use std::rc::Rc;

const MAX_PAGES: usize = RISCV_MAX_MEMORY / RISCV_PAGESIZE;
const INVALID_PAGE_INDEX: u16 = 0xFFFF;

/// A sparse flat memory implementation, it allocates pages only when requested,
/// but besides that, it does not permission checking.
pub struct SparseMemory {
    // Stores the indices of each page in pages data structure, if a page hasn't
    // been initialized, the corresponding position will be filled with
    // INVALID_PAGE_INDEX. Considering u16 takes 2 bytes, this add an additional
    // of 64KB extra storage cost assuming we have 128MB memory.
    indices: [u16; MAX_PAGES],
    pages: Vec<Page>,
}

impl SparseMemory {
    pub fn new() -> Self {
        debug_assert!(MAX_PAGES < INVALID_PAGE_INDEX as usize);
        Self {
            indices: [INVALID_PAGE_INDEX; MAX_PAGES],
            pages: Vec::new(),
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

impl Memory for SparseMemory {
    fn mmap(
        &mut self,
        addr: usize,
        size: usize,
        _prot: u32,
        source: Option<Rc<Box<[u8]>>>,
        offset: usize,
    ) -> Result<(), Error> {
        // For simplicity, we implement this using store_bytes for now. Later
        // if needed, we can change this to load page from source on demand.
        if let Some(source) = source {
            let real_size = min(size, source.len() - offset);
            let value = &source[offset..offset + real_size];
            return self.store_bytes(addr, value);
        }
        Ok(())
    }

    fn munmap(&mut self, addr: usize, size: usize) -> Result<(), Error> {
        let mut current_page_addr = round_page(addr);
        let mut current_page_offset = addr - current_page_addr;
        let mut erased_size = 0;
        while erased_size < size {
            let page = self.fetch_page(current_page_addr)?;
            let bytes = min(RISCV_PAGESIZE - current_page_offset, size - erased_size);
            unsafe {
                let slice_ptr = page[current_page_offset..current_page_offset + bytes].as_mut_ptr();
                ptr::write_bytes(slice_ptr, b'0', bytes);
            }
            current_page_addr += RISCV_PAGESIZE;
            current_page_offset = 0;
            erased_size += bytes;
        }
        Ok(())
    }

    fn load8(&mut self, addr: usize) -> Result<u8, Error> {
        self.load(addr, 1).map(|v| v as u8)
    }

    fn load16(&mut self, addr: usize) -> Result<u16, Error> {
        self.load(addr, 2).map(|v| v as u16)
    }

    fn load32(&mut self, addr: usize) -> Result<u32, Error> {
        self.load(addr, 4).map(|v| v as u32)
    }

    fn load64(&mut self, addr: usize) -> Result<u64, Error> {
        self.load(addr, 8)
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
            unsafe {
                let slice_ptr = page[current_page_offset..bytes].as_mut_ptr();
                ptr::write_bytes(slice_ptr, value, bytes);
            }
            remaining_size -= bytes;
            current_page_addr += RISCV_PAGESIZE;
            current_page_offset = 0;
        }
        Ok(())
    }

    fn store8(&mut self, addr: usize, value: u8) -> Result<(), Error> {
        self.store_bytes(addr, &[value])
    }

    fn store16(&mut self, addr: usize, value: u16) -> Result<(), Error> {
        // RISC-V is little-endian by specification
        self.store_bytes(addr, &[(value & 0xFF) as u8, (value >> 8) as u8])
    }

    fn store32(&mut self, addr: usize, value: u32) -> Result<(), Error> {
        // RISC-V is little-endian by specification
        self.store_bytes(
            addr,
            &[
                (value & 0xFF) as u8,
                ((value >> 8) & 0xFF) as u8,
                ((value >> 16) & 0xFF) as u8,
                ((value >> 24) & 0xFF) as u8,
            ],
        )
    }

    fn store64(&mut self, addr: usize, value: u64) -> Result<(), Error> {
        // RISC-V is little-endian by specification
        self.store_bytes(
            addr,
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

impl Default for SparseMemory {
    fn default() -> Self {
        Self::new()
    }
}
