#[macro_use]
extern crate criterion;

use ckb_vm::{
    machine::VERSION0, memory::wxorx::WXorXMemory, Bytes, DefaultCoreMachine,
    DefaultMachineBuilder, SparseMemory, TraceMachine, ISA_IMC,
};
#[cfg(has_asm)]
use ckb_vm::{
    machine::{
        aot::AotCompilingMachine,
        asm::{AsmCoreMachine, AsmMachine, AsmWrapMachine},
        VERSION1,
    },
    ISA_B, ISA_MOP,
};
use criterion::Criterion;

fn interpret_benchmark(c: &mut Criterion) {
    c.bench_function("interpret secp256k1_bench", |b| {
        let path = "benches/data/secp256k1_bench";
        let code = std::fs::read(path).unwrap().into();
        let args: Vec<Bytes> = vec!["secp256k1_bench",
                                      "033f8cf9c4d51a33206a6c1c6b27d2cc5129daa19dbd1fc148d395284f6b26411f",
                                      "304402203679d909f43f073c7c1dcf8468a485090589079ee834e6eed92fea9b09b06a2402201e46f1075afa18f306715e7db87493e7b7e779569aa13c64ab3d09980b3560a3",
                                      "foo",
                                      "bar"].into_iter().map(|a| a.into()).collect();
        b.iter(|| {
            let core_machine = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
                ISA_IMC,
                VERSION0,
                u64::max_value(),
            );
            let mut machine = TraceMachine::new(
                DefaultMachineBuilder::new(core_machine)
                    .build(),
            );
            machine
                .load_program(&code, &args[..])
                .unwrap();
            machine.run().unwrap();
        });
    });
}

#[cfg(has_asm)]
fn asm_benchmark(c: &mut Criterion) {
    c.bench_function("interpret secp256k1_bench via assembly", |b| {
        let path = "benches/data/secp256k1_bench";
        let code = std::fs::read(path).unwrap().into();
        let args: Vec<Bytes> = vec!["secp256k1_bench",
                                      "033f8cf9c4d51a33206a6c1c6b27d2cc5129daa19dbd1fc148d395284f6b26411f",
                                      "304402203679d909f43f073c7c1dcf8468a485090589079ee834e6eed92fea9b09b06a2402201e46f1075afa18f306715e7db87493e7b7e779569aa13c64ab3d09980b3560a3",
                                      "foo",
                                      "bar"].into_iter().map(|a| a.into()).collect();
        b.iter(|| {
            let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
            let asm_wrap = AsmWrapMachine::new(asm_core, false);
            let core = DefaultMachineBuilder::new(asm_wrap).build();
            let mut machine = AsmMachine::new(core);
            machine.load_program(&code, &args[..]).unwrap();
            machine.run().unwrap()
        });
    });
}

#[cfg(has_asm)]
fn aot_benchmark(c: &mut Criterion) {
    c.bench_function("aot secp256k1_bench", |b| {
        let path = "benches/data/secp256k1_bench";
        let code = std::fs::read(path).unwrap().into();
        let args: Vec<Bytes> = vec!["secp256k1_bench",
                                      "033f8cf9c4d51a33206a6c1c6b27d2cc5129daa19dbd1fc148d395284f6b26411f",
                                      "304402203679d909f43f073c7c1dcf8468a485090589079ee834e6eed92fea9b09b06a2402201e46f1075afa18f306715e7db87493e7b7e779569aa13c64ab3d09980b3560a3",
                                      "foo",
                                      "bar"].into_iter().map(|a| a.into()).collect();
        b.iter(|| {
            let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
            let asm_wrap = AsmWrapMachine::new(asm_core, true);
            let core = DefaultMachineBuilder::new(asm_wrap).build();
            let mut machine = AsmMachine::new(core);
            machine.load_program(&code, &args[..]).unwrap();
            machine.run().unwrap()
        });
    });
}

#[cfg(has_asm)]
fn aot_compiling_benchmark(c: &mut Criterion) {
    c.bench_function("compiling secp256k1_bench for aot", |b| {
        let path = "benches/data/secp256k1_bench";
        let code: Bytes = std::fs::read(path).unwrap().into();
        b.iter(|| {
            AotCompilingMachine::load(&code, ISA_IMC, VERSION0)
                .unwrap()
                .compile(&None)
                .unwrap()
        });
    });
}

#[cfg(has_asm)]
fn mop_benchmark(c: &mut Criterion) {
    c.bench_function("interpret secp256k1_bench via assembly mop", |b| {
        let path = "benches/data/secp256k1_bench";
        let code = std::fs::read(path).unwrap().into();
        let args: Vec<Bytes> = vec!["secp256k1_bench",
                                      "033f8cf9c4d51a33206a6c1c6b27d2cc5129daa19dbd1fc148d395284f6b26411f",
                                      "304402203679d909f43f073c7c1dcf8468a485090589079ee834e6eed92fea9b09b06a2402201e46f1075afa18f306715e7db87493e7b7e779569aa13c64ab3d09980b3560a3",
                                      "foo",
                                      "bar"].into_iter().map(|a| a.into()).collect();
        b.iter(|| {
            let asm_core = AsmCoreMachine::new(ISA_IMC | ISA_B | ISA_MOP, VERSION1, u64::max_value());
            let asm_wrap = AsmWrapMachine::new(asm_core, false);
            let core = DefaultMachineBuilder::new(asm_wrap).build();
            let mut machine = AsmMachine::new(core);
            machine.load_program(&code, &args).unwrap();
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
