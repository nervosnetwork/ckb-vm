#[macro_use]
extern crate criterion;

use bytes::Bytes;
#[cfg(has_asm)]
use ckb_vm::machine::{aot::AotCompilingMachine, asm::AsmMachine, VERSION0};
use ckb_vm::{run, SparseMemory};
use criterion::Criterion;
use std::fs::File;
use std::io::Read;

fn interpret_benchmark(c: &mut Criterion) {
    c.bench_function("interpret secp256k1_bench", |b| {
        let mut file = File::open("benches/data/secp256k1_bench").unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let buffer = Bytes::from(buffer);
        let args: Vec<Bytes> = vec!["secp256k1_bench",
                                      "033f8cf9c4d51a33206a6c1c6b27d2cc5129daa19dbd1fc148d395284f6b26411f",
                                      "304402203679d909f43f073c7c1dcf8468a485090589079ee834e6eed92fea9b09b06a2402201e46f1075afa18f306715e7db87493e7b7e779569aa13c64ab3d09980b3560a3",
                                      "foo",
                                      "bar"].into_iter().map(|a| a.into()).collect();

        b.iter(|| run::<u64, SparseMemory<u64>>(&buffer, &args[..]).unwrap());
    });
}

#[cfg(has_asm)]
fn asm_benchmark(c: &mut Criterion) {
    c.bench_function("interpret secp256k1_bench via assembly", |b| {
        let mut file = File::open("benches/data/secp256k1_bench").unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let buffer = Bytes::from(buffer);
        let args: Vec<Bytes> = vec!["secp256k1_bench",
                                      "033f8cf9c4d51a33206a6c1c6b27d2cc5129daa19dbd1fc148d395284f6b26411f",
                                      "304402203679d909f43f073c7c1dcf8468a485090589079ee834e6eed92fea9b09b06a2402201e46f1075afa18f306715e7db87493e7b7e779569aa13c64ab3d09980b3560a3",
                                      "foo",
                                      "bar"].into_iter().map(|a| a.into()).collect();

        b.iter(|| {
            let mut machine = AsmMachine::default();
            machine.load_program(&buffer, &args[..]).unwrap();
            machine.run().unwrap()
        });
    });
}

#[cfg(has_asm)]
fn aot_benchmark(c: &mut Criterion) {
    c.bench_function("aot secp256k1_bench", |b| {
        let mut file = File::open("benches/data/secp256k1_bench").unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let buffer = Bytes::from(buffer);
        let args: Vec<Bytes> = vec!["secp256k1_bench",
                                      "033f8cf9c4d51a33206a6c1c6b27d2cc5129daa19dbd1fc148d395284f6b26411f",
                                      "304402203679d909f43f073c7c1dcf8468a485090589079ee834e6eed92fea9b09b06a2402201e46f1075afa18f306715e7db87493e7b7e779569aa13c64ab3d09980b3560a3",
                                      "foo",
                                      "bar"].into_iter().map(|a| a.into()).collect();
        let mut aot_machine = AotCompilingMachine::load(&buffer.clone(), None, VERSION0).unwrap();
        let result = aot_machine.compile().unwrap();

        b.iter(|| {
            let mut machine = AsmMachine::default_with_aot_code(&result);
            machine.load_program(&buffer, &args[..]).unwrap();
            machine.run().unwrap()
        });
    });
}

#[cfg(has_asm)]
fn aot_compiling_benchmark(c: &mut Criterion) {
    c.bench_function("compiling secp256k1_bench for aot", |b| {
        let mut file = File::open("benches/data/secp256k1_bench").unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let buffer = Bytes::from(buffer);

        b.iter(|| {
            AotCompilingMachine::load(&buffer.clone(), None, VERSION0)
                .unwrap()
                .compile()
                .unwrap()
        });
    });
}

#[cfg(not(has_asm))]
criterion_group!(benches, interpret_benchmark,);

#[cfg(has_asm)]
criterion_group!(
    benches,
    interpret_benchmark,
    asm_benchmark,
    aot_benchmark,
    aot_compiling_benchmark
);
criterion_main!(benches);
