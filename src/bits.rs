#[inline(always)]
pub fn roundup(x: u64, round: u64) -> u64 {
    debug_assert!(round.is_power_of_two());
    // x + (((!x) + 1) & (round - 1))
    x.wrapping_add((!x).wrapping_add(1) & (round.wrapping_sub(1)))
}

#[inline(always)]
pub fn rounddown(x: u64, round: u64) -> u64 {
    debug_assert!(round.is_power_of_two());
    // x & !(round - 1)
    x & !(round.wrapping_sub(1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_roundup() {
        assert_eq!(0, roundup(0, 16));
        assert_eq!(16, roundup(1, 16));
        assert_eq!(16, roundup(15, 16));
        assert_eq!(16, roundup(16, 16));
        assert_eq!(32, roundup(17, 16));
        assert_eq!(u64::MAX - 15, roundup(u64::MAX - 15, 16));
        assert_eq!(0, roundup(u64::MAX, 16));
    }

    #[test]
    fn test_rounddown() {
        assert_eq!(0, rounddown(0, 16));
        assert_eq!(0, rounddown(1, 16));
        assert_eq!(0, rounddown(15, 16));
        assert_eq!(16, rounddown(16, 16));
        assert_eq!(16, rounddown(17, 16));
        assert_eq!(u64::MAX - 15, rounddown(u64::MAX, 16));
    }

    proptest! {
        #[test]
        #[cfg_attr(all(miri, feature = "miri-ci"), ignore)]
        fn roundup_proptest(x: u64, round in (0u32..16).prop_map(|d| 2u64.pow(d))) {
            prop_assume!(x.checked_add(round).is_some(), "avoid integer overflow");
            let result = roundup(x, round);

            // multiple of round
            assert_eq!(result % round, 0);

            // lower bound
            assert!(result >= x);

            // upper bound
            assert!(result < x + round);
        }

        #[test]
        #[cfg_attr(all(miri, feature = "miri-ci"), ignore)]
        fn rounddown_proptest(x: u64, round in (0u32..16).prop_map(|d| 2u64.pow(d))) {
            let result = rounddown(x, round);

            // multiple of round
            assert_eq!(result % round, 0);

            // upper bound
            assert!(result <= x);

            // lower bound
            if let Some(lower_bound) = x.checked_sub(round) {
                assert!(result > lower_bound);
            } else {
                assert_eq!(result, 0);
            }
        }
    }
}
