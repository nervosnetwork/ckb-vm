use super::Error;
use crate::machine::SupportMachine;

pub trait Syscalls<Mac: SupportMachine> {
    fn initialize(&mut self, machine: &mut Mac) -> Result<(), Error>;
    /// Returned bool means if the syscall has been processed, if
    /// a module returns false, Machine would continue to leverage
    /// the next syscall module to process.
    ///
    /// See Syscalls impl for Vec<BoxedSyscalls>.
    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, Error>;
}

/// No syscalls.
impl<Mac: SupportMachine> Syscalls<Mac> for () {
    /// No-op.
    fn initialize(&mut self, _machine: &mut Mac) -> Result<(), Error> {
        Ok(())
    }
    /// Always return Ok(false).
    fn ecall(&mut self, _machine: &mut Mac) -> Result<bool, Error> {
        Ok(false)
    }
}

/// When initialization is not necessary, you can use a simple closure to handle syscalls.
impl<Mac, F> Syscalls<Mac> for F
where
    Mac: SupportMachine,
    F: FnMut(&mut Mac) -> Result<bool, Error>,
{
    fn initialize(&mut self, _machine: &mut Mac) -> Result<(), Error> {
        Ok(())
    }
    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, Error> {
        self(machine)
    }
}

pub type BoxedSyscalls<Mac> = Box<dyn Syscalls<Mac> + Send + Sync + 'static>;

impl<Mac: SupportMachine> Syscalls<Mac> for BoxedSyscalls<Mac> {
    fn initialize(&mut self, machine: &mut Mac) -> Result<(), Error> {
        (**self).initialize(machine)
    }
    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, Error> {
        (**self).ecall(machine)
    }
}

impl<Mac: SupportMachine> Syscalls<Mac> for Vec<BoxedSyscalls<Mac>> {
    fn initialize(&mut self, machine: &mut Mac) -> Result<(), Error> {
        for s in self {
            s.initialize(machine)?;
        }
        Ok(())
    }
    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, Error> {
        for s in self {
            let processed = s.ecall(machine)?;
            if processed {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
