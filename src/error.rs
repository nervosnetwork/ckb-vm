#[derive(Debug, PartialEq, Clone, Eq, Display)]
pub enum Error {
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
    #[display(fmt = "elf error: segment is unreadable vaddr=0x{:x}", "_0")]
    ElfSegmentUnreadable(u64),
    #[display(
        fmt = "elf error: segment is writable and executable vaddr=0x{:x}",
        "_0"
    )]
    ElfSegmentWritableAndExecutable(u64),
    #[display(fmt = "elf error: segment addr or size is wrong vaddr=0x{:x}", "_0")]
    ElfSegmentAddrOrSizeError(u64),
    // External error type is for the debugging tool of CKB-VM, it should not be
    // used in this project.
    #[display(fmt = "external error: {}", "_0")]
    External(String),
    #[display(fmt = "invalid syscall {}", "_0")]
    InvalidEcall(u64),
    #[display(
        fmt = "invalid instruction pc=0x{:x} instruction=0x{:x}",
        "pc",
        "instruction"
    )]
    InvalidInstruction { pc: u64, instruction: u32 },
    #[display(fmt = "invalid operand {}", "_0")]
    InvalidOp(u16),
    #[display(fmt = "invalid version")]
    InvalidVersion,
    #[display(fmt = "I/O error: {:?} {}", "kind", "data")]
    IO {
        kind: std::io::ErrorKind,
        data: String,
    },
    #[display(fmt = "memory error: out of bound addr=0x{:x}, kind={:?}", "_0", "_1")]
    MemOutOfBound(u64, OutOfBoundKind),
    #[display(fmt = "memory error: out of stack")]
    MemOutOfStack,
    #[display(fmt = "memory error: unaligned page access addr=0x{:x}", "_0")]
    MemPageUnalignedAccess(u64),
    #[display(fmt = "memory error: write on executable page page_index={}", "_0")]
    MemWriteOnExecutablePage(u64),
    #[display(fmt = "memory error: write on freezed page page_index={}", "_0")]
    MemWriteOnFreezedPage(u64),
    #[display(fmt = "pause")]
    Pause,
    #[display(fmt = "unexpected error")]
    Unexpected(String),
}

#[derive(Debug, PartialEq, Clone, Eq, Display)]
pub enum OutOfBoundKind {
    Memory,
    ExternalData,
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO {
            kind: error.kind(),
            data: error.to_string(),
        }
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
