#[macro_use]
extern crate criterion;

use ckb_vm::bits;
use criterion::Criterion;

#[inline(always)]
pub fn roundup_via_remainder(x: usize, round: usize) -> usize {
    let remainder = x % round;
    if remainder > 0 {
        x - remainder + round
    } else {
        x - remainder
    }
}

#[inline(always)]
pub fn rounddown_via_remainder(x: usize, round: usize) -> usize {
    let remainder = x % round;
    x - remainder
}

#[inline(always)]
pub fn roundup_via_multiplication(x: usize, round: usize) -> usize {
    if x == 0 {
        0
    } else {
        ((x - 1) / round + 1) * round
    }
}

#[inline(always)]
pub fn rounddown_via_multiplication(x: usize, round: usize) -> usize {
    x / round * round
}

const ROUNDS: &[usize] = &[1, 2, 4, 8, 16, 32];

macro_rules! round_bench {
    ($f:expr) => {
        (0..9999).for_each(|x| {
            (&ROUNDS).iter().for_each(|round| {
                $f(x, *round);
            })
        })
    };
}

fn roundup_benchmark(c: &mut Criterion) {
    c.bench_function("rounup via remainder", |b| {
        b.iter(|| round_bench!(roundup_via_remainder))
    });
    c.bench_function("rounup via bit ops", |b| {
        b.iter(|| round_bench!(bits::roundup))
    });
    c.bench_function("rounup via multication", |b| {
        b.iter(|| round_bench!(roundup_via_multiplication))
    });
}

fn rounddown_benchmark(c: &mut Criterion) {
    c.bench_function("rounup via remainder", |b| {
        b.iter(|| round_bench!(rounddown_via_remainder))
    });
    c.bench_function("rounup via bit ops", |b| {
        b.iter(|| round_bench!(bits::rounddown))
    });
    c.bench_function("rounup via multication", |b| {
        b.iter(|| round_bench!(rounddown_via_multiplication))
    });
}

criterion_group!(benches, roundup_benchmark, rounddown_benchmark);
criterion_main!(benches);
