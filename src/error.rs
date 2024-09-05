#[derive(Debug, PartialEq, Clone, Eq, Display)]
pub enum Error {
    #[display("asm error: {_0}")]
    Asm(u8),
    #[display("cycles error: max cycles exceeded")]
    CyclesExceeded,
    #[display("cycles error: overflow")]
    CyclesOverflow,
    #[display("elf error: bits")]
    ElfBits,
    #[display("elf error: {_0}")]
    ElfParseError(String),
    #[display("elf error: segment is unreadable vaddr=0x{_0:x}")]
    ElfSegmentUnreadable(u64),
    #[display("elf error: segment is writable and executable vaddr=0x{_0:x}")]
    ElfSegmentWritableAndExecutable(u64),
    #[display("elf error: segment addr or size is wrong vaddr=0x{_0:x}")]
    ElfSegmentAddrOrSizeError(u64),
    // When users need to implement traits defined in CKB-VM, they can use
    // this error type to wrap their own errors.
    #[display("external error: {_0}")]
    External(String),
    #[display("invalid syscall {_0}")]
    InvalidEcall(u64),
    #[display("invalid instruction pc=0x{pc:x} instruction=0x{instruction:x}")]
    InvalidInstruction { pc: u64, instruction: u32 },
    #[display("invalid operand {_0}")]
    InvalidOp(u16),
    #[display("invalid version")]
    InvalidVersion,
    #[display("I/O error: {kind:?} {data}")]
    IO {
        kind: std::io::ErrorKind,
        data: String,
    },
    #[display("memory error: out of bound addr=0x{_0:x}, kind={_1:?}")]
    MemOutOfBound(u64, OutOfBoundKind),
    #[display("memory error: out of stack")]
    MemOutOfStack,
    #[display("memory error: unaligned page access addr=0x{_0:x}")]
    MemPageUnalignedAccess(u64),
    #[display("memory error: write on executable page page_index={_0}")]
    MemWriteOnExecutablePage(u64),
    #[display("memory error: write on freezed page page_index={_0}")]
    MemWriteOnFreezedPage(u64),
    #[display("pause")]
    Pause,
    #[display("snapshot data load error")]
    SnapshotDataLoadError,
    #[display("unexpected error")]
    Unexpected(String),
    #[display("yield")]
    Yield,
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
