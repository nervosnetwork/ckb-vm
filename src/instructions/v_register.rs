use ckb_vm_definitions::VLEN;
pub use eint::{E1024, E128, E16, E256, E32, E512, E64, E8};

pub struct RegisterFile([u8; (VLEN >> 3) * 32]);

impl Default for RegisterFile {
    fn default() -> Self {
        Self([0; (VLEN >> 3) * 32])
    }
}

impl RegisterFile {
    pub fn element_ref(&self, reg: usize, sew: u64, n: usize) -> &[u8] {
        let lb = (sew as usize) >> 3;
        let i0 = reg * (VLEN >> 3) + lb * n;
        let i1 = i0 + lb;
        &self.0[i0..i1]
    }

    pub fn element_mut(&mut self, reg: usize, sew: u64, n: usize) -> &mut [u8] {
        let lb = (sew as usize) >> 3;
        let i0 = reg * (VLEN >> 3) + lb * n;
        let i1 = i0 + lb;
        &mut self.0[i0..i1]
    }

    pub fn get_bit(&self, reg: usize, n: usize) -> bool {
        let n = reg * VLEN + n;
        (self.0[n / 8] << (7 - n % 8) >> 7) != 0
    }

    pub fn set_bit(&mut self, reg: usize, n: usize) {
        let n = reg * VLEN + n;
        self.0[n / 8] |= 1 << (n % 8)
    }

    pub fn clr_bit(&mut self, reg: usize, n: usize) {
        let n = reg * VLEN + n;
        self.0[n / 8] &= !(1 << (n % 8))
    }
}
