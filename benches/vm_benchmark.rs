#[macro_use]
extern crate criterion;

use bytes::Bytes;
#[cfg(has_asm)]
use ckb_vm::{
    machine::{
        aot::AotCompilingMachine,
        asm::{AsmCoreMachine, AsmMachine},
        DefaultMachineBuilder, VERSION0, VERSION1,
    },
    ISA_B, ISA_IMC, ISA_MOP,
};
use ckb_vm::{run, SparseMemory};
use criterion::Criterion;
use std::fs;

fn interpret_benchmark(c: &mut Criterion) {
    c.bench_function("interpret secp256k1_bench", |b| {
        let buffer = fs::read("benches/data/secp256k1_bench").unwrap().into();
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
        let buffer = fs::read("benches/data/secp256k1_bench").unwrap().into();
        let args: Vec<Bytes> = vec!["secp256k1_bench",
                                      "033f8cf9c4d51a33206a6c1c6b27d2cc5129daa19dbd1fc148d395284f6b26411f",
                                      "304402203679d909f43f073c7c1dcf8468a485090589079ee834e6eed92fea9b09b06a2402201e46f1075afa18f306715e7db87493e7b7e779569aa13c64ab3d09980b3560a3",
                                      "foo",
                                      "bar"].into_iter().map(|a| a.into()).collect();

        b.iter(|| {
            let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
            let core = DefaultMachineBuilder::new(asm_core).build();
            let mut machine = AsmMachine::new(core, None);
            machine.load_program(&buffer, &args[..]).unwrap();
            machine.run().unwrap()
        });
    });
}

#[cfg(has_asm)]
fn aot_benchmark(c: &mut Criterion) {
    c.bench_function("aot secp256k1_bench", |b| {
        let buffer: Bytes = fs::read("benches/data/secp256k1_bench").unwrap().into();
        let args: Vec<Bytes> = vec!["secp256k1_bench",
                                      "033f8cf9c4d51a33206a6c1c6b27d2cc5129daa19dbd1fc148d395284f6b26411f",
                                      "304402203679d909f43f073c7c1dcf8468a485090589079ee834e6eed92fea9b09b06a2402201e46f1075afa18f306715e7db87493e7b7e779569aa13c64ab3d09980b3560a3",
                                      "foo",
                                      "bar"].into_iter().map(|a| a.into()).collect();
        let mut aot_machine = AotCompilingMachine::load(&buffer.clone(), None, ISA_IMC, VERSION0).unwrap();
        let result = aot_machine.compile().unwrap();

        b.iter(|| {
            let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
            let core = DefaultMachineBuilder::new(asm_core).build();
            let mut machine = AsmMachine::new(core, Some(&result));
            machine.load_program(&buffer, &args[..]).unwrap();
            machine.run().unwrap()
        });
    });
}

#[cfg(has_asm)]
fn aot_compiling_benchmark(c: &mut Criterion) {
    c.bench_function("compiling secp256k1_bench for aot", |b| {
        let buffer: Bytes = fs::read("benches/data/secp256k1_bench").unwrap().into();
        b.iter(|| {
            AotCompilingMachine::load(&buffer.clone(), None, ISA_IMC, VERSION0)
                .unwrap()
                .compile()
                .unwrap()
        });
    });
}

#[cfg(has_asm)]
fn mop_benchmark(c: &mut Criterion) {
    c.bench_function("interpret secp256k1_bench via assembly mop", |b| {
        let buffer = fs::read("benches/data/secp256k1_bench").unwrap().into();
        let args: Vec<Bytes> = vec!["secp256k1_bench",
                                      "033f8cf9c4d51a33206a6c1c6b27d2cc5129daa19dbd1fc148d395284f6b26411f",
                                      "304402203679d909f43f073c7c1dcf8468a485090589079ee834e6eed92fea9b09b06a2402201e46f1075afa18f306715e7db87493e7b7e779569aa13c64ab3d09980b3560a3",
                                      "foo",
                                      "bar"].into_iter().map(|a| a.into()).collect();
        b.iter(|| {
            let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_B | ISA_MOP, VERSION1, u64::max_value());
            let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
                .build();
            let mut machine = AsmMachine::new(core, None);
            machine.load_program(&buffer, &args).unwrap();
            machine.run().unwrap()
        });
    });
}

#[cfg(not(has_asm))]
criterion_group!(benches, interpret_benchmark);

#[cfg(has_asm)]
criterion_group!(
    benches,
    interpret_benchmark,
    asm_benchmark,
    aot_benchmark,
    aot_compiling_benchmark,
    mop_benchmark,
);
criterion_main!(benches);
