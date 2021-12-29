#[derive(Debug, PartialEq, Clone, Eq, Display)]
pub enum Error {
    #[display(fmt = "aot error: dynasm ret {}", "_0")]
    AotDynasm(i32),
    #[display(fmt = "aot error: limit reached maximum dummy sections")]
    AotLimitReachedMaximumDummySections,
    #[display(fmt = "aot error: limit reached maximum labels")]
    AotLimitReachedMaximumLabels,
    #[display(fmt = "aot error: limit reached maximum sections")]
    AotLimitReachedMaximumSections,
    #[display(fmt = "asm error: {}", "_0")]
    Asm(u8),
    #[display(fmt = "cycles error: max cycles exceeded")]
    CyclesExceeded,
    #[display(fmt = "cycles error: overflow")]
    CyclesOverflow,
    #[display(fmt = "elf error: bits")]
    ElfBits,
    #[display(fmt = "elf error: {}", "_0")]
    ElfParseError(String),
    #[display(fmt = "elf error: segments is unreadable")]
    ElfSegmentUnreadable,
    #[display(fmt = "elf error: segments is writable and executable")]
    ElfSegmentWritableAndExecutable,
    #[display(fmt = "invalid syscall {}", "_0")]
    InvalidEcall(u64),
    #[display(
        fmt = "invalid instruction pc=0x{:x} instruction=0x{:x}",
        "pc",
        "instruction"
    )]
    InvalidInstruction { pc: u64, instruction: u32 },
    #[display(fmt = "I/O error: {:?}", "_0")]
    IO(std::io::ErrorKind),
    #[display(fmt = "memory error: unaligned page access")]
    MemPageUnalignedAccess,

    #[display(fmt = "out of bound access")]
    OutOfBound,
    #[display(fmt = "invalid operand {}", "_0")]
    InvalidOp(u16),
    #[display(fmt = "invalid permission")] // FIXME: Distinguish which permission
    InvalidPermission,
    #[display(fmt = "invalid version")]
    InvalidVersion,
    #[display(fmt = "unexpected error")]
    Unexpected,
    #[display(fmt = "unimplemented")]
    Unimplemented,
    // Unknown error type is for the debugging tool of CKB-VM, it should not be
    // used in this project.
    #[display(fmt = "external error: {}", "_0")]
    External(String),
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error.kind())
    }
}

impl From<goblin_v023::error::Error> for Error {
    fn from(error: goblin_v023::error::Error) -> Self {
        Error::ElfParseError(error.to_string())
    }
}

impl From<goblin_v040::error::Error> for Error {
    fn from(error: goblin_v040::error::Error) -> Self {
        Error::ElfParseError(error.to_string())
    }
}
