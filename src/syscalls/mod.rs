use super::Error;
use crate::machine::SupportMachine;

/// System call handler.
pub trait Syscalls<Mac: SupportMachine> {
    fn initialize(&mut self, machine: &mut Mac) -> Result<(), Error>;
    // Returned bool means if the syscall has been processed, if
    // a module returns false, Machine would continue to leverage
    // the next syscall module to process.
    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, Error>;
}

// For better backward compatibility because existing code uses `.syscall(Box::new(SyscallType))`.
impl<Mac, T> Syscalls<Mac> for Box<T>
where
    Mac: SupportMachine,
    T: Syscalls<Mac>,
{
    fn initialize(&mut self, machine: &mut Mac) -> Result<(), Error> {
        (**self).initialize(machine)
    }
    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, Error> {
        (**self).ecall(machine)
    }
}

// For better compatibility with existing code that uses Box<dyn Syscalls> or Vec<Box<dyn Syscalls>>.

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
                return Ok(processed);
            }
        }
        Ok(false)
    }
}
