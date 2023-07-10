use ckb_vm_definitions::instructions::Instruction;

use super::Error;
use crate::{machine::SupportMachine, syscalls::Syscalls, Debugger};

pub trait ExecutionContext<Mac: SupportMachine> {
    fn initialize(&mut self, machine: &mut Mac) -> Result<(), Error> {
        // We don't want to change the param name to start with an
        // underscore(_). It doesn't look good in docs. Also when someone
        // implements this method with IDE completion, they will need to remove
        // the staring underscore.
        let _ = machine;
        Ok(())
    }
    /// Return true if the syscall has been processed. If a module returns
    /// false, Machine would continue to leverage the next syscall module to
    /// process.
    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, Error> {
        let _ = machine;
        Ok(false)
    }
    fn ebreak(&mut self, machine: &mut Mac) -> Result<(), Error> {
        let _ = machine;
        Ok(())
    }
    fn instruction_cycles(&self, inst: Instruction) -> u64 {
        let _ = inst;
        0
    }
}

pub type BoxedExecutionContext<Mac> = Box<dyn ExecutionContext<Mac> + Send + Sync + 'static>;

impl<Mac: SupportMachine> ExecutionContext<Mac> for BoxedExecutionContext<Mac> {
    fn initialize(&mut self, machine: &mut Mac) -> Result<(), Error> {
        (**self).initialize(machine)
    }
    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, Error> {
        (**self).ecall(machine)
    }
    fn ebreak(&mut self, machine: &mut Mac) -> Result<(), Error> {
        (**self).ebreak(machine)
    }
    fn instruction_cycles(&self, inst: Instruction) -> u64 {
        (**self).instruction_cycles(inst)
    }
}

impl<Mac: SupportMachine> ExecutionContext<Mac> for () {}

pub struct WithSyscall<Ctx, Sys> {
    pub(super) base: Ctx,
    pub(super) syscall: Sys,
}

impl<Ctx, Sys, Mac> ExecutionContext<Mac> for WithSyscall<Ctx, Sys>
where
    Mac: SupportMachine,
    Ctx: ExecutionContext<Mac>,
    Sys: Syscalls<Mac>,
{
    fn initialize(&mut self, machine: &mut Mac) -> Result<(), Error> {
        self.base.initialize(machine)?;
        self.syscall.initialize(machine)
    }
    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, Error> {
        let processed = self.base.ecall(machine)?;
        if processed {
            return Ok(processed);
        }
        self.syscall.ecall(machine)
    }
    fn ebreak(&mut self, machine: &mut Mac) -> Result<(), Error> {
        self.base.ebreak(machine)
    }
    fn instruction_cycles(&self, inst: Instruction) -> u64 {
        self.base.instruction_cycles(inst)
    }
}

pub struct WithDebugger<Ctx, Dbg> {
    pub(super) base: Ctx,
    pub(super) debugger: Dbg,
}

impl<Ctx, Dbg, Mac> ExecutionContext<Mac> for WithDebugger<Ctx, Dbg>
where
    Mac: SupportMachine,
    Ctx: ExecutionContext<Mac>,
    Dbg: Debugger<Mac>,
{
    fn initialize(&mut self, machine: &mut Mac) -> Result<(), Error> {
        self.base.initialize(machine)?;
        self.debugger.initialize(machine)
    }
    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, Error> {
        self.base.ecall(machine)
    }
    fn ebreak(&mut self, machine: &mut Mac) -> Result<(), Error> {
        self.debugger.ebreak(machine)
    }
    fn instruction_cycles(&self, inst: Instruction) -> u64 {
        self.base.instruction_cycles(inst)
    }
}

pub struct WithCyclesFunc<Ctx, F> {
    pub(super) base: Ctx,
    pub(super) cycles: F,
}

impl<Ctx, F, Mac> ExecutionContext<Mac> for WithCyclesFunc<Ctx, F>
where
    Mac: SupportMachine,
    Ctx: ExecutionContext<Mac>,
    F: Fn(Instruction) -> u64,
{
    fn initialize(&mut self, machine: &mut Mac) -> Result<(), Error> {
        self.base.initialize(machine)
    }
    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, Error> {
        self.base.ecall(machine)
    }
    fn ebreak(&mut self, machine: &mut Mac) -> Result<(), Error> {
        self.base.ebreak(machine)
    }
    fn instruction_cycles(&self, inst: Instruction) -> u64 {
        (self.cycles)(inst)
    }
}

/// ExecutionContext composing.
pub trait ExecutionContextExt<Mac: SupportMachine>: ExecutionContext<Mac> {
    /// Add a syscall handler to the this context.
    fn with_syscall<Sys>(self, syscall: Sys) -> WithSyscall<Self, Sys>
    where
        Self: Sized,
        Sys: Syscalls<Mac>,
    {
        WithSyscall {
            base: self,
            syscall,
        }
    }

    /// Replace the debugger.
    fn with_debugger<Dbg>(self, debugger: Dbg) -> WithDebugger<Self, Dbg>
    where
        Self: Sized,
        Dbg: Debugger<Mac>,
    {
        WithDebugger {
            base: self,
            debugger,
        }
    }

    /// Replace the instruction cycles function.
    fn with_cycles<F>(self, cycles: F) -> WithCyclesFunc<Self, F>
    where
        Self: Sized,
        F: Fn(Instruction) -> u64,
    {
        WithCyclesFunc { base: self, cycles }
    }

    /// Convert the execution context to be boxed and type erased.
    fn boxed(self) -> BoxedExecutionContext<Mac>
    where
        Self: Sized + Send + Sync + 'static,
    {
        Box::new(self)
    }
}

impl<Mac: SupportMachine, T: ExecutionContext<Mac>> ExecutionContextExt<Mac> for T {}
