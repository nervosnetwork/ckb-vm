pub trait Element:
    Copy
    + Clone
    + Default
    + PartialEq
    + Eq
    + std::fmt::Display
    + std::fmt::LowerHex
    + std::ops::BitAnd
    + std::ops::BitAndAssign
    + std::ops::BitOr
    + std::ops::BitOrAssign
    + std::ops::BitXor
    + std::ops::BitXorAssign
    + std::ops::Not
    + std::ops::Neg
    + std::cmp::PartialOrd
    + std::cmp::Ord
    + std::ops::Add
    + std::ops::AddAssign
    + std::ops::Sub
    + std::ops::SubAssign
    + std::ops::Mul
    + std::ops::MulAssign
    + std::ops::Div
    + std::ops::DivAssign
    + std::ops::Rem
    + std::ops::RemAssign
    + std::ops::Shl<u32>
    + std::ops::Shr<u32>
    + From<bool>
{
    /// The size of this integer type in bits.
    const BITS: u32;
    /// The smallest value that can be represented by this integer type.
    const MIN: Self;
    /// The largest value that can be represented by this integer type.
    const MAX: Self;
    /// The one value that can be represented by this integer type.
    const ONE: Self;
    /// The zero value that can be represented by this integer type.
    const ZERO: Self;

    /// For integer operations, the scalar can be taken from the scalar x register specified by rs1. If XLEN>SEW, the
    /// least-significant SEW bits of the x register are used, unless otherwise specified. If XLEN<SEW, the value from
    /// the x register is sign-extended to SEW bits.
    fn vx(x: u64) -> Self;

    /// For integer operations, the scalar can be a 5-bit immediate, imm[4:0], encoded in the rs1 field. The value is
    /// sign-extended to SEW bits, unless otherwise specified.
    fn vi(i: i32) -> Self;

    /// Returns the lower 32 bits.
    fn u32(self) -> u32;

    /// Returns the lower 64 bits.
    fn u64(self) -> u64;

    /// Returns true if self is positive and false if the number is zero or negative.
    fn is_positive(self) -> bool;

    /// Returns true if self is negative and false if the number is zero or positive.
    fn is_negative(self) -> bool;

    /// Returns the number of leading zeros in the binary representation of self.
    fn leading_zeros(self) -> u32;

    /// Calculates self + other
    ///
    /// Returns a tuple of the addition along with a boolean indicating whether an arithmetic overflow would
    /// occur.
    /// If an overflow would have occurred then the wrapped value is returned.
    fn overflowing_add(self, other: Self) -> (Self, bool);

    /// Calculates self - other
    ///
    /// Returns a tuple of the subtraction along with a boolean indicating whether an arithmetic overflow would
    /// occur.
    /// If an overflow would have occurred then the wrapped value is returned.
    fn overflowing_sub(self, other: Self) -> (Self, bool);

    /// Calculates the multiplication of self and other.
    ///
    /// Returns a tuple of the multiplication along with a boolean indicating whether an arithmetic overflow would
    /// occur. If an overflow would have occurred then the wrapped value is returned.
    fn overflowing_mul(self, other: Self) -> (Self, bool);

    /// Calculates the divisor when self is divided by rhs.
    ///
    /// Returns a tuple of the divisor along with a boolean indicating whether an arithmetic overflow would occur. Note
    /// that for unsigned integers overflow never occurs, so the second value is always false.
    fn overflowing_div(self, other: Self) -> (Self, bool);

    /// Calculates the divisor when self is divided by rhs.
    ///
    /// Returns a tuple of the divisor along with a boolean indicating whether an arithmetic overflow would occur. Note
    /// that for unsigned integers overflow never occurs, so the second value is always false.
    fn overflowing_rem(self, other: Self) -> (Self, bool);

    /// Wrapping (modular) addition. Computes self + other, wrapping around at the boundary of the type.
    fn wrapping_add(self, other: Self) -> Self;

    /// Wrapping (modular) subtraction. Computes self - other, wrapping around at the boundary of the type.
    fn wrapping_sub(self, other: Self) -> Self;

    /// Wrapping (modular) multiplication. Computes self * other, wrapping around at the boundary of the type.
    fn wrapping_mul(self, other: Self) -> Self;

    /// Wrapping (modular) division. Computes self / rhs. Wrapped division on unsigned types is just normal division.
    /// There’s no way wrapping could ever happen. This function exists, so that all operations are accounted for in
    /// the wrapping operations.
    fn wrapping_div(self, other: Self) -> Self;

    /// Wrapping (modular) remainder. Computes self % rhs. Wrapped remainder calculation on unsigned types is just the
    /// regular remainder calculation. There’s no way wrapping could ever happen. This function exists, so that all
    /// operations are accounted for in the wrapping operations.
    fn wrapping_rem(self, other: Self) -> Self;

    /// Panic-free bitwise shift-left; yields self << mask(rhs), where mask removes any high-order bits of rhs
    /// that would cause the shift to exceed the bitwidth of the type.
    ///
    /// Note that this is not the same as a rotate-left; the RHS of a wrapping shift-left is restricted to the
    /// range of the type, rather than the bits shifted out of the LHS being returned to the other end. The
    /// primitive integer types all implement a rotate_left function, which may be what you want instead.
    fn wrapping_shl(self, other: u32) -> Self;

    /// Panic-free bitwise shift-right; yields self >> mask(rhs), where mask removes any high-order bits of rhs
    /// that would cause the shift to exceed the bitwidth of the type.
    ///
    /// Note that this is not the same as a rotate-right; the RHS of a wrapping shift-right is restricted to
    /// the range of the type, rather than the bits shifted out of the LHS being returned to the other end. The
    /// primitive integer types all implement a rotate_right function, which may be what you want instead.
    fn wrapping_shr(self, other: u32) -> Self;

    /// Panic-free bitwise sign shift-right.
    fn wrapping_sra(self, other: u32) -> Self;

    /// Function mul_full returns the 256-bit product of x and y: (lo, hi) = x * y
    /// with the product bits' upper half returned in hi and the lower half returned in lo.
    fn widening_mul(self, other: Self) -> (Self, Self);
}

macro_rules! uint_wrap_impl {
    ($name:ident, $uint:ty, $sint:ty) => {
        #[derive(Copy, Clone, Default, PartialEq, Eq)]
        pub struct $name(pub $uint);

        impl std::fmt::LowerHex for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:032x}", self.0)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:032x}", self.0)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:032x}", self.0)
            }
        }

        impl std::convert::From<bool> for $name {
            fn from(b: bool) -> Self {
                Self(b as $uint)
            }
        }

        impl std::ops::BitAnd for $name {
            type Output = Self;
            fn bitand(self, other: Self) -> Self::Output {
                Self(self.0 & other.0)
            }
        }

        impl std::ops::BitAndAssign for $name {
            fn bitand_assign(&mut self, other: Self) {
                self.0 &= other.0
            }
        }

        impl std::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, other: Self) -> Self::Output {
                Self(self.0 | other.0)
            }
        }

        impl std::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, other: Self) {
                self.0 |= other.0
            }
        }

        impl std::ops::BitXor for $name {
            type Output = Self;
            fn bitxor(self, other: Self) -> Self::Output {
                Self(self.0 ^ other.0)
            }
        }

        impl std::ops::BitXorAssign for $name {
            fn bitxor_assign(&mut self, other: Self) {
                self.0 ^= other.0
            }
        }

        impl std::ops::Not for $name {
            type Output = Self;
            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }

        impl std::ops::Neg for $name {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self((!self.0).wrapping_add(1))
            }
        }

        impl std::cmp::PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                return self.0.partial_cmp(&other.0);
            }
        }

        impl std::cmp::Ord for $name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl std::ops::Add for $name {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                Self(self.0.wrapping_add(other.0))
            }
        }

        impl std::ops::AddAssign for $name {
            fn add_assign(&mut self, other: Self) {
                self.0 = self.0.wrapping_add(other.0)
            }
        }

        impl std::ops::Sub for $name {
            type Output = Self;
            fn sub(self, other: Self) -> Self::Output {
                Self(self.0.wrapping_sub(other.0))
            }
        }

        impl std::ops::SubAssign for $name {
            fn sub_assign(&mut self, other: Self) {
                self.0 = self.0.wrapping_sub(other.0)
            }
        }

        impl std::ops::Mul for $name {
            type Output = Self;
            fn mul(self, other: Self) -> Self::Output {
                Self(self.0.wrapping_mul(other.0))
            }
        }

        impl std::ops::MulAssign for $name {
            fn mul_assign(&mut self, other: Self) {
                self.0 = self.0.wrapping_mul(other.0)
            }
        }

        impl std::ops::Div for $name {
            type Output = Self;
            fn div(self, other: Self) -> Self::Output {
                if other.0 == 0 {
                    Self::MAX
                } else {
                    Self(self.0.wrapping_div(other.0))
                }
            }
        }

        impl std::ops::DivAssign for $name {
            fn div_assign(&mut self, other: Self) {
                self.0 = if other.0 == 0 {
                    <$uint>::MAX
                } else {
                    self.0.wrapping_div(other.0)
                }
            }
        }

        impl std::ops::Rem for $name {
            type Output = Self;
            fn rem(self, other: Self) -> Self::Output {
                if other.0 == 0 {
                    self
                } else {
                    Self(self.0.wrapping_rem(other.0))
                }
            }
        }

        impl std::ops::RemAssign for $name {
            fn rem_assign(&mut self, other: Self) {
                self.0 = if other.0 == 0 {
                    self.0
                } else {
                    self.0.wrapping_rem(other.0)
                }
            }
        }

        impl std::ops::Shl<u32> for $name {
            type Output = Self;
            fn shl(self, other: u32) -> Self::Output {
                Self(self.0.wrapping_shl(other))
            }
        }

        impl std::ops::Shr<u32> for $name {
            type Output = Self;
            fn shr(self, other: u32) -> Self::Output {
                Self(self.0.wrapping_shr(other))
            }
        }

        impl Element for $name {
            const BITS: u32 = <$uint>::MIN.leading_zeros();
            const MIN: Self = Self(0);
            const MAX: Self = Self(<$uint>::MAX);
            const ONE: Self = Self(1);
            const ZERO: Self = Self(0);

            fn vx(x: u64) -> Self {
                if Self::BITS <= 64 {
                    Self(x as $uint)
                } else {
                    Self(x as i64 as $uint)
                }
            }

            fn vi(i: i32) -> Self {
                assert!(i >= -16);
                assert!(i <= 15);
                Self(i as $uint)
            }

            fn u32(self) -> u32 {
                self.0 as u32
            }

            fn u64(self) -> u64 {
                self.0 as u64
            }

            fn is_positive(self) -> bool {
                (self.0 as $sint).is_positive()
            }

            fn is_negative(self) -> bool {
                (self.0 as $sint).is_negative()
            }

            fn leading_zeros(self) -> u32 {
                self.0.leading_zeros()
            }

            fn overflowing_add(self, other: Self) -> (Self, bool) {
                let (r, b) = self.0.overflowing_add(other.0);
                (Self(r), b)
            }

            fn overflowing_sub(self, other: Self) -> (Self, bool) {
                let (r, b) = self.0.overflowing_sub(other.0);
                (Self(r), b)
            }

            fn overflowing_mul(self, other: Self) -> (Self, bool) {
                let (r, b) = self.0.overflowing_mul(other.0);
                (Self(r), b)
            }

            fn overflowing_div(self, other: Self) -> (Self, bool) {
                (self.wrapping_div(other), false)
            }

            fn overflowing_rem(self, other: Self) -> (Self, bool) {
                (self.wrapping_rem(other), false)
            }

            fn wrapping_add(self, other: Self) -> Self {
                Self(self.0.wrapping_add(other.0))
            }

            fn wrapping_sub(self, other: Self) -> Self {
                Self(self.0.wrapping_sub(other.0))
            }

            fn wrapping_mul(self, other: Self) -> Self {
                Self(self.0.wrapping_mul(other.0))
            }

            fn wrapping_div(self, other: Self) -> Self {
                if other.0 == 0 {
                    Self::MAX
                } else {
                    Self(self.0.wrapping_div(other.0))
                }
            }

            fn wrapping_rem(self, other: Self) -> Self {
                if other.0 == 0 {
                    self
                } else {
                    Self(self.0.wrapping_rem(other.0))
                }
            }

            fn wrapping_shl(self, other: u32) -> Self {
                Self(self.0.wrapping_shl(other))
            }

            fn wrapping_shr(self, other: u32) -> Self {
                Self(self.0.wrapping_shr(other))
            }

            fn wrapping_sra(self, other: u32) -> Self {
                Self((self.0 as $sint).wrapping_shr(other) as $uint)
            }

            /// Inspired by https://pkg.go.dev/math/bits@go1.17.2#Mul64
            fn widening_mul(self, other: Self) -> (Self, Self) {
                let lo = |x: $uint| x << Self::BITS / 2 >> Self::BITS / 2;
                let hi = |x: $uint| x >> Self::BITS / 2;
                let x0 = lo(self.0);
                let x1 = hi(self.0);
                let y0 = lo(other.0);
                let y1 = hi(other.0);
                let w0 = x0 * y0;
                let t = x1 * y0 + hi(w0);
                let w1 = lo(t);
                let w2 = hi(t);
                let w1 = w1 + x0 * y1;
                let hi = x1 * y1 + w2 + hi(w1);
                let lo = self.0.wrapping_mul(other.0);
                (Self(lo), Self(hi))
            }
        }

        impl $name {
            /// Create a native endian integer value from its representation as a byte array in big endian.
            pub fn from_be_bytes(bytes: [u8; Self::BITS as usize / 8]) -> Self {
                Self(<$uint>::from_be_bytes(bytes))
            }

            /// Create a native endian integer value from its representation as a byte array in little endian.
            pub fn from_le_bytes(bytes: [u8; Self::BITS as usize / 8]) -> Self {
                Self(<$uint>::from_le_bytes(bytes))
            }

            /// Return the memory representation of this integer as a byte array in big-endian (network) byte order.
            pub fn to_be_bytes(self) -> [u8; Self::BITS as usize / 8] {
                self.0.to_be_bytes()
            }

            /// Return the memory representation of this integer as a byte array in little-endian byte order.
            pub fn to_le_bytes(self) -> [u8; Self::BITS as usize / 8] {
                self.0.to_le_bytes()
            }
        }
    };
}

macro_rules! uint_wrap_from_impl {
    ($name:ty, $uint:ty, $from:ty) => {
        impl From<$from> for $name {
            fn from(small: $from) -> Self {
                Self(small as $uint)
            }
        }
    };
}

uint_wrap_impl!(U8, u8, i8);
uint_wrap_impl!(U16, u16, i16);
uint_wrap_impl!(U32, u32, i32);
uint_wrap_impl!(U64, u64, i64);
uint_wrap_impl!(U128, u128, i128);
uint_wrap_from_impl!(U8, u8, u8);
uint_wrap_from_impl!(U8, u8, i8);
uint_wrap_from_impl!(U16, u16, u8);
uint_wrap_from_impl!(U16, u16, i8);
uint_wrap_from_impl!(U16, u16, u16);
uint_wrap_from_impl!(U16, u16, i16);
uint_wrap_from_impl!(U32, u32, u8);
uint_wrap_from_impl!(U32, u32, i8);
uint_wrap_from_impl!(U32, u32, u16);
uint_wrap_from_impl!(U32, u32, i16);
uint_wrap_from_impl!(U32, u32, u32);
uint_wrap_from_impl!(U32, u32, i32);
uint_wrap_from_impl!(U64, u64, u8);
uint_wrap_from_impl!(U64, u64, i8);
uint_wrap_from_impl!(U64, u64, u16);
uint_wrap_from_impl!(U64, u64, i16);
uint_wrap_from_impl!(U64, u64, u32);
uint_wrap_from_impl!(U64, u64, i32);
uint_wrap_from_impl!(U64, u64, u64);
uint_wrap_from_impl!(U64, u64, i64);
uint_wrap_from_impl!(U128, u128, u8);
uint_wrap_from_impl!(U128, u128, i8);
uint_wrap_from_impl!(U128, u128, u16);
uint_wrap_from_impl!(U128, u128, i16);
uint_wrap_from_impl!(U128, u128, u32);
uint_wrap_from_impl!(U128, u128, i32);
uint_wrap_from_impl!(U128, u128, u64);
uint_wrap_from_impl!(U128, u128, i64);
uint_wrap_from_impl!(U128, u128, u128);
uint_wrap_from_impl!(U128, u128, i128);

macro_rules! uint_impl {
    ($name:ident, $half:ty, $bits:expr) => {
        #[derive(Copy, Clone, Default, PartialEq, Eq)]
        pub struct $name {
            pub lo: $half,
            pub hi: $half,
        }

        impl std::fmt::LowerHex for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:x}{:x}", self.hi, self.lo)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:x}{:x}", self.hi, self.lo)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:x}{:x}", self.hi, self.lo)
            }
        }

        impl std::ops::BitAnd for $name {
            type Output = Self;
            fn bitand(self, other: Self) -> Self::Output {
                Self {
                    lo: self.lo & other.lo,
                    hi: self.hi & other.hi,
                }
            }
        }

        impl std::ops::BitAndAssign for $name {
            fn bitand_assign(&mut self, other: Self) {
                self.lo &= other.lo;
                self.hi &= other.hi;
            }
        }

        impl std::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, other: Self) -> Self::Output {
                Self {
                    lo: self.lo | other.lo,
                    hi: self.hi | other.hi,
                }
            }
        }

        impl std::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, other: Self) {
                self.lo |= other.lo;
                self.hi |= other.hi;
            }
        }

        impl std::ops::BitXor for $name {
            type Output = Self;
            fn bitxor(self, other: Self) -> Self::Output {
                Self {
                    lo: self.lo ^ other.lo,
                    hi: self.hi ^ other.hi,
                }
            }
        }

        impl std::ops::BitXorAssign for $name {
            fn bitxor_assign(&mut self, other: Self) {
                self.lo ^= other.lo;
                self.hi ^= other.hi;
            }
        }

        impl std::ops::Not for $name {
            type Output = Self;
            fn not(self) -> Self::Output {
                Self {
                    lo: !self.lo,
                    hi: !self.hi,
                }
            }
        }

        impl std::ops::Neg for $name {
            type Output = Self;
            fn neg(self) -> Self::Output {
                (!self).wrapping_add(<$name>::ONE)
            }
        }

        impl std::cmp::PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl std::cmp::Ord for $name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                let hi_cmp = self.hi.cmp(&other.hi);
                if hi_cmp != std::cmp::Ordering::Equal {
                    hi_cmp
                } else {
                    self.lo.cmp(&other.lo)
                }
            }
        }

        impl std::ops::Add for $name {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                self.wrapping_add(other)
            }
        }

        impl std::ops::AddAssign for $name {
            fn add_assign(&mut self, other: Self) {
                *self = self.wrapping_add(other)
            }
        }

        impl std::ops::Sub for $name {
            type Output = Self;
            fn sub(self, other: Self) -> Self::Output {
                self.wrapping_sub(other)
            }
        }

        impl std::ops::SubAssign for $name {
            fn sub_assign(&mut self, other: Self) {
                *self = self.wrapping_sub(other)
            }
        }

        impl std::ops::Mul for $name {
            type Output = Self;
            fn mul(self, other: Self) -> Self::Output {
                self.wrapping_mul(other)
            }
        }

        impl std::ops::MulAssign for $name {
            fn mul_assign(&mut self, other: Self) {
                *self = self.wrapping_mul(other)
            }
        }

        impl std::ops::Div for $name {
            type Output = Self;
            fn div(self, other: Self) -> Self::Output {
                self.wrapping_div(other)
            }
        }

        impl std::ops::DivAssign for $name {
            fn div_assign(&mut self, other: Self) {
                *self = self.wrapping_div(other)
            }
        }

        impl std::ops::Rem for $name {
            type Output = Self;
            fn rem(self, other: Self) -> Self::Output {
                self.wrapping_rem(other)
            }
        }

        impl std::ops::RemAssign for $name {
            fn rem_assign(&mut self, other: Self) {
                *self = self.wrapping_rem(other);
            }
        }

        impl std::ops::Shl<u32> for $name {
            type Output = Self;
            fn shl(self, other: u32) -> Self::Output {
                self.wrapping_shl(other)
            }
        }

        impl std::ops::Shr<u32> for $name {
            type Output = Self;
            fn shr(self, other: u32) -> Self::Output {
                self.wrapping_shr(other)
            }
        }

        impl Element for $name {
            const BITS: u32 = <$half>::BITS * 2;
            const MIN: Self = Self {
                lo: <$half>::MIN,
                hi: <$half>::MIN,
            };
            const MAX: Self = Self {
                lo: <$half>::MAX,
                hi: <$half>::MAX,
            };
            const ONE: Self = Self {
                lo: <$half>::ONE,
                hi: <$half>::MIN,
            };
            const ZERO: Self = Self {
                lo: <$half>::MIN,
                hi: <$half>::MIN,
            };

            fn vx(x: u64) -> Self {
                Self::from(x as i64)
            }

            fn vi(i: i32) -> Self {
                assert!(i >= -16);
                assert!(i <= 15);
                Self::from(i)
            }

            fn u32(self) -> u32 {
                self.lo.u32()
            }

            fn u64(self) -> u64 {
                self.lo.u64()
            }

            fn is_positive(self) -> bool {
                self != <$name>::MIN && self.wrapping_shr(Self::BITS - 1) == <$name>::MIN
            }

            fn is_negative(self) -> bool {
                self != <$name>::MIN && self.wrapping_shr(Self::BITS - 1) == <$name>::ONE
            }

            fn leading_zeros(self) -> u32 {
                if self.hi == <$half>::MIN {
                    Self::BITS / 2 + self.lo.leading_zeros()
                } else {
                    self.hi.leading_zeros()
                }
            }

            fn overflowing_add(self, other: Self) -> (Self, bool) {
                let (lo, lo_carry) = self.lo.overflowing_add(other.lo);
                let (hi, hi_carry_1) = self.hi.overflowing_add(<$half>::from(lo_carry));
                let (hi, hi_carry_2) = hi.overflowing_add(other.hi);
                (Self { lo, hi }, hi_carry_1 || hi_carry_2)
            }

            fn overflowing_sub(self, other: Self) -> (Self, bool) {
                let (lo, lo_borrow) = self.lo.overflowing_sub(other.lo);
                let (hi, hi_borrow_1) = self.hi.overflowing_sub(<$half>::from(lo_borrow));
                let (hi, hi_borrow_2) = hi.overflowing_sub(other.hi);
                (Self { lo, hi }, hi_borrow_1 || hi_borrow_2)
            }

            fn overflowing_mul(self, other: Self) -> (Self, bool) {
                let (hi, hi_overflow_mul) = match (self.hi, other.hi) {
                    (_, <$half>::MIN) => self.hi.overflowing_mul(other.lo),
                    (<$half>::MIN, _) => other.hi.overflowing_mul(self.lo),
                    _ => (
                        self.hi
                            .wrapping_mul(other.lo)
                            .wrapping_add(other.hi.wrapping_mul(self.lo)),
                        true,
                    ),
                };
                let lo = self.lo.widening_mul(other.lo);
                let lo = Self { lo: lo.0, hi: lo.1 };
                let (hi, hi_overflow_add) = lo.hi.overflowing_add(hi);
                let lo = Self { lo: lo.lo, hi: hi };
                (lo, hi_overflow_mul || hi_overflow_add)
            }

            fn overflowing_div(self, other: Self) -> (Self, bool) {
                (self.wrapping_div(other), false)
            }

            fn overflowing_rem(self, other: Self) -> (Self, bool) {
                (self.wrapping_rem(other), false)
            }

            fn wrapping_add(self, other: Self) -> Self {
                let (lo, carry) = self.lo.overflowing_add(other.lo);
                let hi = self.hi.wrapping_add(other.hi).wrapping_add(<$half>::from(carry));
                Self { lo, hi }
            }

            fn wrapping_sub(self, other: Self) -> Self {
                let (lo, borrow) = self.lo.overflowing_sub(other.lo);
                let hi = self.hi.wrapping_sub(other.hi).wrapping_sub(<$half>::from(borrow));
                Self { lo, hi }
            }

            fn wrapping_mul(self, other: Self) -> Self {
                let (lo, hi) = self.lo.widening_mul(other.lo);
                let hi = hi
                    .wrapping_add(self.lo.wrapping_mul(other.hi))
                    .wrapping_add(self.hi.wrapping_mul(other.lo));
                Self { lo, hi }
            }

            fn wrapping_div(self, other: Self) -> Self {
                if other == Self::MIN {
                    Self::MAX
                } else {
                    self.div(other).0
                }
            }

            fn wrapping_rem(self, other: Self) -> Self {
                if other == Self::MIN {
                    self
                } else {
                    self.div(other).1
                }
            }

            fn wrapping_shl(self, other: u32) -> Self {
                let shamt = other % Self::BITS;
                if shamt < Self::BITS / 2 {
                    Self {
                        lo: self.lo.wrapping_shl(shamt),
                        hi: self.hi.wrapping_shl(shamt)
                            | self.lo.wrapping_shr(1).wrapping_shr((Self::BITS / 2) - 1 - shamt),
                    }
                } else {
                    Self {
                        lo: <$half>::MIN,
                        hi: self.lo.wrapping_shl(shamt - Self::BITS / 2),
                    }
                }
            }

            fn wrapping_shr(self, other: u32) -> Self {
                let shamt = other % Self::BITS;
                if shamt < Self::BITS / 2 {
                    Self {
                        lo: self.lo.wrapping_shr(shamt)
                            | self.hi.wrapping_shl(1).wrapping_shl((Self::BITS / 2) - 1 - shamt),
                        hi: self.hi.wrapping_shr(shamt),
                    }
                } else {
                    Self {
                        lo: self.hi.wrapping_shr(shamt - Self::BITS / 2),
                        hi: <$half>::MIN,
                    }
                }
            }

            fn wrapping_sra(self, other: u32) -> Self {
                let shamt = other % Self::BITS;
                let hi = if self.is_negative() {
                    Self::MAX << (Self::BITS - shamt)
                } else {
                    Self::MIN
                };
                let lo = self.wrapping_shr(shamt);
                hi | lo
            }

            fn widening_mul(self, other: Self) -> (Self, Self) {
                let lo = |x: Self| Self::from(x.lo);
                let hi = |x: Self| Self::from(x.hi);

                let x0 = lo(self);
                let x1 = hi(self);
                let y0 = lo(other);
                let y1 = hi(other);
                let w0 = x0 * y0;
                let t = x1 * y0 + hi(w0);
                let w1 = lo(t);
                let w2 = hi(t);
                let w1 = w1 + x0 * y1;
                let hi = x1 * y1 + w2 + hi(w1);
                let lo = self.wrapping_mul(other);
                (lo, hi)
            }
        }

        impl $name {
            /// Create a native endian integer value from its representation as a byte array in big endian.
            pub fn from_be_bytes(bytes: [u8; $bits / 8]) -> Self {
                let mut t = [0u8; $bits / 8 / 2];
                let a = 0x00;
                let b = $bits / 8 / 2;
                let c = b;
                let d = $bits / 8;
                t.copy_from_slice(&bytes[a..b]);
                let hi = <$half>::from_be_bytes(t);
                t.copy_from_slice(&bytes[c..d]);
                let lo = <$half>::from_be_bytes(t);
                Self { lo, hi }
            }

            /// Create a native endian integer value from its representation as a byte array in little endian.
            pub fn from_le_bytes(bytes: [u8; $bits / 8]) -> Self {
                let mut t = [0u8; $bits / 8 / 2];
                let a = 0x00;
                let b = $bits / 8 / 2;
                let c = b;
                let d = $bits / 8;
                t.copy_from_slice(&bytes[a..b]);
                let lo = <$half>::from_le_bytes(t);
                t.copy_from_slice(&bytes[c..d]);
                let hi = <$half>::from_le_bytes(t);
                Self { lo, hi }
            }

            /// Return the memory representation of this integer as a byte array in big-endian (network) byte order.
            pub fn to_be_bytes(self) -> [u8; $bits / 8] {
                let mut r = [0u8; $bits / 8];
                let a = 0x00;
                let b = $bits / 8 / 2;
                let c = b;
                let d = $bits / 8;
                r[a..b].copy_from_slice(&self.hi.to_be_bytes());
                r[c..d].copy_from_slice(&self.lo.to_be_bytes());
                return r;
            }

            /// Return the memory representation of this integer as a byte array in little-endian byte order.
            pub fn to_le_bytes(self) -> [u8; $bits / 8] {
                let mut r = [0u8; $bits / 8];
                let a = 0x00;
                let b = $bits / 8 / 2;
                let c = b;
                let d = $bits / 8;
                r[a..b].copy_from_slice(&self.lo.to_le_bytes());
                r[c..d].copy_from_slice(&self.hi.to_le_bytes());
                return r;
            }

            /// div_half_0 returns the quotient and remainder of (hi, lo) divided by y: quo = (hi, lo)/y,
            /// rem = (hi, lo)%y with the dividend bits' upper half in parameter hi and the lower half in parameter lo.
            /// div_half_0 panics for y == 0 (division by zero) or y <= hi (quotient overflow).
            ///
            /// Inspired by https://cs.opensource.google/go/go/+/refs/tags/go1.17.3:src/math/bits/bits.go;l=512
            fn div_half_0(self, y: $half) -> ($half, $half) {
                let twos = <$half>::ONE << (Self::BITS / 4);
                let mask = twos - <$half>::ONE;
                assert!(y != <$half>::ZERO);
                assert!(y > self.hi);
                let s = y.leading_zeros();
                let y = y << s;
                let yn1 = y >> (Self::BITS / 4);
                let yn0 = y & mask;
                let un32 = (self.hi << s)
                    | if s == 0 {
                        <$half>::ZERO
                    } else {
                        self.lo >> (Self::BITS / 2 - s)
                    };
                let un10 = self.lo << s;
                let un1 = un10 >> (Self::BITS / 4);
                let un0 = un10 & mask;
                let mut q1 = un32 / yn1;
                let mut rhat = un32 - q1 * yn1;
                while q1 >= twos || q1 * yn0 > twos * rhat + un1 {
                    q1 -= <$half>::ONE;
                    rhat += yn1;
                    if rhat >= twos {
                        break;
                    }
                }
                let un21 = un32 * twos + un1 - q1 * y;
                let mut q0 = un21 / yn1;
                rhat = un21 - q0 * yn1;
                while q0 >= twos || q0 * yn0 > twos * rhat + un0 {
                    q0 -= <$half>::ONE;
                    rhat += yn1;
                    if rhat >= twos {
                        break;
                    }
                }
                (q1 * twos + q0, (un21 * twos + un0 - q0 * y) >> s)
            }

            fn div_half_1(self, y: $half) -> (Self, $half) {
                if self.hi < y {
                    let (lo, r) = self.div_half_0(y);
                    (Self::from(lo), r)
                } else {
                    let (hi, r) = Self::from(self.hi).div_half_0(y);
                    let (lo, r) = Self { lo: self.lo, hi: r }.div_half_0(y);
                    (Self { lo: lo, hi: hi }, r)
                }
            }

            /// Inspired by https://github.com/Pilatuz/bigx/blob/8615506d17c5/uint128.go#L291
            fn div(self, other: Self) -> (Self, Self) {
                if other.hi == <$half>::ZERO {
                    let (q, r) = self.div_half_1(other.lo);
                    return (q, Self::from(r));
                }
                let n = other.hi.leading_zeros();
                let u1 = self >> 1;
                let v1 = other << n;
                let (tq, _) = u1.div_half_0(v1.hi);
                let mut tq = tq >> (Self::BITS / 2 - 1 - n);
                if tq != <$half>::ZERO {
                    tq -= <$half>::ONE;
                }
                let mut q = Self::from(tq);
                let mut r = self - other * q;
                if r >= other {
                    q += Self::ONE;
                    r = r - other;
                }
                (q, r)
            }
        }
    };
}

macro_rules! uint_impl_from_u {
    ($name:ident, $half:ty) => {
        impl std::convert::From<$half> for $name {
            fn from(small: $half) -> Self {
                Self {
                    lo: small,
                    hi: <$half>::MIN,
                }
            }
        }
    };
    ($name:ident, $half:ty, $from:ty) => {
        impl std::convert::From<$from> for $name {
            fn from(small: $from) -> Self {
                Self {
                    lo: <$half>::from(small),
                    hi: <$half>::MIN,
                }
            }
        }
    };
}

macro_rules! uint_impl_from_i {
    ($name:ident, $half:ty, $from:ty) => {
        impl std::convert::From<$from> for $name {
            fn from(small: $from) -> Self {
                Self {
                    lo: <$half>::from(small),
                    hi: if small > 0 { <$half>::MIN } else { <$half>::MAX },
                }
            }
        }
    };
}

uint_impl!(U256, U128, 256);
uint_impl_from_u!(U256, U128, bool);
uint_impl_from_u!(U256, U128, u8);
uint_impl_from_u!(U256, U128, u16);
uint_impl_from_u!(U256, U128, u32);
uint_impl_from_u!(U256, U128, u64);
uint_impl_from_u!(U256, U128, u128);
uint_impl_from_u!(U256, U128);
uint_impl_from_i!(U256, U128, i8);
uint_impl_from_i!(U256, U128, i16);
uint_impl_from_i!(U256, U128, i32);
uint_impl_from_i!(U256, U128, i64);
uint_impl_from_i!(U256, U128, i128);
uint_impl!(U512, U256, 512);
uint_impl_from_u!(U512, U256, bool);
uint_impl_from_u!(U512, U256, u8);
uint_impl_from_u!(U512, U256, u16);
uint_impl_from_u!(U512, U256, u32);
uint_impl_from_u!(U512, U256, u64);
uint_impl_from_u!(U512, U256, u128);
uint_impl_from_u!(U512, U256, U128);
uint_impl_from_u!(U512, U256);
uint_impl_from_i!(U512, U256, i8);
uint_impl_from_i!(U512, U256, i16);
uint_impl_from_i!(U512, U256, i32);
uint_impl_from_i!(U512, U256, i64);
uint_impl_from_i!(U512, U256, i128);
uint_impl!(U1024, U512, 1024);
uint_impl_from_u!(U1024, U512, bool);
uint_impl_from_u!(U1024, U512, u8);
uint_impl_from_u!(U1024, U512, u16);
uint_impl_from_u!(U1024, U512, u32);
uint_impl_from_u!(U1024, U512, u64);
uint_impl_from_u!(U1024, U512, u128);
uint_impl_from_u!(U1024, U512, U128);
uint_impl_from_u!(U1024, U512, U256);
uint_impl_from_u!(U1024, U512);
uint_impl_from_i!(U1024, U512, i8);
uint_impl_from_i!(U1024, U512, i16);
uint_impl_from_i!(U1024, U512, i32);
uint_impl_from_i!(U1024, U512, i64);
uint_impl_from_i!(U1024, U512, i128);

macro_rules! sint_impl {
    ($name:ident, $uint:ty) => {
        #[derive(Copy, Clone, Default, PartialEq, Eq)]
        pub struct $name {
            pub uint: $uint,
        }

        impl std::convert::From<$uint> for $name {
            fn from(uint: $uint) -> Self {
                Self { uint }
            }
        }

        impl std::cmp::PartialOrd for $name {
            fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(rhs))
            }
        }

        impl std::cmp::Ord for $name {
            fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
                let lhssign = self.uint.is_negative();
                let rhssign = rhs.uint.is_negative();
                match (lhssign, rhssign) {
                    (false, false) => self.uint.cmp(&rhs.uint),
                    (false, true) => std::cmp::Ordering::Greater,
                    (true, false) => std::cmp::Ordering::Less,
                    (true, true) => self.uint.cmp(&rhs.uint),
                }
            }
        }

        impl std::ops::Div for $name {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                self.wrapping_div(rhs)
            }
        }

        impl std::ops::DivAssign for $name {
            fn div_assign(&mut self, rhs: Self) {
                *self = self.wrapping_div(rhs)
            }
        }

        impl std::ops::Rem for $name {
            type Output = Self;
            fn rem(self, rhs: Self) -> Self::Output {
                self.wrapping_rem(rhs)
            }
        }

        impl std::ops::RemAssign for $name {
            fn rem_assign(&mut self, rhs: Self) {
                *self = self.wrapping_rem(rhs);
            }
        }

        impl $name {
            /// Inspired by https://github.com/chfast/intx/blob/master/include/intx/intx.hpp#L760
            fn div(self, rhs: Self) -> (Self, Self) {
                let x = self.uint;
                let y = rhs.uint;
                let x_is_neg = x.is_negative();
                let y_is_neg = y.is_negative();
                let x_abs = if x_is_neg { -x } else { x };
                let y_abs = if y_is_neg { -y } else { y };
                let q_is_neg = x_is_neg ^ y_is_neg;
                let r = x_abs.div(y_abs);
                let quo = r.0;
                let rem = r.1;
                let quo = Self::from(if q_is_neg { -quo } else { quo });
                let rem = Self::from(if x_is_neg { -rem } else { rem });
                (quo, rem)
            }

            /// Wrapping (modular) division. Computes self / rhs, wrapping around at the boundary of the type.
            ///
            /// The only case where such wrapping can occur is when one divides MIN / -1 on a signed type (where MIN is
            /// the negative minimal value for the type); this is equivalent to -MIN, a positive value that is too
            /// large to represent in the type. In such a case, this function returns MIN itself.
            pub fn wrapping_div(self, rhs: Self) -> Self {
                let minus_min = <$uint>::ONE << (<$uint>::BITS - 1);
                let minus_one = <$uint>::MAX;
                if rhs.uint == <$uint>::MIN {
                    Self::from(<$uint>::MAX)
                } else if self.uint == minus_min && rhs.uint == minus_one {
                    Self::from(minus_min)
                } else {
                    self.div(rhs).0
                }
            }

            /// Wrapping (modular) remainder. Computes self % rhs, wrapping around at the boundary of the type.
            ///
            /// Such wrap-around never actually occurs mathematically; implementation artifacts make x % y invalid for
            /// MIN / -1 on a signed type (where MIN is the negative minimal value). In such a case, this function
            /// returns 0.
            pub fn wrapping_rem(self, rhs: Self) -> Self {
                let minus_min = <$uint>::ONE << (<$uint>::BITS - 1);
                let minus_one = <$uint>::MAX;
                if rhs.uint == <$uint>::MIN {
                    self
                } else if self.uint == minus_min && rhs.uint == minus_one {
                    Self::from(<$uint>::MIN)
                } else {
                    self.div(rhs).1
                }
            }

            /// Panic-free bitwise shift-left; yields self << mask(rhs), where mask removes any high-order bits of rhs
            /// that would cause the shift to exceed the bitwidth of the type.
            ///
            /// Note that this is not the same as a rotate-left; the RHS of a wrapping shift-left is restricted to the
            /// range of the type, rather than the bits shifted out of the LHS being returned to the other end. The
            /// primitive integer types all implement a rotate_left function, which may be what you want instead.
            pub fn wrapping_shl(self, rhs: u32) -> Self {
                Self {
                    uint: self.uint.wrapping_shl(rhs),
                }
            }

            /// Panic-free bitwise sign shift-right.
            pub fn wrapping_shr(self, rhs: u32) -> Self {
                Self {
                    uint: self.uint.wrapping_sra(rhs),
                }
            }
        }
    };
}

sint_impl!(I256, U256);
sint_impl!(I512, U512);
sint_impl!(I1024, U1024);
