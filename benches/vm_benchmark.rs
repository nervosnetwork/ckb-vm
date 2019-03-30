#[macro_use]
extern crate criterion;

use ckb_vm::{run, SparseMemory};
use criterion::Criterion;
use std::fs::File;
use std::io::Read;

fn interpret_benchmark(c: &mut Criterion) {
    c.bench_function("interpret secp256k1_bench", |b| {
        let mut file = File::open("benches/data/secp256k1_bench").unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let args: Vec<Vec<u8>> = vec!["secp256k1_bench",
                                      "033f8cf9c4d51a33206a6c1c6b27d2cc5129daa19dbd1fc148d395284f6b26411f",
                                      "304402203679d909f43f073c7c1dcf8468a485090589079ee834e6eed92fea9b09b06a2402201e46f1075afa18f306715e7db87493e7b7e779569aa13c64ab3d09980b3560a3",
                                      "foo",
                                      "bar"].into_iter().map(|a| a.into()).collect();

        b.iter(|| run::<u64, SparseMemory<u64>>(&buffer, &args).unwrap());
    });
}

criterion_group!(benches, interpret_benchmark);
criterion_main!(benches);
