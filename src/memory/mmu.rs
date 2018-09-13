use super::super::{Error, RISCV_MAX_MEMORY, RISCV_PAGESIZE};
use super::{Memory, PROT_EXEC, PROT_READ, PROT_WRITE};

use std::cmp::min;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::rc::Rc;

pub const MAX_VIRTUAL_MEMORY_ENTRIES: usize = 64;

const MAX_PAGES: usize = RISCV_MAX_MEMORY / RISCV_PAGESIZE;
const INVALID_PAGE_INDEX: u8 = 0xFF;

type Page = [u8; RISCV_PAGESIZE];

#[inline(always)]
fn round_page(x: usize) -> usize {
    x & (!(RISCV_PAGESIZE - 1))
}

/// Virtual memory entry that only keeps track of established memory mapping
/// (mostly, if not all, created via mmap)
#[derive(Clone)]
pub struct VirtualMemoryEntry {
    addr: usize,
    size: usize,
    prot: u32,
    source: Option<Rc<Box<[u8]>>>,
    offset: usize,
    /// This reference count value is used to implement a handy trick:
    /// a virtual memory entry can store multiple pages, and it should only
    /// be discarded once all memory pages are unmapped. However, parameters
    /// of munmap() don't necessary need to match parameters of mmap(). In
    /// other words, it's totally fine to mmap 5 pages, but only unmap 3 of
    /// them later. This means we will need to keep track of how many pages
    /// are unmapped in this entry.
    /// That being said, tracking exactly what pages are unmapped is an expensive
    /// operation, so instead of tracking what pages are unmapped, we are only
    /// tracking how many pages are still mmapped using this reference count
    /// value. When it reaches zero, we assume no pages are still mmapped, and
    /// it's safe to destroy current virtual memory entry.
    /// Obviously this doesn't cover all cases, but it's worth pointing out that
    /// it's unspecified behavior to munmap() a page that's not established via
    /// mmap(). So in practice it will be safe to leverage this trick here. All
    /// we need to make sure is to give a hard upper bound on the number of virtual
    /// memory entrys, and halt as soon as this upper bound is reached.
    refcount: usize,
}

/// An MMU implementation with proper protection schemes. Notice this is a correct
/// soft MMU, not necessarily a fast MMU. For the best performance, we should
/// leverage native mmap() calls instead to avoid overheads.
pub struct Mmu {
    vms: Vec<Option<VirtualMemoryEntry>>,
    // A real machine would use 2-level page table, but since here we are dealing
    // with 128MB maximum memory, which is just 32K pages, we can just use a single
    // linear array to boost performance.
    page_table: [u8; MAX_PAGES],
    // key is the start address of the page
    pages: HashMap<usize, Page>,
}

fn handle_page_fault(vm: &VirtualMemoryEntry, addr: usize) -> Result<Page, Error> {
    let mut page = [0; RISCV_PAGESIZE];
    if let Some(ref source) = vm.source {
        let offset = vm.offset + (addr - vm.addr);
        let copied_size = min(RISCV_PAGESIZE, source.len() - offset);
        let (left, _) = page.split_at_mut(copied_size);
        left.copy_from_slice(&source[offset..offset + copied_size]);
    }
    // The page is already initialized with all zeros, so there's no
    // need for us to fill the remaining part as 0 even if source doesn't
    // fill everything
    Ok(page)
}

impl Mmu {
    pub fn new() -> Mmu {
        Mmu {
            vms: vec![None; MAX_VIRTUAL_MEMORY_ENTRIES],
            page_table: [INVALID_PAGE_INDEX; MAX_PAGES],
            pages: HashMap::new(),
        }
    }

    fn munmap_aligned(&mut self, aligned_addr: usize, aligned_size: usize) -> Result<(), Error> {
        let mut current_addr = aligned_addr;
        while current_addr < aligned_addr + aligned_size {
            let current_page = current_addr / RISCV_PAGESIZE;
            let index = self.page_table[current_page];
            if index != INVALID_PAGE_INDEX {
                self.page_table[current_page] = INVALID_PAGE_INDEX;
                self.pages.remove(&current_addr);
                let mut destroy = false;
                if let Some(ref mut vm) = self.vms[index as usize] {
                    vm.refcount -= 1;
                    if vm.refcount == 0 {
                        destroy = true;
                    }
                }
                if destroy {
                    self.vms[index as usize] = None;
                }
            }
            current_addr += RISCV_PAGESIZE;
        }
        Ok(())
    }

    fn fetch_page(&mut self, addr: usize, prot: u32) -> Result<&mut Page, Error> {
        let page = addr / RISCV_PAGESIZE;
        if page > self.page_table.len() {
            return Err(Error::OutOfBound);
        }
        let idx = self.page_table[addr / RISCV_PAGESIZE];
        match self.vms.get(idx as usize) {
            Some(Some(vm)) => {
                if vm.prot & prot != prot {
                    return Err(Error::InvalidPermission);
                }
                let page = match self.pages.entry(addr) {
                    Vacant(entry) => {
                        let page = handle_page_fault(vm, addr)?;
                        entry.insert(page)
                    }
                    Occupied(entry) => entry.into_mut(),
                };
                Ok(page)
            }
            _ => Err(Error::OutOfBound),
        }
    }

    fn load(&mut self, addr: usize, bytes: usize, prot: u32) -> Result<u32, Error> {
        debug_assert!(bytes == 1 || bytes == 2 || bytes == 4);
        let page_addr = round_page(addr);
        let first_page_bytes = min(bytes, RISCV_PAGESIZE - (addr - page_addr));
        let mut shift = 0;
        let mut value: u32 = 0;
        {
            let page = self.fetch_page(page_addr, prot)?;
            for i in 0..first_page_bytes {
                value |= u32::from(page[addr - page_addr + i]) << shift;
                shift += 8;
            }
        }
        let second_page_bytes = bytes - first_page_bytes;
        if second_page_bytes > 0 {
            let second_page = self.fetch_page(page_addr + RISCV_PAGESIZE, PROT_READ)?;
            for &byte in second_page.iter().take(second_page_bytes) {
                value |= u32::from(byte) << shift;
                shift += 8;
            }
        }
        Ok(value)
    }
}

impl Memory for Mmu {
    fn mmap(
        &mut self,
        addr: usize,
        size: usize,
        prot: u32,
        source: Option<Rc<Box<[u8]>>>,
        offset: usize,
    ) -> Result<(), Error> {
        if addr & (RISCV_PAGESIZE - 1) != 0 || size & (RISCV_PAGESIZE - 1) != 0 {
            return Err(Error::Unaligned);
        }
        let mut i = 0;
        while i < self.vms.len() {
            if self.vms[i].is_none() {
                break;
            }
            i += 1;
        }
        if i == self.vms.len() {
            return Err(Error::MaximumMmappingReached);
        }
        debug_assert!(i < MAX_VIRTUAL_MEMORY_ENTRIES);
        // This one is for extra caution, even though right now
        // MAX_VIRTUAL_MEMORY_ENTRIES is 64, which is less than 0xFF, this
        // extra guard can help future-proofing the logic
        debug_assert!(i < 0xFF);
        let pages = size / RISCV_PAGESIZE;
        self.vms[i] = Some(VirtualMemoryEntry {
            addr,
            size,
            prot,
            source,
            offset,
            refcount: pages,
        });
        let mut current_addr = addr;
        while current_addr < addr + size {
            let current_page = current_addr / RISCV_PAGESIZE;
            // munmap overlapped pages
            if self.page_table[current_page] != INVALID_PAGE_INDEX {
                self.munmap_aligned(current_addr, RISCV_PAGESIZE)?;
            }
            self.page_table[current_page] = i as u8;
            current_addr += RISCV_PAGESIZE;
        }
        Ok(())
    }

    fn munmap(&mut self, addr: usize, size: usize) -> Result<(), Error> {
        if addr & (RISCV_PAGESIZE - 1) != 0 || size & (RISCV_PAGESIZE - 1) != 0 {
            return Err(Error::Unaligned);
        }
        self.munmap_aligned(addr, size)
    }

    fn load8(&mut self, addr: usize) -> Result<u8, Error> {
        self.load(addr, 1, PROT_READ).map(|v| v as u8)
    }

    fn load16(&mut self, addr: usize) -> Result<u16, Error> {
        self.load(addr, 2, PROT_READ).map(|v| v as u16)
    }

    fn load32(&mut self, addr: usize) -> Result<u32, Error> {
        self.load(addr, 4, PROT_READ)
    }

    fn execute_load16(&mut self, addr: usize) -> Result<u16, Error> {
        self.load(addr, 2, PROT_EXEC).map(|v| v as u16)
    }

    fn store_bytes(&mut self, addr: usize, value: &[u8]) -> Result<(), Error> {
        let mut copied_bytes = 0;
        let mut current_page_addr = round_page(addr);
        let mut current_page_offset = addr - current_page_addr;
        while copied_bytes < value.len() {
            let page = self.fetch_page(current_page_addr, PROT_WRITE)?;
            let bytes = min(
                RISCV_PAGESIZE - current_page_offset,
                value.len() - copied_bytes,
            );
            let (_, right) = page.split_at_mut(current_page_offset);
            let (slice, _) = right.split_at_mut(bytes);
            slice.copy_from_slice(&value[copied_bytes..copied_bytes + bytes]);

            copied_bytes += bytes;
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
}

impl Default for Mmu {
    fn default() -> Self {
        Self::new()
    }
}
