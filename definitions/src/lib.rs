pub mod asm;
pub mod instructions;
pub mod memory;
pub mod registers;

pub const RISCV_PAGE_SHIFTS: usize = 12;
pub const RISCV_PAGESIZE: usize = 1 << RISCV_PAGE_SHIFTS;
pub const RISCV_GENERAL_REGISTER_NUMBER: usize = 32;
// 4 MB
pub const RISCV_MAX_MEMORY: usize = 4 << 20;
// 1 MB
pub const DEFAULT_STACK_SIZE: usize = 1 << 20;
pub const RISCV_PAGES: usize = RISCV_MAX_MEMORY / RISCV_PAGESIZE;
// 256 KB
pub const MEMORY_FRAME_SHIFTS: usize = 18;
pub const MEMORY_FRAMESIZE: usize = 1 << MEMORY_FRAME_SHIFTS;
pub const MEMORY_FRAMES: usize = RISCV_MAX_MEMORY / MEMORY_FRAMESIZE;
pub const MEMORY_FRAME_PAGE_SHIFTS: usize = MEMORY_FRAME_SHIFTS - RISCV_PAGE_SHIFTS;

pub const ISA_IMC: u8 = 0b0000_0000;
pub const ISA_B: u8 = 0b0000_0001;
pub const ISA_MOP: u8 = 0b0000_0010;
pub const ISA_V: u8 = 0b0000_0100;

pub const VLEN: usize = 2048;
pub const ELEN: usize = 1024;
