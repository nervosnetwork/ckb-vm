mod emitter;
mod instructions;
mod machine;
mod tracer;
mod value;

use bytes::Bytes;

pub use self::{
    machine::{BaselineJitMachine, BaselineJitRunData},
    tracer::{DefaultTracer, TcgTracer},
};

pub fn default_jit_machine(program: &Bytes) -> BaselineJitMachine {
    BaselineJitMachine::new(program.clone(), Box::new(DefaultTracer::default()))
}
