// Source: https://en.wikipedia.org/wiki/Bit_manipulation#Example_of_bit_manipulation
#[inline(always)]
fn power_of_2(x: usize) -> bool {
    x > 0 && ((x & (x - 1)) == 0)
}

#[inline(always)]
pub fn roundup(x: usize, round: usize) -> usize {
    debug_assert!(power_of_2(round));
    if x == 0 {
        0
    } else {
        ((x - 1) / round + 1) * round
    }
}

#[inline(always)]
pub fn rounddown(x: usize, round: usize) -> usize {
    debug_assert!(power_of_2(round));
    x / round * round
}

#[cfg(test)]
mod tests {
    use super::*;

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
    }

    #[test]
    fn test_rounddown() {
        assert_eq!(0, rounddown(0, 16));
        assert_eq!(0, rounddown(1, 16));
        assert_eq!(0, rounddown(15, 16));
        assert_eq!(16, rounddown(16, 16));
        assert_eq!(16, rounddown(17, 16));
    }
}
