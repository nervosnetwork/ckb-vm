pub trait Element:
    Copy
    + Clone
    + Default
    + PartialEq
    + Eq
    + std::fmt::Display
    + std::fmt::LowerHex
    + std::ops::BitAnd<Output = Self>
    + std::ops::BitAndAssign
    + std::ops::BitOr<Output = Self>
    + std::ops::BitOrAssign
    + std::ops::BitXor<Output = Self>
    + std::ops::BitXorAssign
    + std::ops::Not
    + std::ops::Neg
    + std::cmp::PartialOrd
    + std::cmp::Ord
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign
    + std::ops::Sub<Output = Self>
    + std::ops::SubAssign
    + std::ops::Mul<Output = Self>
    + std::ops::MulAssign
    + std::ops::Div<Output = Self>
    + std::ops::DivAssign
    + std::ops::Rem<Output = Self>
    + std::ops::RemAssign
    + std::ops::Shl<u32, Output = Self>
    + std::ops::Shr<u32, Output = Self>
    + From<bool>
{
    /// The size of this integer type in bits.
    const BITS: u32;
    /// The smallest value that can be represented by this integer type.
    const MIN: Self;
    /// The largest value that can be represented by this integer type.
    const MAX: Self;
    /// The smallest signed value that can be represented by this integer type.
    const MIN_S: Self;
    /// The largest signed value that can be represented by this integer type.
    const MAX_S: Self;
    /// The one value that can be represented by this integer type.
    const ONE: Self;
    /// The zero value that can be represented by this integer type.
    const ZERO: Self;

    /// For integer operations, the scalar can be taken from the scalar x register specified by rs1. If XLEN>SEW, the
    /// least-significant SEW bits of the x register are used, unless otherwise specified. If XLEN<SEW, the value from
    /// the x register is sign-extended to SEW bits.
    fn vx_s(x: u64) -> Self;

    /// If XLEN>SEW, the least-significant SEW bits of the x register are used. If XLEN<SEW, the value from the x
    /// register is zero-extended to SEW bits.
    fn vx_u(x: u64) -> Self;

    /// For integer operations, the scalar can be a 5-bit immediate, imm[4:0], encoded in the rs1 field. The value is
    /// sign-extended to SEW bits, unless otherwise specified.
    fn vi_s(i: i32) -> Self;

    /// The value is zero-extended to SEW bits.
    fn vi_u(i: u32) -> Self;

    /// Returns the lower 8 bits.
    fn u8(self) -> u8;

    /// Returns the lower 16 bits.
    fn u16(self) -> u16;

    /// Returns the lower 32 bits.
    fn u32(self) -> u32;

    /// Returns the lower 64 bits.
    fn u64(self) -> u64;

    /// Returns the lower part with same type.
    fn lo_zext(self) -> Self {
        self.wrapping_shl(Self::BITS / 2).wrapping_shr(Self::BITS / 2)
    }

    /// Returns the lower part with sign extented.
    fn lo_sext(self) -> Self {
        self.wrapping_shl(Self::BITS / 2).wrapping_sra(Self::BITS / 2)
    }

    /// Returns the higher partr with same type.
    fn hi_zext(self) -> Self {
        self >> Self::BITS / 2
    }

    /// Returns true if self is positive and false if the number is zero or negative.
    fn is_positive(self) -> bool;

    /// Returns true if self is negative and false if the number is zero or positive.
    fn is_negative(self) -> bool;

    /// Read a native endian integer value from its representation as a byte slice in little endian.
    fn read(b: &[u8]) -> Self;

    /// Save the integer as a byte array in little-endian byte order to memory.
    fn save(&self, b: &mut [u8]);

    /// Save the lower part integer as a byte array in little-endian byte order to memory.
    fn save_lo(&self, b: &mut [u8]);

    /// Returns the number of leading zeros in the binary representation of self.
    fn leading_zeros(self) -> u32;

    /// Returns the number of trailing zeros in the binary representation of self.
    fn trailing_zeros(self) -> u32;

    /// Compare signed.
    fn cmp_s(&self, other: &Self) -> std::cmp::Ordering;

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

    /// Saturating integer addition. Computes self + rhs, saturating at the numeric bounds instead of overflowing.
    fn saturating_add(self, other: Self) -> (Self, bool);

    /// Saturating addition with a signed integer. Computes self + rhs, saturating at the numeric bounds instead of
    /// overflowing.
    fn saturating_add_s(self, other: Self) -> (Self, bool);

    /// Saturating integer subtraction. Computes self - rhs, saturating at the numeric bounds instead of overflowing.
    fn saturating_sub(self, other: Self) -> (Self, bool);

    /// Saturating integer subtraction. Computes self - rhs, saturating at the numeric bounds instead of overflowing.
    fn saturating_sub_s(self, other: Self) -> (Self, bool);

    /// Averaging adds of unsigned integers.
    fn average_add(self, other: Self) -> Self;

    /// Averaging adds of signed integers.
    fn average_add_s(self, other: Self) -> Self;

    /// Averaging subtract of unsigned integers.
    fn average_sub(self, other: Self) -> Self;

    /// Averaging subtract of signed integers.
    fn average_sub_s(self, other: Self) -> Self;

    /// Wrapping (modular) addition. Computes self + other, wrapping around at the boundary of the type.
    fn wrapping_add(self, other: Self) -> Self;

    /// Wrapping (modular) subtraction. Computes self - other, wrapping around at the boundary of the type.
    fn wrapping_sub(self, other: Self) -> Self;

    /// Wrapping (modular) subtraction. Computes other - self, wrapping around at the boundary of the type.
    fn wrapping_rsub(self, other: Self) -> Self {
        other.wrapping_sub(self)
    }

    /// Wrapping (modular) multiplication. Computes self * other, wrapping around at the boundary of the type.
    fn wrapping_mul(self, other: Self) -> Self;

    /// Wrapping (modular) division. Computes self / rhs. Wrapped division on unsigned types is just normal division.
    /// There’s no way wrapping could ever happen. This function exists, so that all operations are accounted for in
    /// the wrapping operations.
    fn wrapping_div(self, other: Self) -> Self;

    /// Wrapping (modular) division signed.
    fn wrapping_div_s(self, other: Self) -> Self;

    /// Wrapping (modular) remainder. Computes self % rhs. Wrapped remainder calculation on unsigned types is just the
    /// regular remainder calculation. There’s no way wrapping could ever happen. This function exists, so that all
    /// operations are accounted for in the wrapping operations.
    fn wrapping_rem(self, other: Self) -> Self;

    /// Wrapping (modular) remainder signed.
    fn wrapping_rem_s(self, other: Self) -> Self;

    /// Panic-free bitwise shift-left; yields self << mask(rhs), where mask removes any high-order bits of rhs
    /// that would cause the shift to exceed the bitwidth of the type.
    ///
    /// Note that this is not the same as a rotate-left; the RHS of a wrapping shift-left is restricted to the
    /// range of the type, rather than the bits shifted out of the LHS being returned to the other end. The
    /// primitive integer types all implement a rotate_left function, which may be what you want instead.
    fn wrapping_shl(self, other: u32) -> Self;

    /// Shift-left with element.
    fn wrapping_shl_e(self, other: Self) -> Self {
        self.wrapping_shl(other.u32())
    }

    /// Panic-free bitwise shift-right; yields self >> mask(rhs), where mask removes any high-order bits of rhs
    /// that would cause the shift to exceed the bitwidth of the type.
    ///
    /// Note that this is not the same as a rotate-right; the RHS of a wrapping shift-right is restricted to
    /// the range of the type, rather than the bits shifted out of the LHS being returned to the other end. The
    /// primitive integer types all implement a rotate_right function, which may be what you want instead.
    fn wrapping_shr(self, other: u32) -> Self;

    /// Shift-right with element.
    fn wrapping_shr_e(self, other: Self) -> Self {
        self.wrapping_shr(other.u32())
    }

    /// Panic-free bitwise sign shift-right.
    fn wrapping_sra(self, other: u32) -> Self;

    /// Sign shift-right with element.
    fn wrapping_sra_e(self, other: Self) -> Self {
        self.wrapping_sra(other.u32())
    }

    /// Calculates self + rhs + carry without the ability to overflow.
    fn carrying_add(self, other: Self, carry: bool) -> (Self, bool) {
        let (r, carry0) = self.overflowing_add(other);
        let (r, carry1) = r.overflowing_add(if carry { Self::ONE } else { Self::MIN });
        (r, carry0 | carry1)
    }

    /// Calculates self - rhs - borrow without the ability to overflow.
    fn carrying_sub(self, other: Self, carry: bool) -> (Self, bool) {
        let (r, borrow0) = self.overflowing_sub(other);
        let (r, borrow1) = r.overflowing_sub(if carry { Self::ONE } else { Self::MIN });
        (r, borrow0 | borrow1)
    }

    /// Widening add.
    fn widening_add(self, other: Self) -> (Self, Self) {
        let (lo, carry) = self.overflowing_add(other);
        (lo, if carry { Self::ONE } else { Self::MIN })
    }

    /// Signed widening add.
    fn widening_add_s(self, other: Self) -> (Self, Self) {
        let hi0 = if self.is_negative() { Self::MAX } else { Self::MIN };
        let hi1 = if other.is_negative() { Self::MAX } else { Self::MIN };
        let (lo, carry) = self.overflowing_add(other);
        let hi = hi0.wrapping_add(hi1).wrapping_add(Self::from(carry));
        (lo, hi)
    }

    /// Widening substract.
    fn widening_sub(self, other: Self) -> (Self, Self) {
        let (lo, borrow) = self.overflowing_sub(other);
        (lo, if borrow { Self::MAX } else { Self::MIN })
    }

    /// Signed widening substract.
    fn widening_sub_s(self, other: Self) -> (Self, Self) {
        let hi0 = if self.is_negative() { Self::MAX } else { Self::MIN };
        let hi1 = if other.is_negative() { Self::MAX } else { Self::MIN };
        let (lo, borrow) = self.overflowing_sub(other);
        let hi = hi0.wrapping_sub(hi1).wrapping_sub(Self::from(borrow));
        (lo, hi)
    }

    /// Function widening_mul returns the product of x and y: (lo, hi) = x * y
    /// with the product bits' upper half returned in hi and the lower half returned in lo.
    ///
    /// Inspired by https://pkg.go.dev/math/bits@go1.17.2#Mul64
    fn widening_mul(self, other: Self) -> (Self, Self) {
        let x0 = self.lo_zext();
        let x1 = self.hi_zext();
        let y0 = other.lo_zext();
        let y1 = other.hi_zext();
        let w0 = x0 * y0;
        let t = x1 * y0 + w0.hi_zext();
        let w1 = t.lo_zext();
        let w2 = t.hi_zext();
        let w1 = w1 + x0 * y1;
        let hi = x1 * y1 + w2 + w1.hi_zext();
        let lo = self.wrapping_mul(other);
        (lo, hi)
    }

    /// Signed interger widening multiple.
    ///
    /// Inspired by https://sqlite.in/?qa=668884/c-32-bit-signed-integer-multiplication-without-using-64-bit-data-type
    fn widening_mul_s(self, other: Self) -> (Self, Self) {
        let (lo, hi) = self.widening_mul(other);
        let hi = hi
            - if self.is_negative() { other } else { Self::MIN }
            - if other.is_negative() { self } else { Self::MIN };
        (lo, hi)
    }

    /// Widening signed and unsigned integer multiply.
    fn widening_mul_su(self, other: Self) -> (Self, Self) {
        if !other.is_negative() {
            self.widening_mul_s(other)
        } else {
            let (lo, hi) = self.widening_mul_s(other);
            let hi = hi + self;
            (lo, hi)
        }
    }
}

pub mod alu {
    use crate::Element;

    /// Set if equal.
    pub fn seq<T: Element>(lhs: T, rhs: T) -> bool {
        lhs == rhs
    }

    /// Set if not equal.
    pub fn sne<T: Element>(lhs: T, rhs: T) -> bool {
        lhs != rhs
    }

    /// Set if less than, unsigned.
    pub fn sltu<T: Element>(lhs: T, rhs: T) -> bool {
        lhs < rhs
    }

    /// Set if less than, signed.
    pub fn slt<T: Element>(lhs: T, rhs: T) -> bool {
        lhs.cmp_s(&rhs) == std::cmp::Ordering::Less
    }

    /// Set if less than or equal, unsigned.
    pub fn sleu<T: Element>(lhs: T, rhs: T) -> bool {
        lhs <= rhs
    }

    /// Set if less than or equal, signed.
    pub fn sle<T: Element>(lhs: T, rhs: T) -> bool {
        lhs.cmp_s(&rhs) != std::cmp::Ordering::Greater
    }

    /// Set if greater than, unsigned.
    pub fn sgtu<T: Element>(lhs: T, rhs: T) -> bool {
        lhs > rhs
    }

    /// Set if greater than, signed.
    pub fn sgt<T: Element>(lhs: T, rhs: T) -> bool {
        lhs.cmp_s(&rhs) == std::cmp::Ordering::Greater
    }

    /// Unsigned maximum.
    pub fn maxu<T: Element>(lhs: T, rhs: T) -> T {
        if lhs > rhs {
            lhs
        } else {
            rhs
        }
    }

    /// Signed maximum.
    pub fn max<T: Element>(lhs: T, rhs: T) -> T {
        if lhs.cmp_s(&rhs) == std::cmp::Ordering::Greater {
            lhs
        } else {
            rhs
        }
    }

    /// Unsigned minimum.
    pub fn minu<T: Element>(lhs: T, rhs: T) -> T {
        if lhs < rhs {
            lhs
        } else {
            rhs
        }
    }

    /// Signed minimum.
    pub fn min<T: Element>(lhs: T, rhs: T) -> T {
        if lhs.cmp_s(&rhs) == std::cmp::Ordering::Less {
            lhs
        } else {
            rhs
        }
    }

    /// Bitwise and.
    pub fn and<T: Element>(lhs: T, rhs: T) -> T {
        lhs & rhs
    }

    /// Bitwise or.
    pub fn or<T: Element>(lhs: T, rhs: T) -> T {
        lhs | rhs
    }

    /// Bitwise xor.
    pub fn xor<T: Element>(lhs: T, rhs: T) -> T {
        lhs ^ rhs
    }

    /// Saturating adds of unsigned integers.
    pub fn saddu<T: Element>(lhs: T, rhs: T) -> T {
        let (r, _) = lhs.saturating_add(rhs);
        r
    }

    /// Saturating adds of signed integers.
    pub fn sadd<T: Element>(lhs: T, rhs: T) -> T {
        let (r, _) = lhs.saturating_add_s(rhs);
        r
    }

    /// Saturating subtract of unsigned integers.
    pub fn ssubu<T: Element>(lhs: T, rhs: T) -> T {
        let (r, _) = lhs.saturating_sub(rhs);
        r
    }

    /// Saturating subtract of signed integers.
    pub fn ssub<T: Element>(lhs: T, rhs: T) -> T {
        let (r, _) = lhs.saturating_sub_s(rhs);
        r
    }

    /// Copy rhs.
    pub fn mv<T: Element>(_: T, rhs: T) -> T {
        rhs
    }

    /// Signed multiply, returning high bits of product.
    pub fn mulh<T: Element>(lhs: T, rhs: T) -> T {
        let (_, hi) = lhs.widening_mul_s(rhs);
        hi
    }

    /// Unsigned multiply, returning high bits of product.
    pub fn mulhu<T: Element>(lhs: T, rhs: T) -> T {
        let (_, hi) = lhs.widening_mul(rhs);
        hi
    }

    /// Signed(vs2)-Unsigned multiply, returning high bits of product.
    pub fn mulhsu<T: Element>(lhs: T, rhs: T) -> T {
        let (_, hi) = lhs.widening_mul_su(rhs);
        hi
    }

    /// Get carry out of addition.
    pub fn madc<T: Element>(lhs: T, rhs: T) -> bool {
        let (_, carry) = lhs.overflowing_add(rhs);
        carry
    }

    /// Get the borrow out of subtraction.
    pub fn msbc<T: Element>(lhs: T, rhs: T) -> bool {
        let (_, borrow) = lhs.overflowing_sub(rhs);
        borrow
    }

    /// Calculates self + rhs + carry without the ability to overflow.
    pub fn adc<T: Element>(lhs: T, rhs: T, carry: bool) -> T {
        let (r, _) = lhs.carrying_add(rhs, carry);
        r
    }

    /// Calculates self - rhs - borrow without the ability to overflow.
    pub fn sbc<T: Element>(lhs: T, rhs: T, borrow: bool) -> T {
        let (r, _) = lhs.carrying_sub(rhs, borrow);
        r
    }
}

macro_rules! uint_wrap_impl {
    ($name:ident, $uint:ty, $sint:ty) => {
        #[derive(Copy, Clone, Default, PartialEq, Eq)]
        pub struct $name(pub $uint);

        impl std::fmt::LowerHex for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let a = format!("{:x}", self.0);
                write!(
                    f,
                    "{}",
                    String::from("0").repeat(Self::BITS as usize / 4 - a.len())
                )?;
                write!(f, "{}", a)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let a = format!("{:x}", self.0);
                write!(
                    f,
                    "{}",
                    String::from("0").repeat(Self::BITS as usize / 4 - a.len())
                )?;
                write!(f, "{}", a)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let a = format!("{:x}", self.0);
                write!(
                    f,
                    "{}",
                    String::from("0").repeat(Self::BITS as usize / 4 - a.len())
                )?;
                write!(f, "{}", a)
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
            const MIN_S: Self = Self(<$sint>::MIN as $uint);
            const MAX_S: Self = Self(<$sint>::MAX as $uint);
            const ONE: Self = Self(1);
            const ZERO: Self = Self(0);

            fn vx_s(x: u64) -> Self {
                if Self::BITS <= 64 {
                    Self(x as $uint)
                } else {
                    Self(x as i64 as $uint)
                }
            }

            fn vx_u(x: u64) -> Self {
                Self(x as $uint)
            }

            fn vi_s(i: i32) -> Self {
                assert!(i >= -16);
                assert!(i <= 15);
                Self(i as $uint)
            }

            fn vi_u(i: u32) -> Self {
                assert!(i <= 31);
                Self(i as $uint)
            }

            fn u8(self) -> u8 {
                self.0 as u8
            }

            fn u16(self) -> u16 {
                self.0 as u16
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

            fn read(b: &[u8]) -> Self {
                let mut buf = [0u8; Self::BITS as usize >> 3];
                buf.copy_from_slice(&b);
                Self(<$uint>::from_le_bytes(buf))
            }

            fn save(&self, b: &mut [u8]) {
                let buf = self.0.to_le_bytes();
                b.copy_from_slice(&buf);
            }

            fn save_lo(&self, b: &mut [u8]) {
                let buf = self.0.to_le_bytes();
                b.copy_from_slice(&buf[0..buf.len() >> 1]);
            }

            fn leading_zeros(self) -> u32 {
                self.0.leading_zeros()
            }

            fn trailing_zeros(self) -> u32 {
                self.0.trailing_zeros()
            }

            fn cmp_s(&self, other: &Self) -> std::cmp::Ordering {
                (self.0 as $sint).cmp(&(other.0 as $sint))
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

            fn saturating_add(self, other: Self) -> (Self, bool) {
                let (r, overflow) = self.overflowing_add(other);
                if overflow {
                    (Self::MAX, overflow)
                } else {
                    (r, overflow)
                }
            }

            fn saturating_add_s(self, other: Self) -> (Self, bool) {
                let r = self.wrapping_add(other);
                if !(self ^ other).is_negative() {
                    if (r ^ self).is_negative() {
                        let r = if self.is_negative() {
                            Self::MIN_S
                        } else {
                            Self::MAX_S
                        };
                        return (r, true);
                    }
                }
                (r, false)
            }

            fn saturating_sub(self, other: Self) -> (Self, bool) {
                if self > other {
                    (self.wrapping_sub(other), false)
                } else {
                    (Self::MIN, true)
                }
            }

            fn saturating_sub_s(self, other: Self) -> (Self, bool) {
                let r = self.wrapping_sub(other);
                if (self ^ other).is_negative() {
                    if (r ^ self).is_negative() {
                        let r = if self.is_negative() {
                            Self::MIN_S
                        } else {
                            Self::MAX_S
                        };
                        return (r, true);
                    }
                }
                (r, false)
            }

            fn average_add(self, other: Self) -> Self {
                (self & other).wrapping_add((self ^ other) >> 1)
            }

            fn average_add_s(self, other: Self) -> Self {
                (self & other).wrapping_add((self ^ other) >> 1)
            }

            fn average_sub(self, other: Self) -> Self {
                self.average_add(-other)
            }

            fn average_sub_s(self, other: Self) -> Self {
                self.average_add_s(-other)
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

            fn wrapping_div_s(self, other: Self) -> Self {
                if other.0 == 0 {
                    Self::MAX
                } else if self.0 == 1 << (Self::BITS - 1) && other == Self::MAX {
                    Self::ONE << (Self::BITS - 1)
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

            fn wrapping_rem_s(self, other: Self) -> Self {
                if other.0 == 0 {
                    self
                } else if self.0 == 1 << (Self::BITS - 1) && other == Self::MAX {
                    Self::MIN
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
    ($name:ty, $from:ty) => {
        impl From<$from> for $name {
            fn from(small: $from) -> Self {
                Self::from(small.0)
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
uint_wrap_from_impl!(U16, U8);
uint_wrap_from_impl!(U32, u32, u8);
uint_wrap_from_impl!(U32, u32, i8);
uint_wrap_from_impl!(U32, u32, u16);
uint_wrap_from_impl!(U32, u32, i16);
uint_wrap_from_impl!(U32, u32, u32);
uint_wrap_from_impl!(U32, u32, i32);
uint_wrap_from_impl!(U32, U8);
uint_wrap_from_impl!(U32, U16);
uint_wrap_from_impl!(U64, u64, u8);
uint_wrap_from_impl!(U64, u64, i8);
uint_wrap_from_impl!(U64, u64, u16);
uint_wrap_from_impl!(U64, u64, i16);
uint_wrap_from_impl!(U64, u64, u32);
uint_wrap_from_impl!(U64, u64, i32);
uint_wrap_from_impl!(U64, u64, u64);
uint_wrap_from_impl!(U64, u64, i64);
uint_wrap_from_impl!(U64, U8);
uint_wrap_from_impl!(U64, U16);
uint_wrap_from_impl!(U64, U32);
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
uint_wrap_from_impl!(U128, U8);
uint_wrap_from_impl!(U128, U16);
uint_wrap_from_impl!(U128, U32);
uint_wrap_from_impl!(U128, U64);

macro_rules! uint_impl {
    ($name:ident, $half:ty) => {
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
            const MIN_S: Self = Self {
                lo: <$half>::MIN,
                hi: <$half>::MIN_S,
            };
            const MAX_S: Self = Self {
                lo: <$half>::MAX,
                hi: <$half>::MAX_S,
            };
            const ONE: Self = Self {
                lo: <$half>::ONE,
                hi: <$half>::MIN,
            };
            const ZERO: Self = Self {
                lo: <$half>::MIN,
                hi: <$half>::MIN,
            };

            fn vx_s(x: u64) -> Self {
                Self::from(x as i64)
            }

            fn vx_u(x: u64) -> Self {
                Self::from(x)
            }

            fn vi_s(i: i32) -> Self {
                assert!(i >= -16);
                assert!(i <= 15);
                Self::from(i)
            }

            fn vi_u(i: u32) -> Self {
                assert!(i <= 31);
                Self::from(i)
            }

            fn u8(self) -> u8 {
                self.lo.u8()
            }

            fn u16(self) -> u16 {
                self.lo.u16()
            }

            fn u32(self) -> u32 {
                self.lo.u32()
            }

            fn u64(self) -> u64 {
                self.lo.u64()
            }

            fn lo_zext(self) -> Self {
                Self::from(self.lo)
            }

            fn hi_zext(self) -> Self {
                Self::from(self.hi)
            }

            fn is_positive(self) -> bool {
                self != <$name>::MIN && self.wrapping_shr(Self::BITS - 1) == <$name>::MIN
            }

            fn is_negative(self) -> bool {
                self != <$name>::MIN && self.wrapping_shr(Self::BITS - 1) == <$name>::ONE
            }

            fn read(b: &[u8]) -> Self {
                let mut buf = [0u8; Self::BITS as usize >> 3];
                buf.copy_from_slice(&b);
                Self {
                    lo: <$half>::read(&b[0..Self::BITS as usize >> 4]),
                    hi: <$half>::read(&b[Self::BITS as usize >> 4..Self::BITS as usize >> 3]),
                }
            }

            fn save(&self, b: &mut [u8]) {
                self.lo.save(&mut b[0..Self::BITS as usize >> 4]);
                self.hi
                    .save(&mut b[Self::BITS as usize >> 4..Self::BITS as usize >> 3]);
            }

            fn save_lo(&self, b: &mut [u8]) {
                self.lo.save(b);
            }

            fn leading_zeros(self) -> u32 {
                if self.hi == <$half>::MIN {
                    Self::BITS / 2 + self.lo.leading_zeros()
                } else {
                    self.hi.leading_zeros()
                }
            }

            fn trailing_zeros(self) -> u32 {
                if self.lo == <$half>::MIN {
                    Self::BITS / 2 + self.hi.trailing_zeros()
                } else {
                    self.lo.trailing_zeros()
                }
            }

            fn cmp_s(&self, other: &Self) -> std::cmp::Ordering {
                let lhssign = self.is_negative();
                let rhssign = other.is_negative();
                match (lhssign, rhssign) {
                    (false, false) => self.cmp(&other),
                    (false, true) => std::cmp::Ordering::Greater,
                    (true, false) => std::cmp::Ordering::Less,
                    (true, true) => self.cmp(&other),
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
                let lo = Self { lo: lo.lo, hi };
                (lo, hi_overflow_mul || hi_overflow_add)
            }

            fn overflowing_div(self, other: Self) -> (Self, bool) {
                (self.wrapping_div(other), false)
            }

            fn overflowing_rem(self, other: Self) -> (Self, bool) {
                (self.wrapping_rem(other), false)
            }

            fn saturating_add(self, other: Self) -> (Self, bool) {
                let (r, overflow) = self.overflowing_add(other);
                if overflow {
                    (Self::MAX, overflow)
                } else {
                    (r, overflow)
                }
            }

            fn saturating_add_s(self, other: Self) -> (Self, bool) {
                let r = self.wrapping_add(other);
                if !(self ^ other).is_negative() {
                    if (r ^ self).is_negative() {
                        let r = if self.is_negative() {
                            Self::MIN_S
                        } else {
                            Self::MAX_S
                        };
                        return (r, true);
                    }
                }
                (r, false)
            }

            fn saturating_sub(self, other: Self) -> (Self, bool) {
                if self > other {
                    (self.wrapping_sub(other), false)
                } else {
                    (Self::MIN, true)
                }
            }

            fn saturating_sub_s(self, other: Self) -> (Self, bool) {
                let r = self.wrapping_sub(other);
                if (self ^ other).is_negative() {
                    if (r ^ self).is_negative() {
                        let r = if self.is_negative() {
                            Self::MIN_S
                        } else {
                            Self::MAX_S
                        };
                        return (r, true);
                    }
                }
                (r, false)
            }

            fn average_add(self, other: Self) -> Self {
                (self & other).wrapping_add((self ^ other) >> 1)
            }

            fn average_add_s(self, other: Self) -> Self {
                (self & other).wrapping_add((self ^ other) >> 1)
            }

            fn average_sub(self, other: Self) -> Self {
                self.average_add(-other)
            }

            fn average_sub_s(self, other: Self) -> Self {
                self.average_add_s(-other)
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

            fn wrapping_div_s(self, other: Self) -> Self {
                let minus_min = Self::ONE << (Self::BITS - 1);
                let minus_one = Self::MAX;
                if other == Self::MIN {
                    Self::MAX
                } else if self == minus_min && other == minus_one {
                    minus_min
                } else {
                    self.divs(other).0
                }
            }

            fn wrapping_rem(self, other: Self) -> Self {
                if other == Self::MIN {
                    self
                } else {
                    self.div(other).1
                }
            }

            fn wrapping_rem_s(self, other: Self) -> Self {
                let minus_min = Self::ONE << (Self::BITS - 1);
                let minus_one = Self::MAX;
                if other == Self::MIN {
                    self
                } else if self == minus_min && other == minus_one {
                    Self::MIN
                } else {
                    self.divs(other).1
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
                let hi = if self.is_negative() && shamt != 0 {
                    Self::MAX << (Self::BITS - shamt)
                } else {
                    Self::MIN
                };
                let lo = self.wrapping_shr(shamt);
                hi | lo
            }
        }

        impl $name {
            /// Create a native endian integer value from its representation as a byte array in big endian.
            pub fn from_be_bytes(bytes: [u8; Self::BITS as usize / 8]) -> Self {
                let mut t = [0u8; Self::BITS as usize / 8 / 2];
                let a = 0x00;
                let b = Self::BITS as usize / 8 / 2;
                let c = b;
                let d = Self::BITS as usize / 8;
                t.copy_from_slice(&bytes[a..b]);
                let hi = <$half>::from_be_bytes(t);
                t.copy_from_slice(&bytes[c..d]);
                let lo = <$half>::from_be_bytes(t);
                Self { lo, hi }
            }

            /// Create a native endian integer value from its representation as a byte array in little endian.
            pub fn from_le_bytes(bytes: [u8; Self::BITS as usize / 8]) -> Self {
                let mut t = [0u8; Self::BITS as usize / 8 / 2];
                let a = 0x00;
                let b = Self::BITS as usize / 8 / 2;
                let c = b;
                let d = Self::BITS as usize / 8;
                t.copy_from_slice(&bytes[a..b]);
                let lo = <$half>::from_le_bytes(t);
                t.copy_from_slice(&bytes[c..d]);
                let hi = <$half>::from_le_bytes(t);
                Self { lo, hi }
            }

            /// Return the memory representation of this integer as a byte array in big-endian (network) byte order.
            pub fn to_be_bytes(self) -> [u8; Self::BITS as usize / 8] {
                let mut r = [0u8; Self::BITS as usize / 8];
                let a = 0x00;
                let b = Self::BITS as usize / 8 / 2;
                let c = b;
                let d = Self::BITS as usize / 8;
                r[a..b].copy_from_slice(&self.hi.to_be_bytes());
                r[c..d].copy_from_slice(&self.lo.to_be_bytes());
                return r;
            }

            /// Return the memory representation of this integer as a byte array in little-endian byte order.
            pub fn to_le_bytes(self) -> [u8; Self::BITS as usize / 8] {
                let mut r = [0u8; Self::BITS as usize / 8];
                let a = 0x00;
                let b = Self::BITS as usize / 8 / 2;
                let c = b;
                let d = Self::BITS as usize / 8;
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
                    (Self { lo, hi }, r)
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
                    r -= other;
                }
                (q, r)
            }

            /// Inspired by https://github.com/chfast/intx/blob/master/include/intx/intx.hpp#L760
            fn divs(self, rhs: Self) -> (Self, Self) {
                let x = self;
                let y = rhs;
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

uint_impl!(U256, U128);
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
uint_impl!(U512, U256);
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
uint_impl!(U1024, U512);
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
uint_impl!(U2048, U1024);
uint_impl_from_u!(U2048, U1024, bool);
uint_impl_from_u!(U2048, U1024, u8);
uint_impl_from_u!(U2048, U1024, u16);
uint_impl_from_u!(U2048, U1024, u32);
uint_impl_from_u!(U2048, U1024, u64);
uint_impl_from_u!(U2048, U1024, u128);
uint_impl_from_u!(U2048, U1024, U128);
uint_impl_from_u!(U2048, U1024, U256);
uint_impl_from_u!(U2048, U1024, U512);
uint_impl_from_u!(U2048, U1024);
uint_impl_from_i!(U2048, U1024, i8);
uint_impl_from_i!(U2048, U1024, i16);
uint_impl_from_i!(U2048, U1024, i32);
uint_impl_from_i!(U2048, U1024, i64);
uint_impl_from_i!(U2048, U1024, i128);
