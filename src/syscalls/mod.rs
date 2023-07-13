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
