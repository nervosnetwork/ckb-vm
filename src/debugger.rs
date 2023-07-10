use crate::{machine::SupportMachine, Error};

pub trait Debugger<Mac: SupportMachine> {
    fn initialize(&mut self, machine: &mut Mac) -> Result<(), Error>;
    fn ebreak(&mut self, machine: &mut Mac) -> Result<(), Error>;
}

// For better backward compatibility because existing code uses `.debugger(Box::new(DebuggerType))`.
impl<Mac, T> Debugger<Mac> for Box<T>
where
    Mac: SupportMachine,
    T: Debugger<Mac>,
{
    fn initialize(&mut self, machine: &mut Mac) -> Result<(), Error> {
        (**self).initialize(machine)
    }
    fn ebreak(&mut self, machine: &mut Mac) -> Result<(), Error> {
        (**self).ebreak(machine)
    }
}
