pub use uintxx::{I1024, I256, I512, U1024, U128, U16, U256, U32, U512, U64, U8};

#[derive(Clone, Copy, Debug)]
pub enum VRegister {
    U1024([U1024; 2]),
    U512([U512; 4]),
    U256([U256; 8]),
    U128([U128; 16]),
    U64([U64; 32]),
    U32([U32; 64]),
    U16([U16; 128]),
    U8([U8; 256]),
}

impl Default for VRegister {
    fn default() -> Self {
        VRegister::U8([U8(0x00); 256])
    }
}
