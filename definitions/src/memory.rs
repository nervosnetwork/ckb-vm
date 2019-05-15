pub const FLAG_FREEZED: u8 = 0b01;
// CKB VM enforces W^X logic, if this flag is set, current memory page will
// be marked as executable, otherwise the page will be writable.
pub const FLAG_EXECUTABLE: u8 = 0b10;
pub const FLAG_WXORX_BIT: u8 = 0b10;
pub const FLAG_WRITABLE: u8 = (!FLAG_EXECUTABLE) & FLAG_WXORX_BIT;
