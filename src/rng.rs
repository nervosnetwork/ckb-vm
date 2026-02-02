// A simple Linear Congruential Generator (LCG) implementation for filling a byte slice with pseudo-random data. Same
// as the one used by POSIX drand48.
// See: https://www.man7.org/linux/man-pages/man3/lcong48.3.html

const LCG_A: u64 = 0x5DEECE66D;
const LCG_C: u64 = 0xB;
const LCG_M: u64 = 1u64 << 48;

/// Fills data with pseudo-random bytes using a Linear Congruential Generator (LCG) seeded with seed.
pub fn fill(seed: u64, data: &mut [u8]) -> u64 {
    let mut seed = seed;
    for byte in data.iter_mut() {
        seed = (LCG_A.wrapping_mul(seed).wrapping_add(LCG_C)) % LCG_M;
        *byte = ((seed >> 40) & 0xFF) as u8;
    }
    seed
}

/// A simple wrapper.
pub struct Rand {
    seed: u64,
}

impl Rand {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    pub fn fill(&mut self, data: &mut [u8]) {
        self.seed = fill(self.seed, data);
    }
}
