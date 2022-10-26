use crate::{machine::SupportMachine, Error};

pub trait Debugger<Mac: SupportMachine>: Send + Sync {
    fn initialize(&mut self, machine: &mut Mac) -> Result<(), Error>;
    fn ebreak(&mut self, machine: &mut Mac) -> Result<(), Error>;
}
