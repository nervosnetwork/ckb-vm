use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

pub trait Register:
    Sized
    + Copy
    + Display
    + PartialEq
    + PartialOrd
    + Ord
    + Eq
    + Not<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
{
    const BITS: usize;
    const SHIFT_MASK: usize;

    fn zero() -> Self;
    fn one() -> Self;

    fn min_value() -> Self;
    fn max_value() -> Self;

    // By default, PartialOrd/Ord compares unsigned value, which is the same
    // as shl/shr.
    fn signed_cmp(&self, other: &Self) -> Ordering;

    fn overflowing_add(self, rhs: Self) -> (Self, bool);
    fn overflowing_sub(self, rhs: Self) -> (Self, bool);
    fn overflowing_mul(self, rhs: Self) -> (Self, bool);
    fn overflowing_div(self, rhs: Self) -> (Self, bool);
    fn overflowing_rem(self, rhs: Self) -> (Self, bool);

    fn overflowing_div_signed(self, rhs: Self) -> (Self, bool);
    fn overflowing_rem_signed(self, rhs: Self) -> (Self, bool);
    fn overflowing_mul_high_signed(self, rhs: Self) -> Self;
    fn overflowing_mul_high_unsigned(self, rhs: Self) -> Self;
    fn overflowing_mul_high_signed_unsigned(self, rhs: Self) -> Self;

    fn signed_shl(self, rhs: usize) -> Self;
    fn signed_shr(self, rhs: usize) -> Self;

    // Zero extend from start_bit to the highest bit, note
    // start_bit is offset by 0
    fn zero_extend(self, start_bit: usize) -> Self;
    // Sign extend from start_bit to the highest bit leveraging
    // bit in (start_bit - 1), note start_bit is offset by 0
    fn sign_extend(self, start_bit: usize) -> Self;

    // NOTE: one alternative solution is to encode those methods using
    // From/Into traits, however we opt for manual conversion here for 2
    // reasons:
    // 1. This leads to more clarity in code, so when we see `to_u8`, we know
    // immediately that the method is called on a register type, while `as u8`
    // tells us it's a different type.
    // 2. Adding those additional methods allows us to implement this trait on
    // plain u32/u64 types.
    fn to_i8(self) -> i8;
    fn to_i16(self) -> i16;
    fn to_i32(self) -> i32;
    fn to_i64(self) -> i64;
    fn to_isize(self) -> isize;
    fn to_u8(self) -> u8;
    fn to_u16(self) -> u16;
    fn to_u32(self) -> u32;
    fn to_u64(self) -> u64;
    fn to_usize(self) -> usize;

    fn from_i8(v: i8) -> Self;
    fn from_i16(v: i16) -> Self;
    fn from_i32(v: i32) -> Self;
    fn from_i64(v: i64) -> Self;
    fn from_isize(v: isize) -> Self;
    fn from_u8(v: u8) -> Self;
    fn from_u16(v: u16) -> Self;
    fn from_u32(v: u32) -> Self;
    fn from_u64(v: u64) -> Self;
    fn from_usize(v: usize) -> Self;
}

impl Register for u32 {
    const BITS: usize = 32;
    const SHIFT_MASK: usize = 0x1F;

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

    fn signed_cmp(&self, other: &u32) -> Ordering {
        (*self as i32).cmp(&(*other as i32))
    }

    fn overflowing_add(self, rhs: u32) -> (u32, bool) {
        self.overflowing_add(rhs)
    }

    fn overflowing_sub(self, rhs: u32) -> (u32, bool) {
        self.overflowing_sub(rhs)
    }

    fn overflowing_mul(self, rhs: u32) -> (u32, bool) {
        self.overflowing_mul(rhs)
    }

    fn overflowing_div(self, rhs: u32) -> (u32, bool) {
        self.overflowing_div(rhs)
    }

    fn overflowing_rem(self, rhs: u32) -> (u32, bool) {
        self.overflowing_rem(rhs)
    }

    fn overflowing_div_signed(self, rhs: u32) -> (u32, bool) {
        let (v, o) = (self as i32).overflowing_div(rhs as i32);
        (v as u32, o)
    }

    fn overflowing_rem_signed(self, rhs: u32) -> (u32, bool) {
        let (v, o) = (self as i32).overflowing_rem(rhs as i32);
        (v as u32, o)
    }

    fn overflowing_mul_high_signed(self, rhs: u32) -> u32 {
        let a = i64::from(self as i32);
        let b = i64::from(rhs as i32);
        let (value, _) = a.overflowing_mul(b);
        (value >> 32) as u32
    }

    fn overflowing_mul_high_unsigned(self, rhs: u32) -> u32 {
        let a = u64::from(self);
        let b = u64::from(rhs);
        let (value, _) = a.overflowing_mul(b);
        (value >> 32) as u32
    }

    fn overflowing_mul_high_signed_unsigned(self, rhs: u32) -> u32 {
        let a = i64::from(self as i32);
        let b = i64::from(rhs);
        let (value, _) = a.overflowing_mul(b);
        (value >> 32) as u32
    }

    fn signed_shl(self, rhs: usize) -> u32 {
        (self as i32).shl(rhs) as u32
    }

    fn signed_shr(self, rhs: usize) -> u32 {
        (self as i32).shr(rhs) as u32
    }

    fn zero_extend(self, start_bit: usize) -> u32 {
        debug_assert!(start_bit < 32 && start_bit > 0);
        (self << (32 - start_bit)) >> (32 - start_bit)
    }

    fn sign_extend(self, start_bit: usize) -> u32 {
        debug_assert!(start_bit < 32 && start_bit > 0);
        (((self << (32 - start_bit)) as i32) >> (32 - start_bit)) as u32
    }

    fn to_i8(self) -> i8 {
        self as i8
    }

    fn to_i16(self) -> i16 {
        self as i16
    }

    fn to_i32(self) -> i32 {
        self as i32
    }

    fn to_i64(self) -> i64 {
        i64::from(self as i32)
    }

    fn to_isize(self) -> isize {
        i64::from(self as i32) as isize
    }

    fn to_u8(self) -> u8 {
        self as u8
    }

    fn to_u16(self) -> u16 {
        self as u16
    }

    fn to_u32(self) -> u32 {
        self
    }

    fn to_u64(self) -> u64 {
        u64::from(self)
    }

    fn to_usize(self) -> usize {
        self as usize
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

    fn from_isize(v: isize) -> u32 {
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

    fn from_usize(v: usize) -> u32 {
        v as u32
    }
}

impl Register for u64 {
    const BITS: usize = 64;
    const SHIFT_MASK: usize = 0x3F;

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

    fn signed_cmp(&self, other: &u64) -> Ordering {
        (*self as i64).cmp(&(*other as i64))
    }

    fn overflowing_add(self, rhs: u64) -> (u64, bool) {
        self.overflowing_add(rhs)
    }

    fn overflowing_sub(self, rhs: u64) -> (u64, bool) {
        self.overflowing_sub(rhs)
    }

    fn overflowing_mul(self, rhs: u64) -> (u64, bool) {
        self.overflowing_mul(rhs)
    }

    fn overflowing_div(self, rhs: u64) -> (u64, bool) {
        self.overflowing_div(rhs)
    }

    fn overflowing_rem(self, rhs: u64) -> (u64, bool) {
        self.overflowing_rem(rhs)
    }

    fn overflowing_div_signed(self, rhs: u64) -> (u64, bool) {
        let (v, o) = (self as i64).overflowing_div(rhs as i64);
        (v as u64, o)
    }

    fn overflowing_rem_signed(self, rhs: u64) -> (u64, bool) {
        let (v, o) = (self as i64).overflowing_rem(rhs as i64);
        (v as u64, o)
    }

    fn overflowing_mul_high_signed(self, rhs: u64) -> u64 {
        let a = i128::from(self as i64);
        let b = i128::from(rhs as i64);
        let (value, _) = a.overflowing_mul(b);
        (value >> 64) as u64
    }

    fn overflowing_mul_high_unsigned(self, rhs: u64) -> u64 {
        let a = u128::from(self);
        let b = u128::from(rhs);
        let (value, _) = a.overflowing_mul(b);
        (value >> 64) as u64
    }

    fn overflowing_mul_high_signed_unsigned(self, rhs: u64) -> u64 {
        let a = i128::from(self as i64);
        let b = i128::from(rhs);
        let (value, _) = a.overflowing_mul(b);
        (value >> 64) as u64
    }

    fn signed_shl(self, rhs: usize) -> u64 {
        (self as i64).shl(rhs) as u64
    }

    fn signed_shr(self, rhs: usize) -> u64 {
        (self as i64).shr(rhs) as u64
    }

    fn zero_extend(self, start_bit: usize) -> u64 {
        debug_assert!(start_bit < 64 && start_bit > 0);
        (self << (64 - start_bit)) >> (64 - start_bit)
    }

    fn sign_extend(self, start_bit: usize) -> u64 {
        debug_assert!(start_bit < 64 && start_bit > 0);
        (((self << (64 - start_bit)) as i64) >> (64 - start_bit)) as u64
    }

    fn to_i8(self) -> i8 {
        self as i8
    }

    fn to_i16(self) -> i16 {
        self as i16
    }

    fn to_i32(self) -> i32 {
        self as i32
    }

    fn to_i64(self) -> i64 {
        self as i64
    }

    fn to_isize(self) -> isize {
        (self as i64) as isize
    }

    fn to_u8(self) -> u8 {
        self as u8
    }

    fn to_u16(self) -> u16 {
        self as u16
    }

    fn to_u32(self) -> u32 {
        self as u32
    }

    fn to_u64(self) -> u64 {
        self
    }

    fn to_usize(self) -> usize {
        self as usize
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

    fn from_isize(v: isize) -> u64 {
        (v as i64) as u64
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

    fn from_usize(v: usize) -> u64 {
        v as u64
    }
}
