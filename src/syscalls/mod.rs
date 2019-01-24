use super::machine::SupportMachine;
use super::Error;
pub trait Syscalls<R, M> {
    fn initialize(&mut self, machine: &mut SupportMachine<REG = R, MEM = M>) -> Result<(), Error>;
    // Returned bool means if the syscall has been processed, if
    // a module returns false, Machine would continue to leverage
    // the next syscall module to process.
    fn ecall(&mut self, machine: &mut SupportMachine<REG = R, MEM = M>) -> Result<bool, Error>;
}
