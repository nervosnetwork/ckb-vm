use super::super::{Error, Register, RISCV_MAX_MEMORY, RISCV_PAGESIZE};
use super::{round_page, Memory, Page, PROT_EXEC, PROT_READ, PROT_WRITE};

use std::cmp::min;
use std::marker::PhantomData;
use std::ptr;
use std::rc::Rc;

pub const MAX_VIRTUAL_MEMORY_ENTRIES: usize = 64;

const MAX_PAGES: usize = RISCV_MAX_MEMORY / RISCV_PAGESIZE;
const MAX_TLB_ENTRIES: usize = 16;
const INVALID_PAGE_INDEX: u8 = 0xFF;

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

#[derive(Clone, Copy)]
pub struct TlbEntry {
    addr: usize,
    prot: u32,
    page_data_index: usize,
}

/// An MMU implementation with proper protection schemes. Notice this is a correct
/// soft MMU, not necessarily a fast MMU. For the best performance, we should
/// leverage native mmap() calls instead to avoid overheads.
pub struct Mmu<R> {
    vms: Vec<Option<VirtualMemoryEntry>>,
    // Page table that stores indices of VirtualMemoryEntry for each page. We are
    // using u8 here to save some memory since we have a hard upper bound of 64
    // virtual memory entries, which would fit in a u8 perfectly.
    // A real machine would use 2-level page table, but since here we are dealing
    // with 128MB maximum memory, which is just 32K pages, we can just use a single
    // linear array to boost performance.
    page_table: [u8; MAX_PAGES],
    // Pages that have been requested and instantiated. We are using linear array
    // here since TLB entry needs to reference individual pages here, and using
    // a hash map would complicate lifetime rule a lot. We can come back here later
    // for optimization work if this indeed becomes a bottleneck.
    // The addresses and data here are stored separately to make it more cache
    // friendly when we are searching for the page via address.
    page_addresses: Vec<usize>,
    page_data: Vec<Page>,
    // Translation lookaside buffer
    tlb: [Option<TlbEntry>; MAX_TLB_ENTRIES],
    _inner: PhantomData<R>,
}

// Generates a new page based on VM mapping, also copy mapped data to the page
// if exists. Notice every page generated here would be zero-filled first.
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

impl<R> Mmu<R> {
    pub fn new() -> Mmu<R> {
        Mmu {
            vms: vec![None; MAX_VIRTUAL_MEMORY_ENTRIES],
            page_table: [INVALID_PAGE_INDEX; MAX_PAGES],
            page_addresses: Vec::new(),
            page_data: Vec::new(),
            tlb: [None; MAX_TLB_ENTRIES],
            _inner: PhantomData,
        }
    }

    // Finds requested page index in page_addresses data structure, the same
    // index could also be used on page_data vec.
    fn find_page_data_index(&self, addr: usize) -> Option<usize> {
        self.page_addresses.iter().position(|a| *a == addr)
    }

    fn munmap_aligned(&mut self, aligned_addr: usize, aligned_size: usize) -> Result<(), Error> {
        for current_addr in (aligned_addr..aligned_addr + aligned_size).step_by(RISCV_PAGESIZE) {
            let current_page = current_addr / RISCV_PAGESIZE;
            let vm_index = self.page_table[current_page];
            if vm_index != INVALID_PAGE_INDEX {
                self.page_table[current_page] = INVALID_PAGE_INDEX;
                let page_data_index = self.find_page_data_index(current_addr);
                if let Some(index) = page_data_index {
                    // page_addresses and page_data share the same indices
                    // for each item, so it's totally safe to do this.
                    self.page_addresses.swap_remove(index);
                    self.page_data.swap_remove(index);
                }
                let tlb_index = current_page % MAX_TLB_ENTRIES;
                let tlb_entry = self.tlb[tlb_index];
                if let Some(ref tlb) = tlb_entry {
                    if tlb.addr == current_addr {
                        self.tlb[tlb_index] = None;
                    }
                }
                let mut destroy_vm_entry = false;
                if let Some(ref mut vm) = self.vms[vm_index as usize] {
                    vm.refcount -= 1;
                    if vm.refcount == 0 {
                        destroy_vm_entry = true;
                    }
                }
                if destroy_vm_entry {
                    self.vms[vm_index as usize] = None;
                }
            }
        }
        Ok(())
    }

    fn fetch_page(&mut self, addr: usize, prot: u32) -> Result<&mut Page, Error> {
        let page = addr / RISCV_PAGESIZE;
        if page > self.page_table.len() {
            return Err(Error::OutOfBound);
        }
        // Try looking TLB first for a fast path
        let tlb_index = page % MAX_TLB_ENTRIES;
        if let Some(entry) = self.tlb[tlb_index] {
            if entry.addr == addr {
                if entry.prot & prot == prot {
                    return Ok(&mut self.page_data[entry.page_data_index]);
                } else {
                    return Err(Error::InvalidPermission);
                }
            }
        }
        // If TLB entry is missing, try looking it from existing instantiated
        // page data. If it still fails, do a full page fault.
        let vm_index = self.page_table[addr / RISCV_PAGESIZE];
        match self.vms.get(vm_index as usize) {
            Some(Some(vm)) => {
                if vm.prot & prot != prot {
                    return Err(Error::InvalidPermission);
                }
                let page_data_index = match self.find_page_data_index(addr) {
                    Some(index) => index,
                    None => {
                        // Do a full page fault here
                        let page = handle_page_fault(vm, addr)?;
                        self.page_addresses.push(addr);
                        self.page_data.push(page);
                        self.page_data.len() - 1
                    }
                };
                self.tlb[tlb_index] = Some(TlbEntry {
                    addr,
                    prot: vm.prot,
                    page_data_index,
                });
                Ok(&mut self.page_data[page_data_index])
            }
            _ => Err(Error::OutOfBound),
        }
    }

    fn load(&mut self, addr: usize, bytes: usize, prot: u32) -> Result<u64, Error> {
        debug_assert!(bytes == 1 || bytes == 2 || bytes == 4 || bytes == 8);
        let page_addr = round_page(addr);
        let first_page_bytes = min(bytes, RISCV_PAGESIZE - (addr - page_addr));
        let mut shift = 0;
        let mut value: u64 = 0;
        {
            let page = self.fetch_page(page_addr, prot)?;
            for i in 0..first_page_bytes {
                value |= u64::from(page[addr - page_addr + i]) << shift;
                shift += 8;
            }
        }
        let second_page_bytes = bytes - first_page_bytes;
        if second_page_bytes > 0 {
            let second_page = self.fetch_page(page_addr + RISCV_PAGESIZE, PROT_READ)?;
            for &byte in second_page.iter().take(second_page_bytes) {
                value |= u64::from(byte) << shift;
                shift += 8;
            }
        }
        Ok(value)
    }
}

impl<R: Register> Memory<R> for Mmu<R> {
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
        let position = self.vms.iter().position(|vm| vm.is_none());
        if let Some(i) = position {
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
            for current_addr in (addr..addr + size).step_by(RISCV_PAGESIZE) {
                let current_page = current_addr / RISCV_PAGESIZE;
                // munmap overlapped pages
                if self.page_table[current_page] != INVALID_PAGE_INDEX {
                    self.munmap_aligned(current_addr, RISCV_PAGESIZE)?;
                }
                self.page_table[current_page] = i as u8;
            }
            Ok(())
        } else {
            Err(Error::MaximumMmappingReached)
        }
    }

    fn munmap(&mut self, addr: usize, size: usize) -> Result<(), Error> {
        if addr & (RISCV_PAGESIZE - 1) != 0 || size & (RISCV_PAGESIZE - 1) != 0 {
            return Err(Error::Unaligned);
        }
        self.munmap_aligned(addr, size)
    }

    fn load8(&mut self, addr: &R) -> Result<R, Error> {
        let v = self.load(addr.to_usize(), 1, PROT_READ).map(|v| v as u8)?;
        Ok(R::from_u8(v))
    }

    fn load16(&mut self, addr: &R) -> Result<R, Error> {
        let v = self.load(addr.to_usize(), 2, PROT_READ).map(|v| v as u16)?;
        Ok(R::from_u16(v))
    }

    fn load32(&mut self, addr: &R) -> Result<R, Error> {
        let v = self.load(addr.to_usize(), 4, PROT_READ).map(|v| v as u32)?;
        Ok(R::from_u32(v))
    }

    fn load64(&mut self, addr: &R) -> Result<R, Error> {
        let v = self.load(addr.to_usize(), 8, PROT_READ)?;
        Ok(R::from_u64(v))
    }

    fn execute_load16(&mut self, addr: usize) -> Result<u16, Error> {
        self.load(addr, 2, PROT_EXEC).map(|v| v as u16)
    }

    fn store_bytes(&mut self, addr: usize, value: &[u8]) -> Result<(), Error> {
        let mut remaining_data = value;
        let mut current_page_addr = round_page(addr);
        let mut current_page_offset = addr - current_page_addr;
        while !remaining_data.is_empty() {
            let page = self.fetch_page(current_page_addr, PROT_WRITE)?;
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
            let page = self.fetch_page(current_page_addr, PROT_WRITE)?;
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

impl<R> Default for Mmu<R> {
    fn default() -> Self {
        Self::new()
    }
}
