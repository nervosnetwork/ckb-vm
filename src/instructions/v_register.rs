pub use uintxx::{U1024, U128, U16, U256, U32, U512, U64, U8};

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

impl VRegister {
    pub fn to_le_bytes(&self) -> [u8; 256] {
        let mut r = [0x00; 256];
        match self {
            VRegister::U8(data) => {
                for (i, e) in data.iter().enumerate() {
                    r[i] = e.0;
                }
            }
            VRegister::U16(data) => {
                for (i, e) in data.iter().enumerate() {
                    let start = i * 2;
                    let end = (i + 1) * 2;
                    r[start..end].copy_from_slice(&e.0.to_le_bytes());
                }
            }
            VRegister::U32(data) => {
                for (i, e) in data.iter().enumerate() {
                    let start = i * 4;
                    let end = (i + 1) * 4;
                    r[start..end].copy_from_slice(&e.0.to_le_bytes());
                }
            }
            VRegister::U64(data) => {
                for (i, e) in data.iter().enumerate() {
                    let start = i * 8;
                    let end = (i + 1) * 8;
                    r[start..end].copy_from_slice(&e.0.to_le_bytes());
                }
            }
            VRegister::U128(data) => {
                for (i, e) in data.iter().enumerate() {
                    let start = i * 16;
                    let end = (i + 1) * 16;
                    r[start..end].copy_from_slice(&e.0.to_le_bytes());
                }
            }
            VRegister::U256(data) => {
                for (i, e) in data.iter().enumerate() {
                    let start = i * 32;
                    let end = (i + 1) * 32;
                    r[start..end].copy_from_slice(&e.to_le_bytes());
                }
            }
            VRegister::U512(data) => {
                for (i, e) in data.iter().enumerate() {
                    let start = i * 64;
                    let end = (i + 1) * 64;
                    r[start..end].copy_from_slice(&e.to_le_bytes());
                }
            }
            VRegister::U1024(data) => {
                for (i, e) in data.iter().enumerate() {
                    let start = i * 128;
                    let end = (i + 1) * 128;
                    r[start..end].copy_from_slice(&e.to_le_bytes());
                }
            }
        }
        r
    }

    pub fn from_le_bytes(vsew: u32, le_byte: [u8; 256]) -> Self {
        match vsew {
            8 => {
                let mut r = [U8::default(); 256];
                for i in 0..256 {
                    r[i] = U8(le_byte[i])
                }
                VRegister::U8(r)
            }
            16 => {
                let mut buf = [0x00; 2];
                let mut r = [U16::default(); 128];
                for i in 0..128 {
                    buf.copy_from_slice(&le_byte[i * 2..(i + 1) * 2]);
                    r[i] = U16::from_le_bytes(buf);
                }
                VRegister::U16(r)
            }
            32 => {
                let mut buf = [0x00; 4];
                let mut r = [U32::default(); 64];
                for i in 0..64 {
                    buf.copy_from_slice(&le_byte[i * 4..(i + 1) * 4]);
                    r[i] = U32::from_le_bytes(buf);
                }
                VRegister::U32(r)
            }
            64 => {
                let mut buf = [0x00; 8];
                let mut r = [U64::default(); 32];
                for i in 0..32 {
                    buf.copy_from_slice(&le_byte[i * 8..(i + 1) * 8]);
                    r[i] = U64::from_le_bytes(buf);
                }
                VRegister::U64(r)
            }
            128 => {
                let mut buf = [0x00; 16];
                let mut r = [U128::default(); 16];
                for i in 0..16 {
                    buf.copy_from_slice(&le_byte[i * 16..(i + 1) * 16]);
                    r[i] = U128::from_le_bytes(buf);
                }
                VRegister::U128(r)
            }
            256 => {
                let mut buf = [0x00; 32];
                let mut r = [U256::default(); 8];
                for i in 0..8 {
                    buf.copy_from_slice(&le_byte[i * 32..(i + 1) * 32]);
                    r[i] = U256::from_le_bytes(buf);
                }
                VRegister::U256(r)
            }
            512 => {
                let mut buf = [0x00; 64];
                let mut r = [U512::default(); 4];
                for i in 0..4 {
                    buf.copy_from_slice(&le_byte[i * 64..(i + 1) * 64]);
                    r[i] = U512::from_le_bytes(buf);
                }
                VRegister::U512(r)
            }
            1024 => {
                let mut buf = [0x00; 128];
                let mut r = [U1024::default(); 2];
                for i in 0..2 {
                    buf.copy_from_slice(&le_byte[i * 128..(i + 1) * 128]);
                    r[i] = U1024::from_le_bytes(buf);
                }
                VRegister::U1024(r)
            }
            _ => unreachable!(),
        }
    }
}
