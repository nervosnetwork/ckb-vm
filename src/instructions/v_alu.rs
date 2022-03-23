use eint::Eint;

/// Set if equal.
pub fn seq<T: Eint>(lhs: T, rhs: T) -> bool {
    lhs == rhs
}

/// Set if not equal.
pub fn sne<T: Eint>(lhs: T, rhs: T) -> bool {
    lhs != rhs
}

/// Set if less than, unsigned.
pub fn sltu<T: Eint>(lhs: T, rhs: T) -> bool {
    lhs < rhs
}

/// Set if less than, signed.
pub fn slt<T: Eint>(lhs: T, rhs: T) -> bool {
    lhs.cmp_s(&rhs) == std::cmp::Ordering::Less
}

/// Set if less than or equal, unsigned.
pub fn sleu<T: Eint>(lhs: T, rhs: T) -> bool {
    lhs <= rhs
}

/// Set if less than or equal, signed.
pub fn sle<T: Eint>(lhs: T, rhs: T) -> bool {
    lhs.cmp_s(&rhs) != std::cmp::Ordering::Greater
}

/// Set if greater than, unsigned.
pub fn sgtu<T: Eint>(lhs: T, rhs: T) -> bool {
    lhs > rhs
}

/// Set if greater than, signed.
pub fn sgt<T: Eint>(lhs: T, rhs: T) -> bool {
    lhs.cmp_s(&rhs) == std::cmp::Ordering::Greater
}

/// Unsigned maximum.
pub fn maxu<T: Eint>(lhs: T, rhs: T) -> T {
    if lhs > rhs {
        lhs
    } else {
        rhs
    }
}

/// Signed maximum.
pub fn max<T: Eint>(lhs: T, rhs: T) -> T {
    if lhs.cmp_s(&rhs) == std::cmp::Ordering::Greater {
        lhs
    } else {
        rhs
    }
}

/// Unsigned minimum.
pub fn minu<T: Eint>(lhs: T, rhs: T) -> T {
    if lhs < rhs {
        lhs
    } else {
        rhs
    }
}

/// Signed minimum.
pub fn min<T: Eint>(lhs: T, rhs: T) -> T {
    if lhs.cmp_s(&rhs) == std::cmp::Ordering::Less {
        lhs
    } else {
        rhs
    }
}

/// Bitwise and.
pub fn and<T: Eint>(lhs: T, rhs: T) -> T {
    lhs & rhs
}

/// Bitwise or.
pub fn or<T: Eint>(lhs: T, rhs: T) -> T {
    lhs | rhs
}

/// Bitwise xor.
pub fn xor<T: Eint>(lhs: T, rhs: T) -> T {
    lhs ^ rhs
}

/// Saturating adds of unsigned integers.
pub fn saddu<T: Eint>(lhs: T, rhs: T) -> T {
    let (r, _) = lhs.saturating_add_u(rhs);
    r
}

/// Saturating adds of signed integers.
pub fn sadd<T: Eint>(lhs: T, rhs: T) -> T {
    let (r, _) = lhs.saturating_add_s(rhs);
    r
}

/// Saturating subtract of unsigned integers.
pub fn ssubu<T: Eint>(lhs: T, rhs: T) -> T {
    let (r, _) = lhs.saturating_sub_u(rhs);
    r
}

/// Saturating subtract of signed integers.
pub fn ssub<T: Eint>(lhs: T, rhs: T) -> T {
    let (r, _) = lhs.saturating_sub_s(rhs);
    r
}

/// Copy rhs.
pub fn mv<T: Eint>(_: T, rhs: T) -> T {
    rhs
}

/// Signed multiply, returning high bits of product.
pub fn mulh<T: Eint>(lhs: T, rhs: T) -> T {
    let (_, hi) = lhs.widening_mul_s(rhs);
    hi
}

/// Unsigned multiply, returning high bits of product.
pub fn mulhu<T: Eint>(lhs: T, rhs: T) -> T {
    let (_, hi) = lhs.widening_mul_u(rhs);
    hi
}

/// Signed(vs2)-Unsigned multiply, returning high bits of product.
pub fn mulhsu<T: Eint>(lhs: T, rhs: T) -> T {
    let (_, hi) = lhs.widening_mul_su(rhs);
    hi
}

/// Get carry out of addition.
pub fn madc<T: Eint>(lhs: T, rhs: T) -> bool {
    let (_, carry) = lhs.overflowing_add_u(rhs);
    carry
}

/// Get the borrow out of subtraction.
pub fn msbc<T: Eint>(lhs: T, rhs: T) -> bool {
    let (_, borrow) = lhs.overflowing_sub_u(rhs);
    borrow
}

/// Calculates self + rhs + carry without the ability to overflow.
pub fn adc<T: Eint>(lhs: T, rhs: T, carry: bool) -> T {
    lhs.wrapping_add(rhs).wrapping_add(T::from(carry))
}

/// Calculates self - rhs - borrow without the ability to overflow.
pub fn sbc<T: Eint>(lhs: T, rhs: T, borrow: bool) -> T {
    lhs.wrapping_sub(rhs).wrapping_sub(T::from(borrow))
}

/// Calculates carry_out(self + rhs + carry) without the ability to overflow.
pub fn madcm<T: Eint>(lhs: T, rhs: T, carry: bool) -> bool {
    let (r, carry_0) = lhs.overflowing_add_u(rhs);
    let (_, carry_1) = r.overflowing_add_u(T::from(carry));
    carry_0 | carry_1
}

/// Calculates borrow_out(self - rhs - borrow) without the ability to overflow.
pub fn msbcm<T: Eint>(lhs: T, rhs: T, borrow: bool) -> bool {
    let (r, carry_0) = lhs.overflowing_sub_u(rhs);
    let (_, carry_1) = r.overflowing_sub_u(T::from(borrow));
    carry_0 | carry_1
}

/// Integer multiply-add, overwrite addend
pub fn macc<T: Eint>(lhs: T, rhs: T, r: T) -> T {
    r + (rhs * lhs)
}

/// Integer multiply-sub, overwrite minuend
pub fn nmsac<T: Eint>(lhs: T, rhs: T, r: T) -> T {
    r - (rhs * lhs)
}

/// Integer multiply-add, overwrite multiplicand
pub fn madd<T: Eint>(lhs: T, rhs: T, r: T) -> T {
    lhs + (rhs * r)
}

/// Integer multiply-sub, overwrite multiplicand
pub fn nmsub<T: Eint>(lhs: T, rhs: T, r: T) -> T {
    lhs - (rhs * r)
}

/// Widening unsigned-integer multiply-add, overwrite addend
pub fn wmaccu<T: Eint>(lhs: T, rhs: T, r_lo: T, r_hi: T) -> (T, T) {
    let (lo, hi) = lhs.widening_mul_u(rhs);
    let (lo, carry) = lo.overflowing_add_u(r_lo);
    let hi = hi.wrapping_add(T::from(carry)).wrapping_add(r_hi);
    (lo, hi)
}

/// Widening signed-integer multiply-add, overwrite addend
pub fn wmacc<T: Eint>(lhs: T, rhs: T, r_lo: T, r_hi: T) -> (T, T) {
    let (lo, hi) = lhs.widening_mul_s(rhs);
    let (lo, carry) = lo.overflowing_add_u(r_lo);
    let hi = hi.wrapping_add(T::from(carry)).wrapping_add(r_hi);
    (lo, hi)
}

/// Widening signed-unsigned-integer multiply-add, overwrite addend
pub fn wmaccsu<T: Eint>(lhs: T, rhs: T, r_lo: T, r_hi: T) -> (T, T) {
    let (lo, hi) = rhs.widening_mul_su(lhs);
    let (lo, carry) = lo.overflowing_add_u(r_lo);
    let hi = hi.wrapping_add(T::from(carry)).wrapping_add(r_hi);
    (lo, hi)
}

/// Widening unsigned-signed-integer multiply-add, overwrite addend
pub fn wmaccus<T: Eint>(lhs: T, rhs: T, r_lo: T, r_hi: T) -> (T, T) {
    wmaccsu(rhs, lhs, r_lo, r_hi)
}

/// The vector integer merge instructions combine two source operands based on a mask
pub fn merge<T: Eint>(lhs: T, rhs: T, mask: bool) -> T {
    if mask {
        rhs
    } else {
        lhs
    }
}

/// Wrapping (modular) subtraction. Computes other - self, wrapping around at the boundary of the type.
pub fn rsub<T: Eint>(lhs: T, rhs: T) -> T {
    rhs.wrapping_sub(lhs)
}

pub fn sll<T: Eint>(lhs: T, rhs: T) -> T {
    lhs.wrapping_shl(rhs.u32())
}

pub fn srl<T: Eint>(lhs: T, rhs: T) -> T {
    lhs.wrapping_shr(rhs.u32())
}

pub fn sra<T: Eint>(lhs: T, rhs: T) -> T {
    lhs.wrapping_sra(rhs.u32())
}

pub fn smul<T: Eint>(lhs: T, rhs: T) -> T {
    if lhs == rhs && lhs == T::MIN_S {
        return T::MAX_S;
    } else {
        let result = lhs.widening_mul_s(rhs);
        let shamt = T::BITS - 1;
        let lo = result.0.wrapping_shr(shamt) | result.1.wrapping_shl(1);
        return lo;
    }
}
