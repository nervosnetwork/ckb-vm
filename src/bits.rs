// Source: https://en.wikipedia.org/wiki/Bit_manipulation#Example_of_bit_manipulation
#[inline(always)]
pub fn power_of_2(x: usize) -> bool {
    x > 0 && ((x & (x - 1)) == 0)
}

#[inline(always)]
pub fn roundup(x: usize, round: usize) -> usize {
    debug_assert!(power_of_2(round));
    // x + (((!x) + 1) & (round - 1))
    x + ((!x).wrapping_add(1) & (round.wrapping_sub(1)))
}

#[inline(always)]
pub fn rounddown(x: usize, round: usize) -> usize {
    debug_assert!(power_of_2(round));
    // x & !(round - 1)
    x & !(round.wrapping_sub(1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_power_of_2() {
        let test_cases = [
            (0, false),
            (1, true),
            (2, true),
            (3, false),
            (4, true),
            (5, false),
        ];
        for test_case in test_cases.iter() {
            assert_eq!(test_case.1, power_of_2(test_case.0))
        }
    }

    #[test]
    fn test_roundup() {
        assert_eq!(0, roundup(0, 16));
        assert_eq!(16, roundup(1, 16));
        assert_eq!(16, roundup(15, 16));
        assert_eq!(16, roundup(16, 16));
        assert_eq!(32, roundup(17, 16));
        assert_eq!(
            usize::max_value() - 15,
            roundup(usize::max_value() - 15, 16)
        );
    }

    #[test]
    fn test_rounddown() {
        assert_eq!(0, rounddown(0, 16));
        assert_eq!(0, rounddown(1, 16));
        assert_eq!(0, rounddown(15, 16));
        assert_eq!(16, rounddown(16, 16));
        assert_eq!(16, rounddown(17, 16));
        assert_eq!(usize::max_value() - 15, rounddown(usize::max_value(), 16));
    }

    proptest! {
        #[test]
        fn roundup_proptest(x: usize, round in (0u32..16).prop_map(|d| 2usize.pow(d))) {
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
        fn rounddown_proptest(x: usize, round in (0u32..16).prop_map(|d| 2usize.pow(d))) {
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
