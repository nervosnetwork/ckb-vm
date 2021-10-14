use super::super::{Error, Register, RISCV_MAX_MEMORY, RISCV_PAGESIZE};
use super::{
    check_permission, get_page_indices, round_page_down, round_page_up, Memory, FLAG_EXECUTABLE,
    FLAG_FREEZED, FLAG_WRITABLE,
};

use bytes::Bytes;

pub struct WXorXMemory<M: Memory> {
    inner: M,
}

impl<M: Memory + Default> Default for WXorXMemory<M> {
    fn default() -> Self {
        Self {
            inner: M::default(),
        }
    }
}

impl<M: Memory> WXorXMemory<M> {
    pub fn inner_mut(&mut self) -> &mut M {
        &mut self.inner
    }
}

impl<M: Memory> Memory for WXorXMemory<M> {
    type REG = M::REG;

    fn init_pages(
        &mut self,
        addr: u64,
        size: u64,
        flags: u8,
        source: Option<Bytes>,
        offset_from_addr: u64,
    ) -> Result<(), Error> {
        if round_page_down(addr) != addr || round_page_up(size) != size {
            return Err(Error::MemPageUnalignedAccess);
        }
        if addr > RISCV_MAX_MEMORY as u64
            || size > RISCV_MAX_MEMORY as u64
            || addr + size > RISCV_MAX_MEMORY as u64
            || offset_from_addr > size
        {
            return Err(Error::MemOutOfBound);
        }
        for page_addr in (addr..addr + size).step_by(RISCV_PAGESIZE) {
            let page = page_addr / RISCV_PAGESIZE as u64;
            if self.fetch_flag(page)? & FLAG_FREEZED != 0 {
                return Err(Error::MemWriteOnExecutablePage);
            }
            self.set_flag(page, flags)?;
        }
        self.inner
            .init_pages(addr, size, flags, source, offset_from_addr)
    }

    fn fetch_flag(&mut self, page: u64) -> Result<u8, Error> {
        self.inner.fetch_flag(page)
    }

    fn set_flag(&mut self, page: u64, flag: u8) -> Result<(), Error> {
        self.inner.set_flag(page, flag)
    }

    fn clear_flag(&mut self, page: u64, flag: u8) -> Result<(), Error> {
        self.inner.clear_flag(page, flag)
    }

    fn execute_load16(&mut self, addr: u64) -> Result<u16, Error> {
        let page_indices = get_page_indices(addr, 2)?;
        check_permission(self, &page_indices, FLAG_EXECUTABLE)?;
        self.inner.execute_load16(addr)
    }

    fn execute_load32(&mut self, addr: u64) -> Result<u32, Error> {
        let page_indices = get_page_indices(addr, 4)?;
        check_permission(self, &page_indices, FLAG_EXECUTABLE)?;
        self.inner.execute_load32(addr)
    }

    fn load_bytes(&mut self, addr: u64, size: u64) -> Result<Vec<u8>, Error> {
        self.inner.load_bytes(addr, size)
    }

    fn load8(&mut self, addr: &Self::REG) -> Result<Self::REG, Error> {
        self.inner.load8(addr)
    }

    fn load16(&mut self, addr: &Self::REG) -> Result<Self::REG, Error> {
        self.inner.load16(addr)
    }

    fn load32(&mut self, addr: &Self::REG) -> Result<Self::REG, Error> {
        self.inner.load32(addr)
    }

    fn load64(&mut self, addr: &Self::REG) -> Result<Self::REG, Error> {
        self.inner.load64(addr)
    }

    fn store8(&mut self, addr: &Self::REG, value: &Self::REG) -> Result<(), Error> {
        let page_indices = get_page_indices(addr.to_u64(), 1)?;
        check_permission(self, &page_indices, FLAG_WRITABLE)?;
        self.inner.store8(addr, value)
    }

    fn store16(&mut self, addr: &Self::REG, value: &Self::REG) -> Result<(), Error> {
        let page_indices = get_page_indices(addr.to_u64(), 2)?;
        check_permission(self, &page_indices, FLAG_WRITABLE)?;
        self.inner.store16(addr, value)
    }

    fn store32(&mut self, addr: &Self::REG, value: &Self::REG) -> Result<(), Error> {
        let page_indices = get_page_indices(addr.to_u64(), 4)?;
        check_permission(self, &page_indices, FLAG_WRITABLE)?;
        self.inner.store32(addr, value)
    }

    fn store64(&mut self, addr: &Self::REG, value: &Self::REG) -> Result<(), Error> {
        let page_indices = get_page_indices(addr.to_u64(), 8)?;
        check_permission(self, &page_indices, FLAG_WRITABLE)?;
        self.inner.store64(addr, value)
    }

    fn store_bytes(&mut self, addr: u64, value: &[u8]) -> Result<(), Error> {
        if value.is_empty() {
            return Ok(());
        }
        let page_indices = get_page_indices(addr, value.len() as u64)?;
        check_permission(self, &page_indices, FLAG_WRITABLE)?;
        self.inner.store_bytes(addr, value)
    }

    fn store_byte(&mut self, addr: u64, size: u64, value: u8) -> Result<(), Error> {
        if size == 0 {
            return Ok(());
        }
        let page_indices = get_page_indices(addr, size)?;
        check_permission(self, &page_indices, FLAG_WRITABLE)?;
        self.inner.store_byte(addr, size, value)
    }
}
