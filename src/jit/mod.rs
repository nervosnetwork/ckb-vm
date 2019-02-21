mod emitter;
mod instructions;
mod machine;
mod tracer;
mod value;

pub use self::{
    machine::{BaselineJitMachine, BaselineJitRunData},
    tracer::{DefaultTracer, TcgTracer},
};

pub fn default_jit_machine(program: &[u8]) -> BaselineJitMachine {
    BaselineJitMachine::new(program, Box::new(DefaultTracer::default()))
}
