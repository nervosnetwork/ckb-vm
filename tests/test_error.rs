use ckb_vm::error::Error;

#[test]
pub fn test_error() {
    assert_eq!(Error::Asm(0).to_string(), "asm error: 0");
    assert_eq!(
        Error::ElfParseError(String::from("abcd")).to_string(),
        "elf error: abcd"
    );
    assert_eq!(
        Error::ElfSegmentUnreadable(0).to_string(),
        "elf error: segment is unreadable vaddr=0x0"
    );
    assert_eq!(
        Error::InvalidInstruction {
            pc: 0,
            instruction: 1
        }
        .to_string(),
        "invalid instruction pc=0x0 instruction=0x1"
    );
    assert_eq!(
        Error::IO {
            kind: std::io::ErrorKind::AddrInUse,
            data: String::from("abcd")
        }
        .to_string(),
        "I/O error: AddrInUse abcd"
    );
}
