use std::error::Error as StdError;
use std::io::{Error as IOError, ErrorKind};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Display)]
pub enum Error {
    #[display(fmt = "parse error")]
    ParseError,
    #[display(fmt = "unaligned page access")]
    Unaligned,
    #[display(fmt = "out of bound access")]
    OutOfBound,
    #[display(fmt = "max cycles exceeded")]
    InvalidCycles,
    #[display(fmt = "invalid instruction {}", "_0")]
    InvalidInstruction(u32),
    #[display(fmt = "invalid syscall {}", "_0")]
    InvalidEcall(u64),
    #[display(fmt = "invalid elf")]
    InvalidElfBits,
    #[display(fmt = "invalid operand {}", "_0")]
    InvalidOp(u16),
    #[display(fmt = "I/O error: {:?}", "_0")]
    IO(ErrorKind),
    #[display(fmt = "dynasm error {}", "_0")]
    Dynasm(i32),
    #[display(fmt = "assembly error {}", "_0")]
    Asm(u8),
    #[display(fmt = "limit reached")] // FIXME: Distinguish which limit
    LimitReached,
    #[display(fmt = "invalid permission")] // FIXME: Distinguish which permission
    InvalidPermission,
    #[display(fmt = "invalid version")]
    InvalidVersion,
    #[display(fmt = "unexpected error")]
    Unexpected,
    #[display(fmt = "unimplemented")]
    Unimplemented,
}

impl StdError for Error {}

impl From<IOError> for Error {
    fn from(error: IOError) -> Self {
        Error::IO(error.kind())
    }
}
