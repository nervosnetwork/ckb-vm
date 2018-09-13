#[inline(always)]
pub fn roundup(x: usize, round: usize) -> usize {
    ((x - 1) / round + 1) * round
}

#[inline(always)]
pub fn rounddown(x: usize, round: usize) -> usize {
    x / round * round
}
