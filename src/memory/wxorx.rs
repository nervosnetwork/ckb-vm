use super::super::{
    bits::{rounddown, roundup},
    Error, Register, RISCV_MAX_MEMORY, RISCV_PAGES, RISCV_PAGESIZE,
};
use super::{check_permission, Memory, FLAG_EXECUTABLE, FLAG_FREEZED, FLAG_WRITABLE};

use bytes::Bytes;
use std::marker::PhantomData;

pub struct WXorXMemory<R: Register, M: Memory<R>> {
    inner: M,
    flags: Vec<u8>,
    _inner: PhantomData<R>,
}

impl<R: Register, M: Memory<R> + Default> Default for WXorXMemory<R, M> {
    fn default() -> Self {
        Self {
            inner: M::default(),
            flags: vec![0; RISCV_PAGES],
            _inner: PhantomData,
        }
    }
}

impl<R: Register, M: Memory<R>> Memory<R> for WXorXMemory<R, M> {
    fn init_pages(
        &mut self,
        addr: u64,
        size: u64,
        flags: u8,
        source: Option<Bytes>,
        offset_from_addr: u64,
    ) -> Result<(), Error> {
        if rounddown(addr, RISCV_PAGESIZE as u64) != addr
            || roundup(size, RISCV_PAGESIZE as u64) != size
        {
            return Err(Error::Unaligned);
        }
        if addr > RISCV_MAX_MEMORY as u64
            || size > RISCV_MAX_MEMORY as u64
            || addr + size > RISCV_MAX_MEMORY as u64
            || offset_from_addr > size
        {
            return Err(Error::OutOfBound);
        }
        for page_addr in (addr..addr + size).step_by(RISCV_PAGESIZE) {
            let page = page_addr as usize / RISCV_PAGESIZE;
            if self.flags[page] & FLAG_FREEZED != 0 {
                return Err(Error::InvalidPermission);
            }
            self.flags[page] = flags;
        }
        self.inner
            .init_pages(addr, size, flags, source, offset_from_addr)
    }

    fn fetch_flag(&mut self, page: u64) -> Result<u8, Error> {
        if page < RISCV_PAGES as u64 {
            Ok(self.flags[page as usize])
        } else {
            Err(Error::OutOfBound)
        }
    }

    fn execute_load16(&mut self, addr: u64) -> Result<u16, Error> {
        check_permission(self, addr, 2, FLAG_EXECUTABLE)?;
        self.inner.execute_load16(addr)
    }

    fn load8(&mut self, addr: &R) -> Result<R, Error> {
        self.inner.load8(addr)
    }

    fn load16(&mut self, addr: &R) -> Result<R, Error> {
        self.inner.load16(addr)
    }

    fn load32(&mut self, addr: &R) -> Result<R, Error> {
        self.inner.load32(addr)
    }

    fn load64(&mut self, addr: &R) -> Result<R, Error> {
        self.inner.load64(addr)
    }

    fn store8(&mut self, addr: &R, value: &R) -> Result<(), Error> {
        check_permission(self, addr.to_u64(), 1, FLAG_WRITABLE)?;
        self.inner.store8(addr, value)
    }

    fn store16(&mut self, addr: &R, value: &R) -> Result<(), Error> {
        check_permission(self, addr.to_u64(), 2, FLAG_WRITABLE)?;
        self.inner.store16(addr, value)
    }

    fn store32(&mut self, addr: &R, value: &R) -> Result<(), Error> {
        check_permission(self, addr.to_u64(), 4, FLAG_WRITABLE)?;
        self.inner.store32(addr, value)
    }

    fn store64(&mut self, addr: &R, value: &R) -> Result<(), Error> {
        check_permission(self, addr.to_u64(), 8, FLAG_WRITABLE)?;
        self.inner.store64(addr, value)
    }

    fn store_bytes(&mut self, addr: u64, value: &[u8]) -> Result<(), Error> {
        check_permission(self, addr, value.len() as u64, FLAG_WRITABLE)?;
        self.inner.store_bytes(addr, value)
    }

    fn store_byte(&mut self, addr: u64, size: u64, value: u8) -> Result<(), Error> {
        check_permission(self, addr, size, FLAG_WRITABLE)?;
        self.inner.store_byte(addr, size, value)
    }
}
