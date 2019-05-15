use super::super::{
    bits::{rounddown, roundup},
    Error, Register, RISCV_MAX_MEMORY, RISCV_PAGESIZE,
};
use super::{Memory, FLAG_EXECUTABLE, FLAG_FREEZED, FLAG_WRITABLE, FLAG_WXORX_BIT};

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
            flags: vec![0; RISCV_MAX_MEMORY / RISCV_PAGESIZE],
            _inner: PhantomData,
        }
    }
}

impl<R: Register, M: Memory<R>> WXorXMemory<R, M> {
    fn check_permission(&self, addr: usize, size: usize, flag: u8) -> Result<(), Error> {
        let e = addr + size;
        let mut current_addr = rounddown(addr, RISCV_PAGESIZE);
        while current_addr < e {
            let page = current_addr / RISCV_PAGESIZE;
            self.flags
                .get(page)
                .ok_or(Error::OutOfBound)
                .and_then(|page_flag| {
                    if (page_flag & FLAG_WXORX_BIT) == (flag & FLAG_WXORX_BIT) {
                        Ok(())
                    } else {
                        Err(Error::InvalidPermission)
                    }
                })?;
            current_addr += RISCV_PAGESIZE;
        }
        Ok(())
    }
}

impl<R: Register, M: Memory<R>> Memory<R> for WXorXMemory<R, M> {
    fn init_pages(
        &mut self,
        addr: usize,
        size: usize,
        flags: u8,
        source: Option<Bytes>,
        offset_from_addr: usize,
    ) -> Result<(), Error> {
        if rounddown(addr, RISCV_PAGESIZE) != addr || roundup(size, RISCV_PAGESIZE) != size {
            return Err(Error::Unaligned);
        }
        if addr > RISCV_MAX_MEMORY
            || size > RISCV_MAX_MEMORY
            || addr + size > RISCV_MAX_MEMORY
            || offset_from_addr > size
        {
            return Err(Error::OutOfBound);
        }
        for page_addr in (addr..addr + size).step_by(RISCV_PAGESIZE) {
            let page = page_addr / RISCV_PAGESIZE;
            if self.flags[page] & FLAG_FREEZED != 0 {
                return Err(Error::InvalidPermission);
            }
            self.flags[page] = flags;
        }
        self.inner
            .init_pages(addr, size, flags, source, offset_from_addr)
    }

    fn execute_load16(&mut self, addr: usize) -> Result<u16, Error> {
        self.check_permission(addr, 2, FLAG_EXECUTABLE)?;
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
        self.check_permission(addr.to_usize(), 1, FLAG_WRITABLE)?;
        self.inner.store8(addr, value)
    }

    fn store16(&mut self, addr: &R, value: &R) -> Result<(), Error> {
        self.check_permission(addr.to_usize(), 2, FLAG_WRITABLE)?;
        self.inner.store16(addr, value)
    }

    fn store32(&mut self, addr: &R, value: &R) -> Result<(), Error> {
        self.check_permission(addr.to_usize(), 4, FLAG_WRITABLE)?;
        self.inner.store32(addr, value)
    }

    fn store64(&mut self, addr: &R, value: &R) -> Result<(), Error> {
        self.check_permission(addr.to_usize(), 8, FLAG_WRITABLE)?;
        self.inner.store64(addr, value)
    }

    fn store_bytes(&mut self, addr: usize, value: &[u8]) -> Result<(), Error> {
        self.check_permission(addr, value.len(), FLAG_WRITABLE)?;
        self.inner.store_bytes(addr, value)
    }

    fn store_byte(&mut self, addr: usize, size: usize, value: u8) -> Result<(), Error> {
        self.check_permission(addr, size, FLAG_WRITABLE)?;
        self.inner.store_byte(addr, size, value)
    }
}
