use crate::error::Error;
use uintxx::Element;
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

pub fn vfunc_add_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(b[i]);
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(b[i]);
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(b[i]);
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(b[i]);
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(b[i]);
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(b[i]);
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(b[i]);
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(b[i]);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_add_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(U1024::vx(rhs));
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(U512::vx(rhs));
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(U256::vx(rhs));
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(U128::vx(rhs));
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(U64::vx(rhs));
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(U32::vx(rhs));
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(U16::vx(rhs));
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(U8::vx(rhs));
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_add_vi(
    lhs: &VRegister,
    imm: i32,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(U1024::vi(imm));
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(U512::vi(imm));
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(U256::vi(imm));
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(U128::vi(imm));
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(U64::vi(imm));
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(U32::vi(imm));
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(U16::vi(imm));
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(U8::vi(imm));
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_sub_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(b[i]);
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(b[i]);
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(b[i]);
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(b[i]);
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(b[i]);
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(b[i]);
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(b[i]);
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(b[i]);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_sub_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(U1024::vx(rhs));
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(U512::vx(rhs));
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(U256::vx(rhs));
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(U128::vx(rhs));
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(U64::vx(rhs));
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(U32::vx(rhs));
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(U16::vx(rhs));
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(U8::vx(rhs));
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_rsub_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = U1024::vx(rhs).wrapping_sub(a[i]);
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = U512::vx(rhs).wrapping_sub(a[i]);
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = U256::vx(rhs).wrapping_sub(a[i]);
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = U128::vx(rhs).wrapping_sub(a[i]);
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = U64::vx(rhs).wrapping_sub(a[i]);
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = U32::vx(rhs).wrapping_sub(a[i]);
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = U16::vx(rhs).wrapping_sub(a[i]);
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = U8::vx(rhs).wrapping_sub(a[i]);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_rsub_vi(
    lhs: &VRegister,
    rhs: i32,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = U1024::vi(rhs).wrapping_sub(a[i]);
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = U512::vi(rhs).wrapping_sub(a[i]);
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = U256::vi(rhs).wrapping_sub(a[i]);
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = U128::vi(rhs).wrapping_sub(a[i]);
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = U64::vi(rhs).wrapping_sub(a[i]);
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = U32::vi(rhs).wrapping_sub(a[i]);
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = U16::vi(rhs).wrapping_sub(a[i]);
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = U8::vi(rhs).wrapping_sub(a[i]);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_mul_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(b[i]);
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(b[i]);
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(b[i]);
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(b[i]);
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(b[i]);
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(b[i]);
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(b[i]);
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(b[i]);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_mul_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(U1024::vx(rhs));
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(U512::vx(rhs));
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(U256::vx(rhs));
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(U128::vx(rhs));
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(U64::vx(rhs));
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(U32::vx(rhs));
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(U16::vx(rhs));
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(U8::vx(rhs));
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_sll_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << b[i].u32();
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << b[i].u32();
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << b[i].u32();
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << b[i].u32();
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << b[i].u32();
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << b[i].u32();
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << b[i].u32();
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << b[i].u32();
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_sll_vi(
    lhs: &VRegister,
    rhs: u32,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << rhs;
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << rhs;
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << rhs;
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << rhs;
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << rhs;
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << rhs;
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << rhs;
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << rhs;
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_srl_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> b[i].u32();
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> b[i].u32();
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> b[i].u32();
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> b[i].u32();
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> b[i].u32();
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> b[i].u32();
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> b[i].u32();
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> b[i].u32();
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_srl_vi(
    lhs: &VRegister,
    rhs: u32,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> rhs;
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> rhs;
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> rhs;
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> rhs;
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> rhs;
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> rhs;
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> rhs;
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> rhs;
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_sra_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sra(b[i].u32())
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sra(b[i].u32())
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sra(b[i].u32())
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sra(b[i].u32())
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sra(b[i].u32())
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sra(b[i].u32())
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sra(b[i].u32())
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sra(b[i].u32())
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_sra_vi(
    lhs: &VRegister,
    rhs: u32,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sra(rhs)
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sra(rhs)
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sra(rhs)
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sra(rhs)
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sra(rhs)
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sra(rhs)
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sra(rhs)
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sra(rhs)
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

impl Default for VRegister {
    fn default() -> Self {
        VRegister::U8([U8(0x00); 256])
    }
}

pub fn vfunc_divu_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(b[i]);
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(b[i]);
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(b[i]);
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(b[i]);
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(b[i]);
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(b[i]);
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(b[i]);
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(b[i]);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_divu_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(U1024::vx(rhs));
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(U512::vx(rhs));
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(U256::vx(rhs));
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(U128::vx(rhs));
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(U64::vx(rhs));
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(U32::vx(rhs));
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(U16::vx(rhs));
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(U8::vx(rhs));
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_remu_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(b[i]);
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(b[i]);
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(b[i]);
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(b[i]);
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(b[i]);
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(b[i]);
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(b[i]);
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(b[i]);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_remu_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(U1024::vx(rhs));
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(U512::vx(rhs));
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(U256::vx(rhs));
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(U128::vx(rhs));
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(U64::vx(rhs));
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(U32::vx(rhs));
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(U16::vx(rhs));
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(U8::vx(rhs));
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_mseq_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = U1024::from(a[i] == b[i])
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = U512::from(a[i] == b[i])
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = U256::from(a[i] == b[i])
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = U128::from(a[i] == b[i])
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = U64::from(a[i] == b[i])
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = U32::from(a[i] == b[i])
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = U16::from(a[i] == b[i])
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = U8::from(a[i] == b[i])
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_mseq_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = U1024::from(a[i] == U1024::vx(rhs))
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = U512::from(a[i] == U512::vx(rhs))
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = U256::from(a[i] == U256::vx(rhs))
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = U128::from(a[i] == U128::vx(rhs))
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = U64::from(a[i] == U64::vx(rhs))
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = U32::from(a[i] == U32::vx(rhs))
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = U16::from(a[i] == U16::vx(rhs))
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = U8::from(a[i] == U8::vx(rhs))
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_mseq_vi(
    lhs: &VRegister,
    rhs: i32,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = U1024::from(a[i] == U1024::vi(rhs))
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = U512::from(a[i] == U512::vi(rhs))
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = U256::from(a[i] == U256::vi(rhs))
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = U128::from(a[i] == U128::vi(rhs))
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = U64::from(a[i] == U64::vi(rhs))
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = U32::from(a[i] == U32::vi(rhs))
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = U16::from(a[i] == U16::vi(rhs))
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = U8::from(a[i] == U8::vi(rhs))
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msne_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = U1024::from(a[i] != b[i]);
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = U512::from(a[i] != b[i]);
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = U256::from(a[i] != b[i]);
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = U128::from(a[i] != b[i]);
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = U64::from(a[i] != b[i]);
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = U32::from(a[i] != b[i]);
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = U16::from(a[i] != b[i]);
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = U8::from(a[i] != b[i]);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msne_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = U1024::from(a[i] != U1024::vx(rhs));
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = U512::from(a[i] != U512::vx(rhs));
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = U256::from(a[i] != U256::vx(rhs));
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = U128::from(a[i] != U128::vx(rhs));
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = U64::from(a[i] != U64::vx(rhs));
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = U32::from(a[i] != U32::vx(rhs));
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = U16::from(a[i] != U16::vx(rhs));
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = U8::from(a[i] != U8::vx(rhs));
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msne_vi(
    lhs: &VRegister,
    rhs: i32,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = U1024::from(a[i] != U1024::vi(rhs));
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = U512::from(a[i] != U512::vi(rhs));
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = U256::from(a[i] != U256::vi(rhs));
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = U128::from(a[i] != U128::vi(rhs));
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = U64::from(a[i] != U64::vi(rhs));
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = U32::from(a[i] != U32::vi(rhs));
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = U16::from(a[i] != U16::vi(rhs));
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = U8::from(a[i] != U8::vi(rhs));
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msltu_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = U1024::from(a[i] < b[i]);
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = U512::from(a[i] < b[i]);
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = U256::from(a[i] < b[i]);
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = U128::from(a[i] < b[i]);
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = U64::from(a[i] < b[i]);
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = U32::from(a[i] < b[i]);
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = U16::from(a[i] < b[i]);
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = U8::from(a[i] < b[i]);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msltu_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = U1024::from(a[i] < U1024::vx(rhs));
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = U512::from(a[i] < U512::vx(rhs));
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = U256::from(a[i] < U256::vx(rhs));
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = U128::from(a[i] < U128::vx(rhs));
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = U64::from(a[i] < U64::vx(rhs));
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = U32::from(a[i] < U32::vx(rhs));
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = U16::from(a[i] < U16::vx(rhs));
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = U8::from(a[i] < U8::vx(rhs));
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msleu_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = U1024::from(a[i] <= b[i]);
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = U512::from(a[i] <= b[i]);
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = U256::from(a[i] <= b[i]);
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = U128::from(a[i] <= b[i]);
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = U64::from(a[i] <= b[i]);
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = U32::from(a[i] <= b[i]);
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = U16::from(a[i] <= b[i]);
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = U8::from(a[i] <= b[i]);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msleu_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = U1024::from(a[i] <= U1024::vx(rhs));
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = U512::from(a[i] <= U512::vx(rhs));
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = U256::from(a[i] <= U256::vx(rhs));
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = U128::from(a[i] <= U128::vx(rhs));
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = U64::from(a[i] <= U64::vx(rhs));
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = U32::from(a[i] <= U32::vx(rhs));
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = U16::from(a[i] <= U16::vx(rhs));
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = U8::from(a[i] <= U8::vx(rhs));
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msleu_vi(
    lhs: &VRegister,
    rhs: i32,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = U1024::from(a[i] <= U1024::vi(rhs));
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = U512::from(a[i] <= U512::vi(rhs));
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = U256::from(a[i] <= U256::vi(rhs));
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = U128::from(a[i] <= U128::vi(rhs));
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = U64::from(a[i] <= U64::vi(rhs));
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = U32::from(a[i] <= U32::vi(rhs));
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = U16::from(a[i] <= U16::vi(rhs));
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = U8::from(a[i] <= U8::vi(rhs));
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msgtu_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = U1024::from(a[i] > U1024::vx(rhs));
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = U512::from(a[i] > U512::vx(rhs));
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = U256::from(a[i] > U256::vx(rhs));
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = U128::from(a[i] > U128::vx(rhs));
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = U64::from(a[i] > U64::vx(rhs));
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = U32::from(a[i] > U32::vx(rhs));
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = U16::from(a[i] > U16::vx(rhs));
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = U8::from(a[i] > U8::vx(rhs));
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msgtu_vi(
    lhs: &VRegister,
    rhs: i32,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = U1024::from(a[i] > U1024::vi(rhs));
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = U512::from(a[i] > U512::vi(rhs));
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = U256::from(a[i] > U256::vi(rhs));
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = U128::from(a[i] > U128::vi(rhs));
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = U64::from(a[i] > U64::vi(rhs));
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = U32::from(a[i] > U32::vi(rhs));
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = U16::from(a[i] > U16::vi(rhs));
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = U8::from(a[i] > U8::vi(rhs));
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msgt_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = if I1024::from(a[i]) > I1024::from(U1024::from(rhs as i64)) {
                    U1024::ONE
                } else {
                    U1024::MIN
                };
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = if I512::from(a[i]) > I512::from(U512::from(rhs as i64)) {
                    U512::ONE
                } else {
                    U512::MIN
                };
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = if I256::from(a[i]) > I256::from(U256::from(rhs as i64)) {
                    U256::ONE
                } else {
                    U256::MIN
                };
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i].0 as i128 > rhs as i64 as i128 {
                    U128::ONE
                } else {
                    U128::ZERO
                };
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i].0 as i64 > rhs as i64 {
                    U64::ONE
                } else {
                    U64::ZERO
                };
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i].0 as i32 > rhs as u32 as i32 {
                    U32::ONE
                } else {
                    U32::ZERO
                };
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i].0 as i16 > rhs as u16 as i16 {
                    U16::ONE
                } else {
                    U16::ZERO
                };
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i].0 as i8 > rhs as u8 as i8 {
                    U8::ONE
                } else {
                    U8::ZERO
                };
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msgt_vi(
    lhs: &VRegister,
    rhs: i32,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = if I1024::from(a[i]) > I1024::from(U1024::from(rhs)) {
                    U1024::ONE
                } else {
                    U1024::MIN
                };
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = if I512::from(a[i]) > I512::from(U512::from(rhs)) {
                    U512::ONE
                } else {
                    U512::MIN
                };
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = if I256::from(a[i]) > I256::from(U256::from(rhs)) {
                    U256::ONE
                } else {
                    U256::MIN
                };
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i].0 as i128 > rhs as i128 {
                    U128::ONE
                } else {
                    U128::ZERO
                };
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i].0 as i64 > rhs as i64 {
                    U64::ONE
                } else {
                    U64::ZERO
                };
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i].0 as i32 > rhs {
                    U32::ONE
                } else {
                    U32::ZERO
                };
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i].0 as i16 > rhs as i16 {
                    U16::ONE
                } else {
                    U16::ZERO
                };
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i].0 as i8 > rhs as i8 {
                    U8::ONE
                } else {
                    U8::ZERO
                };
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_mslt_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = if I1024::from(a[i]) < I1024::from(b[i]) {
                    U1024::ONE
                } else {
                    U1024::MIN
                };
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = if I512::from(a[i]) < I512::from(b[i]) {
                    U512::ONE
                } else {
                    U512::MIN
                };
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = if I256::from(a[i]) < I256::from(b[i]) {
                    U256::ONE
                } else {
                    U256::MIN
                };
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if (a[i].0 as i128) < (b[i].0 as i128) {
                    U128::ONE
                } else {
                    U128::ZERO
                };
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if (a[i].0 as i64) < (b[i].0 as i64) {
                    U64::ONE
                } else {
                    U64::ZERO
                };
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if (a[i].0 as i32) < (b[i].0 as i32) {
                    U32::ONE
                } else {
                    U32::ZERO
                };
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if (a[i].0 as i16) < (b[i].0 as i16) {
                    U16::ONE
                } else {
                    U16::ZERO
                };
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if (a[i].0 as i8) < (b[i].0 as i8) {
                    U8::ONE
                } else {
                    U8::ZERO
                };
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_mslt_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = if I1024::from(a[i]) < I1024::from(U1024::from(rhs as i64)) {
                    U1024::ONE
                } else {
                    U1024::MIN
                };
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = if I512::from(a[i]) < I512::from(U512::from(rhs as i64)) {
                    U512::ONE
                } else {
                    U512::MIN
                };
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = if I256::from(a[i]) < I256::from(U256::from(rhs as i64)) {
                    U256::ONE
                } else {
                    U256::MIN
                };
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if (a[i].0 as i128) < (rhs as i64 as i128) {
                    U128::ONE
                } else {
                    U128::ZERO
                };
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if (a[i].0 as i64) < (rhs as i64) {
                    U64::ONE
                } else {
                    U64::ZERO
                };
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if (a[i].0 as i32) < (rhs as u32 as i32) {
                    U32::ONE
                } else {
                    U32::ZERO
                };
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if (a[i].0 as i16) < (rhs as u16 as i16) {
                    U16::ONE
                } else {
                    U16::ZERO
                };
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if (a[i].0 as i8) < (rhs as u8 as i8) {
                    U8::ONE
                } else {
                    U8::ZERO
                };
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msle_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = if I1024::from(a[i]) <= I1024::from(b[i]) {
                    U1024::ONE
                } else {
                    U1024::MIN
                };
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = if I512::from(a[i]) <= I512::from(b[i]) {
                    U512::ONE
                } else {
                    U512::MIN
                };
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = if I256::from(a[i]) <= I256::from(b[i]) {
                    U256::ONE
                } else {
                    U256::MIN
                };
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if (a[i].0 as i128) <= (b[i].0 as i128) {
                    U128::ONE
                } else {
                    U128::ZERO
                };
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if (a[i].0 as i64) <= (b[i].0 as i64) {
                    U64::ONE
                } else {
                    U64::ZERO
                };
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if (a[i].0 as i32) <= (b[i].0 as i32) {
                    U32::ONE
                } else {
                    U32::ZERO
                };
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if (a[i].0 as i16) <= (b[i].0 as i16) {
                    U16::ONE
                } else {
                    U16::ZERO
                };
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if (a[i].0 as i8) <= (b[i].0 as i8) {
                    U8::ONE
                } else {
                    U8::ZERO
                };
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msle_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = if I1024::from(a[i]) <= I1024::from(U1024::from(rhs as i64)) {
                    U1024::ONE
                } else {
                    U1024::MIN
                };
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = if I512::from(a[i]) <= I512::from(U512::from(rhs as i64)) {
                    U512::ONE
                } else {
                    U512::MIN
                };
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = if I256::from(a[i]) <= I256::from(U256::from(rhs as i64)) {
                    U256::ONE
                } else {
                    U256::MIN
                };
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if (a[i].0 as i128) <= (rhs as i64 as i128) {
                    U128::ONE
                } else {
                    U128::ZERO
                };
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if (a[i].0 as i64) <= (rhs as i64) {
                    U64::ONE
                } else {
                    U64::ZERO
                };
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if (a[i].0 as i32) <= (rhs as u32 as i32) {
                    U32::ONE
                } else {
                    U32::ZERO
                };
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if (a[i].0 as i16) <= (rhs as u16 as i16) {
                    U16::ONE
                } else {
                    U16::ZERO
                };
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if (a[i].0 as i8) <= (rhs as u8 as i8) {
                    U8::ONE
                } else {
                    U8::ZERO
                };
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msle_vi(
    lhs: &VRegister,
    rhs: i32,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = if I1024::from(a[i]) <= I1024::from(U1024::from(rhs)) {
                    U1024::ONE
                } else {
                    U1024::MIN
                };
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = if I512::from(a[i]) <= I512::from(U512::from(rhs)) {
                    U512::ONE
                } else {
                    U512::MIN
                };
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = if I256::from(a[i]) <= I256::from(U256::from(rhs)) {
                    U256::ONE
                } else {
                    U256::MIN
                };
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i].0 as i128 <= rhs as i128 {
                    U128::ONE
                } else {
                    U128::ZERO
                };
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i].0 as i64 <= rhs as i64 {
                    U64::ONE
                } else {
                    U64::ZERO
                };
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i].0 as i32 <= rhs {
                    U32::ONE
                } else {
                    U32::ZERO
                };
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i].0 as i16 <= rhs as i16 {
                    U16::ONE
                } else {
                    U16::ZERO
                };
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i].0 as i8 <= rhs as i8 {
                    U8::ONE
                } else {
                    U8::ZERO
                };
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_div_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = I1024::from(a[i]).wrapping_div(I1024::from(b[i])).uint;
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = I512::from(a[i]).wrapping_div(I512::from(b[i])).uint;
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = I256::from(a[i]).wrapping_div(I256::from(b[i])).uint;
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if b[i].0 == 0 {
                    U128::MAX
                } else if a[i].0 == 1 << 127 && b[i].0 == u128::MAX {
                    U128(1u128 << 127)
                } else {
                    U128((a[i].0 as i128).wrapping_div(b[i].0 as i128) as u128)
                }
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if b[i].0 == 0 {
                    U64(u64::MAX)
                } else if a[i].0 == 1 << 63 && b[i].0 == u64::MAX {
                    U64(1u64 << 63)
                } else {
                    U64((a[i].0 as i64).wrapping_div(b[i].0 as i64) as u64)
                }
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if b[i].0 == 0 {
                    U32(u32::MAX)
                } else if a[i].0 == 1 << 31 && b[i].0 == u32::MAX {
                    U32(1u32 << 31)
                } else {
                    U32((a[i].0 as i32).wrapping_div(b[i].0 as i32) as u32)
                }
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if b[i].0 == 0 {
                    U16(u16::MAX)
                } else if a[i].0 == 1 << 15 && b[i].0 == u16::MAX {
                    U16(1u16 << 15)
                } else {
                    U16((a[i].0 as i16).wrapping_div(b[i].0 as i16) as u16)
                }
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if b[i].0 == 0 {
                    U8(u8::MAX)
                } else if a[i].0 == 1 << 7 && b[i].0 == u8::MAX {
                    U8(1u8 << 7)
                } else {
                    U8((a[i].0 as i8).wrapping_div(b[i].0 as i8) as u8)
                }
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_div_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = I1024::from(a[i])
                    .wrapping_div(I1024::from(U1024::from(rhs as i64)))
                    .uint;
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = I512::from(a[i])
                    .wrapping_div(I512::from(U512::from(rhs as i64)))
                    .uint;
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = I256::from(a[i])
                    .wrapping_div(I256::from(U256::from(rhs as i64)))
                    .uint;
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if rhs == 0 {
                    U128(u128::MAX)
                } else if a[i].0 == 1 << 127 && rhs == u64::MAX {
                    U128(1u128 << 127)
                } else {
                    U128((a[i].0 as i128).wrapping_div(rhs as i128) as u128)
                }
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if rhs == 0 {
                    U64(u64::MAX)
                } else if a[i].0 == 1 << 63 && rhs == u64::MAX {
                    U64(1u64 << 63)
                } else {
                    U64((a[i].0 as i64).wrapping_div(rhs as i64) as u64)
                }
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if rhs == 0 {
                    U32(u32::MAX)
                } else if a[i].0 == 1 << 31 && rhs as u32 == u32::MAX {
                    U32(1u32 << 31)
                } else {
                    U32((a[i].0 as i32).wrapping_div(rhs as i32) as u32)
                }
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if rhs == 0 {
                    U16(u16::MAX)
                } else if a[i].0 == 1 << 15 && rhs as u16 == u16::MAX {
                    U16(1u16 << 15)
                } else {
                    U16((a[i].0 as i16).wrapping_div(rhs as i16) as u16)
                }
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if rhs == 0 {
                    U8(u8::MAX)
                } else if a[i].0 == 1 << 7 && rhs as u8 == u8::MAX {
                    U8(1u8 << 7)
                } else {
                    U8((a[i].0 as i8).wrapping_div(rhs as i8) as u8)
                }
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_rem_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = I1024::from(a[i]).wrapping_rem(I1024::from(b[i])).uint;
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = I512::from(a[i]).wrapping_rem(I512::from(b[i])).uint;
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = I256::from(a[i]).wrapping_rem(I256::from(b[i])).uint;
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if b[i].0 == 0 {
                    a[i]
                } else if a[i].0 == 1 << 127 && b[i].0 == u128::MAX {
                    U128(0)
                } else {
                    U128((a[i].0 as i128).wrapping_rem(b[i].0 as i128) as u128)
                }
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if b[i].0 == 0 {
                    a[i]
                } else if a[i].0 == 1 << 63 && b[i].0 == u64::MAX {
                    U64(0)
                } else {
                    U64((a[i].0 as i64).wrapping_rem(b[i].0 as i64) as u64)
                }
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if b[i].0 == 0 {
                    a[i]
                } else if a[i].0 == 1 << 31 && b[i].0 == u32::MAX {
                    U32(0)
                } else {
                    U32((a[i].0 as i32).wrapping_rem(b[i].0 as i32) as u32)
                }
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if b[i].0 == 0 {
                    a[i]
                } else if a[i].0 == 1 << 15 && b[i].0 == u16::MAX {
                    U16(0)
                } else {
                    U16((a[i].0 as i16).wrapping_rem(b[i].0 as i16) as u16)
                }
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if b[i].0 == 0 {
                    a[i]
                } else if a[i].0 == 1 << 7 && b[i].0 == u8::MAX {
                    U8(0)
                } else {
                    U8((a[i].0 as i8).wrapping_rem(b[i].0 as i8) as u8)
                }
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_rem_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = I1024::from(a[i])
                    .wrapping_rem(I1024::from(U1024::from(rhs as i64)))
                    .uint;
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = I512::from(a[i])
                    .wrapping_rem(I512::from(U512::from(rhs as i64)))
                    .uint;
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = I256::from(a[i])
                    .wrapping_rem(I256::from(U256::from(rhs as i64)))
                    .uint;
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if rhs == 0 {
                    a[i]
                } else if a[i].0 == 1 << 127 && rhs == u64::MAX {
                    U128(0)
                } else {
                    U128((a[i].0 as i128).wrapping_rem(rhs as i128) as u128)
                }
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if rhs == 0 {
                    a[i]
                } else if a[i].0 == 1 << 63 && rhs == u64::MAX {
                    U64(0)
                } else {
                    U64((a[i].0 as i64).wrapping_rem(rhs as i64) as u64)
                }
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if rhs == 0 {
                    a[i]
                } else if a[i].0 == 1 << 31 && rhs as u32 == u32::MAX {
                    U32(0)
                } else {
                    U32((a[i].0 as i32).wrapping_rem(rhs as i32) as u32)
                }
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if rhs == 0 {
                    a[i]
                } else if a[i].0 == 1 << 15 && rhs as u16 == u16::MAX {
                    U16(0)
                } else {
                    U16((a[i].0 as i16).wrapping_rem(rhs as i16) as u16)
                }
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if rhs == 0 {
                    a[i]
                } else if a[i].0 == 1 << 7 && rhs as u8 == u8::MAX {
                    U8(0)
                } else {
                    U8((a[i].0 as i8).wrapping_rem(rhs as i8) as u8)
                }
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}
