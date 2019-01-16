use super::instructions::Register;
use super::machine::SupportMachine;
use super::memory::Memory;
use super::Error;
pub trait Syscalls<R: Register, M: Memory> {
    fn initialize(&mut self, machine: &mut SupportMachine<R, M>) -> Result<(), Error>;
    // Returned bool means if the syscall has been processed, if
    // a module returns false, Machine would continue to leverage
    // the next syscall module to process.
    fn ecall(&mut self, machine: &mut SupportMachine<R, M>) -> Result<bool, Error>;
}
