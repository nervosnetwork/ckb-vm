use std::cmp::min;
use std::fmt::Display;
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

pub trait Register:
    Sized
    + Clone
    + Default
    + Display
    + Not<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Shl<Self, Output = Self>
    + Shr<Self, Output = Self>
{
    const BITS: u8;
    const SHIFT_MASK: u8;

    fn zero() -> Self;
    fn one() -> Self;

    fn min_value() -> Self;
    fn max_value() -> Self;

    // Conditional operations, if the condition evaluated here is true, R::one()
    // will be emitted, otherwise R::zero() will be emitted
    fn eq(&self, other: &Self) -> Self;
    fn lt(&self, other: &Self) -> Self;
    fn lt_s(&self, other: &Self) -> Self;
    fn logical_not(&self) -> Self;

    // self here should be the result of one of the conditional operations, if
    // self is R::one(), true_value will be returned, otherwise false_value will
    // be returned. No values other than R::one() or R::zero() should be accepted
    // as self here.
    fn cond(&self, true_value: &Self, false_value: &Self) -> Self;

    fn overflowing_add(&self, rhs: &Self) -> Self;
    fn overflowing_sub(&self, rhs: &Self) -> Self;
    fn overflowing_mul(&self, rhs: &Self) -> Self;

    // Those 4 methods should implement RISC-V's overflowing strategies:
    // +---------------------------------------------------------------------------------------+
    // | Condition              | Dividend  | Divisor | DIVU[W] | REMU[W] |  DIV[W]   | REM[W] |
    // +------------------------+-----------+---------+---------+---------+-----------+--------+
    // | Division by zero       |     x     |    0    | 2**L-1  |    x    |    -1     |   x    |
    // +------------------------+-----------+---------+---------+---------+-----------+--------+
    // | Overflow (signed only) | −2**(L−1) |   −1    |    -    |    -    | -2**(L-1) |   0    |
    // +---------------------------------------------------------------------------------------+
    fn overflowing_div(&self, rhs: &Self) -> Self;
    fn overflowing_rem(&self, rhs: &Self) -> Self;
    fn overflowing_div_signed(&self, rhs: &Self) -> Self;
    fn overflowing_rem_signed(&self, rhs: &Self) -> Self;

    fn overflowing_mul_high_signed(&self, rhs: &Self) -> Self;
    fn overflowing_mul_high_unsigned(&self, rhs: &Self) -> Self;
    fn overflowing_mul_high_signed_unsigned(&self, rhs: &Self) -> Self;

    // The clz operation counts the number of 0 bits at the MSB end of the argument.
    fn clz(&self) -> Self;
    // The ctz operation counts the number of 0 bits at the LSB end of the argument.
    fn ctz(&self) -> Self;
    // Counts the number of 1 bits.
    fn cpop(&self) -> Self;
    // Carry-less multiply (low-part)
    fn clmul(&self, rhs: &Self) -> Self;
    // Carry-less multiply (high-part)
    fn clmulh(&self, rhs: &Self) -> Self;
    // Carry-less multiply (reversed)
    fn clmulr(&self, rhs: &Self) -> Self;
    fn orcb(&self) -> Self;
    fn rev8(&self) -> Self;

    fn signed_shl(&self, rhs: &Self) -> Self;
    fn signed_shr(&self, rhs: &Self) -> Self;

    // Rotate left/right.
    fn rol(&self, rhs: &Self) -> Self;
    fn ror(&self, rhs: &Self) -> Self;

    // Zero extend from start_bit to the highest bit, note
    // start_bit is offset by 0
    fn zero_extend(&self, start_bit: &Self) -> Self;
    // Sign extend from start_bit to the highest bit leveraging
    // bit in (start_bit - 1), note start_bit is offset by 0
    fn sign_extend(&self, start_bit: &Self) -> Self;

    // NOTE: one alternative solution is to encode those methods using
    // From/Into traits, however we opt for manual conversion here for 2
    // reasons:
    // 1. This leads to more clarity in code, so when we see `to_u8`, we know
    // immediately that the method is called on a register type, while `as u8`
    // tells us it's a different type.
    // 2. Adding those additional methods allows us to implement this trait on
    // plain u32/u64 types.
    fn to_i8(&self) -> i8;
    fn to_i16(&self) -> i16;
    fn to_i32(&self) -> i32;
    fn to_i64(&self) -> i64;
    fn to_u8(&self) -> u8;
    fn to_u16(&self) -> u16;
    fn to_u32(&self) -> u32;
    fn to_u64(&self) -> u64;

    fn from_i8(v: i8) -> Self;
    fn from_i16(v: i16) -> Self;
    fn from_i32(v: i32) -> Self;
    fn from_i64(v: i64) -> Self;
    fn from_u8(v: u8) -> Self;
    fn from_u16(v: u16) -> Self;
    fn from_u32(v: u32) -> Self;
    fn from_u64(v: u64) -> Self;

    fn ne(&self, rhs: &Self) -> Self {
        self.eq(rhs).logical_not()
    }

    fn ge(&self, other: &Self) -> Self {
        self.lt(other).logical_not()
    }

    fn ge_s(&self, other: &Self) -> Self {
        self.lt_s(other).logical_not()
    }
}

impl Register for u32 {
    const BITS: u8 = 32;
    const SHIFT_MASK: u8 = 0x1F;

    fn zero() -> u32 {
        0
    }

    fn one() -> u32 {
        1
    }

    fn min_value() -> u32 {
        u32::min_value()
    }

    fn max_value() -> u32 {
        u32::max_value()
    }

    fn eq(&self, other: &u32) -> u32 {
        (self == other).into()
    }

    fn lt(&self, other: &u32) -> u32 {
        (self < other).into()
    }

    fn lt_s(&self, other: &u32) -> u32 {
        ((*self as i32) < (*other as i32)).into()
    }

    fn logical_not(&self) -> u32 {
        (*self != Self::one()).into()
    }

    fn cond(&self, true_value: &u32, false_value: &u32) -> u32 {
        if *self == Self::one() {
            *true_value
        } else {
            *false_value
        }
    }

    fn overflowing_add(&self, rhs: &u32) -> u32 {
        (*self).overflowing_add(*rhs).0
    }

    fn overflowing_sub(&self, rhs: &u32) -> u32 {
        (*self).overflowing_sub(*rhs).0
    }

    fn overflowing_mul(&self, rhs: &u32) -> u32 {
        (*self).overflowing_mul(*rhs).0
    }

    fn overflowing_div(&self, rhs: &u32) -> u32 {
        if *rhs == 0 {
            Self::max_value()
        } else {
            (*self).overflowing_div(*rhs).0
        }
    }

    fn overflowing_rem(&self, rhs: &u32) -> u32 {
        if *rhs == 0 {
            *self
        } else {
            (*self).overflowing_rem(*rhs).0
        }
    }

    fn overflowing_div_signed(&self, rhs: &u32) -> u32 {
        if *rhs == 0 {
            (-1i32) as u32
        } else {
            let (v, o) = (*self as i32).overflowing_div(*rhs as i32);
            if o {
                // -2**(L-1) implemented using (-1) << (L - 1)
                ((-1i32) as u32) << (<Self as Register>::BITS - 1)
            } else {
                v as u32
            }
        }
    }

    fn overflowing_rem_signed(&self, rhs: &u32) -> u32 {
        if *rhs == 0 {
            *self
        } else {
            let (v, o) = (*self as i32).overflowing_rem(*rhs as i32);
            if o {
                0
            } else {
                v as u32
            }
        }
    }

    fn overflowing_mul_high_signed(&self, rhs: &u32) -> u32 {
        let a = i64::from(*self as i32);
        let b = i64::from(*rhs as i32);
        let (value, _) = a.overflowing_mul(b);
        (value >> 32) as u32
    }

    fn overflowing_mul_high_unsigned(&self, rhs: &u32) -> u32 {
        let a = u64::from(*self);
        let b = u64::from(*rhs);
        let (value, _) = a.overflowing_mul(b);
        (value >> 32) as u32
    }

    fn overflowing_mul_high_signed_unsigned(&self, rhs: &u32) -> u32 {
        let a = i64::from(*self as i32);
        let b = i64::from(*rhs);
        let (value, _) = a.overflowing_mul(b);
        (value >> 32) as u32
    }

    fn signed_shl(&self, rhs: &u32) -> u32 {
        (*self as i32).shl(*rhs) as u32
    }

    fn signed_shr(&self, rhs: &u32) -> u32 {
        (*self as i32).shr(*rhs) as u32
    }

    fn zero_extend(&self, start_bit: &u32) -> u32 {
        let start_bit = min(*start_bit, 32);
        debug_assert!(start_bit > 0);
        (*self << (32 - start_bit)) >> (32 - start_bit)
    }

    fn sign_extend(&self, start_bit: &u32) -> u32 {
        let start_bit = min(*start_bit, 32);
        debug_assert!(start_bit > 0);
        (((*self << (32 - start_bit)) as i32) >> (32 - start_bit)) as u32
    }

    fn clz(&self) -> u32 {
        self.leading_zeros()
    }

    fn ctz(&self) -> u32 {
        self.trailing_zeros()
    }

    fn cpop(&self) -> u32 {
        self.count_ones()
    }

    fn clmul(&self, rhs: &u32) -> u32 {
        let mut x: u32 = 0;
        for i in 0..32 {
            if ((rhs >> i) & 1) != 0 {
                x ^= self << i;
            }
        }
        x
    }

    fn clmulh(&self, rhs: &u32) -> u32 {
        let mut x: u32 = 0;
        for i in 1..32 {
            if ((rhs >> i) & 1) != 0 {
                x ^= self >> (32 - i);
            }
        }
        x
    }

    fn clmulr(&self, rhs: &u32) -> u32 {
        let mut x: u32 = 0;
        for i in 0..32 {
            if ((rhs >> i) & 1) != 0 {
                x ^= self >> (31 - i);
            }
        }
        x
    }

    fn orcb(&self) -> u32 {
        let mut rr = 0;
        if self & 0x000000ff != 0 {
            rr |= 0x000000ff
        }
        if self & 0x0000ff00 != 0 {
            rr |= 0x0000ff00
        }
        if self & 0x00ff0000 != 0 {
            rr |= 0x00ff0000
        }
        if self & 0xff000000 != 0 {
            rr |= 0xff000000
        }
        rr
    }

    fn rev8(&self) -> u32 {
        let mut r = 0;
        let a = self & 0x000000ff;
        r |= a << 24;
        let a = self & 0x0000ff00;
        r |= a << 8;
        let a = self & 0x00ff0000;
        r |= a >> 8;
        let a = self & 0xff000000;
        r |= a >> 24;
        r
    }

    fn rol(&self, rhs: &u32) -> u32 {
        (*self as u32).rotate_left(*rhs) as u32
    }

    fn ror(&self, rhs: &u32) -> u32 {
        (*self as u32).rotate_right(*rhs) as u32
    }

    fn to_i8(&self) -> i8 {
        *self as i8
    }

    fn to_i16(&self) -> i16 {
        *self as i16
    }

    fn to_i32(&self) -> i32 {
        *self as i32
    }

    fn to_i64(&self) -> i64 {
        i64::from(*self as i32)
    }

    fn to_u8(&self) -> u8 {
        *self as u8
    }

    fn to_u16(&self) -> u16 {
        *self as u16
    }

    fn to_u32(&self) -> u32 {
        *self
    }

    fn to_u64(&self) -> u64 {
        u64::from(*self)
    }

    fn from_i8(v: i8) -> u32 {
        i32::from(v) as u32
    }

    fn from_i16(v: i16) -> u32 {
        i32::from(v) as u32
    }

    fn from_i32(v: i32) -> u32 {
        v as u32
    }

    fn from_i64(v: i64) -> u32 {
        (v as i32) as u32
    }

    fn from_u8(v: u8) -> u32 {
        u32::from(v)
    }

    fn from_u16(v: u16) -> u32 {
        u32::from(v)
    }

    fn from_u32(v: u32) -> u32 {
        v
    }

    fn from_u64(v: u64) -> u32 {
        v as u32
    }
}

impl Register for u64 {
    const BITS: u8 = 64;
    const SHIFT_MASK: u8 = 0x3F;

    fn zero() -> u64 {
        0
    }

    fn one() -> u64 {
        1
    }

    fn min_value() -> u64 {
        u64::min_value()
    }

    fn max_value() -> u64 {
        u64::max_value()
    }

    fn eq(&self, other: &u64) -> u64 {
        (self == other).into()
    }

    fn lt(&self, other: &u64) -> u64 {
        (self < other).into()
    }

    fn lt_s(&self, other: &u64) -> u64 {
        ((*self as i64) < (*other as i64)).into()
    }

    fn logical_not(&self) -> u64 {
        (*self != Self::one()).into()
    }

    fn cond(&self, true_value: &u64, false_value: &u64) -> u64 {
        if *self == Self::one() {
            *true_value
        } else {
            *false_value
        }
    }

    fn overflowing_add(&self, rhs: &u64) -> u64 {
        (*self).overflowing_add(*rhs).0
    }

    fn overflowing_sub(&self, rhs: &u64) -> u64 {
        (*self).overflowing_sub(*rhs).0
    }

    fn overflowing_mul(&self, rhs: &u64) -> u64 {
        (*self).overflowing_mul(*rhs).0
    }

    fn overflowing_div(&self, rhs: &u64) -> u64 {
        if *rhs == 0 {
            Self::max_value()
        } else {
            (*self).overflowing_div(*rhs).0
        }
    }

    fn overflowing_rem(&self, rhs: &u64) -> u64 {
        if *rhs == 0 {
            *self
        } else {
            (*self).overflowing_rem(*rhs).0
        }
    }

    fn overflowing_div_signed(&self, rhs: &u64) -> u64 {
        if *rhs == 0 {
            (-1i64) as u64
        } else {
            let (v, o) = (*self as i64).overflowing_div(*rhs as i64);
            if o {
                // -2**(L-1) implemented using (-1) << (L - 1)
                ((-1i64) as u64) << (<Self as Register>::BITS - 1)
            } else {
                v as u64
            }
        }
    }

    fn overflowing_rem_signed(&self, rhs: &u64) -> u64 {
        if *rhs == 0 {
            *self
        } else {
            let (v, o) = (*self as i64).overflowing_rem(*rhs as i64);
            if o {
                0
            } else {
                v as u64
            }
        }
    }

    fn overflowing_mul_high_signed(&self, rhs: &u64) -> u64 {
        let a = i128::from(*self as i64);
        let b = i128::from(*rhs as i64);
        let (value, _) = a.overflowing_mul(b);
        (value >> 64) as u64
    }

    fn overflowing_mul_high_unsigned(&self, rhs: &u64) -> u64 {
        let a = u128::from(*self);
        let b = u128::from(*rhs);
        let (value, _) = a.overflowing_mul(b);
        (value >> 64) as u64
    }

    fn overflowing_mul_high_signed_unsigned(&self, rhs: &u64) -> u64 {
        let a = i128::from(*self as i64);
        let b = i128::from(*rhs);
        let (value, _) = a.overflowing_mul(b);
        (value >> 64) as u64
    }

    fn signed_shl(&self, rhs: &u64) -> u64 {
        (*self as i64).shl(*rhs) as u64
    }

    fn signed_shr(&self, rhs: &u64) -> u64 {
        (*self as i64).shr(*rhs) as u64
    }

    fn zero_extend(&self, start_bit: &u64) -> u64 {
        let start_bit = min(*start_bit, 64);
        debug_assert!(start_bit > 0);
        (*self << (64 - start_bit)) >> (64 - start_bit)
    }

    fn sign_extend(&self, start_bit: &u64) -> u64 {
        let start_bit = min(*start_bit, 64);
        debug_assert!(start_bit > 0);
        (((*self << (64 - start_bit)) as i64) >> (64 - start_bit)) as u64
    }

    fn clz(&self) -> u64 {
        self.leading_zeros() as u64
    }

    fn ctz(&self) -> u64 {
        self.trailing_zeros() as u64
    }

    fn cpop(&self) -> u64 {
        self.count_ones() as u64
    }

    fn clmul(&self, rhs: &u64) -> u64 {
        let mut x: u64 = 0;
        for i in 0..64 {
            if ((rhs >> i) & 1) != 0 {
                x ^= self << i;
            }
        }
        x
    }

    fn clmulh(&self, rhs: &u64) -> u64 {
        let mut x: u64 = 0;
        for i in 1..64 {
            if ((rhs >> i) & 1) != 0 {
                x ^= self >> (64 - i);
            }
        }
        x
    }

    fn clmulr(&self, rhs: &u64) -> u64 {
        let mut x: u64 = 0;
        for i in 0..64 {
            if ((rhs >> i) & 1) != 0 {
                x ^= self >> (63 - i);
            }
        }
        x
    }

    fn orcb(&self) -> u64 {
        let mut rr = 0;
        if self & 0x00000000000000ff != 0 {
            rr |= 0x00000000000000ff
        }
        if self & 0x000000000000ff00 != 0 {
            rr |= 0x000000000000ff00
        }
        if self & 0x0000000000ff0000 != 0 {
            rr |= 0x0000000000ff0000
        }
        if self & 0x00000000ff000000 != 0 {
            rr |= 0x00000000ff000000
        }
        if self & 0x000000ff00000000 != 0 {
            rr |= 0x000000ff00000000
        }
        if self & 0x0000ff0000000000 != 0 {
            rr |= 0x0000ff0000000000
        }
        if self & 0x00ff000000000000 != 0 {
            rr |= 0x00ff000000000000
        }
        if self & 0xff00000000000000 != 0 {
            rr |= 0xff00000000000000
        }
        rr
    }

    fn rev8(&self) -> u64 {
        let mut r = 0;
        let a = self & 0x00000000000000ff;
        r |= a << 56;
        let a = self & 0x000000000000ff00;
        r |= a << 40;
        let a = self & 0x0000000000ff0000;
        r |= a << 24;
        let a = self & 0x00000000ff000000;
        r |= a << 8;
        let a = self & 0x000000ff00000000;
        r |= a >> 8;
        let a = self & 0x0000ff0000000000;
        r |= a >> 24;
        let a = self & 0x00ff000000000000;
        r |= a >> 40;
        let a = self & 0xff00000000000000;
        r |= a >> 56;
        r
    }

    fn rol(&self, rhs: &u64) -> u64 {
        (*self as u64).rotate_left((*rhs) as u32) as u64
    }

    fn ror(&self, rhs: &u64) -> u64 {
        (*self as u64).rotate_right((*rhs) as u32) as u64
    }

    fn to_i8(&self) -> i8 {
        *self as i8
    }

    fn to_i16(&self) -> i16 {
        *self as i16
    }

    fn to_i32(&self) -> i32 {
        *self as i32
    }

    fn to_i64(&self) -> i64 {
        *self as i64
    }

    fn to_u8(&self) -> u8 {
        *self as u8
    }

    fn to_u16(&self) -> u16 {
        *self as u16
    }

    fn to_u32(&self) -> u32 {
        *self as u32
    }

    fn to_u64(&self) -> u64 {
        *self
    }

    fn from_i8(v: i8) -> u64 {
        i64::from(v) as u64
    }

    fn from_i16(v: i16) -> u64 {
        i64::from(v) as u64
    }

    fn from_i32(v: i32) -> u64 {
        i64::from(v) as u64
    }

    fn from_i64(v: i64) -> u64 {
        v as u64
    }

    fn from_u8(v: u8) -> u64 {
        u64::from(v)
    }

    fn from_u16(v: u16) -> u64 {
        u64::from(v)
    }

    fn from_u32(v: u32) -> u64 {
        u64::from(v)
    }

    fn from_u64(v: u64) -> u64 {
        v
    }
}
