use super::Error;
use crate::machine::SupportMachine;

pub trait Syscalls<Mac: SupportMachine> {
    fn initialize(&mut self, machine: &mut Mac) -> Result<(), Error>;
    // Returned bool means if the syscall has been processed, if
    // a module returns false, Machine would continue to leverage
    // the next syscall module to process.
    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, Error>;
}
