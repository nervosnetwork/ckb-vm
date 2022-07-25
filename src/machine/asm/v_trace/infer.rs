use crate::{CoreMachine, Error, Machine, Memory};
use bytes::Bytes;
use ckb_vm_definitions::{ELEN, VLEN};

/// vtype CSR
pub struct Vtype(u32);

impl Vtype {
    pub fn new(v: u32) -> Self {
        let mut r = Self(0);
        r.set(v);
        r
    }

    pub fn set(&mut self, v: u32) {
        // Reserved bits must all be zero
        if (v >> 8) != 0 {
            self.0 = 1 << 31;
            return;
        }
        self.0 = v;

        if self.vlmul() == 0.0625
            || self.vsew() as f64
                > if self.vlmul() > 1.0 {
                    1.0
                } else {
                    self.vlmul()
                } * ELEN as f64
        {
            self.0 = 1 << 31;
        }
    }

    pub fn vill(&self) -> bool {
        self.0 & (1 << 31) != 0
    }

    pub fn vma(&self) -> bool {
        self.0 & (1 << 7) != 0
    }

    pub fn vta(&self) -> bool {
        self.0 & (1 << 6) != 0
    }

    pub fn vsew(&self) -> u64 {
        1 << (((self.0 >> 3) & 0x7) + 3)
    }

    pub fn vlmul(&self) -> f64 {
        match self.0 & 0x7 {
            0b000 => 1.0,
            0b001 => 2.0,
            0b010 => 4.0,
            0b011 => 8.0,
            0b111 => 0.5,
            0b110 => 0.25,
            0b101 => 0.125,
            _ => 0.0625,
        }
    }

    pub fn vlmax(&self) -> u64 {
        if self.vill() {
            0
        } else {
            ((VLEN as u64 / self.vsew()) as f64 * self.vlmul()) as u64
        }
    }
}

/// An abstract machine that is designed to do *typechecking* and
/// vtype inference on V instructions.
pub struct VInferMachine {
    vtype: Vtype,
    pc: u64,
    next_pc: u64,
    registers: [u64; 32],
}

impl Default for VInferMachine {
    fn default() -> Self {
        Self {
            // Start with illegal vtype state
            vtype: Vtype::new(1 << 31),
            pc: 0x1000,
            next_pc: 0,
            registers: [0u64; 32],
        }
    }
}

impl Memory for VInferMachine {
    type REG = u64;

    fn init_pages(
        &mut self,
        _addr: u64,
        _size: u64,
        _flags: u8,
        _source: Option<Bytes>,
        _offset_from_addr: u64,
    ) -> Result<(), Error> {
        unimplemented!()
    }

    fn fetch_flag(&mut self, _page: u64) -> Result<u8, Error> {
        unimplemented!()
    }

    fn set_flag(&mut self, _page: u64, _flag: u8) -> Result<(), Error> {
        unimplemented!()
    }

    fn clear_flag(&mut self, _page: u64, _flag: u8) -> Result<(), Error> {
        unimplemented!()
    }

    fn store_byte(&mut self, _addr: u64, _size: u64, _value: u8) -> Result<(), Error> {
        unimplemented!()
    }

    fn store_bytes(&mut self, _addr: u64, _value: &[u8]) -> Result<(), Error> {
        unimplemented!()
    }

    fn execute_load16(&mut self, _addr: u64) -> Result<u16, Error> {
        unimplemented!()
    }

    fn execute_load32(&mut self, _addr: u64) -> Result<u32, Error> {
        unimplemented!()
    }

    fn load_bytes(&mut self, _addr: u64, _size: u64) -> Result<Vec<u8>, Error> {
        unimplemented!()
    }

    fn load8(&mut self, _addr: &Self::REG) -> Result<Self::REG, Error> {
        unimplemented!()
    }

    fn load16(&mut self, _addr: &Self::REG) -> Result<Self::REG, Error> {
        unimplemented!()
    }

    fn load32(&mut self, _addr: &Self::REG) -> Result<Self::REG, Error> {
        unimplemented!()
    }

    fn load64(&mut self, _addr: &Self::REG) -> Result<Self::REG, Error> {
        unimplemented!()
    }

    fn store8(&mut self, _addr: &Self::REG, _value: &Self::REG) -> Result<(), Error> {
        unimplemented!()
    }

    fn store16(&mut self, _addr: &Self::REG, _value: &Self::REG) -> Result<(), Error> {
        unimplemented!()
    }

    fn store32(&mut self, _addr: &Self::REG, _value: &Self::REG) -> Result<(), Error> {
        unimplemented!()
    }

    fn store64(&mut self, _addr: &Self::REG, _value: &Self::REG) -> Result<(), Error> {
        unimplemented!()
    }
}

impl CoreMachine for VInferMachine {
    type REG = u64;
    type MEM = Self;

    fn pc(&self) -> &Self::REG {
        &self.pc
    }

    fn update_pc(&mut self, pc: Self::REG) {
        self.next_pc = pc;
    }

    fn commit_pc(&mut self) {
        self.pc = self.next_pc;
    }

    fn memory(&self) -> &Self::MEM {
        self
    }

    fn memory_mut(&mut self) -> &mut Self::MEM {
        self
    }

    fn registers(&self) -> &[Self::REG] {
        &self.registers
    }

    fn set_register(&mut self, idx: usize, value: Self::REG) {
        self.registers[idx] = value;
    }

    fn element_ref(&self, _reg: usize, _sew: u64, _n: usize) -> &[u8] {
        unimplemented!()
    }

    fn element_mut(&mut self, _reg: usize, _sew: u64, _n: usize) -> &mut [u8] {
        unimplemented!()
    }

    fn get_bit(&self, _reg: usize, _n: usize) -> bool {
        unimplemented!()
    }

    fn set_bit(&mut self, _reg: usize, _n: usize) {
        unimplemented!()
    }

    fn clr_bit(&mut self, _reg: usize, _n: usize) {
        unimplemented!()
    }

    fn set_vl(&mut self, _rd: usize, _rs1: usize, _avl: u64, new_type: u64) {
        self.vtype.set(new_type as u32);
    }

    fn vl(&self) -> u64 {
        0
    }

    fn vlmax(&self) -> u64 {
        self.vtype.vlmax()
    }

    fn vsew(&self) -> u64 {
        self.vtype.vsew()
    }

    fn vlmul(&self) -> f64 {
        self.vtype.vlmul()
    }

    fn vta(&self) -> bool {
        self.vtype.vta()
    }

    fn vma(&self) -> bool {
        self.vtype.vma()
    }

    fn vill(&self) -> bool {
        self.vtype.vill()
    }

    fn vlenb(&self) -> u64 {
        unimplemented!()
    }

    fn version(&self) -> u32 {
        unimplemented!()
    }

    fn isa(&self) -> u8 {
        unimplemented!()
    }

    fn v_to_mem(&mut self, _reg: usize, _sew: u64, _n: usize, _addr: u64) -> Result<(), Error> {
        unimplemented!()
    }

    fn mem_to_v(&mut self, _reg: usize, _sew: u64, _n: usize, _addr: u64) -> Result<(), Error> {
        unimplemented!()
    }
}

impl Machine for VInferMachine {
    fn ecall(&mut self) -> Result<(), Error> {
        unimplemented!()
    }

    fn ebreak(&mut self) -> Result<(), Error> {
        unimplemented!()
    }
}
