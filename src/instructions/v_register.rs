use ckb_vm_definitions::VLEN;
pub use uintxx::{U1024, U128, U16, U256, U32, U512, U64, U8};

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
}
